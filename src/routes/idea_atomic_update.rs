use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;
use crate::database::idea::{ActiveModel, Entity};

#[derive(Debug, Deserialize)]
pub struct AtomicUpdateIdea {
    name: String,
    description: Option<String>,
}

pub async fn idea_atomic_update(
    Path(idea_id): Path<Uuid>,
    State(app_state): State<AppState>,
    Json(idea_data): Json<AtomicUpdateIdea>,
) -> Result<(), StatusCode> {
    // Create active mode
    let active_idea = ActiveModel {
        id: Set(idea_id),
        name: Set(idea_data.name),
        description: Set(idea_data.description),
    };

    Entity::update(active_idea)
        .exec(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
    // Update entity
}
