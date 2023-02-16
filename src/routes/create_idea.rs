use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::idea::{ActiveModel, Entity};

use super::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    id: Uuid,
}

pub async fn create_idea(
    State(app_state): State<AppState>,
    Json(user_data): Json<CreateUser>,
) -> Result<Json<CreateUserResponse>, StatusCode> {
    // create active model
    let active_idea = ActiveModel {
        name: Set(user_data.name),
        description: Set(user_data.description),
        ..Default::default()
    };

    // insert into the entity
    let result = Entity::insert(active_idea)
        .exec(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateUserResponse {
        id: result.last_insert_id,
    }))
}
