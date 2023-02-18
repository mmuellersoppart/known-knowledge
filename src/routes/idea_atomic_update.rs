use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;
use crate::{database::idea, route_utils::app_errors::AppError};

#[derive(Debug, Deserialize)]
pub struct AtomicUpdateIdea {
    id: Option<Uuid>,
    name: String,
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
        description: Set(idea_data.description),
    };

    // full replacement (change the primary key, id)
    if let Some(new_id) = idea_data.id {
        if new_id != idea_id {
            println!("we made it here.");

            active_idea.id = Set(new_id);

            // TODO: check if anything is actually deleted
            idea::Entity::delete_by_id(idea_id).exec(&app_state.db).await.map_err(|_| AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "failed to delete existing idea for replacement".to_owned(),
            })?;


            idea::Entity::insert(active_idea)
                .exec(&app_state.db)
                .await
                .map_err(|_| AppError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "failed to save new idea".to_owned()
                })?;

            return Ok(())
        }
    }

    idea::Entity::update(active_idea)
        .exec(&app_state.db)
        .await
        .map_err(|_| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "atomic update of idea failed {idea_data:?}".to_owned()
        }
        )?;

    Ok(())
    // Update entity
}
