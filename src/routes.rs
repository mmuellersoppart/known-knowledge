use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use sea_orm::DatabaseConnection;

mod hello;
mod idea_atomic_update;
mod idea_create;
mod idea_delete;
mod idea_partial_update;
mod idea_retrieve;
mod ideas_list;

use hello::{hello, helloo};
use idea_atomic_update::idea_atomic_update;
use idea_create::idea_create;
use idea_delete::idea_delete;
use idea_partial_update::idea_partial_update;
use idea_retrieve::idea_retrieve;
use ideas_list::ideas_list;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/helloo", get(helloo))
        .route("/idea/:idea_id", get(idea_retrieve))
        .route("/idea", get(ideas_list))
        .route("/idea", post(idea_create))
        .route("/idea/:idea_id", delete(idea_delete))
        .route("/idea/:idea_id", put(idea_atomic_update))
        .route("/idea/:idea_id", patch(idea_partial_update))
        .with_state(AppState { db })
}
