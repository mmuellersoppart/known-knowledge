use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::EntityTrait;
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;
use crate::database::idea;

#[derive(Debug, Deserialize)]
pub struct PartialUpdateIdea {
    uuid: Option<Uuid>,
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
) -> Result<(), StatusCode> {
    // create active model
    let active_idea = idea::Entity::find_by_id(idea_id)
        .one(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // update
    Ok(())
}
