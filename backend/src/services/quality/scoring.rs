use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    services::{
        notifications::{self, ScoreSnapshot},
        trust::reputation,
    },
};

const FORMULA_V1_TOML: &str = include_str!("../../../scoring/formula_v1.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct Formula {
    pub meta: FormulaMeta,
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FormulaMeta {
    pub version: String,
    #[allow(dead_code)]
    pub created: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dimensions {
    pub freshness: FreshnessWeights,
    pub adoption: AdoptionWeights,
    pub reliability: ReliabilityWeights,
    pub abandonment: AbandonmentWeights,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FreshnessWeights {
    pub weight: f64,
    pub half_life_days: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdoptionWeights {
    pub weight: f64,
    pub saturation: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReliabilityWeights {
    pub weight: f64,
    pub min_sample: u32,
    pub neutral_default: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AbandonmentWeights {
    pub weight: f64,
    pub regret_rate_threshold: f64,
}

pub fn load_v1() -> Result<Formula> {
    toml::from_str(FORMULA_V1_TOML).context("parsing scoring/formula_v1.toml")
}

#[derive(Debug, Clone)]
pub struct ArtifactMetrics {
    pub resolve_count: i32,
    pub build_success_count: i32,
    pub build_failure_count: i32,
    pub regret_count: i32,
    pub last_update: DateTime<Utc>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct ComputedScore {
    pub freshness: f64,
    pub adoption: f64,
    pub reliability: f64,
    pub abandonment: f64,
    pub overall: f64,
}

pub fn compute_score(
    metrics: &ArtifactMetrics,
    formula: &Formula,
    now: DateTime<Utc>,
) -> ComputedScore {
    let freshness = freshness_score(
        metrics.last_update,
        now,
        formula.dimensions.freshness.half_life_days,
    );
    let adoption = adoption_score(
        metrics.resolve_count,
        formula.dimensions.adoption.saturation,
    );
    let reliability = reliability_score(
        metrics.build_success_count,
        metrics.build_failure_count,
        formula.dimensions.reliability.min_sample,
        formula.dimensions.reliability.neutral_default,
    );
    let regret_rate = if metrics.resolve_count > 0 {
        metrics.regret_count as f64 / metrics.resolve_count as f64
    } else {
        0.0
    };
    let abandonment = abandonment_score(
        freshness,
        regret_rate,
        formula.dimensions.abandonment.regret_rate_threshold,
    );

    let overall = (freshness * formula.dimensions.freshness.weight
        + adoption * formula.dimensions.adoption.weight
        + reliability * formula.dimensions.reliability.weight
        + (1.0 - abandonment) * formula.dimensions.abandonment.weight)
        .clamp(0.0, 1.0);

    ComputedScore {
        freshness,
        adoption,
        reliability,
        abandonment,
        overall,
    }
}

fn freshness_score(last_update: DateTime<Utc>, now: DateTime<Utc>, half_life_days: f64) -> f64 {
    let age_days = (now - last_update).num_seconds().max(0) as f64 / 86_400.0;
    0.5_f64.powf(age_days / half_life_days).clamp(0.0, 1.0)
}

fn adoption_score(resolve_count: i32, saturation: f64) -> f64 {
    if resolve_count <= 0 {
        return 0.0;
    }
    let numer = ((resolve_count as f64) + 1.0).ln();
    let denom = (saturation + 1.0).ln();
    (numer / denom).clamp(0.0, 1.0)
}

fn reliability_score(success: i32, failure: i32, min_sample: u32, neutral_default: f64) -> f64 {
    let total = success.saturating_add(failure);
    if (total as u32) < min_sample {
        return neutral_default;
    }
    (success as f64 / total as f64).clamp(0.0, 1.0)
}

fn abandonment_score(freshness: f64, regret_rate: f64, regret_threshold: f64) -> f64 {
    let base = 1.0 - freshness;
    let bump = if regret_rate > regret_threshold {
        (regret_rate - regret_threshold).min(0.5)
    } else {
        0.0
    };
    (base + bump).clamp(0.0, 1.0)
}

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

fn normalize_flags(signals: Vec<String>) -> Vec<String> {
    signals
        .into_iter()
        .map(|s| match s.as_str() {
            "security_issue" => "security-issue".to_string(),
            other => other.to_string(),
        })
        .collect()
}

async fn load_active_flag_consensus(
    db: &PgPool,
    reputations: &HashMap<Uuid, reputation::UserReputation>,
    config: Option<&AppConfig>,
) -> Result<HashMap<Uuid, Vec<String>>> {
    let rows: Vec<ActiveSignalRow> = sqlx::query_as(
        r#"
        SELECT external_artifact_id, signal::text AS signal, actor_user_id
        FROM quality_signals
        WHERE external_artifact_id IS NOT NULL
          AND is_passive = FALSE
          AND review_status = 'accepted'
          AND signal IN ('broken', 'security_issue', 'deprecated')
        "#,
    )
    .fetch_all(db)
    .await
    .context("loading active signals for consensus")?;

    let min_reputation = config.map(|c| c.active_signal_min_reputation).unwrap_or(0.45);
    let default_consensus = config.map(|c| c.active_signal_default_consensus).unwrap_or(2);
    let severe_consensus = config.map(|c| c.active_signal_severe_consensus).unwrap_or(3);

    let mut per_artifact: HashMap<Uuid, HashMap<String, HashSet<Uuid>>> = HashMap::new();
    for row in rows {
        let Some(artifact_id) = row.external_artifact_id else {
            continue;
        };
        let Some(user_id) = row.actor_user_id else {
            continue;
        };
        let Some(rep) = reputations.get(&user_id) else {
            continue;
        };
        if !rep.active_signal_eligible(min_reputation) {
            continue;
        }

        per_artifact
            .entry(artifact_id)
            .or_default()
            .entry(row.signal)
            .or_default()
            .insert(user_id);
    }

    Ok(per_artifact
        .into_iter()
        .map(|(artifact_id, by_signal)| {
            let flags = by_signal
                .into_iter()
                .filter_map(|(signal, users)| {
                    let needed = if signal == "security_issue" || signal == "broken" {
                        severe_consensus
                    } else {
                        default_consensus
                    };
                    (users.len() as u32 >= needed).then_some(signal)
                })
                .collect::<Vec<_>>();
            (artifact_id, normalize_flags(flags))
        })
        .collect())
}

#[derive(sqlx::FromRow)]
struct ActiveSignalRow {
    external_artifact_id: Option<Uuid>,
    signal: String,
    actor_user_id: Option<Uuid>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn test_formula() -> Formula {
        load_v1().expect("formula v1 loads")
    }

    #[test]
    fn formula_v1_loads() {
        let f = test_formula();
        assert_eq!(f.meta.version, "v1");
        let total: f64 = f.dimensions.freshness.weight
            + f.dimensions.adoption.weight
            + f.dimensions.reliability.weight
            + f.dimensions.abandonment.weight;
        assert!((total - 1.0).abs() < 1e-9, "dimension weights sum to 1");
    }

    #[test]
    fn freshness_decays_exponentially() {
        let now = Utc::now();
        let fresh = freshness_score(now, now, 180.0);
        let six_months = freshness_score(now - Duration::days(180), now, 180.0);
        let one_year = freshness_score(now - Duration::days(360), now, 180.0);
        assert!((fresh - 1.0).abs() < 1e-9);
        assert!((six_months - 0.5).abs() < 1e-9);
        assert!((one_year - 0.25).abs() < 1e-9);
    }

    #[test]
    fn adoption_is_zero_for_no_resolves_and_grows_log() {
        assert_eq!(adoption_score(0, 1000.0), 0.0);
        let s10 = adoption_score(10, 1000.0);
        let s100 = adoption_score(100, 1000.0);
        let s1000 = adoption_score(1000, 1000.0);
        assert!(s10 < s100 && s100 < s1000);
        assert!((s1000 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn reliability_floors_on_small_sample() {
        let r = reliability_score(2, 0, 5, 0.5);
        assert_eq!(r, 0.5);
        let r = reliability_score(9, 1, 5, 0.5);
        assert!((r - 0.9).abs() < 1e-9);
    }

    #[test]
    fn overall_score_is_clamped_and_uses_weights() {
        let formula = test_formula();
        let now = Utc::now();

        let perfect = ArtifactMetrics {
            resolve_count: 1000,
            build_success_count: 100,
            build_failure_count: 0,
            regret_count: 0,
            last_update: now,
            flags: vec![],
        };
        let ps = compute_score(&perfect, &formula, now);
        assert!(ps.overall > 0.85);

        let dead = ArtifactMetrics {
            resolve_count: 0,
            build_success_count: 0,
            build_failure_count: 0,
            regret_count: 0,
            last_update: now - Duration::days(1800),
            flags: vec![],
        };
        let ds = compute_score(&dead, &formula, now);
        assert!(ds.overall < 0.25);
    }
}
