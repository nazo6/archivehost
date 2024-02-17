use async_graphql::{ComplexObject, Object, Result, SimpleObject};
use db::entity::archive;
use sea_orm::{
    ColumnTrait as _, EntityTrait as _, PaginatorTrait as _, QueryFilter as _, QuerySelect,
};

use crate::constant::CONN;

#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub struct QueryRoot {
    pub site_list: SiteList,
}

#[ComplexObject]
impl QueryRoot {
    async fn paths(&self, host: String, mime: Option<String>) -> Result<Vec<String>> {
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
            .all(&*CONN)
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
    #[tracing::instrument(skip(self), err(Debug, level = "warn"))]
    async fn total_count(&self) -> Result<u64> {
        let res = archive::Entity::find()
            .select_only()
            .column(archive::Column::UrlHost)
            .distinct()
            .count(&*CONN)
            .await?;
        Ok(res)
    }

    #[tracing::instrument(skip(self), err(Debug, level = "warn"))]
    async fn hosts(&self, offset: Option<u64>, limit: Option<u64>) -> Result<Vec<String>> {
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
            .all(&*CONN)
            .await?
            .into_iter()
            .map(|d| d.0)
            .collect();
        Ok(hosts)
    }
}