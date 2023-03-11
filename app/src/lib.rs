mod route_utils;
mod routes;

use migration::{Migrator, MigratorTrait};
use routes::create_routes;
use sea_orm::Database;
use tracing::{event, Level, span};

pub async fn run() {
    // load env variables

    let db_url = dotenvy::var("DATABASE_URL")
        .unwrap_or("postgre://postgres:password@localhost:5433/postgres".to_string());
    event!(Level::INFO, "{db_url}");

    // connect to database
    let db = Database::connect(&db_url).await.expect("failed to connect");

    let span = span!(Level::INFO, "Running migrations");
    let _guard = span.enter();

    Migrator::up(&db, None).await.expect("migration failed");

    drop(_guard);

    // define the application
    let app = create_routes(db);

    event!(Level::INFO, "Listening on 0.0.0.0:3002");
    axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}