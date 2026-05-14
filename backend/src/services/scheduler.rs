use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use sqlx::PgPool;

use crate::{
    config::AppConfig,
    services::{
        ingestion::github::{build_client, ingest_repo},
        notification_digest,
        quality::recompute_all_scores,
        use_case_watches,
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

pub fn spawn_digest_loop(db: PgPool, config: AppConfig, interval: Duration) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            run_digest_cycle(&db, &config).await;
        }
    });
}

async fn run_cycle(db: &PgPool, config: &AppConfig) {
    let start = Instant::now();
    tracing::info!("scheduler: cycle start");

    let refreshed = refresh_github_repos(db, config).await;
    let recompute_ok = match recompute_all_scores(db).await {
        Ok(report) => {
            tracing::info!(
                refreshed,
                externals_processed = report.externals_processed,
                formula_version = %report.formula_version,
                elapsed_ms = start.elapsed().as_millis() as u64,
                "scheduler: cycle done"
            );
            true
        }
        Err(e) => {
            tracing::error!(error = ?e, "scheduler: recompute failed");
            false
        }
    };

    if !recompute_ok {
        return;
    }

    match use_case_watches::evaluate_enabled_watches(db, config).await {
        Ok(inserted) => tracing::info!(
            inserted,
            elapsed_ms = start.elapsed().as_millis() as u64,
            "scheduler: use-case watch notifications done"
        ),
        Err(e) => tracing::warn!(error = ?e, "scheduler: use-case watch notifications failed"),
    }
}

async fn run_digest_cycle(db: &PgPool, config: &AppConfig) {
    let window_minutes = i64::try_from(config.digest_interval_secs / 60).unwrap_or(30);
    match notification_digest::run_due_digests(db, config, chrono::Utc::now(), window_minutes).await
    {
        Ok(delivered) => tracing::info!(delivered, "scheduler: digest cycle done"),
        Err(e) => tracing::warn!(error = ?e, "scheduler: digest cycle failed"),
    }
}

async fn refresh_github_repos(db: &PgPool, config: &AppConfig) -> usize {
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

    let mut rows = list_watched_repo_targets(db).await.unwrap_or_else(|e| {
        tracing::error!(error = ?e, "scheduler: failed to list watched repos");
        Vec::new()
    });
    let stale = list_stale_corpus_repo_targets(db)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(error = ?e, "scheduler: failed to list stale corpus repos");
            Vec::new()
        });
    rows.extend(stale);

    let rows = dedupe_refresh_targets(rows);
    let total = rows.len();
    if total == 0 {
        return 0;
    }
    tracing::info!(
        total,
        stale_after_secs = corpus_refresh_stale_after().as_secs(),
        "scheduler: refreshing github repos"
    );

    let mut refreshed = 0usize;
    for (owner, name) in &rows {
        match ingest_repo(&client, db, config, owner, name).await {
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

async fn list_watched_repo_targets(db: &PgPool) -> Result<Vec<(String, String)>, sqlx::Error> {
    sqlx::query_as(
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
}

async fn list_stale_corpus_repo_targets(db: &PgPool) -> Result<Vec<(String, String)>, sqlx::Error> {
    let stale_after_secs = i32::try_from(corpus_refresh_stale_after().as_secs()).unwrap_or(86_400);
    sqlx::query_as(
        r#"
        SELECT e.github_owner, e.github_repo
        FROM external_artifacts e
        WHERE e.source = 'github'
          AND e.github_owner IS NOT NULL
          AND e.github_repo IS NOT NULL
          AND (
            e.priors_fetched_at IS NULL
            OR e.priors_fetched_at <= NOW() - make_interval(secs => $1)
          )
        ORDER BY e.priors_fetched_at ASC NULLS FIRST, e.created_at ASC
        "#,
    )
    .bind(stale_after_secs)
    .fetch_all(db)
    .await
}

fn corpus_refresh_stale_after() -> Duration {
    Duration::from_secs(86_400)
}

fn dedupe_refresh_targets(rows: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for (owner, name) in rows {
        let key = (owner.to_ascii_lowercase(), name.to_ascii_lowercase());
        if seen.insert(key) {
            deduped.push((owner, name));
        }
    }
    deduped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_refresh_stale_after_is_24_hours() {
        assert_eq!(corpus_refresh_stale_after().as_secs(), 86_400);
    }

    #[test]
    fn digest_scheduler_window_uses_interval_minutes() {
        assert_eq!(
            i64::try_from(Duration::from_secs(1_800).as_secs() / 60).unwrap(),
            30
        );
    }

    #[test]
    fn dedupe_refresh_targets_preserves_first_seen_order() {
        let targets = dedupe_refresh_targets(vec![
            ("facebook".to_string(), "react".to_string()),
            ("vercel".to_string(), "next.js".to_string()),
            ("Facebook".to_string(), "React".to_string()),
        ]);

        assert_eq!(
            targets,
            vec![
                ("facebook".to_string(), "react".to_string()),
                ("vercel".to_string(), "next.js".to_string()),
            ]
        );
    }
}
