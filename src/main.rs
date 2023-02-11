use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // define the application
    let app = Router::new().route("/hello", get(|| async {"hello world"}));

    axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
