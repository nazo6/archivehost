use async_graphql::*;
use db::entity::download_queue_group as dqg;
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};
use uuid::Uuid;

use crate::app::serve::AppState;
use async_graphql::Enum;

use super::download_queue_group::DownloadGroup;

/// Download task.
/// Corresponds to `download_queue` table.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct DownloadTask {
    #[graphql(skip)]
    pub id: Uuid,
    pub url: String,
    #[graphql(skip)]
    pub group_id: Uuid,
    pub download_status: DownloadStatus,
    pub message: Option<String>,
    // cdx data
    pub timestamp: i64,
    pub mime: String,
    pub status_code: Option<i32>,
}

impl From<db::entity::download_queue::Model> for DownloadTask {
    fn from(value: db::entity::download_queue::Model) -> Self {
        Self {
            id: value.id,
            url: value.url,
            group_id: value.group_id,
            download_status: value.download_status.into(),
            message: value.message,
            timestamp: value.timestamp,
            mime: value.mime,
            status_code: value.status_code,
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Success,
    Failed,
    Skipped,
}
impl From<db::entity::download_queue::DownloadStatus> for DownloadStatus {
    fn from(s: db::entity::download_queue::DownloadStatus) -> Self {
        match s {
            db::entity::download_queue::DownloadStatus::Pending => Self::Pending,
            db::entity::download_queue::DownloadStatus::Downloading => Self::Downloading,
            db::entity::download_queue::DownloadStatus::Success => Self::Success,
            db::entity::download_queue::DownloadStatus::Failed => Self::Failed,
            db::entity::download_queue::DownloadStatus::Skipped => Self::Skipped,
        }
    }
}

#[ComplexObject]
impl DownloadTask {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn group(&self, ctx: &Context<'_>) -> Result<DownloadGroup> {
        let group = dqg::Entity::find()
            .filter(dqg::Column::Id.eq(self.group_id))
            .one(&ctx.data::<AppState>().unwrap().conn)
            .await?
            .ok_or("Group not found")?;

        Ok(DownloadGroup {
            id: group.id,
            url: group.url,
            to: group.to,
            from: group.from,
            failed: group.failed,
            download_type: group.download_type.into(),
        })
    }
}
