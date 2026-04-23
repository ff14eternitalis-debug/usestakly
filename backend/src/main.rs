use std::time::Duration;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;
use usestakly_backend::{app, config, db, services, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init();
    let config = config::AppConfig::from_env()?;
    let db = db::connect(&config.database_url)
        .await
        .context("failed to connect to database")?;

    if config.scheduler_enabled {
        let interval = Duration::from_secs(config.recompute_interval_secs);
        services::scheduler::spawn_recompute_loop(db.clone(), config.clone(), interval);
        info!(
            interval_secs = config.recompute_interval_secs,
            "scheduler: recompute loop spawned"
        );
    } else {
        info!("scheduler: disabled (set APP_SCHEDULER_ENABLED=true to enable)");
    }

    let app = app::build_app(config.clone(), db);
    let listener = TcpListener::bind((&config.host[..], config.port)).await?;

    info!(
        "backend listening on http://{}:{}",
        config.host, config.port
    );
    axum::serve(listener, app).await?;
    Ok(())
}
