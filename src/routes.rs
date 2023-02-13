use axum::{routing::get, Router};

mod hello;

use hello::{hello, helloo};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/helloo", get(helloo))
}
