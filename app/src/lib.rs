mod route_utils;
mod routes;

use migration::{Migrator, MigratorTrait};
use routes::create_routes;
use sea_orm::Database;

pub async fn run() {
    // load env variables
    let db_url = dotenvy::var("DATABASE_URL")
        .unwrap_or("postgre://postgres:password@localhost:5433/postgres".to_string());

    println!("{db_url}");

    // connect to database
    let db = Database::connect(&db_url).await.expect("failed to connect");

    println!("{db:?}");

    println!("Do migration");
    Migrator::up(&db, None).await.expect("migration failed");

    // define the application
    let app = create_routes(db);

    axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
