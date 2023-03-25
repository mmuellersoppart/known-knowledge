//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "card")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub deck_id: Uuid,
    pub front: Option<String>,
    pub back: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::exerciseable_notecard::Entity",
        from = "Column::DeckId",
        to = "super::exerciseable_notecard::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ExerciseableNotecard,
}

impl Related<super::exerciseable_notecard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExerciseableNotecard.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}