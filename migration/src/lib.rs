pub use sea_orm_migration::prelude::*;

pub mod m20230302_071301_create_idea_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230302_071301_create_idea_table::Migration),
        ]
    }
}

#[cfg(test)]
mod create_idea_test {
    use sea_orm_migration::prelude::*;

    use super::m20230302_071301_create_idea_table::Idea;

    #[test]
    fn raw_sql_test() {
        let table_query = Table::create()
                    .table(Idea::Table)
                    .if_not_exists()
                    .col(
                            ColumnDef::new(Idea::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Idea::Name).string().not_null())
                    .col(ColumnDef::new(Idea::Context).string())
                    .col(ColumnDef::new(Idea::Description).string())
                    .to_owned();

        assert_eq!(table_query.to_string(PostgresQueryBuilder),[
            r#"CREATE TABLE IF NOT EXISTS "idea" ("#,
            r#""id" DEFAULT gen_random_uuid() PRIMARY KEY,"#,
            r#""name" VARCHAR(64) NOT NULL,"#,
            r#""context" VARCHAR(64),"#,
            r#""description" VARCHAR(512),"#,
            r#")"#,
            ].join(" ")
        )
    }
}
