use sea_orm_migration::{prelude::*, sea_orm::Schema};

pub mod entity {
    pub mod archive;
    // pub mod download_queue;
    // pub mod download_queue_group;
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240215_01_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        manager
            .create_table(schema.create_table_from_entity(entity::archive::Entity))
            .await?;
        // manager
        //     .create_table(schema.create_table_from_entity(entity::download_queue::Entity))
        //     .await?;
        // manager
        //     .create_table(schema.create_table_from_entity(entity::download_queue_group::Entity))
        //     .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .drop_table(
        //         Table::drop()
        //             .table(entity::download_queue_group::Entity)
        //             .to_owned(),
        //     )
        //     .await?;
        // manager
        //     .drop_table(
        //         Table::drop()
        //             .table(entity::download_queue::Entity)
        //             .to_owned(),
        //     )
        //     .await?;
        manager
            .drop_table(Table::drop().table(entity::archive::Entity).to_owned())
            .await?;

        Ok(())
    }
}
