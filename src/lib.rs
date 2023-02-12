mod routes;

use routes::create_routes;

pub async fn run() {
    // define the application
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
