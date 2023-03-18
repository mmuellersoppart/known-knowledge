use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type, sea_orm::ConnectionTrait};

use crate::iden::ExerciseType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ExerciseType::Table)
                    .values([ExerciseType::External, ExerciseType::Notecard])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("drop type explanation_type;")
            .await?;

        Ok(())
    }
}
