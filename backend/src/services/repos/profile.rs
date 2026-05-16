use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::repo::{RepoProfile, RepoSignal},
    services::{quality::load_v2, trust::signal_events},
};

use super::rows::{ProfileRow, SignalRow};

pub async fn get_repo_profile(
    db: &PgPool,
    config: &AppConfig,
    artifact_id: Uuid,
) -> Result<RepoProfile, ApiError> {
    let formula = load_v2()?;
    let formula_version = formula.meta.version.clone();
    let reliability_min_sample = f64::from(formula.dimensions.reliability.min_sample);

    let row: ProfileRow = sqlx::query_as(
        r#"
        SELECT
          e.id                   AS artifact_id,
          e.github_owner         AS owner,
          e.github_repo          AS name,
          e.html_url             AS html_url,
          e.description          AS description,
          e.language             AS language,
          e.license_spdx         AS license_spdx,
          e.topics               AS topics,
          e.stars_count          AS stars_count,
          e.forks_count          AS forks_count,
          e.open_issues_count    AS open_issues_count,
          e.archived             AS archived,
          e.last_commit_at       AS last_commit_at,
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
          ), '[]'::jsonb) AS categories,
          radar.maturity_band         AS radar_maturity_band,
          radar.radar_relevance::float8 AS radar_relevance,
          radar.trend_signal::float8  AS radar_trend_signal,
          radar.explanation           AS radar_explanation,
          e.subscribers_count         AS subscribers_count,
          e.default_branch            AS default_branch,
          e.priors_fetched_at         AS priors_fetched_at,
          e.structural_signals_at     AS structural_signals_at,
          e.distinct_contributors_90d AS distinct_contributors_90d,
          e.commits_30d               AS commits_30d,
          e.has_ci                    AS has_ci,
          e.releases_count            AS releases_count,
          e.last_release_at           AS last_release_at,
          e.owner_last_activity_at     AS owner_last_activity_at,
          e.owner_inactive_days        AS owner_inactive_days,
          ascore.formula_version      AS quality_formula_version,
          ascore.freshness::float8    AS quality_freshness,
          ascore.adoption::float8     AS quality_adoption,
          ascore.reliability::float8  AS quality_reliability,
          ascore.abandonment::float8  AS quality_abandonment,
          ascore.vitality::float8     AS quality_vitality,
          ascore.overall::float8      AS quality_overall,
          ascore.resolve_count        AS quality_resolve_count,
          ascore.build_success_count  AS quality_build_success_count,
          ascore.build_failure_count  AS quality_build_failure_count,
          ascore.regret_count         AS quality_regret_count,
          ascore.flags                AS quality_flags,
          ascore.computed_at          AS quality_computed_at
        FROM external_artifacts e
        LEFT JOIN artifact_scores ascore
          ON ascore.external_artifact_id = e.id
          AND ascore.formula_version = $1
        LEFT JOIN repo_radar_snapshots radar
          ON radar.external_artifact_id = e.id
        WHERE e.id = $2
          AND e.source = 'github'
        "#,
    )
    .bind(&formula_version)
    .bind(artifact_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Repo not found"))?;

    let signals = get_repo_signals(db, artifact_id).await?;

    let previous_overall: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT overall::float8
        FROM artifact_scores
        WHERE external_artifact_id = $1
          AND formula_version = 'v1.1'
        "#,
    )
    .bind(artifact_id)
    .fetch_optional(db)
    .await?;

    Ok(row.into_profile(
        signals,
        previous_overall,
        config.structural_stale_secs,
        reliability_min_sample,
    ))
}

pub async fn get_repo_signals(db: &PgPool, artifact_id: Uuid) -> Result<Vec<RepoSignal>, ApiError> {
    let signals: Vec<SignalRow> = sqlx::query_as(
        r#"
        SELECT
          id                      AS id,
          signal::text            AS signal,
          is_passive              AS is_passive,
          evidence_url            AS evidence_url,
          evidence_description    AS evidence_description,
          review_status           AS review_status,
          review_note             AS review_note,
          disputed_at             AS disputed_at,
          dispute_reason          AS dispute_reason,
          created_at              AS created_at
        FROM quality_signals
        WHERE external_artifact_id = $1
        ORDER BY created_at DESC
        LIMIT 10
        "#,
    )
    .bind(artifact_id)
    .fetch_all(db)
    .await?;
    let ids = signals.iter().map(|s| s.id).collect::<Vec<_>>();
    let events = signal_events::list_events_for_signals(db, &ids).await?;

    Ok(signals
        .into_iter()
        .map(|signal| {
            let signal_id = signal.id;
            let event_list = events.get(&signal_id).cloned().unwrap_or_default();
            signal.into_signal(event_list)
        })
        .collect())
}
