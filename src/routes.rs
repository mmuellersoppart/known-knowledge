use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

mod hello;
mod list_ideas;
mod retrieve_idea;

use hello::{hello, helloo};
use list_ideas::list_ideas;
use retrieve_idea::retrieve_idea;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/helloo", get(helloo))
        .route("/idea/:idea_id", get(retrieve_idea))
        .route("/idea", get(list_ideas))
        .with_state(AppState { db })
}
