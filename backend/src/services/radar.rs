use chrono::Utc;
use serde_json::json;
use sqlx::{PgPool, types::Json};
use uuid::Uuid;

use crate::app::error::ApiError;
use crate::domain::repo::{RepoCategory, RepoRadarSnapshot};

#[derive(Debug, Clone)]
pub struct RepoRadarInput {
    pub archived: bool,
    pub stars_count: i32,
    pub last_commit_at: Option<chrono::DateTime<Utc>>,
    pub quality_overall: Option<f64>,
    pub quality_freshness: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub quality_vitality: Option<f64>,
    pub quality_flags: Vec<String>,
    pub categories: Vec<RepoCategory>,
    pub commits_30d: Option<i32>,
    pub distinct_contributors_90d: Option<i32>,
    pub has_ci: Option<bool>,
    pub releases_count: Option<i32>,
}

pub fn compute_radar_snapshot(input: &RepoRadarInput) -> RepoRadarSnapshot {
    let category_confidence = input
        .categories
        .iter()
        .map(|category| category.confidence)
        .fold(0.0_f64, f64::max);
    let has_clear_category = category_confidence >= 0.55;
    let overall = input.quality_overall.unwrap_or(0.0);
    let freshness = input.quality_freshness.unwrap_or(0.0);
    let abandonment = input.quality_abandonment.unwrap_or(1.0);
    let vitality = input.quality_vitality.unwrap_or(0.5);
    let commits_30d = input.commits_30d.unwrap_or_default();
    let contributors_90d = input.distinct_contributors_90d.unwrap_or_default();
    let has_activity = freshness >= 0.65 || commits_30d >= 5;
    let has_structure = contributors_90d >= 2
        || input.has_ci.unwrap_or(false)
        || input.releases_count.unwrap_or_default() > 0;
    let has_severe_flags = input
        .quality_flags
        .iter()
        .any(|flag| flag == "security-issue" || flag == "broken");

    let trend_signal = trend_signal(input, freshness, commits_30d, contributors_90d);
    let radar_relevance = ((category_confidence * 0.7) + (vitality * 0.3)).clamp(0.0, 1.0);
    let mut reasons = Vec::new();

    let maturity_band =
        if input.archived || abandonment >= 0.45 || freshness <= 0.25 || has_severe_flags {
            reasons.push("stale_or_flagged");
            "stale"
        } else if !has_clear_category && overall < 0.55 {
            reasons.push("weak_category_signal");
            "noisy"
        } else if overall >= 0.70
            && abandonment <= 0.15
            && freshness >= 0.65
            && has_clear_category
            && has_structure
        {
            reasons.push("strong_quality");
            reasons.push("clear_category");
            reasons.push("healthy_activity");
            "established"
        } else if has_clear_category
            && has_activity
            && has_structure
            && abandonment <= 0.25
            && (trend_signal >= 0.45 || overall >= 0.50)
        {
            reasons.push("clear_category");
            reasons.push("recent_activity");
            "emerging"
        } else {
            reasons.push("thin_evidence");
            "experimental"
        };

    RepoRadarSnapshot {
        maturity_band: maturity_band.to_string(),
        radar_relevance,
        trend_signal,
        explanation: json!({
            "reasons": reasons,
            "categoryConfidence": category_confidence,
            "overall": overall,
            "freshness": freshness,
            "abandonment": abandonment,
            "vitality": vitality,
            "commits30d": commits_30d,
            "contributors90d": contributors_90d,
            "stars": input.stars_count
        }),
    }
}

pub async fn refresh_all_repo_radar_snapshots(db: &PgPool) -> Result<u64, ApiError> {
    let rows = sqlx::query_as::<_, RepoRadarRow>(
        r#"
        SELECT
          e.id AS artifact_id,
          e.archived,
          e.stars_count,
          e.last_commit_at,
          e.commits_30d,
          e.distinct_contributors_90d,
          e.has_ci,
          e.releases_count,
          ascore.freshness::float8 AS quality_freshness,
          ascore.abandonment::float8 AS quality_abandonment,
          ascore.vitality::float8 AS quality_vitality,
          ascore.overall::float8 AS quality_overall,
          ascore.flags AS quality_flags,
          COALESCE((
            SELECT jsonb_agg(
              jsonb_build_object(
                'category', rc.category,
                'confidence', rc.confidence,
                'source', rc.source,
                'evidence', rc.evidence
              )
              ORDER BY rc.confidence DESC, rc.category
            )
            FROM repo_categories rc
            WHERE rc.external_artifact_id = e.id
          ), '[]'::jsonb) AS categories
        FROM external_artifacts e
        LEFT JOIN artifact_scores ascore
          ON ascore.external_artifact_id = e.id
          AND ascore.formula_version = 'v2.0'
        WHERE e.source = 'github'
          AND e.github_owner IS NOT NULL
          AND e.github_repo IS NOT NULL
        "#,
    )
    .fetch_all(db)
    .await?;

    let mut changed = 0_u64;
    for row in rows {
        let artifact_id = row.artifact_id;
        let snapshot = compute_radar_snapshot(&row.into_input());
        upsert_repo_radar_snapshot(db, artifact_id, &snapshot).await?;
        changed = changed.saturating_add(1);
    }
    Ok(changed)
}

pub async fn upsert_repo_radar_snapshot(
    db: &PgPool,
    artifact_id: Uuid,
    snapshot: &RepoRadarSnapshot,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO repo_radar_snapshots (
          external_artifact_id, maturity_band, radar_relevance, trend_signal, explanation, computed_at
        )
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (external_artifact_id) DO UPDATE SET
          maturity_band = EXCLUDED.maturity_band,
          radar_relevance = EXCLUDED.radar_relevance,
          trend_signal = EXCLUDED.trend_signal,
          explanation = EXCLUDED.explanation,
          computed_at = NOW()
        "#,
    )
    .bind(artifact_id)
    .bind(&snapshot.maturity_band)
    .bind(snapshot.radar_relevance)
    .bind(snapshot.trend_signal)
    .bind(&snapshot.explanation)
    .execute(db)
    .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct RepoRadarRow {
    artifact_id: Uuid,
    archived: bool,
    stars_count: i32,
    last_commit_at: Option<chrono::DateTime<Utc>>,
    commits_30d: Option<i32>,
    distinct_contributors_90d: Option<i32>,
    has_ci: Option<bool>,
    releases_count: Option<i32>,
    quality_freshness: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_vitality: Option<f64>,
    quality_overall: Option<f64>,
    quality_flags: Option<Vec<String>>,
    categories: Json<Vec<RepoCategory>>,
}

impl RepoRadarRow {
    fn into_input(self) -> RepoRadarInput {
        RepoRadarInput {
            archived: self.archived,
            stars_count: self.stars_count,
            last_commit_at: self.last_commit_at,
            quality_overall: self.quality_overall,
            quality_freshness: self.quality_freshness,
            quality_abandonment: self.quality_abandonment,
            quality_vitality: self.quality_vitality,
            quality_flags: self.quality_flags.unwrap_or_default(),
            categories: self.categories.0,
            commits_30d: self.commits_30d,
            distinct_contributors_90d: self.distinct_contributors_90d,
            has_ci: self.has_ci,
            releases_count: self.releases_count,
        }
    }
}

fn trend_signal(
    input: &RepoRadarInput,
    freshness: f64,
    commits_30d: i32,
    contributors_90d: i32,
) -> f64 {
    let commit_score = (f64::from(commits_30d) / 30.0).clamp(0.0, 1.0);
    let contributor_score = (f64::from(contributors_90d) / 5.0).clamp(0.0, 1.0);
    let release_score = (f64::from(input.releases_count.unwrap_or_default()) / 3.0).clamp(0.0, 1.0);
    let recency_bonus = match input.last_commit_at {
        Some(last_commit_at) if Utc::now() - last_commit_at <= chrono::Duration::days(14) => 0.15,
        _ => 0.0,
    };
    (freshness * 0.35
        + commit_score * 0.30
        + contributor_score * 0.20
        + release_score * 0.15
        + recency_bonus)
        .clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn input() -> RepoRadarInput {
        RepoRadarInput {
            archived: false,
            stars_count: 1_000,
            last_commit_at: Some(Utc::now() - Duration::days(3)),
            quality_overall: Some(0.74),
            quality_freshness: Some(0.95),
            quality_abandonment: Some(0.05),
            quality_vitality: Some(0.8),
            quality_flags: Vec::new(),
            categories: vec![RepoCategory {
                category: "testing".to_string(),
                confidence: 0.9,
                source: "github_metadata+readme".to_string(),
                evidence: json!({}),
            }],
            commits_30d: Some(20),
            distinct_contributors_90d: Some(4),
            has_ci: Some(true),
            releases_count: Some(3),
        }
    }

    #[test]
    fn established_repo_has_mature_quality_and_activity() {
        let snapshot = compute_radar_snapshot(&input());

        assert_eq!(snapshot.maturity_band, "established");
        assert!(snapshot.trend_signal >= 0.6);
        assert!(
            snapshot.explanation["reasons"]
                .as_array()
                .unwrap()
                .iter()
                .any(|reason| reason == "strong_quality")
        );
    }

    #[test]
    fn emerging_repo_is_recent_clear_and_active_but_not_proven() {
        let mut input = input();
        input.stars_count = 42;
        input.quality_overall = Some(0.52);
        input.quality_vitality = Some(0.72);
        input.releases_count = Some(1);

        let snapshot = compute_radar_snapshot(&input);

        assert_eq!(snapshot.maturity_band, "emerging");
    }

    #[test]
    fn stale_repo_has_high_abandonment_or_archive_signal() {
        let mut input = input();
        input.quality_abandonment = Some(0.62);
        input.quality_freshness = Some(0.2);

        let snapshot = compute_radar_snapshot(&input);

        assert_eq!(snapshot.maturity_band, "stale");
    }

    #[test]
    fn unclear_repo_with_weak_category_confidence_is_noisy() {
        let mut input = input();
        input.categories[0].confidence = 0.2;
        input.quality_overall = Some(0.48);
        input.commits_30d = Some(0);
        input.distinct_contributors_90d = Some(1);

        let snapshot = compute_radar_snapshot(&input);

        assert_eq!(snapshot.maturity_band, "noisy");
    }

    #[test]
    fn early_repo_with_clear_category_but_thin_signals_is_experimental() {
        let mut input = input();
        input.stars_count = 8;
        input.quality_overall = Some(0.4);
        input.commits_30d = Some(2);
        input.distinct_contributors_90d = Some(1);
        input.has_ci = Some(false);
        input.releases_count = Some(0);

        let snapshot = compute_radar_snapshot(&input);

        assert_eq!(snapshot.maturity_band, "experimental");
    }
}
