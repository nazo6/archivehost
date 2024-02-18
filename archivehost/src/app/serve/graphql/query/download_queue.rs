use async_graphql::*;

use super::super::interface::download_queue_group::DownloadGroup;
use crate::app::serve::{
    graphql::common::{
        download_group::get_download_groups,
        download_stats::{get_download_stats, DownloadStats},
    },
    AppState,
};

#[derive(Default)]
pub struct DownloadQueueQuery;

#[Object]
impl DownloadQueueQuery {
    async fn stats(&self, ctx: &Context<'_>) -> Result<DownloadStats> {
        let s = ctx.data::<AppState>().unwrap().clone();
        get_download_stats(&s.conn).await
    }

    async fn groups(&self, ctx: &Context<'_>) -> Result<Vec<DownloadGroup>> {
        let s = ctx.data::<AppState>().unwrap().clone();
        get_download_groups(&s.conn).await
    }
}
