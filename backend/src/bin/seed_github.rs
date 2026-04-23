use std::{fs, path::PathBuf, time::Duration};

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use tokio::time::sleep;
use tracing::{info, warn};
use usestakly_backend::{
    config::AppConfig,
    db,
    services::ingestion::github::{build_client, ingest_repo},
    telemetry,
};

#[derive(Debug, Deserialize)]
struct SeedFile {
    repos: Vec<SeedRepo>,
}

#[derive(Debug, Deserialize)]
struct SeedRepo {
    owner: String,
    name: String,
    #[serde(default)]
    note: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init();

    let config = AppConfig::from_env()?;
    let token = config
        .github_token
        .as_deref()
        .ok_or_else(|| anyhow!("GITHUB_TOKEN is required for seeding"))?;

    let seed_path = seed_file_path();
    let raw = fs::read_to_string(&seed_path)
        .with_context(|| format!("failed to read seed file at {}", seed_path.display()))?;
    let seed: SeedFile =
        toml::from_str(&raw).with_context(|| format!("failed to parse {}", seed_path.display()))?;

    info!(
        count = seed.repos.len(),
        path = %seed_path.display(),
        "loaded seed file"
    );

    let pool = db::connect(&config.database_url)
        .await
        .context("failed to connect to database")?;
    let client = build_client(token).map_err(|e| anyhow!("github client build failed: {e:?}"))?;

    let mut ok = 0usize;
    let mut failed = 0usize;
    for (idx, repo) in seed.repos.iter().enumerate() {
        let slug = format!("{}/{}", repo.owner, repo.name);
        match ingest_repo(&client, &pool, &config, &repo.owner, &repo.name).await {
            Ok((id, meta)) => {
                info!(
                    slug = %slug,
                    id = %id,
                    stars = meta.stars_count,
                    archived = meta.archived,
                    language = meta.language.as_deref().unwrap_or("-"),
                    note = repo.note.as_deref().unwrap_or(""),
                    "ingested"
                );
                ok += 1;
            }
            Err(e) => {
                warn!(slug = %slug, error = ?e, "ingestion failed");
                failed += 1;
            }
        }

        if idx + 1 < seed.repos.len() {
            sleep(Duration::from_millis(500)).await;
        }
    }

    info!(ok, failed, "seed run complete");
    if failed > 0 {
        return Err(anyhow!("{failed} repo(s) failed to ingest"));
    }
    Ok(())
}

fn seed_file_path() -> PathBuf {
    if let Ok(p) = std::env::var("SEED_FILE") {
        return PathBuf::from(p);
    }
    PathBuf::from("seeds/top_repos.toml")
}
