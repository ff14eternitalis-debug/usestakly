use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;
use usestakly_backend::{app, config, db, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init();
    let config = config::AppConfig::from_env()?;
    let db = db::connect(&config.database_url)
        .await
        .context("failed to connect to database")?;

    let app = app::build_app(config.clone(), db);
    let listener = TcpListener::bind((&config.host[..], config.port)).await?;

    info!(
        "backend listening on http://{}:{}",
        config.host, config.port
    );
    axum::serve(listener, app).await?;
    Ok(())
}
