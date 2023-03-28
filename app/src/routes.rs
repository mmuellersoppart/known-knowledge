use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

mod exercise;
mod explanation;
mod hello;
mod idea_atomic_update;
mod idea_create;
mod idea_delete;
mod idea_partial_update;
mod idea_retrieve;
mod ideas_list;

use hello::{hello, helloo};
use idea_atomic_update::idea_atomic_update;
use idea_create::{idea_create, ideas_create};
use idea_delete::idea_delete;
use idea_partial_update::idea_partial_update;
use idea_retrieve::idea_retrieve;
use ideas_list::ideas_list;

use exercise::exercise_create::{create_exercise_external, create_exercise_notecard};
use explanation::explanation_create::{create_explanation_external, create_explanation_markdown};

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
        .route("/ideas", post(ideas_create))
        .route("/idea/:idea_id", delete(idea_delete))
        .route("/idea/:idea_id", put(idea_atomic_update))
        .route("/idea/:idea_id", patch(idea_partial_update))
        .route(
            "/idea/:idea_id/explanation/markdown",
            post(create_explanation_markdown),
        )
        .route(
            "/idea/:idea_id/explanation/external",
            post(create_explanation_external),
        )
        .route(
            "/idea/:idea_id/exercise/external",
            post(create_exercise_external),
        )
        .route(
            "/idea/:idea_id/exercise/notecard",
            post(create_exercise_notecard),
        )
        .with_state(AppState { db })
        .layer(TraceLayer::new_for_http())
}
