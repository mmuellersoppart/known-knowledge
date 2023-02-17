use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::EntityTrait;
use uuid::Uuid;

use super::AppState;

use crate::database::idea::Entity;

pub async fn delete_idea(
    Path(idea_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<(), StatusCode> {
    let _ = Entity::delete_by_id(idea_id)
        .exec(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    Ok(())
}
