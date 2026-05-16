use std::env;

use anyhow::{Result, anyhow};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchedulerConfig {
    pub enabled: bool,
    pub recompute_interval_secs: u64,
    pub digest_interval_secs: u64,
    pub corpus_refresh_stale_secs: u64,
    pub ingest_max_repos_per_cycle: usize,
    pub run_on_startup: bool,
}

impl SchedulerConfig {
    pub fn from_env(app_env: &str) -> Result<Self> {
        let production_like = matches!(
            app_env.trim().to_ascii_lowercase().as_str(),
            "production" | "staging" | "prod"
        );

        let enabled = match env::var("APP_SCHEDULER_ENABLED") {
            Ok(value) => parse_bool(&value),
            Err(_) => production_like,
        };

        let recompute_interval_secs = parse_u64_env(
            "APP_RECOMPUTE_INTERVAL_SECS",
            if production_like { 1_800 } else { 3_600 },
        )?;
        ensure_min_secs(
            "APP_RECOMPUTE_INTERVAL_SECS",
            recompute_interval_secs,
            60,
        )?;

        let digest_interval_secs =
            parse_u64_env("APP_DIGEST_INTERVAL_SECS", 1_800)?;
        ensure_min_secs("APP_DIGEST_INTERVAL_SECS", digest_interval_secs, 60)?;

        let corpus_refresh_stale_secs = parse_u64_env(
            "APP_CORPUS_REFRESH_STALE_SECS",
            recompute_interval_secs,
        )?;
        ensure_min_secs(
            "APP_CORPUS_REFRESH_STALE_SECS",
            corpus_refresh_stale_secs,
            60,
        )?;

        let ingest_max_repos_per_cycle = parse_usize_env(
            "APP_INGEST_MAX_REPOS_PER_CYCLE",
            if production_like { 100 } else { 40 },
        )?;
        if ingest_max_repos_per_cycle == 0 {
            return Err(anyhow!("APP_INGEST_MAX_REPOS_PER_CYCLE must be >= 1"));
        }

        let run_on_startup = match env::var("APP_SCHEDULER_RUN_ON_STARTUP") {
            Ok(value) => parse_bool(&value),
            Err(_) => enabled && production_like,
        };

        Ok(Self {
            enabled,
            recompute_interval_secs,
            digest_interval_secs,
            corpus_refresh_stale_secs,
            ingest_max_repos_per_cycle,
            run_on_startup,
        })
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "true" | "1" | "yes"
    )
}

fn parse_u64_env(name: &str, default: u64) -> Result<u64> {
    match env::var(name) {
        Ok(value) => value
            .parse::<u64>()
            .map_err(|_| anyhow!("{name} must be a valid u64")),
        Err(_) => Ok(default),
    }
}

fn parse_usize_env(name: &str, default: usize) -> Result<usize> {
    match env::var(name) {
        Ok(value) => value
            .parse::<usize>()
            .map_err(|_| anyhow!("{name} must be a valid usize")),
        Err(_) => Ok(default),
    }
}

fn ensure_min_secs(name: &str, value: u64, min: u64) -> Result<()> {
    if value < min {
        return Err(anyhow!("{name} must be >= {min} (got {value})"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool_accepts_common_truthy_values() {
        assert!(parse_bool("true"));
        assert!(parse_bool(" YES "));
        assert!(!parse_bool("false"));
    }
}
