pub use sea_orm_migration::prelude::*;

pub mod m20230302_071301_create_idea_table;
mod m20230304_062227_populate_ideas;
mod m20230311_085935_create_idea_explanation_table;
mod m20230311_172014_create_exercise_type;
mod m20230311_173221_create_exercise_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230302_071301_create_idea_table::Migration),
            Box::new(m20230304_062227_populate_ideas::Migration),
            Box::new(m20230311_085935_create_idea_explanation_table::Migration),
            Box::new(m20230311_172014_create_exercise_type::Migration),
            Box::new(m20230311_173221_create_exercise_table::Migration),
        ]
    }
}

/// Learn more at https://docs.rs/seaquery#iden
#[derive(Iden)]
pub enum Idea {
    Table,
    Id,
    Name,
    Context,
    Description,
}

#[derive(Iden)]
pub enum Explanation {
    Table,
    Id,
    IdeaId,
    Content,
    Website,
}

#[derive(Iden)]
pub enum ExerciseType {
    Table,
    #[iden = "External"]
    External,
    #[iden = "Notecard"]
    Notecard,
}

#[derive(Iden)]
pub enum Exercise {
    Table,
    Id,
    IdeaId,
    Type,
}
