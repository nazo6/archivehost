use async_graphql::*;
use sea_orm::ColumnTrait as _;
use sea_orm::EntityTrait as _;
use sea_orm::QueryFilter as _;
use uuid::Uuid;

use async_graphql::Enum;

use db::entity::download_queue as dq;

use crate::app::serve::graphql::common::download_stats::get_download_stats_with_condition;
use crate::app::serve::graphql::common::download_stats::DownloadStats;
use crate::app::serve::AppState;

use super::download_queue::DownloadTask;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct DownloadGroup {
    #[graphql(skip)]
    pub id: Uuid,
    pub url: String,
    pub to: Option<i64>,
    pub from: Option<i64>,
    pub download_type: DownloadType,
    pub failed: Option<String>,
}

impl From<db::entity::download_queue_group::Model> for DownloadGroup {
    fn from(value: db::entity::download_queue_group::Model) -> Self {
        Self {
            id: value.id,
            url: value.url,
            to: value.to,
            from: value.from,
            download_type: value.download_type.into(),
            failed: value.failed,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Copy, Enum)]
pub enum DownloadType {
    Single,
    Batch,
}

impl From<db::entity::download_queue_group::DownloadType> for DownloadType {
    fn from(value: db::entity::download_queue_group::DownloadType) -> Self {
        match value {
            db::entity::download_queue_group::DownloadType::Batch => Self::Batch,
            db::entity::download_queue_group::DownloadType::Single => Self::Single,
        }
    }
}

#[ComplexObject]
impl DownloadGroup {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<DownloadTask>> {
        let s = ctx.data::<AppState>().unwrap();

        let res = dq::Entity::find()
            .filter(dq::Column::GroupId.eq(self.id))
            .all(&s.conn)
            .await?
            .into_iter()
            .map(|item| DownloadTask {
                id: item.id,
                url: item.url,
                group_id: item.group_id,
                download_status: item.download_status.into(),
                message: item.message,
                timestamp: item.timestamp,
                mime: item.mime,
                status_code: item.status_code,
            })
            .collect();

        Ok(res)
    }
    async fn task_stats(&self, ctx: &Context<'_>) -> Result<DownloadStats> {
        let s = ctx.data::<AppState>().unwrap();
        get_download_stats_with_condition(&s.conn, Some(dq::Column::GroupId.eq(self.id))).await
    }
}
