use std::sync::{Mutex, OnceLock};

use anyhow::{Context, Result};
use chrono::Utc;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use sqlx::{FromRow, PgPool};
use tokio::task;
use uuid::Uuid;

use crate::{app::error::ApiError, config::AppConfig};

const EMBEDDING_DIMENSIONS: usize = 384;

static MODEL: OnceLock<Mutex<TextEmbedding>> = OnceLock::new();

pub fn enabled(config: &AppConfig) -> bool {
    config.semantic_search_enabled
}

pub fn build_search_document(
    owner: &str,
    name: &str,
    description: Option<&str>,
    language: Option<&str>,
    topics: &[String],
) -> String {
    let mut parts = vec![format!("{owner}/{name}")];
    if let Some(description) = description.filter(|s| !s.trim().is_empty()) {
        parts.push(description.trim().to_string());
    }
    if let Some(language) = language.filter(|s| !s.trim().is_empty()) {
        parts.push(format!("language: {}", language.trim()));
    }
    if !topics.is_empty() {
        parts.push(format!("topics: {}", topics.join(", ")));
    }
    format!("passage: {}", parts.join(" | "))
}

pub async fn embed_passage(text: String, config: &AppConfig) -> Result<Option<Vec<f32>>, ApiError> {
    if !enabled(config) {
        return Ok(None);
    }
    embed_text(text).await.map(Some).map_err(map_anyhow)
}

pub async fn embed_query(text: &str, config: &AppConfig) -> Result<Option<Vec<f32>>, ApiError> {
    if !enabled(config) || text.trim().is_empty() {
        return Ok(None);
    }
    embed_text(format!("query: {}", text.trim()))
        .await
        .map(Some)
        .map_err(map_anyhow)
}

pub async fn update_repo_embedding(
    db: &PgPool,
    artifact_id: Uuid,
    embedding: &[f32],
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        UPDATE external_artifacts
        SET embedding = CAST($2 AS vector(384)),
            embedding_updated_at = $3
        WHERE id = $1
        "#,
    )
    .bind(artifact_id)
    .bind(to_pgvector_literal(embedding))
    .bind(Utc::now())
    .execute(db)
    .await?;

    Ok(())
}

pub async fn backfill_repo_embeddings(
    db: &PgPool,
    config: &AppConfig,
    limit: i64,
    only_missing: bool,
) -> Result<usize, ApiError> {
    if !enabled(config) {
        return Err(ApiError::forbidden(
            "semantic search is disabled (set APP_SEMANTIC_SEARCH_ENABLED=true)",
        ));
    }

    let limit = limit.clamp(1, 500);
    let rows: Vec<RepoEmbeddingRow> = sqlx::query_as(
        r#"
        SELECT
          id,
          github_owner AS owner,
          github_repo AS name,
          description,
          language,
          topics
        FROM external_artifacts
        WHERE source = 'github'
          AND github_owner IS NOT NULL
          AND github_repo IS NOT NULL
          AND ($1 = FALSE OR embedding IS NULL)
        ORDER BY
          CASE WHEN embedding IS NULL THEN 0 ELSE 1 END,
          priors_fetched_at DESC NULLS LAST,
          id
        LIMIT $2
        "#,
    )
    .bind(only_missing)
    .bind(limit)
    .fetch_all(db)
    .await?;

    let mut updated = 0usize;
    for row in rows {
        if let Some(embedding) = embed_passage(
            build_search_document(
                &row.owner,
                &row.name,
                row.description.as_deref(),
                row.language.as_deref(),
                &row.topics,
            ),
            config,
        )
        .await?
        {
            update_repo_embedding(db, row.id, &embedding).await?;
            updated += 1;
        }
    }

    Ok(updated)
}

fn map_anyhow(err: anyhow::Error) -> ApiError {
    ApiError::internal(format!("semantic embedding failed: {err}"))
}

async fn embed_text(text: String) -> Result<Vec<f32>> {
    task::spawn_blocking(move || {
        let model = model()?;
        let mut guard = model
            .lock()
            .map_err(|_| anyhow::anyhow!("embedding model lock poisoned"))?;
        let embeddings = guard.embed(vec![text], None).context("embedding text")?;
        let vector = embeddings
            .into_iter()
            .next()
            .context("missing embedding output")?;
        if vector.len() != EMBEDDING_DIMENSIONS {
            return Err(anyhow::anyhow!(
                "unexpected embedding dimension {}, expected {}",
                vector.len(),
                EMBEDDING_DIMENSIONS
            ));
        }
        Ok(vector)
    })
    .await
    .context("joining embedding task")?
}

fn model() -> Result<&'static Mutex<TextEmbedding>> {
    if let Some(model) = MODEL.get() {
        return Ok(model);
    }

    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(false),
    )
    .context("initializing fastembed model")?;
    let _ = MODEL.set(Mutex::new(model));
    MODEL
        .get()
        .context("embedding model missing after initialization")
}

pub fn to_pgvector_literal(values: &[f32]) -> String {
    let inner = values
        .iter()
        .map(|value| format!("{value:.8}"))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{inner}]")
}

#[derive(FromRow)]
struct RepoEmbeddingRow {
    id: Uuid,
    owner: String,
    name: String,
    description: Option<String>,
    language: Option<String>,
    topics: Vec<String>,
}
