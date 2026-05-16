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
    let run_on_startup = config.scheduler_run_on_startup;
    tokio::spawn(async move {
        if run_on_startup {
            tracing::info!("scheduler: startup ingestion cycle");
            run_cycle(&db, &config).await;
        }

        let mut ticker = tokio::time::interval(interval);
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
    tracing::info!(
        stale_after_secs = config.corpus_refresh_stale_secs,
        max_repos = config.ingest_max_repos_per_cycle,
        "scheduler: cycle start"
    );

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

    let watched = list_watched_repo_targets(db).await.unwrap_or_else(|e| {
        tracing::error!(error = ?e, "scheduler: failed to list watched repos");
        Vec::new()
    });
    let stale = list_stale_corpus_repo_targets(
        db,
        config.corpus_refresh_stale_secs,
        config.ingest_max_repos_per_cycle,
    )
    .await
    .unwrap_or_else(|e| {
        tracing::error!(error = ?e, "scheduler: failed to list stale corpus repos");
        Vec::new()
    });

    let rows = build_refresh_targets(watched, stale, config.ingest_max_repos_per_cycle);
    let total = rows.len();
    if total == 0 {
        return 0;
    }
    tracing::info!(
        total,
        stale_after_secs = config.corpus_refresh_stale_secs,
        max_per_cycle = config.ingest_max_repos_per_cycle,
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

fn build_refresh_targets(
    watched: Vec<(String, String)>,
    stale_corpus: Vec<(String, String)>,
    max_per_cycle: usize,
) -> Vec<(String, String)> {
    let mut targets = dedupe_refresh_targets(watched);
    if targets.len() >= max_per_cycle {
        return targets;
    }

    let mut seen = targets
        .iter()
        .map(|(owner, name)| (owner.to_ascii_lowercase(), name.to_ascii_lowercase()))
        .collect::<HashSet<_>>();

    for (owner, name) in stale_corpus {
        if targets.len() >= max_per_cycle {
            break;
        }
        let key = (owner.to_ascii_lowercase(), name.to_ascii_lowercase());
        if seen.insert(key) {
            targets.push((owner, name));
        }
    }

    targets
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

async fn list_stale_corpus_repo_targets(
    db: &PgPool,
    stale_after_secs: u64,
    limit: usize,
) -> Result<Vec<(String, String)>, sqlx::Error> {
    let stale_after_secs = i32::try_from(stale_after_secs).unwrap_or(i32::MAX);
    let limit = i64::try_from(limit).unwrap_or(i64::MAX);
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
        LIMIT $2
        "#,
    )
    .bind(stale_after_secs)
    .bind(limit)
    .fetch_all(db)
    .await
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
    fn build_refresh_targets_prioritizes_watchlist() {
        let targets = build_refresh_targets(
            vec![
                ("eslint".to_string(), "eslint".to_string()),
                ("vitejs".to_string(), "vite".to_string()),
            ],
            vec![
                ("facebook".to_string(), "react".to_string()),
                ("expressjs".to_string(), "express".to_string()),
            ],
            3,
        );

        assert_eq!(
            targets,
            vec![
                ("eslint".to_string(), "eslint".to_string()),
                ("vitejs".to_string(), "vite".to_string()),
                ("facebook".to_string(), "react".to_string()),
            ]
        );
    }

    #[test]
    fn build_refresh_targets_keeps_all_watched_when_over_cap() {
        let watched: Vec<_> = (0..5)
            .map(|i| (format!("owner{i}"), format!("repo{i}")))
            .collect();
        let targets = build_refresh_targets(watched.clone(), vec![("x".into(), "y".into())], 3);
        assert_eq!(targets, watched);
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
