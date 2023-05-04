use sea_orm_migration::prelude::*;
use crate::iden::Usr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Usr::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Usr::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Usr::Username).string_len(64).not_null())
                    .col(ColumnDef::new(Usr::Password).string().not_null())
                    .col(ColumnDef::new(Usr::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Usr::Token).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Usr::Table).to_owned())
            .await
    }
}




#[cfg(test)]
mod create_idea_test {
    use sea_orm_migration::prelude::*;
    use crate::iden::Usr;

    #[test]
    fn raw_sql_test() {
        let table_query = Table::create()
            .table(Usr::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Usr::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .extra("DEFAULT gen_random_uuid()".to_string()),
            )
            .col(ColumnDef::new(Usr::Username).string_len(64).not_null().unique_key())
            .col(ColumnDef::new(Usr::Password).string_len(64).not_null())
            .col(ColumnDef::new(Usr::DeletedAt).timestamp_with_time_zone().default(Keyword::Null))
            .col(ColumnDef::new(Usr::Token).string().default(Keyword::Null))
            .to_owned();

        assert_eq!(table_query.to_string(PostgresQueryBuilder),[
            r#"CREATE TABLE IF NOT EXISTS "usr" ("#,
            r#""id" uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),"#,
            r#""username" varchar(64) NOT NULL UNIQUE,"#,
            r#""password" varchar(64) NOT NULL,"#,
            r#""deleted_at" timestamp with time zone DEFAULT NULL,"#,
            r#""token" varchar DEFAULT NULL"#,
            r#")"#,
        ].join(" ")
        )
    }
}