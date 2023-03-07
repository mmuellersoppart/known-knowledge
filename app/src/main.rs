use app::run;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {

    // sea-orm, without the filtering though
//    tracing_subscriber::fmt()
//        .with_max_level(tracing::Level::DEBUG)
//        .with_test_writer()
//        .init();


    tracing_subscriber::registry()
        .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sqlx=debug,sea-orm=debug,app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    run().await;
}
