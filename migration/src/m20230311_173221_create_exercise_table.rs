use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::sea_query::extension::postgres::Type;

use crate::iden::{
    Card, Exercise, Exerciseable, ExerciseableExternal, ExerciseableNotecard, ExerciseableType,
    Idea,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // exerciseable types
        manager
            .create_type(
                Type::create()
                    .as_enum(ExerciseableType::Table)
                    .values([ExerciseableType::External, ExerciseableType::Notecard])
                    .to_owned(),
            )
            .await?;

        // Exerciseable. Polymorphic Association Table, SuperTable method.
        // This table helps with referential integrity by erasing the id type of the sub tables
        manager
            .create_table(
                Table::create()
                    .table(Exerciseable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Exerciseable::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        //  External: exercise sub type
        let mut exerciseable_external_fk = ForeignKey::create()
            .name("externalexerciseable")
            .from(
                ExerciseableExternal::Table,
                ExerciseableExternal::ExerciseableId,
            )
            .to(Exerciseable::Table, Exerciseable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ExerciseableExternal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExerciseableExternal::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(ExerciseableExternal::Link)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExerciseableExternal::ExerciseableId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(&mut exerciseable_external_fk)
                    .to_owned(),
            )
            .await?;

        // Notecard: exercise sub type
        let mut exerciseable_notecard_fk = ForeignKey::create()
            .name("notecardexerciseable")
            .from(
                ExerciseableNotecard::Table,
                ExerciseableNotecard::ExerciseableId,
            )
            .to(Exerciseable::Table, Exerciseable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ExerciseableNotecard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExerciseableNotecard::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(ExerciseableNotecard::ExerciseableId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(&mut exerciseable_notecard_fk)
                    .to_owned(),
            )
            .await?;

        // card table - connects to the Exerciseable Notecard
        let mut notecard_deck_fk = ForeignKey::create()
            .name("notecard_deck")
            .from(Card::Table, Card::DeckId)
            .to(ExerciseableNotecard::Table, ExerciseableNotecard::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Card::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Card::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Card::DeckId).uuid().not_null()) // each explanation is it's own deck (unordered)
                    .col(ColumnDef::new(Card::Front).string_len(512))
                    .col(ColumnDef::new(Card::Back).string_len(512))
                    .foreign_key(&mut notecard_deck_fk)
                    .to_owned(),
            )
            .await?;

        // Create the exercise table
        let mut idea_fk = ForeignKey::create()
            .name("exerciseideas")
            .from(Exercise::Table, Exercise::IdeaId)
            .to(Idea::Table, Idea::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut exerciseable_fk = ForeignKey::create()
            .name("exerciseexerciseable")
            .from(Exercise::Table, Exercise::ExerciseableId)
            .to(Exerciseable::Table, Exerciseable::Id)
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
                    .col(ColumnDef::new(Exercise::ExerciseableId).uuid().not_null())
                    .col(ColumnDef::new(Exercise::ExerciseableType).enumeration(
                        ExerciseableType::Table,
                        [ExerciseableType::External, ExerciseableType::Notecard],
                    ))
                    .col(ColumnDef::new(Exercise::Title).string_len(128).not_null().unique_key())
                    .col(
                        ColumnDef::new(Exercise::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Keyword::CurrentTimestamp),
                    )
                    .col(ColumnDef::new(Exercise::UpdatedAt).timestamp_with_time_zone())
                    .foreign_key(&mut idea_fk)
                    .foreign_key(&mut exerciseable_fk)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Exercise::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Card::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExerciseableNotecard::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExerciseableExternal::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared("drop type exerciseable_type;")
            .await?;

        Ok(())
    }
}
