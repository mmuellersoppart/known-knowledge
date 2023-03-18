use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};
use sea_orm_migration::sea_orm::ConnectionTrait;

use crate::iden::ExplanationType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                    Type::create()
                    .as_enum(ExplanationType::Table)
                    .values([ExplanationType::External, ExplanationType::Markdown])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("drop type exercise_type;")
            .await?;

        Ok(())
        // TODO: remove type
    }
}