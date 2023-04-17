use axum::{extract::State, http::StatusCode, Json};
use sea_orm::EntityTrait;
use serde::Serialize;
use uuid::Uuid;

use super::AppState;
use::entity::idea;

#[derive(Debug, Serialize)]
pub struct ListIdea {
    id: Uuid,
    name: String,
    context: Option<String>,
}

pub async fn ideas_list(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ListIdea>>, StatusCode> {
    let ideas = idea::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ideas: Vec<ListIdea> = ideas
        .iter()
        .map(|idea| ListIdea {
            id: idea.id,
            name: idea.name.clone(),
            context: idea.context.clone(),
        })
        .collect();
    Ok(Json(ideas))
}
