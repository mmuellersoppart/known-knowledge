use sea_orm_migration::prelude::*;

use crate::iden::{Exercise, ExerciseType, Idea};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut idea_foreign_key = ForeignKey::create()
            .name("userspost")
            .from(Exercise::Table, Exercise::IdeaId)
            .to(Idea::Table, Idea::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Exercise::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Exercise::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Exercise::IdeaId).uuid().not_null())
                    .col(ColumnDef::new(Exercise::Type).enumeration(
                        ExerciseType::Table,
                        [ExerciseType::External, ExerciseType::Notecard],
                    ))
                    .foreign_key(&mut idea_foreign_key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Exercise::Table).to_owned())
            .await
    }
}
