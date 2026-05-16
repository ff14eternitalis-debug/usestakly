use std::time::Duration;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing::{info, warn};
use usestakly_backend::{app, config, db, services, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init();
    let config = config::AppConfig::from_env()?;
    let db = db::connect(&config.database_url)
        .await
        .context("failed to connect to database")?;

    match services::repo_categories::backfill_missing_repo_categories(&db).await {
        Ok(count) if count > 0 => info!(count, "repo category backfill completed"),
        Ok(_) => {}
        Err(err) => warn!(?err, "repo category backfill failed"),
    }
    match services::radar::refresh_all_repo_radar_snapshots(&db).await {
        Ok(count) if count > 0 => info!(count, "repo radar refresh completed"),
        Ok(_) => {}
        Err(err) => warn!(?err, "repo radar refresh failed"),
    }

    if config.scheduler_enabled {
        let interval = Duration::from_secs(config.recompute_interval_secs);
        services::scheduler::spawn_recompute_loop(db.clone(), config.clone(), interval);
        let digest_interval = Duration::from_secs(config.digest_interval_secs);
        services::scheduler::spawn_digest_loop(db.clone(), config.clone(), digest_interval);
        info!(
            interval_secs = config.recompute_interval_secs,
            stale_after_secs = config.corpus_refresh_stale_secs,
            max_repos_per_cycle = config.ingest_max_repos_per_cycle,
            run_on_startup = config.scheduler_run_on_startup,
            "scheduler: recompute loop spawned"
        );
        info!(
            interval_secs = config.digest_interval_secs,
            "scheduler: digest loop spawned"
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
