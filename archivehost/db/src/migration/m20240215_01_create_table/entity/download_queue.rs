use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "download_queue")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub url: String,
    pub group_id: Uuid,
    pub status: Status,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::download_queue_group::Entity",
        from = "Column::GroupId",
        to = "super::download_queue_group::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    DownloadQueueGroup,
}

impl Related<super::download_queue_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DownloadQueueGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, PartialEq, Eq, Debug, Clone)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    Pending,
    #[sea_orm(num_value = 1)]
    Downloading,
    #[sea_orm(num_value = 2)]
    Success,
    #[sea_orm(num_value = 3)]
    Failed,
    #[sea_orm(num_value = 4)]
    Skipped,
}
