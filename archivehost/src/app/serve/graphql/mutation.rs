use async_graphql::{Context, Object, Result};

use crate::{
    app::serve::download_queue::{DownloadOption, DownloadType},
    common::timestamp::Timestamp,
};

use super::super::State;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn download_site(
        &self,
        ctx: &Context<'_>,
        url_part: String,
        from: Option<String>,
        to: Option<String>,
    ) -> Result<bool> {
        let dtc = &ctx.data::<State>().unwrap().download_task_controller;

        let from = match from {
            Some(v) => Some(Timestamp::from_str(&v)?),
            None => None,
        };
        let to = match to {
            Some(v) => Some(Timestamp::from_str(&v)?),
            None => None,
        };

        dtc.add_task(DownloadOption {
            mode: DownloadType::Batch,
            url: url_part,
            from,
            to,
        });
        Ok(true)
    }
}
