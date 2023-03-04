pub use sea_orm_migration::prelude::*;

pub mod m20230302_071301_create_idea_table;
mod m20230304_062227_populate_ideas;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230302_071301_create_idea_table::Migration),
            Box::new(m20230304_062227_populate_ideas::Migration),
        ]
    }
}

