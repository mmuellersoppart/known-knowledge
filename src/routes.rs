use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/hello", get(|| async { "hello_world" }))
}
