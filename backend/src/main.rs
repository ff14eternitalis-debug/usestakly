mod app;
mod auth;
mod config;
mod db;
mod domain;
mod handlers;
mod mcp;
mod search;
mod security;
mod services;
mod telemetry;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("[usestakly-backend] main:start");
    telemetry::init();
    eprintln!("[usestakly-backend] telemetry:ready");
    let config = config::AppConfig::from_env()?;
    eprintln!("[usestakly-backend] config:ready");
    let db = db::connect(&config.database_url)
        .await
        .context("failed to connect to database")?;
    eprintln!("[usestakly-backend] db:ready");

    let app = app::build_app(config.clone(), db);
    eprintln!("[usestakly-backend] app:ready");
    let listener = TcpListener::bind((&config.host[..], config.port)).await?;
    eprintln!("[usestakly-backend] listener:ready");

    info!(
        "backend listening on http://{}:{}",
        config.host, config.port
    );
    eprintln!("[usestakly-backend] serve:start");
    axum::serve(listener, app).await?;
    eprintln!("[usestakly-backend] serve:stopped");
    Ok(())
}
