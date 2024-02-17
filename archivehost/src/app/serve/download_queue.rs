use std::sync::Arc;

use sea_orm::{ActiveModelTrait as _, Set};
use sea_orm::{ColumnTrait as _, EntityTrait, QueryFilter as _};
use tokio::sync::Notify;
use tracing::warn;
use tracing::{error, info};

use db::entity::download_queue as q;
use db::entity::download_queue::Status;
use db::entity::download_queue_group as qg;

use crate::common::download::cdx::{get_latest_page_cdx, get_latest_pages_index};
use crate::common::download::{download_and_save_page, DownloadStatus};
use crate::common::timestamp::Timestamp;
use crate::common::wayback_client::{CdxLine, WaybackClient};
use crate::constant::CONN;

pub struct DownloadQueueController {
    // Notify downloader when new tasks are available
    notifier: Arc<Notify>,
    client: Arc<WaybackClient>,
}

pub struct DownloadOption {
    pub mode: DownloadType,
    pub url: String,
    pub from: Option<Timestamp>,
    pub to: Option<Timestamp>,
}
pub enum DownloadType {
    Single,
    Batch,
}
impl From<DownloadType> for db::entity::download_queue_group::DownloadType {
    fn from(val: DownloadType) -> Self {
        match val {
            DownloadType::Single => db::entity::download_queue_group::DownloadType::Single,
            DownloadType::Batch => db::entity::download_queue_group::DownloadType::Batch,
        }
    }
}

async fn execute_download(
    client: &WaybackClient,
    task: &q::Model,
) -> Result<DownloadStatus, eyre::Error> {
    let timestamp = Timestamp::from_unix_time(task.timestamp);
    let status_code = task
        .status_code
        .map(|v| http::StatusCode::from_u16(v as u16))
        .transpose();
    match (timestamp, status_code) {
        (Ok(timestamp), Ok(status_code)) => {
            let res =
                download_and_save_page(client, &task.url, &task.mime, &timestamp, status_code)
                    .await?;
            Ok(res)
        }
        (Err(e), _) => Err(e.wrap_err("Failed to parse timestamp")),
        (_, Err(e)) => Err(eyre::eyre!("Failed to parse status code: {:?}", e)),
    }
}

impl DownloadQueueController {
    pub fn start(concurrency: usize) -> Self {
        let notifier = Arc::new(Notify::new());
        let client = Arc::new(WaybackClient::default());
        {
            let notifier = notifier.clone();
            let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
            let client = client.clone();
            tokio::spawn(async move {
                loop {
                    notifier.notified().await;
                    loop {
                        match q::Entity::find()
                            .filter(q::Column::DownloadStatus.eq(Status::Pending))
                            .find_also_related(qg::Entity)
                            .one(&*CONN)
                            .await
                        {
                            Ok(Some((task, Some(_task_group)))) => {
                                if let Err(e) = q::Entity::update(q::ActiveModel {
                                    id: Set(task.id),
                                    download_status: Set(Status::Downloading),
                                    ..Default::default()
                                })
                                .filter(q::Column::Id.eq(task.id))
                                .filter(q::Column::DownloadStatus.eq(Status::Pending))
                                .exec(&*CONN)
                                .await
                                {
                                    warn!(
                                        "Failed to start download. Maybe the task was already picked up by another worker? Task: {} Error: {:?}",
                                        task.id, e
                                    );
                                    continue;
                                }
                                let permit = semaphore.clone().acquire_owned().await.unwrap();
                                let client = client.clone();
                                tokio::spawn(async move {
                                    info!("Downloading {}", task.url);
                                    let _permit = permit;
                                    let (message, status) = match execute_download(&client, &task)
                                        .await
                                    {
                                        Ok(status) => match status {
                                            DownloadStatus::Done => (None, Status::Success),
                                            DownloadStatus::Skipped(s) => {
                                                (Some(s), Status::Skipped)
                                            }
                                            DownloadStatus::FixDb(s) => (Some(s), Status::Skipped),
                                        },
                                        Err(e) => {
                                            warn!("Failed to download page: {:?}", e);
                                            (Some(e.to_string()), Status::Failed)
                                        }
                                    };

                                    if let Err(e) = q::Entity::update(q::ActiveModel {
                                        id: Set(task.id),
                                        download_status: Set(status),
                                        message: Set(message),
                                        ..Default::default()
                                    })
                                    .filter(q::Column::Id.eq(task.id))
                                    .exec(&*CONN)
                                    .await
                                    {
                                        warn!(
                                            "Failed to update download status for {}: {:?}",
                                            task.id, e
                                        );
                                    }
                                });
                            }
                            Ok(Some((task, None))) => {
                                error!(
                                    "Task {} has no group! This should never occur. Skipping.",
                                    task.id
                                );
                            }
                            Ok(None) => break,
                            Err(_) => {
                                warn!("Failed to query download queue. Retrying in 1s.");
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            }
                        }
                    }
                }
            });
        }
        Self { notifier, client }
    }

    pub fn add_task(&self, opts: DownloadOption) {
        let client = self.client.clone();
        let notifier = self.notifier.clone();
        tokio::spawn(async move {
            let to_download: eyre::Result<Option<Vec<CdxLine>>> = match opts.mode {
                DownloadType::Single => get_latest_page_cdx(
                    &client,
                    opts.url.clone(),
                    opts.from.clone(),
                    opts.to.clone(),
                )
                .await
                .map(|cdx| cdx.map(|c| vec![c])),
                DownloadType::Batch => get_latest_pages_index(
                    &client,
                    opts.url.clone(),
                    opts.from.clone(),
                    opts.to.clone(),
                )
                .await
                .map(|items| {
                    if items.is_empty() {
                        None
                    } else {
                        Some(items.into_values().collect())
                    }
                }),
            };

            let now = uuid::Timestamp::now(uuid::NoContext);
            let group_id = uuid::Uuid::new_v7(now);

            let to_download = match to_download {
                Ok(Some(cdx)) => cdx,
                Ok(None) => {
                    let group = qg::ActiveModel {
                        id: Set(group_id),
                        url: Set(opts.url.clone()),
                        download_type: Set(opts.mode.into()),
                        failed: Set(Some("No pages found".to_string())),
                        from: Set(opts.from.map(|t| t.unix_time())),
                        to: Set(opts.to.map(|t| t.unix_time())),
                    };
                    let _ = group.insert(&*CONN).await.inspect_err(|e| {
                        error!("Failed to insert download group: {:?}", e);
                    });
                    return;
                }
                Err(e) => {
                    error!("Failed to fetch index: {:?}", e);
                    let group = qg::ActiveModel {
                        id: Set(group_id),
                        url: Set(opts.url.clone()),
                        download_type: Set(opts.mode.into()),
                        failed: Set(Some("Failed to fetch index".to_string())),
                        from: Set(opts.from.map(|t| t.unix_time())),
                        to: Set(opts.to.map(|t| t.unix_time())),
                    };
                    let _ = group.insert(&*CONN).await.inspect_err(|e| {
                        error!("Failed to insert download group: {:?}", e);
                    });
                    return;
                }
            };

            let res = qg::ActiveModel {
                id: Set(group_id),
                url: Set(opts.url.clone()),
                download_type: Set(opts.mode.into()),
                failed: Set(None),
                from: Set(opts.from.map(|t| t.unix_time())),
                to: Set(opts.to.map(|t| t.unix_time())),
            }
            .insert(&*CONN)
            .await;
            if let Err(e) = res {
                error!("Failed to insert download group: {:?}", e);
                return;
            }

            let inserts = to_download.into_iter().map(|cdx| q::ActiveModel {
                id: Set(uuid::Uuid::new_v7(now)),
                group_id: Set(group_id),
                url: Set(cdx.original),
                download_status: Set(Status::Pending),
                message: Set(None),
                mime: Set(cdx.mime),
                timestamp: Set(cdx.timestamp.unix_time()),
                status_code: Set(cdx.status_code.map(|v| v.as_u16() as i32)),
            });
            let res = q::Entity::insert_many(inserts).exec(&*CONN).await;
            if let Err(e) = res {
                error!("Failed to insert download queue: {:?}", e);
                return;
            }
            notifier.notify_one();
        });
    }
}
