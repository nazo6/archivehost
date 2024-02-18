use async_graphql::*;
use futures_util::Stream;
use tracing::error;

use super::super::interface::download_queue_group::DownloadGroup;
use crate::app::serve::{
    graphql::common::{
        download_group::get_download_groups,
        download_stats::{get_download_stats, DownloadStats},
    },
    AppState,
};

#[derive(Default)]
pub struct DownloadQueueSubscription;

#[Subscription]
impl DownloadQueueSubscription {
    async fn download_queue_stats(&self, ctx: &Context<'_>) -> impl Stream<Item = DownloadStats> {
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

    async fn download_queue_groups(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Vec<DownloadGroup>> {
        let s = ctx.data::<AppState>().unwrap().clone();

        let mut rx = s.dl_q.subscribe_changes();
        async_stream::stream!({
            yield get_download_groups(&s.conn).await.unwrap_or_default();
            while (rx.recv().await).is_some() {
                let Ok(groups) = get_download_groups(&s.conn).await else {
                    error!("Failed to get download stats");
                    continue;
                };
                yield groups;
            }
        })
    }
}
