mod route_utils;
mod routes;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use migration::{Migrator, MigratorTrait};
use routes::create_routes;
use sea_orm::Database;
use tracing::{event, Level, span};

pub async fn run() {
    // load env variables

    let db_url = dotenvy::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:password@kdatabase".to_string());
    event!(Level::INFO, "{db_url}");

    // connect to database
    let db = Database::connect(&db_url).await.expect("failed to connect");

    let span = span!(Level::INFO, "Running migrations");
    let _guard = span.enter();

    Migrator::up(&db, None).await.expect("migration failed");

    drop(_guard);

    // define the application
    let app = create_routes(db);

    //add port through env
    let mut socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80);

    let port = dotenvy::var("BE_PORT");
    if let Ok(port) = port {
        socket_addr.set_port(port.parse().unwrap());
    }

    event!(Level::INFO, "Listening on {socket_addr:?}");
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}