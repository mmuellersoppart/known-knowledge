use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;
use crate::{database::idea, route_utils::app_errors::AppError};

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
        .map_err(|_| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Retrieval Failed.".to_owned()
        })?;

    let mut active_idea = if let Some(idea) = db_idea {
        idea.into_active_model()
    } else {
        return Err(AppError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Idea not found.".to_owned() });
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
        .map_err(|_| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to update entry".to_owned()
        })?;

    Ok(())
}
