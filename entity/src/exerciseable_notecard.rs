//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "exerciseable_notecard")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub exerciseable_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::card::Entity")]
    Card,
    #[sea_orm(has_many = "super::exercise::Entity")]
    Exercise,
    #[sea_orm(
        belongs_to = "super::exerciseable::Entity",
        from = "Column::ExerciseableId",
        to = "super::exerciseable::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Exerciseable,
}

impl Related<super::card::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Card.def()
    }
}

impl Related<super::exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exercise.def()
    }
}

impl Related<super::exerciseable::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exerciseable.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}