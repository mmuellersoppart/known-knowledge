use sea_orm_migration::prelude::*;
use crate::sea_orm::ActiveValue::Set;
use crate::sea_orm::prelude::Uuid;

use entity::usr;
use crate::sea_orm::ActiveModelTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        usr::ActiveModel {
            id: Set(Uuid::parse_str("154f87da-4311-4580-9634-2dfa02e348e2").expect("uuid failed to parse during population of usr")),
            username: Set("u1".to_owned()),
            password: Set("p1".to_owned()),
            ..Default::default()
        }
            .insert(db)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .truncate_table(Table::truncate().table(crate::iden::Usr::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
