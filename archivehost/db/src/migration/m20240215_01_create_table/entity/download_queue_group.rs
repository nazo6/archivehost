use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "download_queue_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub url: String,
    pub to: Option<i64>,
    pub from: Option<i64>,
    pub download_type: DownloadType,
    pub failed: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::download_queue::Entity")]
    DownloadQueue,
}

impl Related<super::download_queue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DownloadQueue.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, PartialEq, Eq, Debug, Clone)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum DownloadType {
    #[sea_orm(num_value = 0)]
    Single,
    #[sea_orm(num_value = 1)]
    Batch,
}
