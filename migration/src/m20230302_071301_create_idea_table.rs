use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!()


    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}

/*
manager
            .create_table(
                Table::create()
                    .table(Idea::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Idea::Id)
                            .uuid()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Idea::Name).string().not_null())
                    .col(ColumnDef::new(Idea::Context).string())
                    .col(ColumnDef::new(Idea::Description).string())
                    .to_owned(),
            )
            .await
*/

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Idea {
    Table,
    Id,
    Name,
    Context,
    Description
}


