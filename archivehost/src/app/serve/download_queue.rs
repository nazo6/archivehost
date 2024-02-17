use std::sync::Arc;

use db::entity::download_queue::Status;
use sea_orm::Set;
use sea_orm::{ColumnTrait as _, EntityTrait, QueryFilter as _};
use tokio::sync::Notify;

use db::entity::download_queue as q;
use db::entity::download_queue_group as qg;
use tracing::error;
use tracing::warn;

use crate::common::download::get_latest_pages_index;
use crate::common::timestamp::Timestamp;
use crate::common::wayback_client::{CdxLine, WaybackClient};
use crate::config::CONN;

struct DownloadGroup {
    from: Timestamp,
    to: Timestamp,
    url: String,
    id: uuid::Uuid,
    tasks: Vec<DownloadTask>,
}

struct DownloadTask {
    cdx: CdxLine,
    group_id: uuid::Uuid,
}

pub struct DownloadQueueController {
    // Notify downloader when new tasks are available
    notifier: Arc<Notify>,
    client: WaybackClient,
    queue: VecQeue<DownloadTask>,
}

impl DownloadQueueController {
    pub fn start(concurrency: usize) -> Self {
        let notifier = Arc::new(Notify::new());
        {
            let notifier = notifier.clone();
            let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
            tokio::spawn(async move {
                loop {
                    notifier.notified().await;
                    loop {
                        match q::Entity::find()
                            .filter(q::Column::Status.eq(Status::Pending))
                            .find_also_related(qg::Entity)
                            .one(&*CONN)
                            .await
                        {
                            Ok(Some((task, Some(task_group)))) => {
                                if let Err(e) = q::Entity::update(q::ActiveModel {
                                    status: Set(Status::Downloading),
                                    ..Default::default()
                                })
                                .filter(q::Column::Id.eq(task.id))
                                .filter(q::Column::Status.eq(Status::Pending))
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
                                tokio::spawn(async move {
                                    let _permit = permit;
                                });
                            }
                            Ok(Some((task, None))) => {
                                error!(
                                    "Task {} has no group! This should nver occur. Skipping.",
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
        Self {
            notifier,
            client: WaybackClient::default(),
        }
    }

    pub async fn add_task(&self, opts: DownloadOption) -> Result<(), eyre::Error> {
        match opts.mode {
            DownloadMode::Single => {
                q::Entity::insert(q::ActiveModel {
                    url: Set(opts.url),
                    status: Set(Status::Pending),
                    ..Default::default()
                })
                .exec(&*CONN)
                .await?;
            }
            DownloadMode::Batch => {
                let pages =
                    get_latest_pages_index(&self.client, opts.url, opts.from, opts.to).await?;
                let now = uuid::Timestamp::now(uuid::NoContext);
                let group_id = uuid::Uuid::new_v7(now);
                let inserts = pages.into_iter().map(|(url, _)| q::ActiveModel {
                    id: Set(uuid::Uuid::new_v7(now)),
                    group_id: Set(group_id),
                    url: Set(url),
                    status: Set(Status::Pending),
                });
            }
        }

        Ok(())
    }
}
