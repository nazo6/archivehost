use async_graphql::{Context, Object, Result};

use super::super::State;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn download_site(&self, ctx: &Context<'_>, urlPart: String) -> Result<bool> {
        // let dtc = &ctx.data::<State>().unwrap().download_task_controller;
        Ok(true)
    }
}
