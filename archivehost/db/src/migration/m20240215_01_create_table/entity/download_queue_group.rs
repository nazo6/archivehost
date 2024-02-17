use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "download_queue_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub url: String,
    pub to: i32,
    pub from: i32,
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
