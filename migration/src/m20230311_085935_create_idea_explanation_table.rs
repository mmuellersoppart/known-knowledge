use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::sea_query::extension::postgres::Type;

use crate::iden::{ExplainableExternal, ExplainableMarkdown, ExplainableType, Explanation, Idea, Explainable};

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

        // Explainable. Polymorphic Association Table, SuperTable method.
        // This table helps with referential integrity by erasing the id type of the sub tables
        manager
           .create_table(
                   Table::create()
                        .table(Explainable::Table)
                        .if_not_exists()
                        .col(ColumnDef::new(Explainable::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                        ).to_owned()
              )
        .await?;

        //  markdown: explanation sub type
        let mut explainable_markdown_fk = ForeignKey::create()
            .name("markdownexplainable")
            .from(ExplainableMarkdown::Table, ExplainableMarkdown::ExplainableId)
            .to(Explainable::Table, Explainable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

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
                    .col(ColumnDef::new(ExplainableMarkdown::ExplainableId).uuid().not_null())
                    .col(
                        ColumnDef::new(ExplainableMarkdown::Markdown)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(&mut explainable_markdown_fk)
                    .to_owned(),
            )
            .await?;


        // external: explanation sub type
        let mut explainable_external_fk = ForeignKey::create()
            .name("externalexplainable")
            .from(ExplainableExternal::Table, ExplainableExternal::ExplainableId)
            .to(Explainable::Table, Explainable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

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
                    .col(ColumnDef::new(ExplainableMarkdown::ExplainableId).uuid().not_null())
                    .col(
                        ColumnDef::new(ExplainableExternal::Link)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(&mut explainable_external_fk)
                    .to_owned(),
            )
            .await?;

        // explanation table
        let mut idea_fk = ForeignKey::create()
            .name("explanationsidea")
            .from(Explanation::Table, Explanation::IdeaId)
            .to(Idea::Table, Idea::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut explainable_fk = ForeignKey::create()
            .name("explanationsexplainable")
            .from(Explanation::Table, Explanation::ExplainableId)
            .to(Explainable::Table, Explainable::Id)
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
                    .col(ColumnDef::new(Explanation::ExplainableType).enumeration(
                        ExplainableType::Table,
                        [ExplainableType::External, ExplainableType::Markdown],
                    ))
                    .col(ColumnDef::new(Explanation::IdeaId).uuid().not_null())
                    .col(ColumnDef::new(Explanation::ExplainableId).uuid().not_null())
                    .col(ColumnDef::new(Explanation::Title).string_len(128).not_null().unique_key())
                    .col(ColumnDef::new(Explanation::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Keyword::CurrentTimestamp),
                    )
                    .col(ColumnDef::new(Explanation::UpdatedAt).timestamp_with_time_zone())
                    .foreign_key(&mut idea_fk)
                    .foreign_key(&mut explainable_fk)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Explanation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExplainableExternal::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExplainableMarkdown::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Explainable::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared("drop type explainable_type;")
            .await?;

        Ok(())
    }
}
