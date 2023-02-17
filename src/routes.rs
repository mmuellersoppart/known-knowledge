use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;

mod atomic_update_idea;
mod create_idea;
mod delete_idea;
mod hello;
mod list_ideas;
mod retrieve_idea;

use atomic_update_idea::atomic_update_idea;
use create_idea::create_idea;
use delete_idea::delete_idea;
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
        .route("/idea", post(create_idea))
        .route("/idea/:idea_id", delete(delete_idea))
        .route("/idea/:idea_id", put(atomic_update_idea))
        .with_state(AppState { db })
}
