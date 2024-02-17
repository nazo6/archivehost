use async_graphql::{Context, Object, Result};

use crate::{
    app::serve::download_queue::{DownloadOption, DownloadType},
    common::timestamp::Timestamp,
};

use super::{super::State, MutationRoot};

#[Object]
impl MutationRoot {
    #[tracing::instrument(skip(self, ctx), err(Debug))]
    async fn download_site(
        &self,
        ctx: &Context<'_>,
        url_part: String,
        from: Option<String>,
        to: Option<String>,
    ) -> Result<bool> {
        let dlq = &ctx.data::<State>().unwrap().dl_q;

        let from = match from {
            Some(v) => Some(Timestamp::from_str(&v)?),
            None => None,
        };
        let to = match to {
            Some(v) => Some(Timestamp::from_str(&v)?),
            None => None,
        };

        dlq.add_task(DownloadOption {
            mode: DownloadType::Batch,
            url: url_part,
            from,
            to,
        });
        Ok(true)
    }
    #[tracing::instrument(skip(self, ctx), err(Debug))]
    async fn clear_download_queue(&self, ctx: &Context<'_>) -> Result<bool> {
        let dlq = &ctx.data::<State>().unwrap().dl_q;
        dlq.clear_download_queue().await?;

        Ok(true)
    }
}
