use sea_orm_migration::prelude::*;
use tracing::{event, Level};

use crate::iden::{Idea, Usr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let mut usr_fk = ForeignKey::create()
            .name("useridea")
            .from(Idea::Table, Idea::UsrId)
            .to(Usr::Table, Usr::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();
        event!(Level::INFO, "Create ideas table.");

        manager
            .create_table(
                Table::create()
                    .table(Idea::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Idea::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Idea::Name).string_len(64).not_null())
                    .col(ColumnDef::new(Idea::Context).string_len(64))
                    .col(ColumnDef::new(Idea::Description).string_len(512))
                    .col(ColumnDef::new(Idea::CreatedAt).timestamp_with_time_zone().not_null().default(Keyword::CurrentTimestamp))
                    .col(ColumnDef::new(Idea::UpdatedAt).timestamp_with_time_zone().not_null().default(Keyword::CurrentTimestamp))
                    .col(ColumnDef::new(Idea::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Idea::UsrId).uuid())
                    .foreign_key(&mut usr_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Idea::Table).to_owned())
            .await
    }
}

#[cfg(test)]
mod create_idea_test {
    use sea_orm_migration::prelude::*;
    use crate::iden::Usr;

    use crate::m20230302_071301_create_idea_table::Idea;

    #[test]
    fn raw_sql_test() {
        let mut usr_fk = ForeignKey::create()
            .name("useridea")
            .from(Idea::Table, Idea::UsrId)
            .to(Usr::Table, Usr::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let table_query = Table::create()
            .table(Idea::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Idea::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .extra("DEFAULT gen_random_uuid()".to_string()),
            )
            .col(ColumnDef::new(Idea::Name).string_len(64).not_null())
            .col(ColumnDef::new(Idea::Context).string_len(64))
            .col(ColumnDef::new(Idea::Description).string_len(512))
            .col(ColumnDef::new(Idea::UsrId).uuid())
            .foreign_key(&mut usr_fk)
            .to_owned();

        assert_eq!(
            table_query.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "idea" ("#,
                r#""id" uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),"#,
                r#""name" varchar(64) NOT NULL,"#,
                r#""context" varchar(64),"#,
                r#""description" varchar(512)"#,
                r#")"#,
            ]
            .join(" ")
        )
    }
}
