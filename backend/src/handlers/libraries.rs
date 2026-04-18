use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use serde::Serialize;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::library::{CreateLibraryRequest, LibraryRecord, UpdateLibraryRequest},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryListResponse {
    pub items: Vec<LibraryRecord>,
}

pub async fn list_libraries(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<LibraryListResponse>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;

    let items = sqlx::query_as::<_, LibraryRecord>(
        r#"
        SELECT
          id,
          owner_id,
          slug,
          name,
          description,
          visibility::text AS visibility,
          trust_level::text AS trust_level,
          is_default,
          default_stack,
          allowed_domains,
          metadata,
          created_at,
          updated_at
        FROM libraries
        WHERE owner_id = $1
        ORDER BY is_default DESC, updated_at DESC
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(LibraryListResponse { items }))
}

pub async fn get_library(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(library_id): Path<Uuid>,
) -> Result<Json<LibraryRecord>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let library = find_library_by_id(&state, library_id, user.id).await?;
    Ok(Json(library))
}

pub async fn create_library(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateLibraryRequest>,
) -> Result<Json<LibraryRecord>, ApiError> {
    payload.validate()?;
    validate_library_slug(&payload.slug)?;
    let visibility = normalize_visibility(payload.visibility.as_deref())?;
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let mut tx = state.db.begin().await?;

    if payload.is_default.unwrap_or(false) {
        sqlx::query("UPDATE libraries SET is_default = FALSE WHERE owner_id = $1")
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    }

    let library = sqlx::query_as::<_, LibraryRecord>(
        r#"
        INSERT INTO libraries (
          owner_id,
          slug,
          name,
          description,
          visibility,
          trust_level,
          is_default,
          default_stack,
          allowed_domains,
          metadata
        )
        VALUES ($1, $2, $3, $4, CAST($5 AS visibility), $6, $7, $8, $9, $10)
        RETURNING
          id,
          owner_id,
          slug,
          name,
          description,
          visibility::text AS visibility,
          trust_level::text AS trust_level,
          is_default,
          default_stack,
          allowed_domains,
          metadata,
          created_at,
          updated_at
        "#,
    )
    .bind(user.id)
    .bind(payload.slug)
    .bind(payload.name)
    .bind(payload.description)
    .bind(visibility.clone())
    .bind(trust_level_for_visibility(&visibility))
    .bind(payload.is_default.unwrap_or(false))
    .bind(payload.default_stack.unwrap_or_else(default_object))
    .bind(payload.allowed_domains.unwrap_or_else(default_array))
    .bind(payload.metadata.unwrap_or_else(default_object))
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Json(library))
}

pub async fn update_library(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(library_id): Path<Uuid>,
    Json(payload): Json<UpdateLibraryRequest>,
) -> Result<Json<LibraryRecord>, ApiError> {
    payload.validate()?;
    if let Some(slug) = &payload.slug {
        validate_library_slug(slug)?;
    }

    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    find_library_by_id(&state, library_id, user.id).await?;
    let mut tx = state.db.begin().await?;

    if payload.is_default.unwrap_or(false) {
        sqlx::query("UPDATE libraries SET is_default = FALSE WHERE owner_id = $1")
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    }

    let mut builder = QueryBuilder::<Postgres>::new("UPDATE libraries SET ");
    let mut separated = builder.separated(", ");
    let mut changed = false;

    if let Some(name) = payload.name {
        changed = true;
        separated.push("name = ").push_bind(name);
    }
    if let Some(slug) = payload.slug {
        changed = true;
        separated.push("slug = ").push_bind(slug);
    }
    if let Some(description) = payload.description {
        changed = true;
        separated.push("description = ").push_bind(description);
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
    if let Some(is_default) = payload.is_default {
        changed = true;
        separated.push("is_default = ").push_bind(is_default);
    }
    if let Some(default_stack) = payload.default_stack {
        changed = true;
        separated.push("default_stack = ").push_bind(default_stack);
    }
    if let Some(allowed_domains) = payload.allowed_domains {
        changed = true;
        separated
            .push("allowed_domains = ")
            .push_bind(allowed_domains);
    }
    if let Some(metadata) = payload.metadata {
        changed = true;
        separated.push("metadata = ").push_bind(metadata);
    }

    if !changed {
        return Err(ApiError::bad_request(
            "No library fields provided for update",
        ));
    }

    separated.push("updated_at = NOW()");
    builder.push(" WHERE id = ").push_bind(library_id);
    builder.push(" AND owner_id = ").push_bind(user.id);
    builder.push(
        r#"
        RETURNING
          id,
          owner_id,
          slug,
          name,
          description,
          visibility::text AS visibility,
          trust_level::text AS trust_level,
          is_default,
          default_stack,
          allowed_domains,
          metadata,
          created_at,
          updated_at
        "#,
    );

    let library = builder
        .build_query_as::<LibraryRecord>()
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Json(library))
}

async fn find_library_by_id(
    state: &AppState,
    library_id: Uuid,
    owner_id: Uuid,
) -> Result<LibraryRecord, ApiError> {
    let library = sqlx::query_as::<_, LibraryRecord>(
        r#"
        SELECT
          id,
          owner_id,
          slug,
          name,
          description,
          visibility::text AS visibility,
          trust_level::text AS trust_level,
          is_default,
          default_stack,
          allowed_domains,
          metadata,
          created_at,
          updated_at
        FROM libraries
        WHERE id = $1 AND owner_id = $2
        "#,
    )
    .bind(library_id)
    .bind(owner_id)
    .fetch_one(&state.db)
    .await?;

    Ok(library)
}

fn validate_library_slug(slug: &str) -> Result<(), ApiError> {
    if !slug.starts_with('@') || slug.matches('/').count() != 1 || slug.contains(' ') {
        return Err(ApiError::bad_request(
            "Library slug must look like @owner/library-name",
        ));
    }

    Ok(())
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
