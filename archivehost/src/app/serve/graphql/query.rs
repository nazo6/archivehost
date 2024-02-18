use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use db::entity::archive;
use sea_orm::{
    ColumnTrait as _, EntityTrait as _, PaginatorTrait as _, QueryFilter as _, QuerySelect,
};

use crate::app::serve::AppState;

#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub struct QueryRoot {
    pub site_list: SiteList,
}

#[ComplexObject]
impl QueryRoot {
    async fn paths(
        &self,
        ctx: &Context<'_>,
        host: String,
        mime: Option<String>,
    ) -> Result<Vec<String>> {
        let s = ctx.data::<AppState>().unwrap().clone();

        let mut paths_q = archive::Entity::find()
            .select_only()
            .column(archive::Column::UrlPath)
            .distinct()
            .filter(archive::Column::UrlHost.eq(host));
        if let Some(mime) = mime {
            paths_q = paths_q.filter(archive::Column::Mime.eq(mime));
        }

        let paths = paths_q
            .into_tuple::<(String,)>()
            .all(&s.conn)
            .await?
            .into_iter()
            .map(|d| d.0)
            .collect();
        Ok(paths)
    }
}

#[derive(Default)]
pub struct SiteList;

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
