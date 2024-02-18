//! Things that is common with Query and Subscription

pub mod download_group {
    use async_graphql::*;
    use sea_orm::EntityTrait as _;

    use db::entity::download_queue_group as dqg;

    use super::super::interface::download_queue_group::DownloadGroup;

    pub async fn get_download_groups(
        conn: &sea_orm::DatabaseConnection,
    ) -> Result<Vec<DownloadGroup>> {
        let res = dqg::Entity::find()
            .all(conn)
            .await?
            .into_iter()
            .map(|item| item.into())
            .collect();
        Ok(res)
    }
}

pub mod download_stats {
    use async_graphql::*;
    use db::entity::download_queue::{self, DownloadStatus};
    use sea_orm::{
        sea_query::{IntoCondition, SimpleExpr},
        ColumnTrait as _, EntityTrait as _, FromQueryResult, QueryFilter as _, QuerySelect,
    };

    #[derive(SimpleObject, Default)]
    pub struct DownloadStats {
        pending: i32,
        downloading: i32,
        success: i32,
        failed: i32,
        skipped: i32,
    }

    pub async fn get_download_stats(conn: &sea_orm::DatabaseConnection) -> Result<DownloadStats> {
        get_download_stats_with_condition::<SimpleExpr>(conn, None).await
    }

    /// Get download stats with condition
    /// Condition is expr of download_queue
    pub async fn get_download_stats_with_condition<F: IntoCondition>(
        conn: &sea_orm::DatabaseConnection,
        filter: Option<F>,
    ) -> Result<DownloadStats> {
        #[derive(Debug, FromQueryResult)]
        struct SelectResult {
            download_status: DownloadStatus,
            count: i32,
        }
        let mut query = download_queue::Entity::find().select_only();

        if let Some(filter) = filter {
            query = query.filter(filter);
        }

        let res = query
            .column(download_queue::Column::DownloadStatus)
            .column_as(download_queue::Column::Id.count(), "count")
            .group_by(download_queue::Column::DownloadStatus)
            .into_model::<SelectResult>()
            .all(conn)
            .await?;
        let mut stats = DownloadStats::default();
        for r in res {
            match r.download_status {
                DownloadStatus::Pending => stats.pending = r.count,
                DownloadStatus::Downloading => stats.downloading = r.count,
                DownloadStatus::Success => stats.success = r.count,
                DownloadStatus::Failed => stats.failed = r.count,
                DownloadStatus::Skipped => stats.skipped = r.count,
            }
        }

        Ok(stats)
    }
}
