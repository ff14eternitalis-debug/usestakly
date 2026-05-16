use std::collections::HashMap;

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
            formula::{Formula, load_v2},
            weighting::{
                SignalObservation, SignalWeightBreakdown, WeightedCounts,
                aggregate_weighted_counts, explain_signals,
            },
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoringExplain {
    pub formula_version: String,
    pub external_artifact_id: Uuid,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub last_update: DateTime<Utc>,
    pub flags: Vec<String>,
    pub weighted_counts: WeightedCountsReport,
    pub score: ScoreReport,
    pub vitality_inputs: VitalityInputsReport,
    pub signals: Vec<SignalExplainEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedCountsReport {
    pub resolve: f64,
    pub build_success: f64,
    pub build_failure: f64,
    pub regret: f64,
    pub raw_resolve: i32,
    pub raw_build_success: i32,
    pub raw_build_failure: i32,
    pub raw_regret: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreReport {
    pub freshness: f64,
    pub adoption: f64,
    pub reliability: f64,
    pub abandonment: f64,
    pub vitality: f64,
    pub overall: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VitalityInputsReport {
    pub structural_signals_at: Option<DateTime<Utc>>,
    pub distinct_contributors_90d: Option<i32>,
    pub commits_30d: Option<i32>,
    pub has_ci: Option<bool>,
    pub last_release_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalExplainEntry {
    pub signal_id: Uuid,
    pub outcome: String,
    pub bucket: Option<String>,
    pub actor_user_id: Option<Uuid>,
    pub reporter_tier: Option<String>,
    pub reporter_score: Option<f64>,
    pub outcome_weight: f64,
    pub reputation_multiplier: f64,
    pub dedup_multiplier: f64,
    pub n_prev_same_user: u32,
    pub weight: f64,
    pub created_at: DateTime<Utc>,
}

impl From<SignalWeightBreakdown> for SignalExplainEntry {
    fn from(b: SignalWeightBreakdown) -> Self {
        Self {
            signal_id: b.signal_id,
            outcome: b.outcome,
            bucket: b.bucket.map(|s| s.to_string()),
            actor_user_id: b.actor_user_id,
            reporter_tier: b.reporter_tier,
            reporter_score: b.reporter_score,
            outcome_weight: b.outcome_weight,
            reputation_multiplier: b.reputation_multiplier,
            dedup_multiplier: b.dedup_multiplier,
            n_prev_same_user: b.n_prev_same_user,
            weight: b.weight,
            created_at: b.created_at,
        }
    }
}

pub async fn recompute_all_scores(db: &PgPool) -> Result<ScoringReport> {
    recompute_all_scores_with_config(db, None).await
}

pub async fn recompute_all_scores_with_config(
    db: &PgPool,
    config: Option<&AppConfig>,
) -> Result<ScoringReport> {
    let formula = load_v2()?;
    let now = Utc::now();

    let externals_processed = recompute_externals_with_config(db, &formula, now, config).await?;

    Ok(ScoringReport {
        formula_version: formula.meta.version,
        externals_processed,
        computed_at: now,
    })
}

pub async fn recompute_external_artifact(
    db: &PgPool,
    config: Option<&AppConfig>,
    artifact_id: Uuid,
) -> Result<()> {
    let formula = load_v2()?;
    let now = Utc::now();

    let external: ExternalRow = sqlx::query_as(
        r#"
        SELECT
          id,
          last_commit_at,
          structural_signals_at,
          distinct_contributors_90d,
          commits_30d,
          has_ci,
          last_release_at
        FROM external_artifacts
        WHERE id = $1
        "#,
    )
    .bind(artifact_id)
    .fetch_optional(db)
    .await
    .context("loading external artifact for recompute")?
    .ok_or_else(|| anyhow::anyhow!("external artifact not found"))?;

    let signal_rows: Vec<PassiveSignalRow> = sqlx::query_as(
        r#"
        SELECT id, external_artifact_id, signal::text AS signal, actor_user_id, created_at
        FROM quality_signals
        WHERE external_artifact_id = $1 AND is_passive = TRUE
        ORDER BY created_at
        "#,
    )
    .bind(artifact_id)
    .fetch_all(db)
    .await
    .context("loading passive signals for artifact")?;

    let signals: Vec<SignalObservation> = signal_rows
        .into_iter()
        .map(|row| SignalObservation {
            signal_id: row.id,
            external_artifact_id: row.external_artifact_id,
            outcome: row.signal,
            actor_user_id: row.actor_user_id,
            created_at: row.created_at,
        })
        .collect();

    let reputations = reputation::list_user_reputations(db)
        .await
        .map_err(|e| anyhow::anyhow!("loading user reputations: {}", e.message))?;
    let approved_flags = load_active_flag_consensus(db, &reputations, config).await?;
    let counts = aggregate_weighted_counts(&signals, &reputations, &formula.weighting);
    let metrics = build_metrics(
        counts,
        external.last_commit_at.unwrap_or(now),
        approved_flags
            .get(&artifact_id)
            .cloned()
            .unwrap_or_else(|| normalize_flags(vec![])),
        VitalityInputs {
            structural_signals_at: external.structural_signals_at,
            distinct_contributors_90d: external.distinct_contributors_90d,
            commits_30d: external.commits_30d,
            has_ci: external.has_ci,
            last_release_at: external.last_release_at,
        },
    );
    let score = compute_score(&metrics, &formula, now);
    let prev = notifications::fetch_prev_snapshot(db, artifact_id, &formula.meta.version).await?;
    upsert_external_score(db, artifact_id, &score, &metrics, &formula.meta.version).await?;
    let new_snapshot = ScoreSnapshot {
        overall: score.overall,
        abandonment: score.abandonment,
        flags: metrics.flags.clone(),
    };
    if let Err(e) = notifications::detect_and_emit(
        db,
        artifact_id,
        prev.as_ref(),
        &new_snapshot,
        config,
        config.and_then(AppConfig::notification_secret),
    )
    .await
    {
        tracing::warn!(artifact_id = %artifact_id, error = ?e, "failed to emit notifications");
    }
    crate::services::radar::refresh_repo_radar_snapshot(db, artifact_id)
        .await
        .map_err(|e| anyhow::anyhow!("radar refresh failed: {}", e.message))?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct ExternalRow {
    id: Uuid,
    last_commit_at: Option<DateTime<Utc>>,
    structural_signals_at: Option<DateTime<Utc>>,
    distinct_contributors_90d: Option<i32>,
    commits_30d: Option<i32>,
    has_ci: Option<bool>,
    last_release_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct ExplainRepoRow {
    github_owner: Option<String>,
    github_repo: Option<String>,
    last_commit_at: Option<DateTime<Utc>>,
    structural_signals_at: Option<DateTime<Utc>>,
    distinct_contributors_90d: Option<i32>,
    commits_30d: Option<i32>,
    has_ci: Option<bool>,
    last_release_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct PassiveSignalRow {
    id: Uuid,
    external_artifact_id: Uuid,
    signal: String,
    actor_user_id: Option<Uuid>,
    created_at: DateTime<Utc>,
}

async fn recompute_externals_with_config(
    db: &PgPool,
    formula: &Formula,
    now: DateTime<Utc>,
    config: Option<&AppConfig>,
) -> Result<usize> {
    let externals: Vec<ExternalRow> = sqlx::query_as(
        r#"
        SELECT
          id,
          last_commit_at,
          structural_signals_at,
          distinct_contributors_90d,
          commits_30d,
          has_ci,
          last_release_at
        FROM external_artifacts
        "#,
    )
    .fetch_all(db)
    .await
    .context("loading external artifacts")?;

    let signal_rows: Vec<PassiveSignalRow> = sqlx::query_as(
        r#"
        SELECT
          id,
          external_artifact_id,
          signal::text AS signal,
          actor_user_id,
          created_at
        FROM quality_signals
        WHERE external_artifact_id IS NOT NULL
          AND is_passive = TRUE
        ORDER BY external_artifact_id, created_at
        "#,
    )
    .fetch_all(db)
    .await
    .context("loading passive quality signals")?;

    let reputations = reputation::list_user_reputations(db)
        .await
        .map_err(|e| anyhow::anyhow!("loading user reputations: {}", e.message))?;
    let approved_flags = load_active_flag_consensus(db, &reputations, config).await?;

    let mut by_artifact: HashMap<Uuid, Vec<SignalObservation>> = HashMap::new();
    for row in signal_rows {
        by_artifact
            .entry(row.external_artifact_id)
            .or_default()
            .push(SignalObservation {
                signal_id: row.id,
                external_artifact_id: row.external_artifact_id,
                outcome: row.signal,
                actor_user_id: row.actor_user_id,
                created_at: row.created_at,
            });
    }

    let mut processed = 0;
    for external in externals {
        let signals = by_artifact.get(&external.id).cloned().unwrap_or_default();
        let counts = aggregate_weighted_counts(&signals, &reputations, &formula.weighting);

        let metrics = build_metrics(
            counts,
            external.last_commit_at.unwrap_or(now),
            approved_flags
                .get(&external.id)
                .cloned()
                .unwrap_or_else(|| normalize_flags(vec![])),
            VitalityInputs {
                structural_signals_at: external.structural_signals_at,
                distinct_contributors_90d: external.distinct_contributors_90d,
                commits_30d: external.commits_30d,
                has_ci: external.has_ci,
                last_release_at: external.last_release_at,
            },
        );
        let score = compute_score(&metrics, formula, now);

        let prev = notifications::fetch_prev_snapshot(db, external.id, &formula.meta.version)
            .await
            .context("fetching previous score snapshot")?;
        upsert_external_score(db, external.id, &score, &metrics, &formula.meta.version).await?;
        let new_snapshot = ScoreSnapshot {
            overall: score.overall,
            abandonment: score.abandonment,
            flags: metrics.flags.clone(),
        };
        let notification_secret = config.and_then(AppConfig::notification_secret);
        if let Err(e) = notifications::detect_and_emit(
            db,
            external.id,
            prev.as_ref(),
            &new_snapshot,
            config,
            notification_secret,
        )
        .await
        {
            tracing::warn!(artifact_id = %external.id, error = ?e, "failed to emit notifications");
        }
        processed += 1;
    }
    if let Err(e) = crate::services::radar::refresh_all_repo_radar_snapshots(db).await {
        tracing::warn!(error = ?e, "failed to refresh repo radar snapshots");
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
          freshness, adoption, reliability, abandonment, vitality, overall,
          resolve_count, build_success_count, build_failure_count, regret_count,
          flags, computed_at
        )
        VALUES (
          'external', $1, $2,
          $3, $4, $5, $6, $7, $8,
          $9, $10, $11, $12,
          $13, NOW()
        )
        ON CONFLICT (external_artifact_id, formula_version)
          WHERE external_artifact_id IS NOT NULL
        DO UPDATE SET
          freshness = EXCLUDED.freshness,
          adoption = EXCLUDED.adoption,
          reliability = EXCLUDED.reliability,
          abandonment = EXCLUDED.abandonment,
          vitality = EXCLUDED.vitality,
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
    .bind(score.vitality)
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

pub async fn explain_external_scoring(
    db: &PgPool,
    config: Option<&AppConfig>,
    external_artifact_id: Uuid,
) -> Result<ScoringExplain> {
    let formula = load_v2()?;
    let now = Utc::now();

    let repo: Option<ExplainRepoRow> = sqlx::query_as(
        r#"
        SELECT
          github_owner,
          github_repo,
          last_commit_at,
          structural_signals_at,
          distinct_contributors_90d,
          commits_30d,
          has_ci,
          last_release_at
        FROM external_artifacts
        WHERE id = $1
        "#,
    )
    .bind(external_artifact_id)
    .fetch_optional(db)
    .await
    .context("loading external artifact")?;

    let Some(repo) = repo else {
        anyhow::bail!("external_artifact {external_artifact_id} not found");
    };
    let owner = repo.github_owner;
    let name = repo.github_repo;
    let last_commit_at = repo.last_commit_at;
    let vitality_inputs = VitalityInputs {
        structural_signals_at: repo.structural_signals_at,
        distinct_contributors_90d: repo.distinct_contributors_90d,
        commits_30d: repo.commits_30d,
        has_ci: repo.has_ci,
        last_release_at: repo.last_release_at,
    };

    let signal_rows: Vec<PassiveSignalRow> = sqlx::query_as(
        r#"
        SELECT
          id,
          external_artifact_id,
          signal::text AS signal,
          actor_user_id,
          created_at
        FROM quality_signals
        WHERE external_artifact_id = $1
          AND is_passive = TRUE
        ORDER BY created_at
        "#,
    )
    .bind(external_artifact_id)
    .fetch_all(db)
    .await
    .context("loading signals for explain")?;

    let signals: Vec<SignalObservation> = signal_rows
        .into_iter()
        .map(|row| SignalObservation {
            signal_id: row.id,
            external_artifact_id: row.external_artifact_id,
            outcome: row.signal,
            actor_user_id: row.actor_user_id,
            created_at: row.created_at,
        })
        .collect();

    let reputations = reputation::list_user_reputations(db)
        .await
        .map_err(|e| anyhow::anyhow!("loading reputations: {}", e.message))?;
    let approved_flags = load_active_flag_consensus(db, &reputations, config).await?;

    let counts = aggregate_weighted_counts(&signals, &reputations, &formula.weighting);
    let breakdown = explain_signals(&signals, &reputations, &formula.weighting);

    let flags = approved_flags
        .get(&external_artifact_id)
        .cloned()
        .unwrap_or_default();
    let metrics = build_metrics(
        counts.clone(),
        last_commit_at.unwrap_or(now),
        flags.clone(),
        vitality_inputs.clone(),
    );
    let score = compute_score(&metrics, &formula, now);

    Ok(ScoringExplain {
        formula_version: formula.meta.version,
        external_artifact_id,
        owner,
        name,
        last_update: metrics.last_update,
        flags,
        weighted_counts: WeightedCountsReport {
            resolve: counts.resolve,
            build_success: counts.build_success,
            build_failure: counts.build_failure,
            regret: counts.regret,
            raw_resolve: counts.raw_resolve,
            raw_build_success: counts.raw_build_success,
            raw_build_failure: counts.raw_build_failure,
            raw_regret: counts.raw_regret,
        },
        score: ScoreReport {
            freshness: score.freshness,
            adoption: score.adoption,
            reliability: score.reliability,
            abandonment: score.abandonment,
            vitality: score.vitality,
            overall: score.overall,
        },
        vitality_inputs: VitalityInputsReport {
            structural_signals_at: vitality_inputs.structural_signals_at,
            distinct_contributors_90d: vitality_inputs.distinct_contributors_90d,
            commits_30d: vitality_inputs.commits_30d,
            has_ci: vitality_inputs.has_ci,
            last_release_at: vitality_inputs.last_release_at,
        },
        signals: breakdown.into_iter().map(Into::into).collect(),
    })
}

#[derive(Debug, Clone)]
struct VitalityInputs {
    structural_signals_at: Option<DateTime<Utc>>,
    distinct_contributors_90d: Option<i32>,
    commits_30d: Option<i32>,
    has_ci: Option<bool>,
    last_release_at: Option<DateTime<Utc>>,
}

fn build_metrics(
    counts: WeightedCounts,
    last_update: DateTime<Utc>,
    flags: Vec<String>,
    vitality: VitalityInputs,
) -> ArtifactMetrics {
    ArtifactMetrics {
        resolve_count: counts.raw_resolve,
        build_success_count: counts.raw_build_success,
        build_failure_count: counts.raw_build_failure,
        regret_count: counts.raw_regret,
        weighted_resolve: counts.resolve,
        weighted_build_success: counts.build_success,
        weighted_build_failure: counts.build_failure,
        weighted_regret: counts.regret,
        last_update,
        flags,
        structural_signals_at: vitality.structural_signals_at,
        distinct_contributors_90d: vitality.distinct_contributors_90d,
        commits_30d: vitality.commits_30d,
        has_ci: vitality.has_ci,
        last_release_at: vitality.last_release_at,
    }
}
