use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::EntityTrait;
use serde::Serialize;
use uuid::Uuid;

use super::AppState;
use crate::database::idea;

#[derive(Debug, Serialize)]
pub struct PublicIdea {
    uuid: Uuid,
    name: String,
    description: Option<String>,
}

pub async fn idea_retrieve(
    Path(idea_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<Json<PublicIdea>, StatusCode> {
    let idea = idea::Entity::find_by_id(idea_id)
        .one(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(idea) = idea {
        Ok(Json(PublicIdea {
            uuid: idea.id,
            name: idea.name,
            description: idea.description,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
