pub mod error;

use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::AppConfig,
    handlers::{health, libraries, me, snippets},
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
}

pub fn build_app(config: AppConfig, db: PgPool) -> Router {
    let state = AppState { config, db };

    Router::new()
        .route("/health", get(health::health))
        .route("/api/me", get(me::me))
        .route(
            "/api/libraries",
            get(libraries::list_libraries).post(libraries::create_library),
        )
        .route(
            "/api/libraries/{library_id}",
            get(libraries::get_library).patch(libraries::update_library),
        )
        .route(
            "/api/snippets",
            get(snippets::list_snippets).post(snippets::create_snippet),
        )
        .route(
            "/api/snippets/{snippet_id}",
            get(snippets::get_snippet)
                .patch(snippets::update_snippet)
                .delete(snippets::delete_snippet),
        )
        .route(
            "/api/snippets/{snippet_id}/versions",
            get(snippets::list_snippet_versions).post(snippets::create_snippet_version),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
