use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::{
        reference::{QualityContext, SearchFilter},
        repo::{RepoProfile, RepoSearchResult, RepoSignal},
    },
    services::{quality::load_v1, semantic_search, trust::signal_events},
};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 200;
const LEXICAL_MIN_SCORE: f64 = 0.35;
const SEMANTIC_MIN_SCORE: f64 = 0.2;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RepoSort {
    #[default]
    Score,
    Stars,
    Recency,
    Abandonment,
}

impl RepoSort {
    pub fn parse(input: Option<&str>) -> Self {
        match input
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref()
        {
            Some("stars") => Self::Stars,
            Some("recency") | Some("recent") | Some("freshness") => Self::Recency,
            Some("abandonment") | Some("risk") => Self::Abandonment,
            _ => Self::Score,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Score => "score",
            Self::Stars => "stars",
            Self::Recency => "recency",
            Self::Abandonment => "abandonment",
        }
    }
}

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
    pub topics: Vec<String>,
    pub score_min: Option<f64>,
    pub abandonment_max: Option<f64>,
    pub include_archived: bool,
    pub sort: RepoSort,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn search_github_repos(
    db: &PgPool,
    config: &AppConfig,
    filters: &RepoSearchFilters,
) -> Result<Vec<RepoSearchResult>, ApiError> {
    let formula_version = load_v1()?.meta.version;
    let limit = filters.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = filters.offset.unwrap_or_default().max(0);
    let topics = normalize_topics(&filters.topics);
    let score_min = filters
        .score_min
        .filter(|value| (0.0..=1.0).contains(value));
    let abandonment_max = filters
        .abandonment_max
        .filter(|value| (0.0..=1.0).contains(value));
    let query_tokens = tokenize_query(filters.query.as_deref());
    let semantic_query =
        semantic_search::embed_query(filters.query.as_deref().unwrap_or(""), config)
            .await?
            .map(|embedding| semantic_search::to_pgvector_literal(&embedding));

    let rows: Vec<RepoRow> = sqlx::query_as(
        r#"
        WITH repo_candidates AS (
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
            ascore.formula_version      AS quality_formula_version,
            ascore.freshness::float8    AS quality_freshness,
            ascore.adoption::float8     AS quality_adoption,
            ascore.reliability::float8  AS quality_reliability,
            ascore.abandonment::float8  AS quality_abandonment,
            ascore.overall::float8      AS quality_overall,
            ascore.resolve_count        AS quality_resolve_count,
            ascore.build_success_count  AS quality_build_success_count,
            ascore.build_failure_count  AS quality_build_failure_count,
            ascore.regret_count         AS quality_regret_count,
            ascore.flags                AS quality_flags,
            ascore.computed_at          AS quality_computed_at,
            CASE
              WHEN $9::text IS NULL OR e.embedding IS NULL THEN NULL
              ELSE (1 - (e.embedding <=> CAST($9 AS vector(384))))::float8
            END AS semantic_score,
            CASE
              WHEN $2::text IS NULL THEN NULL
              ELSE LEAST(
                1.0::float8,
                COALESCE((
                  SELECT AVG(
                    CASE
                      WHEN e.github_owner ILIKE '%' || token || '%'
                        OR e.github_repo ILIKE '%' || token || '%'
                        OR (e.github_owner || '/' || e.github_repo) ILIKE '%' || token || '%'
                        THEN 1.0::float8
                      WHEN EXISTS (SELECT 1 FROM unnest(e.topics) topic WHERE topic ILIKE '%' || token || '%')
                        THEN 0.95::float8
                      WHEN COALESCE(e.language, '') ILIKE '%' || token || '%'
                        THEN 0.75::float8
                      WHEN COALESCE(e.description, '') ILIKE '%' || token || '%'
                        THEN 0.60::float8
                      ELSE 0.0::float8
                    END
                  )
                  FROM unnest(COALESCE($10::text[], ARRAY[]::text[])) token
                ), 0.0)
                + CASE
                    WHEN (e.github_owner || '/' || e.github_repo) ILIKE '%' || $2 || '%' THEN 0.35::float8
                    WHEN e.github_repo ILIKE '%' || $2 || '%' THEN 0.25::float8
                    WHEN COALESCE(e.description, '') ILIKE '%' || $2 || '%' THEN 0.15::float8
                    ELSE 0.0::float8
                  END
              )::float8
            END AS lexical_score
          FROM external_artifacts e
          LEFT JOIN artifact_scores ascore
            ON ascore.external_artifact_id = e.id
            AND ascore.formula_version = $1
          WHERE e.source = 'github'
            AND e.github_owner IS NOT NULL
            AND e.github_repo IS NOT NULL
            AND ($4::text IS NULL OR e.language ILIKE $4)
            AND ($5::text IS NULL OR e.license_spdx = $5)
            AND ($6::int  IS NULL OR e.stars_count >= $6)
            AND ($7 OR e.archived = FALSE)
        )
        SELECT *
        FROM repo_candidates
        WHERE (
            $2::text IS NULL
            OR owner ILIKE '%' || $2 || '%'
            OR name ILIKE '%' || $2 || '%'
            OR COALESCE(description, '') ILIKE '%' || $2 || '%'
            OR EXISTS (SELECT 1 FROM unnest(topics) t WHERE t ILIKE '%' || $2 || '%')
            OR COALESCE(lexical_score, 0.0) >= $11
            OR COALESCE(semantic_score, 0.0) >= $12
          )
          AND (
            $3 = 'explore'
            OR (
              quality_formula_version IS NOT NULL
              AND (
                (
                  $3 = 'auto'
                  AND COALESCE(quality_overall, 0.0) >= 0.45
                  AND COALESCE(quality_abandonment, 1.0) <= 0.35
                  AND NOT ('security-issue' = ANY(COALESCE(quality_flags, ARRAY[]::text[])))
                  AND NOT ('broken' = ANY(COALESCE(quality_flags, ARRAY[]::text[])))
                )
                OR (
                  $3 = 'strict'
                  AND COALESCE(quality_overall, 0.0) >= 0.60
                  AND COALESCE(quality_freshness, 0.0) >= 0.75
                  AND COALESCE(quality_abandonment, 1.0) <= 0.20
                  AND COALESCE(array_length(quality_flags, 1), 0) = 0
                )
              )
            )
          )
          AND (
            COALESCE(cardinality($14::text[]), 0) = 0
            OR NOT EXISTS (
              SELECT 1
              FROM unnest($14::text[]) required_topic
              WHERE NOT (
                EXISTS (
                  SELECT 1
                  FROM unnest(topics) topic
                  WHERE topic ILIKE required_topic
                )
                OR COALESCE(description, '') ILIKE '%' || required_topic || '%'
                OR name ILIKE '%' || required_topic || '%'
              )
            )
          )
          AND ($15::float8 IS NULL OR COALESCE(quality_overall, 0.0) >= $15)
          AND ($16::float8 IS NULL OR COALESCE(quality_abandonment, 1.0) <= $16)
        ORDER BY
                 CASE WHEN $17 = 'stars' THEN stars_count END DESC NULLS LAST,
                 CASE WHEN $17 = 'recency' THEN last_commit_at END DESC NULLS LAST,
                 CASE WHEN $17 = 'abandonment' THEN quality_abandonment END ASC NULLS LAST,
                 CASE WHEN $17 = 'score' THEN quality_overall END DESC NULLS LAST,
                 CASE
                   WHEN $2::text IS NULL THEN quality_overall
                   ELSE (
                     COALESCE(quality_overall, 0.0) * 0.35
                     + COALESCE(lexical_score, 0.0) * 0.40
                     + COALESCE(semantic_score, 0.0) * 0.25
                   )
                 END DESC NULLS LAST,
                 COALESCE(lexical_score, 0.0) DESC NULLS LAST,
                 COALESCE(semantic_score, 0.0) DESC NULLS LAST,
                 quality_overall DESC NULLS LAST,
                 stars_count DESC,
                 last_commit_at DESC NULLS LAST
        LIMIT $8
        OFFSET $13
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
    .bind(semantic_query)
    .bind(&query_tokens)
    .bind(LEXICAL_MIN_SCORE)
    .bind(SEMANTIC_MIN_SCORE)
    .bind(offset)
    .bind(&topics)
    .bind(score_min)
    .bind(abandonment_max)
    .bind(filters.sort.as_str())
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
    quality_resolve_count: Option<i32>,
    quality_build_success_count: Option<i32>,
    quality_build_failure_count: Option<i32>,
    quality_regret_count: Option<i32>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    lexical_score: Option<f64>,
    #[allow(dead_code)]
    semantic_score: Option<f64>,
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
            resolve_count: self.quality_resolve_count.unwrap_or_default(),
            build_success_count: self.quality_build_success_count.unwrap_or_default(),
            build_failure_count: self.quality_build_failure_count.unwrap_or_default(),
            regret_count: self.quality_regret_count.unwrap_or_default(),
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
    quality_resolve_count: Option<i32>,
    quality_build_success_count: Option<i32>,
    quality_build_failure_count: Option<i32>,
    quality_regret_count: Option<i32>,
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
            quality_resolve_count: self.quality_resolve_count,
            quality_build_success_count: self.quality_build_success_count,
            quality_build_failure_count: self.quality_build_failure_count,
            quality_regret_count: self.quality_regret_count,
            quality_flags: self.quality_flags,
            quality_computed_at: self.quality_computed_at,
            lexical_score: None,
            semantic_score: None,
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

fn tokenize_query(query: Option<&str>) -> Vec<String> {
    let mut tokens = Vec::new();
    for token in query
        .unwrap_or_default()
        .split(|c: char| !c.is_alphanumeric())
        .map(|token| token.trim().to_lowercase())
        .filter(|token| token.len() >= 2)
    {
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    tokens
}

fn normalize_topics(topics: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    for topic in topics {
        let topic = topic
            .trim()
            .trim_start_matches('#')
            .to_ascii_lowercase()
            .replace('_', "-");
        if !topic.is_empty() && !normalized.contains(&topic) {
            normalized.push(topic);
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::{normalize_topics, tokenize_query};

    #[test]
    fn tokenize_query_normalizes_and_deduplicates() {
        let tokens = tokenize_query(Some("React UI, react  typescript"));
        assert_eq!(tokens, vec!["react", "ui", "typescript"]);
    }

    #[test]
    fn tokenize_query_drops_short_empty_tokens() {
        let tokens = tokenize_query(Some("a / c++ / rpc"));
        assert_eq!(tokens, vec!["rpc"]);
    }

    #[test]
    fn normalize_topics_deduplicates_and_normalizes() {
        let topics = normalize_topics(&[
            "#React".to_string(),
            "data_grid".to_string(),
            "react".to_string(),
        ]);
        assert_eq!(topics, vec!["react", "data-grid"]);
    }
}
