pub use sea_orm_migration::prelude::*;

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
pub enum ExplanationType {
    Table,
    #[iden = "External"]
    External,
    #[iden = "Markdown"]
    Markdown,
}

#[derive(Iden)]
pub enum Exercise {
    Table,
    Id,
    IdeaId,
    Type,
}

