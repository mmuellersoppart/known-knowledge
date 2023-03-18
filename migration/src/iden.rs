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
pub enum ExplainableType {
    Table,
    #[iden = "External"]
    External,
    #[iden = "Markdown"]
    Markdown,
}


#[derive(Iden)]
pub enum ExplainableMarkdown {
    Table,
    Id,
    Title,
    Markdown,
    CreatedAt,
    UpdatedAt,
}


#[derive(Iden)]
pub enum ExplainableExternal {
    Table,
    Id,
    Title,
    Link,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Explanation {
    Table,
    Id,
    IdeaId,
    Type,
    ExplainableId,
}

// will add fk to respective tables
// explanation type field as well

#[derive(Iden)]
pub enum ExerciseableType {
    Table,
    #[iden = "External"]
    External,
    #[iden = "Notecard"]
    Notecard,
}

#[derive(Iden)]
pub enum Card {  // a deck of notecards
    Table,
    Id,
    DeckId,
    Front,
    Back
}

#[derive(Iden)]
pub enum ExerciseableNotecard {  // a deck of notecards
    Table,
    Id,
    Title,  // deck name
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum ExerciseableExternal {
    Table,
    Id,
    Title,
    Link,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Exercise {
    Table,
    Id,
    IdeaId,
    Type,
    ExerciseableId
}

