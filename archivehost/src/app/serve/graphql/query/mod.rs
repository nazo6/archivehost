use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use db::entity::archive;
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _, QuerySelect};

use crate::app::serve::AppState;

mod download_queue;
mod site_list;

#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub(super) struct QueryRoot {
    site_list: site_list::SiteList,
    download_queue: download_queue::DownloadQueueQuery,
}

#[derive(SimpleObject)]
struct SitePath {
    path: String,
    mime: Option<String>,
    timestamp: i64,
}

#[ComplexObject]
impl QueryRoot {
    async fn site_paths(
        &self,
        ctx: &Context<'_>,
        host: String,
        mime: Option<String>,
    ) -> Result<Vec<SitePath>> {
        let s = ctx.data::<AppState>().unwrap().clone();

        let mut paths_q = archive::Entity::find()
            .select_only()
            .column(archive::Column::Timestamp)
            .column(archive::Column::Mime)
            .column(archive::Column::UrlPath)
            .distinct()
            .filter(archive::Column::UrlHost.eq(host));
        if let Some(mime) = mime {
            paths_q = paths_q.filter(archive::Column::Mime.eq(mime));
        }

        let paths = paths_q
            .into_tuple::<(i64, String, String)>()
            .all(&s.conn)
            .await?
            .into_iter()
            .map(|d| SitePath {
                timestamp: d.0,
                mime: Some(d.1),
                path: d.2,
            })
            .collect();
        Ok(paths)
    }
}
