use async_graphql::{Context, Object, Result};
use db::entity::archive;
use sea_orm::{EntityTrait as _, PaginatorTrait as _, QuerySelect};

use crate::app::serve::AppState;

#[derive(Default)]
pub(super) struct SiteList;

#[Object]
impl SiteList {
    #[tracing::instrument(skip(self, ctx), err(Debug, level = "warn"))]
    async fn total_count(&self, ctx: &Context<'_>) -> Result<u64> {
        let s = ctx.data::<AppState>().unwrap().clone();
        let res = archive::Entity::find()
            .select_only()
            .column(archive::Column::UrlHost)
            .distinct()
            .count(&s.conn)
            .await?;
        Ok(res)
    }

    #[tracing::instrument(skip(self, ctx), err(Debug, level = "warn"))]
    async fn hosts(
        &self,
        ctx: &Context<'_>,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<String>> {
        let s = ctx.data::<AppState>().unwrap().clone();
        let offset = offset.unwrap_or(0);
        let limit = if let Some(limit) = limit {
            limit.min(100).max(1)
        } else {
            100
        };

        let hosts = archive::Entity::find()
            .select_only()
            .column(archive::Column::UrlHost)
            .distinct()
            .offset(offset)
            .limit(limit)
            .into_tuple::<(String,)>()
            .all(&s.conn)
            .await?
            .into_iter()
            .map(|d| d.0)
            .collect();
        Ok(hosts)
    }
}
