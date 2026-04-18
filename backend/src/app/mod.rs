pub mod error;

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::AppConfig,
    handlers::{auth, health, libraries, me, snippets},
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
}

pub fn build_app(config: AppConfig, db: PgPool) -> Router {
    let frontend_origin = config
        .frontend_base_url
        .parse::<HeaderValue>()
        .unwrap_or_else(|_| HeaderValue::from_static("http://localhost:5173"));
    let state = AppState { config, db };

    Router::new()
        .route("/health", get(health::health))
        .route("/api/auth/github/start", get(auth::github_start))
        .route("/api/auth/github/callback", get(auth::github_callback))
        .route("/api/auth/discord/start", get(auth::discord_start))
        .route("/api/auth/discord/callback", get(auth::discord_callback))
        .route("/api/auth/logout", post(auth::logout))
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
        .layer(
            CorsLayer::new()
                .allow_origin(frontend_origin)
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
