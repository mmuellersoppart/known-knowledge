//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "usr")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub token: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::exercise::Entity")]
    Exercise,
    #[sea_orm(has_many = "super::explanation::Entity")]
    Explanation,
    #[sea_orm(has_many = "super::idea::Entity")]
    Idea,
}

impl Related<super::exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exercise.def()
    }
}

impl Related<super::explanation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Explanation.def()
    }
}

impl Related<super::idea::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Idea.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}