use async_graphql::*;
use db::entity::download_queue::{self, DownloadStatus};
use futures_util::Stream;
use sea_orm::{ColumnTrait as _, EntityTrait as _, FromQueryResult, QuerySelect};
use tracing::error;

use crate::app::serve::AppState;

pub struct Subscription;

#[derive(SimpleObject, Default)]
struct DownloadStats {
    pending: i32,
    downloading: i32,
    success: i32,
    failed: i32,
    skipped: i32,
}

async fn get_download_stats(conn: &sea_orm::DatabaseConnection) -> Result<DownloadStats> {
    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        download_status: DownloadStatus,
        count: i32,
    }
    let res = download_queue::Entity::find()
        .select_only()
        .column(download_queue::Column::DownloadStatus)
        .column_as(download_queue::Column::Id.count(), "count")
        .group_by(download_queue::Column::DownloadStatus)
        .into_model::<SelectResult>()
        .all(conn)
        .await?;
    let mut stats = DownloadStats::default();
    for r in res {
        match r.download_status {
            DownloadStatus::Pending => stats.pending = r.count,
            DownloadStatus::Downloading => stats.downloading = r.count,
            DownloadStatus::Success => stats.success = r.count,
            DownloadStatus::Failed => stats.failed = r.count,
            DownloadStatus::Skipped => stats.skipped = r.count,
        }
    }

    Ok(stats)
}

#[Subscription]
impl Subscription {
    async fn download_stats(&self, ctx: &Context<'_>) -> impl Stream<Item = DownloadStats> {
        let s = ctx.data::<AppState>().unwrap().clone();

        let mut rx = s.dl_q.subscribe_changes();
        async_stream::stream!({
            yield get_download_stats(&s.conn).await.unwrap_or_default();
            while (rx.recv().await).is_some() {
                let Ok(stats) = get_download_stats(&s.conn).await else {
                    error!("Failed to get download stats");
                    continue;
                };
                yield stats;
            }
        })
    }
}
