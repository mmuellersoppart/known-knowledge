pub use sea_orm_migration::prelude::*;

/// Learn more at https://docs.rs/seaquery#iden
#[derive(Iden)]
pub enum Usr {
    Table,
    Id,
    Username,
    Password,
    DeletedAt,
    Token
}

#[derive(Iden)]
pub enum Idea {
    Table,
    Id,
    Name,
    Context,
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    UsrId,
}

#[derive(Iden)]
pub enum ExplainableType {
    Table,
    #[iden = "External"]
    External,
    #[iden = "Markdown"]
    Markdown,
}

#[derive(Iden)]
pub enum Explainable {
    Table,
    Id,
}

#[derive(Iden)]
pub enum ExplainableMarkdown {
    Table,
    Id,
    ExplainableId,
    Markdown,
}

#[derive(Iden)]
pub enum ExplainableExternal {
    Table,
    Id,
    ExplainableId,
    Link,
}

#[derive(Iden)]
pub enum Explanation {
    Table,
    Id,
    IdeaId,
    UsrId,
    ExplainableId,
    ExplainableType, // redundant but useful for list view
    // common to all explainable sub tables
    Title,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
pub enum ExerciseableType {
    Table,
    #[iden = "External"]
    External,
    #[iden = "Notecard"]
    Notecard,
}

#[derive(Iden)]
pub enum Exerciseable {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Card {
    // a deck of notecards
    Table,
    Id,
    DeckId,
    Front,
    Back,
}

#[derive(Iden)]
pub enum ExerciseableNotecard {
    // a deck of notecards
    Table,
    Id,
    ExerciseableId,
}

#[derive(Iden)]
pub enum ExerciseableExternal {
    Table,
    Id,
    ExerciseableId,
    Link,
}

#[derive(Iden)]
pub enum Exercise {
    Table,
    Id,
    IdeaId,
    UsrId,
    ExerciseableId,
    ExerciseableType, // redundant but useful for list view
    // common to all explainable sub tables
    Title,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
