use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use sea_orm_migration::sea_orm::ConnectionTrait;

use crate::iden::{Idea, Explanation, ExplainableType, ExplainableMarkdown, ExplainableExternal};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // explanation types
        manager
            .create_type(
                    Type::create()
                    .as_enum(ExplainableType::Table)
                    .values([ExplainableType::External, ExplainableType::Markdown])
                    .to_owned(),
            )
            .await?;

        // explanation sub type: markdown
        manager
            .create_table(
                    Table::create()
                    .table(ExplainableMarkdown::Table)
                    .if_not_exists()
                    .col(
                            ColumnDef::new(ExplainableMarkdown::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(ExplainableMarkdown::Title).string_len(128).not_null())
                    .col(ColumnDef::new(ExplainableMarkdown::Markdown).string().not_null())
                    .col(ColumnDef::new(ExplainableMarkdown::CreatedAt).timestamp_with_time_zone().not_null().default(Keyword::CurrentTimestamp))
                    .col(ColumnDef::new(ExplainableMarkdown::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // explanation sub type: external
        manager
            .create_table(
                    Table::create()
                    .table(ExplainableExternal::Table)
                    .if_not_exists()
                    .col(
                            ColumnDef::new(ExplainableExternal::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(ExplainableExternal::Title).string_len(128).not_null())
                    .col(ColumnDef::new(ExplainableExternal::Link).string().not_null())
                    .col(ColumnDef::new(ExplainableExternal::CreatedAt).timestamp_with_time_zone().not_null().default(Keyword::CurrentTimestamp))
                    .col(ColumnDef::new(ExplainableExternal::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

        // explanation table
        let mut idea_fk = ForeignKey::create()
            .name("explanationsidea")
            .from(Explanation::Table, Explanation::IdeaId)
            .to(Idea::Table, Idea::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut explainable_markdown_fk = ForeignKey::create()
            .name("explanationsmarkdown")
            .from(Explanation::Table, Explanation::ExplainableId)
            .to(ExplainableMarkdown::Table, ExplainableMarkdown::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut explainable_external_fk = ForeignKey::create()
            .name("explanationsexternal")
            .from(Explanation::Table, Explanation::ExplainableId)
            .to(ExplainableExternal::Table, ExplainableExternal::Id)
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
                    .col(ColumnDef::new(Explanation::Type).enumeration(
                            ExplainableType::Table,
                        [ExplainableType::External, ExplainableType::Markdown],
                    ))
                    .col(ColumnDef::new(Explanation::IdeaId).uuid().not_null())
                    .col(ColumnDef::new(Explanation::ExplainableId).uuid().not_null())
                    .foreign_key(&mut idea_fk)
                    .foreign_key(&mut explainable_external_fk)
                    .foreign_key(&mut explainable_markdown_fk)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(ExplainableExternal::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExplainableMarkdown::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Explanation::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared("drop type exercise_type;")
            .await?;

        Ok(())
}
}


