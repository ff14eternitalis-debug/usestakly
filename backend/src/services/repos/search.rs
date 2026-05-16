use sqlx::PgPool;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::repo::RepoSearchResult,
    services::{quality::load_v2, repo_explain::ExplainContext, semantic_search},
};

use super::{
    RepoSearchFilters,
    normalize::{normalize_maturity_bands, normalize_topics, tokenize_query},
    rows::RepoRow,
};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 200;
const LEXICAL_MIN_SCORE: f64 = 0.35;
const SEMANTIC_MIN_SCORE: f64 = 0.2;

pub async fn search_github_repos(
    db: &PgPool,
    config: &AppConfig,
    filters: &RepoSearchFilters,
) -> Result<Vec<RepoSearchResult>, ApiError> {
    let formula_version = load_v2()?.meta.version;
    let limit = filters.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = filters.offset.unwrap_or_default().max(0);
    let topics = normalize_topics(&filters.topics);
    let maturity_bands = normalize_maturity_bands(&filters.maturity_bands);
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
          LEFT JOIN repo_radar_snapshots radar
            ON radar.external_artifact_id = e.id
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
            OR EXISTS (
              SELECT 1
              FROM repo_categories rc
              WHERE rc.external_artifact_id = artifact_id
                AND rc.category ILIKE '%' || $2 || '%'
            )
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
                OR EXISTS (
                  SELECT 1
                  FROM repo_categories rc
                  WHERE rc.external_artifact_id = artifact_id
                    AND rc.category ILIKE required_topic
                )
              )
            )
          )
          AND ($15::float8 IS NULL OR COALESCE(quality_overall, 0.0) >= $15)
          AND ($16::float8 IS NULL OR COALESCE(quality_abandonment, 1.0) <= $16)
          AND (
            COALESCE(cardinality($18::text[]), 0) = 0
            OR radar_maturity_band = ANY($18::text[])
          )
        ORDER BY
                 CASE WHEN $17 = 'trend' THEN radar_trend_signal END DESC NULLS LAST,
                 CASE WHEN $17 = 'trend' THEN radar_relevance END DESC NULLS LAST,
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
    .bind(&maturity_bands)
    .fetch_all(db)
    .await?;

    let explain_context = ExplainContext {
        filter: filters.filter,
        query: filters.query.clone(),
        query_tokens: query_tokens.clone(),
        topics_filter: topics.clone(),
    };

    Ok(rows
        .into_iter()
        .map(|row| row.into_search_result_with_explanation(&explain_context))
        .collect())
}
