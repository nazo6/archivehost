use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "archive")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub url_scheme: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub url_host: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub url_path: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub timestamp: i64,
    pub mime: String,
    pub status: Option<i32>,
    pub save_path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
