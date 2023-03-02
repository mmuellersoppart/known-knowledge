use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;
use entity::idea;
use crate::{route_utils::app_errors::AppError};

#[derive(Debug, Deserialize)]
pub struct PartialUpdateIdea {
    name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    description: Option<Option<String>>,
}

pub async fn idea_partial_update(
    Path(idea_id): Path<Uuid>,
    State(app_state): State<AppState>,
    Json(idea_data): Json<PartialUpdateIdea>,
) -> Result<(), AppError> {
    // create active model
    let db_idea = idea::Entity::find_by_id(idea_id)
        .one(&app_state.db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Retrieval Failed."))?;

    let mut active_idea = if let Some(idea) = db_idea {
        idea.into_active_model()
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Idea not found.",
        ));
    };

    if let Some(name) = idea_data.name {
        active_idea.name = Set(name)
    }

    if let Some(description) = idea_data.description {
        active_idea.description = Set(description)
    }

    active_idea
        .update(&app_state.db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to update entry"))?;

    Ok(())
}
