use std::time::{Duration, Instant};

use sqlx::PgPool;

use crate::{
    config::AppConfig,
    services::{
        ingestion::github::{build_client, ingest_repo},
        quality::scoring::recompute_all_scores,
    },
};

pub fn spawn_recompute_loop(db: PgPool, config: AppConfig, interval: Duration) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        // interval() fires immediately on the first tick; consume it so the first real
        // run happens after `interval`, not at boot (avoids recompute on every restart).
        ticker.tick().await;
        loop {
            ticker.tick().await;
            run_cycle(&db, &config).await;
        }
    });
}

async fn run_cycle(db: &PgPool, config: &AppConfig) {
    let start = Instant::now();
    tracing::info!("scheduler: cycle start");

    let refreshed = refresh_watched_repos(db, config).await;
    match recompute_all_scores(db).await {
        Ok(report) => tracing::info!(
            refreshed,
            externals_processed = report.externals_processed,
            formula_version = %report.formula_version,
            elapsed_ms = start.elapsed().as_millis() as u64,
            "scheduler: cycle done"
        ),
        Err(e) => tracing::error!(error = ?e, "scheduler: recompute failed"),
    }
}

async fn refresh_watched_repos(db: &PgPool, config: &AppConfig) -> usize {
    let Some(token) = config.github_token.as_deref() else {
        tracing::warn!("scheduler: GITHUB_TOKEN missing, skipping GitHub refresh");
        return 0;
    };
    let client = match build_client(token) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = ?e, "scheduler: failed to build github client");
            return 0;
        }
    };

    let rows: Vec<(String, String)> = match sqlx::query_as(
        r#"
        SELECT DISTINCT e.github_owner, e.github_repo
        FROM watched_artifacts w
        JOIN external_artifacts e ON e.id = w.external_artifact_id
        WHERE w.muted = FALSE
          AND e.source = 'github'
          AND e.github_owner IS NOT NULL
          AND e.github_repo IS NOT NULL
        "#,
    )
    .fetch_all(db)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!(error = ?e, "scheduler: failed to list watched repos");
            return 0;
        }
    };

    let total = rows.len();
    if total == 0 {
        return 0;
    }
    tracing::info!(total, "scheduler: refreshing watched repos");

    let mut refreshed = 0usize;
    for (owner, name) in &rows {
        match ingest_repo(&client, db, owner, name).await {
            Ok(_) => refreshed += 1,
            Err(e) => tracing::warn!(
                owner = %owner,
                name = %name,
                error = ?e,
                "scheduler: repo refresh failed"
            ),
        }
    }
    refreshed
}
