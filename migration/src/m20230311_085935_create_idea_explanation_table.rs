use sea_orm_migration::prelude::*;

use crate::iden::{Idea, Explanation};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let mut idea_foreign_key = ForeignKey::create()
            .name("userspost")
            .from(Explanation::Table, Explanation::IdeaId)
            .to(Idea::Table, Idea::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Explanation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Explanation::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Explanation::Content).string().not_null())
                    .col(ColumnDef::new(Explanation::Website).string().not_null())
                    .col(ColumnDef::new(Explanation::IdeaId).uuid().not_null())
                    .foreign_key(&mut idea_foreign_key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Explanation::Table).to_owned())
            .await
    }
}


