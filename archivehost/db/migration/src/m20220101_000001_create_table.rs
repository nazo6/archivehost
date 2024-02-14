use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Archive::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Archive::UrlScheme).string().not_null())
                    .col(ColumnDef::new(Archive::UrlHost).string().not_null())
                    .col(ColumnDef::new(Archive::UrlPath).string().not_null())
                    .col(ColumnDef::new(Archive::Timestamp).integer().not_null())
                    .col(ColumnDef::new(Archive::Mime).string().not_null())
                    .col(ColumnDef::new(Archive::Status).integer())
                    .col(ColumnDef::new(Archive::SavePath).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(Archive::UrlScheme)
                            .col(Archive::UrlHost)
                            .col(Archive::UrlPath)
                            .col(Archive::Timestamp),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Archive::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Archive {
    Table,
    UrlScheme,
    UrlHost,
    UrlPath,
    Timestamp,
    Mime,
    Status,
    SavePath,
}
