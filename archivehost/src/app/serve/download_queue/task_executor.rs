use std::sync::Arc;

use sea_orm::Set;
use sea_orm::{ColumnTrait as _, EntityTrait, QueryFilter as _};
use tokio::sync::Notify;
use tracing::warn;
use tracing::{error, info};

use db::entity::download_queue as q;
use db::entity::download_queue::DownloadStatus;
use db::entity::download_queue_group as qg;

use crate::common::download::{download_and_save_page, DownloadResult};
use crate::common::timestamp::Timestamp;
use crate::common::wayback_client::WaybackClient;

async fn execute_download(
    conn: &sea_orm::DatabaseConnection,
    client: &WaybackClient,
    task: &q::Model,
) -> Result<DownloadResult, eyre::Error> {
    let timestamp = Timestamp::from_unix_time(task.timestamp);
    let status_code = task
        .status_code
        .map(|v| http::StatusCode::from_u16(v as u16))
        .transpose();
    match (timestamp, status_code) {
        (Ok(timestamp), Ok(status_code)) => {
            let res = download_and_save_page(
                conn,
                client,
                &task.url,
                &task.mime,
                &timestamp,
                status_code,
            )
            .await?;
            Ok(res)
        }
        (Err(e), _) => Err(e.wrap_err("Failed to parse timestamp")),
        (_, Err(e)) => Err(eyre::eyre!("Failed to parse status code: {:?}", e)),
    }
}

pub(super) async fn start_task_executor(
    conn: sea_orm::DatabaseConnection,
    new_task_notifier: Arc<Notify>,
    task_completed_notifier: Arc<Notify>,
    semaphore: Arc<tokio::sync::Semaphore>,
    client: Arc<WaybackClient>,
) {
    let conn = conn.clone();
    loop {
        new_task_notifier.notified().await;
        loop {
            match q::Entity::find()
                .filter(q::Column::DownloadStatus.eq(DownloadStatus::Pending))
                .find_also_related(qg::Entity)
                .one(&conn)
                .await
            {
                Ok(Some((task, Some(_task_group)))) => {
                    if let Err(e) = q::Entity::update(q::ActiveModel {
                        id: Set(task.id),
                        download_status: Set(DownloadStatus::Downloading),
                        ..Default::default()
                    })
                    .filter(q::Column::Id.eq(task.id))
                    .filter(q::Column::DownloadStatus.eq(DownloadStatus::Pending))
                    .exec(&conn)
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
                    let task_completed_notifier = task_completed_notifier.clone();
                    let conn = conn.clone();
                    tokio::spawn(async move {
                        info!("Downloading {}", task.url);
                        let _permit = permit;
                        let (message, status) = match execute_download(&conn, &client, &task).await
                        {
                            Ok(status) => match status {
                                DownloadResult::Done => (None, DownloadStatus::Success),
                                DownloadResult::Skipped(s) => (Some(s), DownloadStatus::Skipped),
                                DownloadResult::FixDb(s) => (Some(s), DownloadStatus::Skipped),
                            },
                            Err(e) => {
                                warn!("Failed to download page: {:?}", e);
                                (Some(e.to_string()), DownloadStatus::Failed)
                            }
                        };

                        if let Err(e) = q::Entity::update(q::ActiveModel {
                            id: Set(task.id),
                            download_status: Set(status),
                            message: Set(message),
                            ..Default::default()
                        })
                        .filter(q::Column::Id.eq(task.id))
                        .exec(&conn)
                        .await
                        {
                            warn!("Failed to update download status for {}: {:?}", task.id, e);
                        }

                        task_completed_notifier.notify_waiters();
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
}
