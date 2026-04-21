use serde_json::Value;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    domain::reference::{QualityContext, ResolvedSnippet, SnippetReference},
    services::quality::scoring::load_v1,
};

pub async fn resolve_reference(
    db: &PgPool,
    reference: &SnippetReference,
    user_id: Option<Uuid>,
) -> Result<ResolvedSnippet, ApiError> {
    let formula_version = load_v1()?.meta.version;

    let target_version = reference.version.as_deref();

    let row: ResolvedRow = sqlx::query_as(
        r#"
        SELECT
          s.id                   AS snippet_id,
          l.slug                 AS library_slug,
          s.slug                 AS snippet_slug,
          s.name                 AS name,
          s.description          AS description,
          s.language             AS language,
          sv.version             AS version,
          sv.code                AS code,
          sv.dependencies        AS dependencies,
          ascore.formula_version AS quality_formula_version,
          ascore.freshness       AS quality_freshness,
          ascore.adoption        AS quality_adoption,
          ascore.reliability     AS quality_reliability,
          ascore.abandonment     AS quality_abandonment,
          ascore.overall         AS quality_overall,
          ascore.flags           AS quality_flags,
          ascore.computed_at     AS quality_computed_at
        FROM libraries l
        JOIN snippets s          ON s.library_id = l.id
        JOIN snippet_versions sv ON sv.snippet_id = s.id
          AND (
            ($3::text IS NULL     AND sv.id = s.current_version_id) OR
            ($3::text IS NOT NULL AND sv.version = $3)
          )
        LEFT JOIN artifact_scores ascore
          ON ascore.snippet_id = s.id
          AND ascore.formula_version = $4
        WHERE l.slug = $1
          AND s.slug = $2
          AND (s.visibility = 'public' OR s.owner_id = $5)
        "#,
    )
    .bind(&reference.library)
    .bind(&reference.snippet)
    .bind(target_version)
    .bind(&formula_version)
    .bind(user_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Reference not resolvable"))?;

    let canonical_reference = format!("{}:{}@{}", row.library_slug, row.snippet_slug, row.version);
    let quality = row.quality();

    Ok(ResolvedSnippet {
        reference: format!("@{}", canonical_reference),
        snippet_id: row.snippet_id,
        library_slug: row.library_slug,
        snippet_slug: row.snippet_slug,
        name: row.name,
        description: row.description,
        language: row.language,
        version: row.version,
        code: row.code,
        dependencies: row.dependencies,
        canonical_reference,
        quality,
    })
}

#[derive(FromRow)]
struct ResolvedRow {
    snippet_id: Uuid,
    library_slug: String,
    snippet_slug: String,
    name: String,
    description: Option<String>,
    language: String,
    version: String,
    code: String,
    dependencies: Value,
    quality_formula_version: Option<String>,
    quality_freshness: Option<f64>,
    quality_adoption: Option<f64>,
    quality_reliability: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_overall: Option<f64>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ResolvedRow {
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
}
