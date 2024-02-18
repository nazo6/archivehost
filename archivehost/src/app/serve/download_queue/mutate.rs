use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait as _, DbErr, Set, TransactionTrait as _};
use tracing::error;

use db::entity::download_queue as q;
use db::entity::download_queue::DownloadStatus;
use db::entity::download_queue_group as qg;

use crate::common::download::cdx::{get_latest_page_cdx, get_latest_pages_index};
use crate::common::timestamp::Timestamp;
use crate::common::wayback_client::CdxLine;

use super::DownloadQueueController;

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

impl DownloadQueueController {
    pub fn add_task(&self, opts: DownloadOption) {
        let client = self.client.clone();
        let notifier = self.new_task_notifier.clone();
        let conn = self.conn.clone();
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
                    let _ = group.insert(&conn).await.inspect_err(|e| {
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
                    let _ = group.insert(&conn).await.inspect_err(|e| {
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
            .insert(&conn)
            .await;
            if let Err(e) = res {
                error!("Failed to insert download group: {:?}", e);
                return;
            }

            let inserts = to_download.into_iter().map(|cdx| q::ActiveModel {
                id: Set(uuid::Uuid::new_v7(now)),
                group_id: Set(group_id),
                url: Set(cdx.original),
                download_status: Set(DownloadStatus::Pending),
                message: Set(None),
                mime: Set(cdx.mime),
                timestamp: Set(cdx.timestamp.unix_time()),
                status_code: Set(cdx.status_code.map(|v| v.as_u16() as i32)),
            });
            let res = q::Entity::insert_many(inserts).exec(&conn).await;
            if let Err(e) = res {
                error!("Failed to insert download queue: {:?}", e);
                return;
            }
            notifier.notify_waiters();
        });
    }

    pub async fn clear_download_queue(&self) -> Result<(), eyre::Error> {
        self.conn
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    q::Entity::delete_many().exec(txn).await?;
                    qg::Entity::delete_many().exec(txn).await?;

                    Ok(())
                })
            })
            .await?;

        self.state_changed_notifier.notify_waiters();

        Ok(())
    }
}
