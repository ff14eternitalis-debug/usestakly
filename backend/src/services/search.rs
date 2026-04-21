use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    domain::reference::{QualityContext, SearchFilter, SearchResult},
    services::quality::scoring::load_v1,
};

const DEFAULT_LIMIT: i64 = 50;

pub async fn search_snippets(
    db: &PgPool,
    query: Option<&str>,
    filter: SearchFilter,
    user_id: Option<Uuid>,
) -> Result<Vec<SearchResult>, ApiError> {
    let formula_version = load_v1()?.meta.version;

    let rows: Vec<SearchRow> = sqlx::query_as(
        r#"
        SELECT
          s.id                   AS snippet_id,
          l.slug                 AS library_slug,
          s.slug                 AS snippet_slug,
          s.name                 AS name,
          s.description          AS description,
          s.language             AS language,
          sv.version             AS current_version,
          ascore.formula_version AS quality_formula_version,
          ascore.freshness       AS quality_freshness,
          ascore.adoption        AS quality_adoption,
          ascore.reliability     AS quality_reliability,
          ascore.abandonment     AS quality_abandonment,
          ascore.overall         AS quality_overall,
          ascore.flags           AS quality_flags,
          ascore.computed_at     AS quality_computed_at
        FROM snippets s
        JOIN libraries l ON l.id = s.library_id
        LEFT JOIN snippet_versions sv ON sv.id = s.current_version_id
        LEFT JOIN artifact_scores ascore
          ON ascore.snippet_id = s.id
          AND ascore.formula_version = $1
        WHERE (s.visibility = 'public' OR s.owner_id = $2)
          AND (
            $3::text IS NULL OR
            s.slug ILIKE '%' || $3 || '%' OR
            s.name ILIKE '%' || $3 || '%' OR
            COALESCE(s.description, '') ILIKE '%' || $3 || '%'
          )
          AND (
            $4 = 'explore'
            OR (
              ascore.id IS NOT NULL
              AND (
                (
                  $4 = 'auto'
                  AND ascore.reliability >= 0.9
                  AND ascore.abandonment <= 0.3
                  AND NOT ('security-issue' = ANY(ascore.flags))
                  AND NOT ('broken' = ANY(ascore.flags))
                )
                OR (
                  $4 = 'strict'
                  AND ascore.reliability >= 0.95
                  AND ascore.abandonment <= 0.2
                  AND ascore.overall >= 0.85
                  AND COALESCE(array_length(ascore.flags, 1), 0) = 0
                )
              )
            )
          )
        ORDER BY ascore.overall DESC NULLS LAST, s.updated_at DESC
        LIMIT $5
        "#,
    )
    .bind(&formula_version)
    .bind(user_id)
    .bind(query)
    .bind(filter.as_str())
    .bind(DEFAULT_LIMIT)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(SearchRow::into_result).collect())
}

#[derive(FromRow)]
struct SearchRow {
    snippet_id: Uuid,
    library_slug: String,
    snippet_slug: String,
    name: String,
    description: Option<String>,
    language: String,
    current_version: Option<String>,
    quality_formula_version: Option<String>,
    quality_freshness: Option<f64>,
    quality_adoption: Option<f64>,
    quality_reliability: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_overall: Option<f64>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<DateTime<Utc>>,
}

impl SearchRow {
    fn into_result(self) -> SearchResult {
        let canonical_reference = match &self.current_version {
            Some(v) => format!("{}:{}@{}", self.library_slug, self.snippet_slug, v),
            None => format!("{}:{}", self.library_slug, self.snippet_slug),
        };
        let quality = match (self.quality_formula_version, self.quality_computed_at) {
            (Some(formula_version), Some(computed_at)) => Some(QualityContext {
                formula_version,
                freshness: self.quality_freshness,
                adoption: self.quality_adoption,
                reliability: self.quality_reliability,
                abandonment: self.quality_abandonment,
                overall: self.quality_overall,
                flags: self.quality_flags.unwrap_or_default(),
                computed_at,
            }),
            _ => None,
        };
        SearchResult {
            snippet_id: self.snippet_id,
            library_slug: self.library_slug,
            snippet_slug: self.snippet_slug,
            name: self.name,
            description: self.description,
            language: self.language,
            current_version: self.current_version,
            canonical_reference,
            quality,
        }
    }
}
