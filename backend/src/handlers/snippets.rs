use axum::{
    Json,
    extract::{Path, Query, State},
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::snippet::{
        CreateSnippetRequest, CreateSnippetVersionRequest, SnippetDetail, SnippetRecord,
        SnippetVersionRecord, UpdateSnippetRequest,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetListQuery {
    pub library_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetListResponse {
    pub items: Vec<SnippetDetail>,
}

pub async fn list_snippets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SnippetListQuery>,
) -> Result<Json<SnippetListResponse>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let snippets = if let Some(library_id) = query.library_id {
        sqlx::query_as::<_, SnippetRecord>(
            r#"
            SELECT
              id,
              library_id,
              owner_id,
              slug,
              domain::text AS domain,
              kind,
              category,
              name,
              description,
              language,
              runtime,
              framework,
              framework_version,
              visibility::text AS visibility,
              trust_level::text AS trust_level,
              license,
              current_version_id,
              rule_set_id,
              created_at,
              updated_at
            FROM snippets
            WHERE owner_id = $1 AND library_id = $2
            ORDER BY updated_at DESC
            "#,
        )
        .bind(user.id)
        .bind(library_id)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, SnippetRecord>(
            r#"
            SELECT
              id,
              library_id,
              owner_id,
              slug,
              domain::text AS domain,
              kind,
              category,
              name,
              description,
              language,
              runtime,
              framework,
              framework_version,
              visibility::text AS visibility,
              trust_level::text AS trust_level,
              license,
              current_version_id,
              rule_set_id,
              created_at,
              updated_at
            FROM snippets
            WHERE owner_id = $1
            ORDER BY updated_at DESC
            "#,
        )
        .bind(user.id)
        .fetch_all(&state.db)
        .await?
    };

    let items = hydrate_details(&state, snippets).await?;
    Ok(Json(SnippetListResponse { items }))
}

pub async fn get_snippet(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(snippet_id): Path<Uuid>,
) -> Result<Json<SnippetDetail>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let snippet = find_snippet_record(&state, snippet_id, user.id).await?;
    Ok(Json(hydrate_detail(&state, snippet).await?))
}

pub async fn create_snippet(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateSnippetRequest>,
) -> Result<Json<SnippetDetail>, ApiError> {
    payload.validate()?;
    validate_snippet_inputs(
        &payload.slug,
        &payload.domain,
        payload.visibility.as_deref(),
    )?;

    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    ensure_library_access(&state, payload.library_id, user.id).await?;
    let mut tx = state.db.begin().await?;
    let visibility = normalize_visibility(payload.visibility.as_deref())?;

    let snippet = sqlx::query_as::<_, SnippetRecord>(
        r#"
        INSERT INTO snippets (
          library_id,
          owner_id,
          slug,
          domain,
          kind,
          category,
          name,
          description,
          language,
          runtime,
          framework,
          framework_version,
          visibility,
          trust_level,
          license
        )
        VALUES (
          $1,
          $2,
          $3,
          CAST($4 AS snippet_domain),
          $5,
          $6,
          $7,
          $8,
          $9,
          $10,
          $11,
          $12,
          CAST($13 AS visibility),
          $14,
          $15
        )
        RETURNING
          id,
          library_id,
          owner_id,
          slug,
          domain::text AS domain,
          kind,
          category,
          name,
          description,
          language,
          runtime,
          framework,
          framework_version,
          visibility::text AS visibility,
          trust_level::text AS trust_level,
          license,
          current_version_id,
          rule_set_id,
          created_at,
          updated_at
        "#,
    )
    .bind(payload.library_id)
    .bind(user.id)
    .bind(payload.slug)
    .bind(payload.domain)
    .bind(payload.kind)
    .bind(payload.category)
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.language)
    .bind(payload.runtime)
    .bind(payload.framework)
    .bind(payload.framework_version)
    .bind(visibility.clone())
    .bind(trust_level_for_visibility(&visibility))
    .bind(payload.license.unwrap_or_else(|| "MIT".to_string()))
    .fetch_one(&mut *tx)
    .await?;

    let version = insert_version(&mut tx, snippet.id, payload.initial_version).await?;
    sqlx::query("UPDATE snippets SET current_version_id = $1 WHERE id = $2")
        .bind(version.id)
        .bind(snippet.id)
        .execute(&mut *tx)
        .await?;

    sync_tags(&mut tx, snippet.id, payload.tags.unwrap_or_default()).await?;
    tx.commit().await?;

    let detail = hydrate_detail(
        &state,
        find_snippet_record(&state, snippet.id, user.id).await?,
    )
    .await?;
    Ok(Json(detail))
}

pub async fn update_snippet(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(snippet_id): Path<Uuid>,
    Json(payload): Json<UpdateSnippetRequest>,
) -> Result<Json<SnippetDetail>, ApiError> {
    payload.validate()?;
    if let Some(slug) = &payload.slug {
        validate_snippet_slug(slug)?;
    }
    if let Some(visibility) = payload.visibility.as_deref() {
        normalize_visibility(Some(visibility))?;
    }

    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    find_snippet_record(&state, snippet_id, user.id).await?;
    let mut tx = state.db.begin().await?;
    let mut builder = sqlx::QueryBuilder::new("UPDATE snippets SET ");
    let mut separated = builder.separated(", ");
    let mut changed = false;

    if let Some(slug) = payload.slug {
        changed = true;
        separated.push("slug = ").push_bind(slug);
    }
    if let Some(name) = payload.name {
        changed = true;
        separated.push("name = ").push_bind(name);
    }
    if let Some(category) = payload.category {
        changed = true;
        separated.push("category = ").push_bind(category);
    }
    if let Some(description) = payload.description {
        changed = true;
        separated.push("description = ").push_bind(description);
    }
    if let Some(runtime) = payload.runtime {
        changed = true;
        separated.push("runtime = ").push_bind(runtime);
    }
    if let Some(framework) = payload.framework {
        changed = true;
        separated.push("framework = ").push_bind(framework);
    }
    if let Some(framework_version) = payload.framework_version {
        changed = true;
        separated
            .push("framework_version = ")
            .push_bind(framework_version);
    }
    if let Some(visibility) = payload.visibility {
        changed = true;
        let normalized = normalize_visibility(Some(&visibility))?;
        separated
            .push("visibility = CAST(")
            .push_bind(normalized.clone())
            .push(" AS visibility)");
        separated
            .push("trust_level = ")
            .push_bind(trust_level_for_visibility(&normalized));
    }
    if let Some(license) = payload.license {
        changed = true;
        separated.push("license = ").push_bind(license);
    }

    if !changed && payload.tags.is_none() {
        return Err(ApiError::bad_request(
            "No snippet fields provided for update",
        ));
    }

    if changed {
        separated.push("updated_at = NOW()");
        builder.push(" WHERE id = ").push_bind(snippet_id);
        builder.push(" AND owner_id = ").push_bind(user.id);
        builder.build().execute(&mut *tx).await?;
    }

    if let Some(tags) = payload.tags {
        sync_tags(&mut tx, snippet_id, tags).await?;
    }

    tx.commit().await?;
    let detail = hydrate_detail(
        &state,
        find_snippet_record(&state, snippet_id, user.id).await?,
    )
    .await?;
    Ok(Json(detail))
}

pub async fn delete_snippet(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(snippet_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let result = sqlx::query("DELETE FROM snippets WHERE id = $1 AND owner_id = $2")
        .bind(snippet_id)
        .bind(user.id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Snippet not found"));
    }

    Ok(Json(
        serde_json::json!({ "deleted": true, "id": snippet_id }),
    ))
}

pub async fn list_snippet_versions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(snippet_id): Path<Uuid>,
) -> Result<Json<Vec<SnippetVersionRecord>>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    find_snippet_record(&state, snippet_id, user.id).await?;
    let versions = sqlx::query_as::<_, SnippetVersionRecord>(
        r#"
        SELECT
          id,
          snippet_id,
          version,
          code,
          variables,
          css_classes,
          dependencies,
          exports,
          imports,
          compatibility,
          metadata,
          content_hash,
          risk_level,
          created_at
        FROM snippet_versions
        WHERE snippet_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(snippet_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(versions))
}

pub async fn create_snippet_version(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(snippet_id): Path<Uuid>,
    Json(payload): Json<CreateSnippetVersionRequest>,
) -> Result<Json<SnippetDetail>, ApiError> {
    payload.validate()?;
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    find_snippet_record(&state, snippet_id, user.id).await?;
    let mut tx = state.db.begin().await?;

    let version = insert_version(&mut tx, snippet_id, payload).await?;
    sqlx::query("UPDATE snippets SET current_version_id = $1 WHERE id = $2 AND owner_id = $3")
        .bind(version.id)
        .bind(snippet_id)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    let detail = hydrate_detail(
        &state,
        find_snippet_record(&state, snippet_id, user.id).await?,
    )
    .await?;
    Ok(Json(detail))
}

async fn hydrate_details(
    state: &AppState,
    snippets: Vec<SnippetRecord>,
) -> Result<Vec<SnippetDetail>, ApiError> {
    let mut items = Vec::with_capacity(snippets.len());
    for snippet in snippets {
        items.push(hydrate_detail(state, snippet).await?);
    }
    Ok(items)
}

async fn hydrate_detail(
    state: &AppState,
    snippet: SnippetRecord,
) -> Result<SnippetDetail, ApiError> {
    let tags = fetch_tags(&state.db, snippet.id).await?;
    let current_version = match snippet.current_version_id {
        Some(version_id) => Some(fetch_version_by_id(&state.db, version_id).await?),
        None => None,
    };
    let library_slug = fetch_library_slug(&state.db, snippet.library_id).await?;
    let canonical_reference = match &current_version {
        Some(version) => format!("{library_slug}:{}@{}", snippet.slug, version.version),
        None => format!("{library_slug}:{}", snippet.slug),
    };

    Ok(SnippetDetail {
        snippet,
        current_version,
        tags,
        canonical_reference,
    })
}

async fn find_snippet_record(
    state: &AppState,
    snippet_id: Uuid,
    owner_id: Uuid,
) -> Result<SnippetRecord, ApiError> {
    let snippet = sqlx::query_as::<_, SnippetRecord>(
        r#"
        SELECT
          id,
          library_id,
          owner_id,
          slug,
          domain::text AS domain,
          kind,
          category,
          name,
          description,
          language,
          runtime,
          framework,
          framework_version,
          visibility::text AS visibility,
          trust_level::text AS trust_level,
          license,
          current_version_id,
          rule_set_id,
          created_at,
          updated_at
        FROM snippets
        WHERE id = $1 AND owner_id = $2
        "#,
    )
    .bind(snippet_id)
    .bind(owner_id)
    .fetch_one(&state.db)
    .await?;

    Ok(snippet)
}

async fn ensure_library_access(
    state: &AppState,
    library_id: Uuid,
    owner_id: Uuid,
) -> Result<(), ApiError> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM libraries WHERE id = $1 AND owner_id = $2",
    )
    .bind(library_id)
    .bind(owner_id)
    .fetch_one(&state.db)
    .await?;

    if exists == 0 {
        return Err(ApiError::forbidden("Library not found for current user"));
    }

    Ok(())
}

async fn insert_version(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snippet_id: Uuid,
    payload: CreateSnippetVersionRequest,
) -> Result<SnippetVersionRecord, ApiError> {
    let content_hash = content_hash(&payload.code);
    let risk_level = payload.risk_level.unwrap_or_else(|| "safe".to_string());

    let version = sqlx::query_as::<_, SnippetVersionRecord>(
        r#"
        INSERT INTO snippet_versions (
          snippet_id,
          version,
          code,
          variables,
          css_classes,
          dependencies,
          exports,
          imports,
          compatibility,
          metadata,
          content_hash,
          risk_level
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING
          id,
          snippet_id,
          version,
          code,
          variables,
          css_classes,
          dependencies,
          exports,
          imports,
          compatibility,
          metadata,
          content_hash,
          risk_level,
          created_at
        "#,
    )
    .bind(snippet_id)
    .bind(payload.version)
    .bind(payload.code)
    .bind(payload.variables.unwrap_or_else(default_array))
    .bind(payload.css_classes)
    .bind(payload.dependencies.unwrap_or_else(default_array))
    .bind(payload.exports.unwrap_or_else(default_array))
    .bind(payload.imports.unwrap_or_else(default_array))
    .bind(payload.compatibility.unwrap_or_else(default_object))
    .bind(payload.metadata.unwrap_or_else(default_object))
    .bind(content_hash)
    .bind(risk_level)
    .fetch_one(&mut **tx)
    .await?;

    Ok(version)
}

async fn sync_tags(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    snippet_id: Uuid,
    tags: Vec<String>,
) -> Result<(), ApiError> {
    sqlx::query("DELETE FROM snippet_tags WHERE snippet_id = $1")
        .bind(snippet_id)
        .execute(&mut **tx)
        .await?;

    for raw_tag in tags {
        let tag = normalize_tag(&raw_tag)?;
        let tag_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO tags (name)
            VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id
            "#,
        )
        .bind(tag)
        .fetch_one(&mut **tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO snippet_tags (snippet_id, tag_id)
            VALUES ($1, $2)
            ON CONFLICT (snippet_id, tag_id) DO NOTHING
            "#,
        )
        .bind(snippet_id)
        .bind(tag_id)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

async fn fetch_tags(db: &sqlx::PgPool, snippet_id: Uuid) -> Result<Vec<String>, ApiError> {
    let tags = sqlx::query_scalar::<_, String>(
        r#"
        SELECT t.name
        FROM tags t
        JOIN snippet_tags st ON st.tag_id = t.id
        WHERE st.snippet_id = $1
        ORDER BY t.name ASC
        "#,
    )
    .bind(snippet_id)
    .fetch_all(db)
    .await?;

    Ok(tags)
}

async fn fetch_version_by_id(
    db: &sqlx::PgPool,
    version_id: Uuid,
) -> Result<SnippetVersionRecord, ApiError> {
    let version = sqlx::query_as::<_, SnippetVersionRecord>(
        r#"
        SELECT
          id,
          snippet_id,
          version,
          code,
          variables,
          css_classes,
          dependencies,
          exports,
          imports,
          compatibility,
          metadata,
          content_hash,
          risk_level,
          created_at
        FROM snippet_versions
        WHERE id = $1
        "#,
    )
    .bind(version_id)
    .fetch_one(db)
    .await?;

    Ok(version)
}

async fn fetch_library_slug(db: &sqlx::PgPool, library_id: Uuid) -> Result<String, ApiError> {
    let slug = sqlx::query_scalar::<_, String>("SELECT slug FROM libraries WHERE id = $1")
        .bind(library_id)
        .fetch_one(db)
        .await?;

    Ok(slug)
}

fn content_hash(code: &str) -> String {
    hex::encode(Sha256::digest(code.as_bytes()))
}

fn normalize_tag(input: &str) -> Result<String, ApiError> {
    let tag = input.trim().to_lowercase();
    if tag.is_empty() || tag.len() > 50 || tag.contains(' ') {
        return Err(ApiError::bad_request(
            "Tags must be non-empty, <= 50 chars, and contain no spaces",
        ));
    }
    Ok(tag)
}

fn validate_snippet_inputs(
    slug: &str,
    domain: &str,
    visibility: Option<&str>,
) -> Result<(), ApiError> {
    validate_snippet_slug(slug)?;
    normalize_domain(domain)?;
    normalize_visibility(visibility)?;
    Ok(())
}

fn validate_snippet_slug(slug: &str) -> Result<(), ApiError> {
    if slug.contains(' ') || slug.len() < 3 {
        return Err(ApiError::bad_request(
            "Snippet slug must be at least 3 characters and contain no spaces",
        ));
    }

    Ok(())
}

fn normalize_domain(domain: &str) -> Result<&str, ApiError> {
    match domain {
        "frontend" | "backend" | "devops" | "data" | "shared" => Ok(domain),
        _ => Err(ApiError::bad_request("Unsupported domain")),
    }
}

fn normalize_visibility(value: Option<&str>) -> Result<String, ApiError> {
    match value.unwrap_or("private") {
        "private" => Ok("private".to_string()),
        "public" => Ok("public".to_string()),
        other => Err(ApiError::bad_request(format!(
            "Unsupported visibility: {other}"
        ))),
    }
}

fn trust_level_for_visibility(visibility: &str) -> &'static str {
    match visibility {
        "public" => "public_unverified",
        _ => "private",
    }
}

fn default_object() -> serde_json::Value {
    serde_json::json!({})
}

fn default_array() -> serde_json::Value {
    serde_json::json!([])
}
