use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    cli::run_cli(migration::Migrator).await;
}
