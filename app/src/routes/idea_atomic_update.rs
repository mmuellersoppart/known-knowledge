use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;
use crate::route_utils::app_errors::AppError;
use entity::idea;

#[derive(Debug, Deserialize)]
pub struct AtomicUpdateIdea {
    id: Option<Uuid>,
    name: String,
    context: Option<String>,
    description: Option<String>,
}

pub async fn idea_atomic_update(
    Path(idea_id): Path<Uuid>,
    State(app_state): State<AppState>,
    Json(idea_data): Json<AtomicUpdateIdea>,
) -> Result<(), AppError> {
    // Create active mode
    let mut active_idea = idea::ActiveModel {
        id: Set(idea_id),
        name: Set(idea_data.name),
        context: Set(idea_data.context),
        description: Set(idea_data.description),
        created_at: Default::default(),
        updated_at: Default::default(),
        deleted_at: Default::default(),
        usr_id: Default::default(),
    };

    // full replacement (change the primary key, id)
    if let Some(new_id) = idea_data.id {
        if new_id != idea_id {
            active_idea.id = Set(new_id);

            let result = idea::Entity::delete_by_id(idea_id)
                .exec(&app_state.db)
                .await
                .map_err(|_| {
                    AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to delete existing idea for replacement",
                    )
                })?;

            if result.rows_affected == 0 {
                let idea_uuid = idea_id.clone().to_string();
                let msg = format!("The id, {}, is not associated with an idea", idea_uuid);
                return Err(AppError::new(StatusCode::NOT_FOUND, msg));
            }

            idea::Entity::insert(active_idea)
                .exec(&app_state.db)
                .await
                .map_err(|_| {
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to save new idea")
                })?;

            return Ok(());
        }
    }

    idea::Entity::update(active_idea)
        .exec(&app_state.db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "atomic update of idea failed {idea_data:?}",
            )
        })?;

    Ok(())
    // Update entity
}
