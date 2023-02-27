use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::idea::{self, ActiveModel, Entity},
    route_utils::app_errors::AppError,
};

use super::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateIdea {
    name: String,
    context: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateIdeaResponse {
    id: Uuid,
}

pub async fn idea_create(
    State(app_state): State<AppState>,
    Json(idea_data): Json<CreateIdea>,
) -> Result<Json<CreateIdeaResponse>, StatusCode> {
    // create active model
    let active_idea = ActiveModel {
        name: Set(idea_data.name),
        context: Set(idea_data.context),
        description: Set(idea_data.description),
        ..Default::default()
    };

    // insert into the entity
    let result = Entity::insert(active_idea)
        .exec(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateIdeaResponse {
        id: result.last_insert_id,
    }))
}

/// Bulk create ideas
pub async fn ideas_create(
    State(app_state): State<AppState>,
    Json(ideas): Json<Vec<CreateIdea>>,
) -> Result<(), AppError> {
    let ideas: Vec<idea::ActiveModel> = ideas
        .into_iter()
        .map(|idea| idea::ActiveModel {
            name: Set(idea.name),
            context: Set(idea.context),
            description: Set(idea.description),
            ..Default::default()
        })
        .collect();

    idea::Entity::insert_many(ideas)
        .exec(&app_state.db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert ideas."))?;

    Ok(())
}
