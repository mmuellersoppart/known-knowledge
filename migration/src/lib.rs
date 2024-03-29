pub use sea_orm_migration::prelude::*;

pub mod iden;

mod m20230301_011111_create_usr_table;
mod m20230302_071301_create_idea_table;
mod m20230304_062227_populate_ideas;
mod m20230311_085935_create_idea_explanation_table;
mod m20230311_173221_create_exercise_table;
mod m20230505_055305_populate_user;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230301_011111_create_usr_table::Migration),
            Box::new(m20230302_071301_create_idea_table::Migration),
            Box::new(m20230304_062227_populate_ideas::Migration),
            Box::new(m20230311_085935_create_idea_explanation_table::Migration),
            Box::new(m20230311_173221_create_exercise_table::Migration),
            Box::new(m20230505_055305_populate_user::Migration),
        ]
    }
}