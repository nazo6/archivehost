use async_graphql::{Object, Result, SimpleObject};
use entity::archive;
use sea_orm::{EntityTrait as _, PaginatorTrait as _, QuerySelect};

use crate::config::CONN;

pub struct QueryRoot;

#[derive(SimpleObject)]
struct SiteListResult {
    total_count: u64,
    hosts: Vec<String>,
}

#[Object]
impl QueryRoot {
    #[tracing::instrument(skip(self), err(Debug))]
    async fn site_list(&self, offset: Option<u64>, limit: Option<u64>) -> Result<SiteListResult> {
        let offset = offset.unwrap_or(0);
        let limit = if let Some(limit) = limit {
            limit.min(100).max(1)
        } else {
            100
        };

        let total_count = archive::Entity::find()
            .select_only()
            .column(archive::Column::UrlHost)
            .distinct()
            .count(&*CONN)
            .await?;

        let hosts = archive::Entity::find()
            .select_only()
            .column(archive::Column::UrlHost)
            .distinct()
            .offset(offset)
            .limit(limit)
            .into_tuple::<(String,)>()
            .all(&*CONN)
            .await?
            .into_iter()
            .map(|d| d.0)
            .collect();

        Ok(SiteListResult { total_count, hosts })
    }
}
