use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

mod get_idea;
mod hello;

use get_idea::get_idea;
use hello::{hello, helloo};

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/helloo", get(helloo))
        .route("/idea/:idea_id", get(get_idea))
        // .route("/ideas", get(list_ideas))
        .with_state(AppState { db })
}
