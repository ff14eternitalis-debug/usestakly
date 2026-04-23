use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    services::{
        notifications::{self, ScoreSnapshot},
        quality::{
            compute::{ArtifactMetrics, ComputedScore, compute_score},
            flags::{load_active_flag_consensus, normalize_flags},
            formula::{Formula, load_v1},
        },
        trust::reputation,
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoringReport {
    pub formula_version: String,
    pub externals_processed: usize,
    pub computed_at: DateTime<Utc>,
}

pub async fn recompute_all_scores(db: &PgPool) -> Result<ScoringReport> {
    recompute_all_scores_with_config(db, None).await
}

pub async fn recompute_all_scores_with_config(
    db: &PgPool,
    config: Option<&AppConfig>,
) -> Result<ScoringReport> {
    let formula = load_v1()?;
    let now = Utc::now();

    let externals_processed = recompute_externals_with_config(db, &formula, now, config).await?;

    Ok(ScoringReport {
        formula_version: formula.meta.version,
        externals_processed,
        computed_at: now,
    })
}

#[derive(sqlx::FromRow)]
struct ExternalMetricsRow {
    id: Uuid,
    last_commit_at: Option<DateTime<Utc>>,
    resolve_count: i64,
    build_success_count: i64,
    build_failure_count: i64,
    regret_count: i64,
    active_flags: Vec<String>,
}

async fn recompute_externals_with_config(
    db: &PgPool,
    formula: &Formula,
    now: DateTime<Utc>,
    config: Option<&AppConfig>,
) -> Result<usize> {
    let rows: Vec<ExternalMetricsRow> = sqlx::query_as(
        r#"
        SELECT
          e.id AS id,
          e.last_commit_at AS last_commit_at,
          COUNT(*) FILTER (WHERE qs.signal = 'resolve') AS resolve_count,
          COUNT(*) FILTER (WHERE qs.signal = 'build_success') AS build_success_count,
          COUNT(*) FILTER (WHERE qs.signal = 'build_failure') AS build_failure_count,
          COUNT(*) FILTER (WHERE qs.signal = 'regret') AS regret_count,
          ARRAY[]::text[] AS active_flags
        FROM external_artifacts e
        LEFT JOIN quality_signals qs ON qs.external_artifact_id = e.id
        GROUP BY e.id, e.last_commit_at
        "#,
    )
    .fetch_all(db)
    .await
    .context("loading external artifact metrics")?;
    let reputations = reputation::list_user_reputations(db)
        .await
        .map_err(|e| anyhow::anyhow!("loading user reputations: {}", e.message))?;
    let approved_flags = load_active_flag_consensus(db, &reputations, config).await?;

    let mut processed = 0;
    for row in rows {
        let metrics = ArtifactMetrics {
            resolve_count: row.resolve_count as i32,
            build_success_count: row.build_success_count as i32,
            build_failure_count: row.build_failure_count as i32,
            regret_count: row.regret_count as i32,
            last_update: row.last_commit_at.unwrap_or(now),
            flags: approved_flags
                .get(&row.id)
                .cloned()
                .unwrap_or_else(|| normalize_flags(row.active_flags)),
        };
        let score = compute_score(&metrics, formula, now);

        let prev = notifications::fetch_prev_snapshot(db, row.id, &formula.meta.version)
            .await
            .context("fetching previous score snapshot")?;
        upsert_external_score(db, row.id, &score, &metrics, &formula.meta.version).await?;
        let new_snapshot = ScoreSnapshot {
            overall: score.overall,
            abandonment: score.abandonment,
            flags: metrics.flags.clone(),
        };
        if let Err(e) =
            notifications::detect_and_emit(db, row.id, prev.as_ref(), &new_snapshot).await
        {
            tracing::warn!(artifact_id = %row.id, error = ?e, "failed to emit notifications");
        }
        processed += 1;
    }
    Ok(processed)
}

async fn upsert_external_score(
    db: &PgPool,
    external_id: Uuid,
    score: &ComputedScore,
    metrics: &ArtifactMetrics,
    formula_version: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO artifact_scores (
          artifact_kind, external_artifact_id, formula_version,
          freshness, adoption, reliability, abandonment, overall,
          resolve_count, build_success_count, build_failure_count, regret_count,
          flags, computed_at
        )
        VALUES (
          'external', $1, $2,
          $3, $4, $5, $6, $7,
          $8, $9, $10, $11,
          $12, NOW()
        )
        ON CONFLICT (external_artifact_id, formula_version)
          WHERE external_artifact_id IS NOT NULL
        DO UPDATE SET
          freshness = EXCLUDED.freshness,
          adoption = EXCLUDED.adoption,
          reliability = EXCLUDED.reliability,
          abandonment = EXCLUDED.abandonment,
          overall = EXCLUDED.overall,
          resolve_count = EXCLUDED.resolve_count,
          build_success_count = EXCLUDED.build_success_count,
          build_failure_count = EXCLUDED.build_failure_count,
          regret_count = EXCLUDED.regret_count,
          flags = EXCLUDED.flags,
          computed_at = EXCLUDED.computed_at
        "#,
    )
    .bind(external_id)
    .bind(formula_version)
    .bind(score.freshness)
    .bind(score.adoption)
    .bind(score.reliability)
    .bind(score.abandonment)
    .bind(score.overall)
    .bind(metrics.resolve_count)
    .bind(metrics.build_success_count)
    .bind(metrics.build_failure_count)
    .bind(metrics.regret_count)
    .bind(&metrics.flags)
    .execute(db)
    .await
    .context("upserting external score")?;
    Ok(())
}
