use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    domain::{
        reference::{QualityContext, SearchFilter},
        repo::{RepoProfile, RepoSearchResult, RepoSignal},
    },
    services::{quality::scoring::load_v1, trust::signal_events},
};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 200;

pub async fn find_github_artifact_id(
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<Option<Uuid>, ApiError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT id
        FROM external_artifacts
        WHERE source = 'github'
          AND github_owner = $1
          AND github_repo = $2
        LIMIT 1
        "#,
    )
    .bind(owner)
    .bind(name)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|(id,)| id))
}

#[derive(Debug, Clone, Default)]
pub struct RepoSearchFilters {
    pub query: Option<String>,
    pub filter: SearchFilter,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub stars_min: Option<i32>,
    pub include_archived: bool,
    pub limit: Option<i64>,
}

pub async fn search_github_repos(
    db: &PgPool,
    filters: &RepoSearchFilters,
) -> Result<Vec<RepoSearchResult>, ApiError> {
    let formula_version = load_v1()?.meta.version;
    let limit = filters.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

    let rows: Vec<RepoRow> = sqlx::query_as(
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
          e.last_commit_at            AS last_commit_at,
          ascore.formula_version      AS quality_formula_version,
          ascore.freshness::float8    AS quality_freshness,
          ascore.adoption::float8     AS quality_adoption,
          ascore.reliability::float8  AS quality_reliability,
          ascore.abandonment::float8  AS quality_abandonment,
          ascore.overall::float8      AS quality_overall,
          ascore.flags                AS quality_flags,
          ascore.computed_at          AS quality_computed_at
        FROM external_artifacts e
        LEFT JOIN artifact_scores ascore
          ON ascore.external_artifact_id = e.id
          AND ascore.formula_version = $1
        WHERE e.source = 'github'
          AND e.github_owner IS NOT NULL
          AND e.github_repo IS NOT NULL
          AND (
            $2::text IS NULL
            OR e.github_owner ILIKE '%' || $2 || '%'
            OR e.github_repo  ILIKE '%' || $2 || '%'
            OR COALESCE(e.description, '') ILIKE '%' || $2 || '%'
            OR EXISTS (SELECT 1 FROM unnest(e.topics) t WHERE t ILIKE '%' || $2 || '%')
          )
          AND ($4::text IS NULL OR e.language ILIKE $4)
          AND ($5::text IS NULL OR e.license_spdx = $5)
          AND ($6::int  IS NULL OR e.stars_count >= $6)
          AND ($7 OR e.archived = FALSE)
          AND (
            $3 = 'explore'
            OR (
              ascore.id IS NOT NULL
              AND (
                (
                  $3 = 'auto'
                  AND ascore.reliability >= 0.9
                  AND ascore.abandonment <= 0.3
                  AND NOT ('security-issue' = ANY(ascore.flags))
                  AND NOT ('broken' = ANY(ascore.flags))
                )
                OR (
                  $3 = 'strict'
                  AND ascore.reliability >= 0.95
                  AND ascore.abandonment <= 0.2
                  AND ascore.overall >= 0.85
                  AND COALESCE(array_length(ascore.flags, 1), 0) = 0
                )
              )
            )
          )
        ORDER BY ascore.overall DESC NULLS LAST,
                 e.stars_count DESC,
                 e.last_commit_at DESC NULLS LAST
        LIMIT $8
        "#,
    )
    .bind(&formula_version)
    .bind(filters.query.as_deref())
    .bind(filters.filter.as_str())
    .bind(filters.language.as_deref())
    .bind(filters.license_spdx.as_deref())
    .bind(filters.stars_min)
    .bind(filters.include_archived)
    .bind(limit)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(RepoRow::into_search_result).collect())
}

pub async fn get_repo_profile(db: &PgPool, artifact_id: Uuid) -> Result<RepoProfile, ApiError> {
    let formula_version = load_v1()?.meta.version;

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
          e.subscribers_count         AS subscribers_count,
          e.default_branch            AS default_branch,
          e.priors_fetched_at         AS priors_fetched_at,
          ascore.formula_version      AS quality_formula_version,
          ascore.freshness::float8    AS quality_freshness,
          ascore.adoption::float8     AS quality_adoption,
          ascore.reliability::float8  AS quality_reliability,
          ascore.abandonment::float8  AS quality_abandonment,
          ascore.overall::float8      AS quality_overall,
          ascore.flags                AS quality_flags,
          ascore.computed_at          AS quality_computed_at
        FROM external_artifacts e
        LEFT JOIN artifact_scores ascore
          ON ascore.external_artifact_id = e.id
          AND ascore.formula_version = $1
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

    Ok(row.into_profile(signals))
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

#[derive(FromRow)]
struct RepoRow {
    artifact_id: Uuid,
    owner: Option<String>,
    name: Option<String>,
    html_url: Option<String>,
    description: Option<String>,
    language: Option<String>,
    license_spdx: Option<String>,
    topics: Vec<String>,
    stars_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    archived: bool,
    last_commit_at: Option<DateTime<Utc>>,
    quality_formula_version: Option<String>,
    quality_freshness: Option<f64>,
    quality_adoption: Option<f64>,
    quality_reliability: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_overall: Option<f64>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<DateTime<Utc>>,
}

impl RepoRow {
    fn quality(&self) -> Option<QualityContext> {
        let formula_version = self.quality_formula_version.clone()?;
        let computed_at = self.quality_computed_at?;
        Some(QualityContext {
            formula_version,
            freshness: self.quality_freshness,
            adoption: self.quality_adoption,
            reliability: self.quality_reliability,
            abandonment: self.quality_abandonment,
            overall: self.quality_overall,
            flags: self.quality_flags.clone().unwrap_or_default(),
            computed_at,
        })
    }

    fn into_search_result(self) -> RepoSearchResult {
        let owner = self.owner.clone().unwrap_or_default();
        let name = self.name.clone().unwrap_or_default();
        let full_name = format!("{owner}/{name}");
        let html_url = self
            .html_url
            .clone()
            .unwrap_or_else(|| format!("https://github.com/{full_name}"));
        let quality = self.quality();
        RepoSearchResult {
            artifact_id: self.artifact_id,
            owner,
            name,
            full_name,
            html_url,
            description: self.description,
            language: self.language,
            license_spdx: self.license_spdx,
            topics: self.topics,
            stars_count: self.stars_count,
            forks_count: self.forks_count,
            open_issues_count: self.open_issues_count,
            archived: self.archived,
            last_commit_at: self.last_commit_at,
            quality,
        }
    }
}

#[derive(FromRow)]
struct ProfileRow {
    artifact_id: Uuid,
    owner: Option<String>,
    name: Option<String>,
    html_url: Option<String>,
    description: Option<String>,
    language: Option<String>,
    license_spdx: Option<String>,
    topics: Vec<String>,
    stars_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    archived: bool,
    last_commit_at: Option<DateTime<Utc>>,
    subscribers_count: i32,
    default_branch: Option<String>,
    priors_fetched_at: Option<DateTime<Utc>>,
    quality_formula_version: Option<String>,
    quality_freshness: Option<f64>,
    quality_adoption: Option<f64>,
    quality_reliability: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_overall: Option<f64>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<DateTime<Utc>>,
}

impl ProfileRow {
    fn into_profile(self, recent_signals: Vec<RepoSignal>) -> RepoProfile {
        let approved_flags = self.quality_flags.clone().unwrap_or_default();
        let repo = RepoRow {
            artifact_id: self.artifact_id,
            owner: self.owner,
            name: self.name,
            html_url: self.html_url,
            description: self.description,
            language: self.language,
            license_spdx: self.license_spdx,
            topics: self.topics,
            stars_count: self.stars_count,
            forks_count: self.forks_count,
            open_issues_count: self.open_issues_count,
            archived: self.archived,
            last_commit_at: self.last_commit_at,
            quality_formula_version: self.quality_formula_version,
            quality_freshness: self.quality_freshness,
            quality_adoption: self.quality_adoption,
            quality_reliability: self.quality_reliability,
            quality_abandonment: self.quality_abandonment,
            quality_overall: self.quality_overall,
            quality_flags: self.quality_flags,
            quality_computed_at: self.quality_computed_at,
        }
        .into_search_result();
        RepoProfile {
            repo,
            subscribers_count: self.subscribers_count,
            default_branch: self.default_branch,
            priors_fetched_at: self.priors_fetched_at,
            recent_signals: recent_signals
                .into_iter()
                .filter(|signal| {
                    signal.is_passive
                        || approved_flags
                            .iter()
                            .any(|flag| flag == &normalize_public_signal(&signal.signal))
                })
                .collect(),
        }
    }
}

#[derive(FromRow)]
struct SignalRow {
    id: Uuid,
    signal: String,
    is_passive: bool,
    evidence_url: Option<String>,
    evidence_description: Option<String>,
    review_status: String,
    review_note: Option<String>,
    disputed_at: Option<DateTime<Utc>>,
    dispute_reason: Option<String>,
    created_at: DateTime<Utc>,
}

impl SignalRow {
    fn into_signal(self, events: Vec<crate::domain::repo::RepoSignalEvent>) -> RepoSignal {
        RepoSignal {
            id: self.id,
            signal: self.signal,
            is_passive: self.is_passive,
            evidence_url: self.evidence_url,
            evidence_description: self.evidence_description,
            review_status: self.review_status,
            review_note: self.review_note,
            disputed_at: self.disputed_at,
            dispute_reason: self.dispute_reason,
            created_at: self.created_at,
            events,
        }
    }
}

fn normalize_public_signal(signal: &str) -> String {
    match signal {
        "security_issue" => "security-issue".to_string(),
        other => other.to_string(),
    }
}
