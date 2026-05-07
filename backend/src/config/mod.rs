use anyhow::{Result, anyhow};
use std::env;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub dev_user_id: Uuid,
    pub dev_user_email: String,
    pub dev_user_username: String,
    pub dev_user_display_name: Option<String>,
    pub dev_user_avatar_url: Option<String>,
    pub app_base_url: String,
    pub frontend_base_url: String,
    pub app_session_secret: Option<String>,
    pub app_notification_secret: Option<String>,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub discord_client_id: Option<String>,
    pub discord_client_secret: Option<String>,
    pub admin_api_token: Option<String>,
    pub github_token: Option<String>,
    pub scheduler_enabled: bool,
    pub recompute_interval_secs: u64,
    pub mcp_auth_failure_limit_per_minute: usize,
    pub mcp_read_limit_per_minute: usize,
    pub mcp_write_limit_per_hour: u32,
    pub mcp_log_usage_cooldown_secs: u64,
    pub mcp_negative_signal_window_hours: u64,
    pub active_signal_min_reputation: f64,
    pub active_signal_default_consensus: u32,
    pub active_signal_severe_consensus: u32,
    pub semantic_search_enabled: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        if let Err(e) = dotenvy::dotenv()
            && !matches!(&e, dotenvy::Error::Io(io) if io.kind() == std::io::ErrorKind::NotFound)
        {
            tracing::warn!(error = ?e, "failed to load .env");
        }

        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "4000".to_string())
            .parse::<u16>()
            .map_err(|_| anyhow!("APP_PORT must be a valid u16"))?;
        let database_url =
            env::var("DATABASE_URL").map_err(|_| anyhow!("DATABASE_URL is required"))?;
        let dev_user_id = env::var("DEV_USER_ID")
            .unwrap_or_else(|_| "00000000-0000-0000-0000-000000000001".to_string())
            .parse::<Uuid>()
            .map_err(|_| anyhow!("DEV_USER_ID must be a valid UUID"))?;
        let dev_user_email =
            env::var("DEV_USER_EMAIL").unwrap_or_else(|_| "dev@project-k.local".to_string());
        let dev_user_username =
            env::var("DEV_USER_USERNAME").unwrap_or_else(|_| "projectk-dev".to_string());
        let dev_user_display_name = env::var("DEV_USER_DISPLAY_NAME").ok();
        let dev_user_avatar_url = env::var("DEV_USER_AVATAR_URL").ok();
        let app_base_url =
            env::var("APP_BASE_URL").unwrap_or_else(|_| format!("http://{}:{}", host, port));
        let frontend_base_url =
            env::var("FRONTEND_BASE_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
        let app_session_secret = env::var("APP_SESSION_SECRET").ok();
        let app_notification_secret = env::var("APP_NOTIFICATION_SECRET")
            .ok()
            .filter(|value| !value.trim().is_empty());
        let github_client_id = env::var("GITHUB_CLIENT_ID").ok();
        let github_client_secret = env::var("GITHUB_CLIENT_SECRET").ok();
        let discord_client_id = env::var("DISCORD_CLIENT_ID").ok();
        let discord_client_secret = env::var("DISCORD_CLIENT_SECRET").ok();
        let admin_api_token = env::var("ADMIN_API_TOKEN").ok();
        let github_token = env::var("GITHUB_TOKEN")
            .ok()
            .filter(|s| !s.trim().is_empty());
        let scheduler_enabled = env::var("APP_SCHEDULER_ENABLED")
            .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "true" | "1" | "yes"))
            .unwrap_or(false);
        let recompute_interval_secs = env::var("APP_RECOMPUTE_INTERVAL_SECS")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow!("APP_RECOMPUTE_INTERVAL_SECS must be a valid u64"))?;
        if recompute_interval_secs < 60 {
            return Err(anyhow!(
                "APP_RECOMPUTE_INTERVAL_SECS must be >= 60 (got {recompute_interval_secs})"
            ));
        }
        let mcp_auth_failure_limit_per_minute = env::var("APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<usize>()
            .map_err(|_| anyhow!("APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE must be a valid usize"))?;
        if mcp_auth_failure_limit_per_minute == 0 {
            return Err(anyhow!(
                "APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE must be >= 1"
            ));
        }
        let mcp_read_limit_per_minute = env::var("APP_MCP_READ_LIMIT_PER_MINUTE")
            .unwrap_or_else(|_| "120".to_string())
            .parse::<usize>()
            .map_err(|_| anyhow!("APP_MCP_READ_LIMIT_PER_MINUTE must be a valid usize"))?;
        if mcp_read_limit_per_minute == 0 {
            return Err(anyhow!("APP_MCP_READ_LIMIT_PER_MINUTE must be >= 1"));
        }
        let mcp_write_limit_per_hour = env::var("APP_MCP_WRITE_LIMIT_PER_HOUR")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u32>()
            .map_err(|_| anyhow!("APP_MCP_WRITE_LIMIT_PER_HOUR must be a valid u32"))?;
        if mcp_write_limit_per_hour == 0 {
            return Err(anyhow!("APP_MCP_WRITE_LIMIT_PER_HOUR must be >= 1"));
        }
        let mcp_log_usage_cooldown_secs = env::var("APP_MCP_LOG_USAGE_COOLDOWN_SECS")
            .unwrap_or_else(|_| "900".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow!("APP_MCP_LOG_USAGE_COOLDOWN_SECS must be a valid u64"))?;
        if mcp_log_usage_cooldown_secs < 60 {
            return Err(anyhow!(
                "APP_MCP_LOG_USAGE_COOLDOWN_SECS must be >= 60 (got {mcp_log_usage_cooldown_secs})"
            ));
        }
        let mcp_negative_signal_window_hours = env::var("APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<u64>()
            .map_err(|_| anyhow!("APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS must be a valid u64"))?;
        if mcp_negative_signal_window_hours == 0 {
            return Err(anyhow!("APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS must be >= 1"));
        }
        let active_signal_min_reputation = env::var("APP_ACTIVE_SIGNAL_MIN_REPUTATION")
            .unwrap_or_else(|_| "0.45".to_string())
            .parse::<f64>()
            .map_err(|_| anyhow!("APP_ACTIVE_SIGNAL_MIN_REPUTATION must be a valid number"))?;
        if !(0.0..=1.0).contains(&active_signal_min_reputation) {
            return Err(anyhow!(
                "APP_ACTIVE_SIGNAL_MIN_REPUTATION must be between 0 and 1"
            ));
        }
        let active_signal_default_consensus = env::var("APP_ACTIVE_SIGNAL_DEFAULT_CONSENSUS")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()
            .map_err(|_| anyhow!("APP_ACTIVE_SIGNAL_DEFAULT_CONSENSUS must be a valid u32"))?;
        if active_signal_default_consensus == 0 {
            return Err(anyhow!("APP_ACTIVE_SIGNAL_DEFAULT_CONSENSUS must be >= 1"));
        }
        let active_signal_severe_consensus = env::var("APP_ACTIVE_SIGNAL_SEVERE_CONSENSUS")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u32>()
            .map_err(|_| anyhow!("APP_ACTIVE_SIGNAL_SEVERE_CONSENSUS must be a valid u32"))?;
        if active_signal_severe_consensus < active_signal_default_consensus {
            return Err(anyhow!(
                "APP_ACTIVE_SIGNAL_SEVERE_CONSENSUS must be >= APP_ACTIVE_SIGNAL_DEFAULT_CONSENSUS"
            ));
        }
        let semantic_search_enabled = env::var("APP_SEMANTIC_SEARCH_ENABLED")
            .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "true" | "1" | "yes"))
            .unwrap_or(false);

        Ok(Self {
            host,
            port,
            database_url,
            dev_user_id,
            dev_user_email,
            dev_user_username,
            dev_user_display_name,
            dev_user_avatar_url,
            app_base_url,
            frontend_base_url,
            app_session_secret,
            app_notification_secret,
            github_client_id,
            github_client_secret,
            discord_client_id,
            discord_client_secret,
            admin_api_token,
            github_token,
            scheduler_enabled,
            recompute_interval_secs,
            mcp_auth_failure_limit_per_minute,
            mcp_read_limit_per_minute,
            mcp_write_limit_per_hour,
            mcp_log_usage_cooldown_secs,
            mcp_negative_signal_window_hours,
            active_signal_min_reputation,
            active_signal_default_consensus,
            active_signal_severe_consensus,
            semantic_search_enabled,
        })
    }

    pub fn github_auth_enabled(&self) -> bool {
        self.github_client_id.is_some()
            && self.github_client_secret.is_some()
            && self.app_session_secret.is_some()
    }

    pub fn discord_auth_enabled(&self) -> bool {
        self.discord_client_id.is_some()
            && self.discord_client_secret.is_some()
            && self.app_session_secret.is_some()
    }

    pub fn auth_enabled(&self) -> bool {
        self.github_auth_enabled() || self.discord_auth_enabled()
    }

    pub fn notification_secret(&self) -> Option<&str> {
        self.app_notification_secret.as_deref()
    }

    pub fn github_callback_url(&self) -> String {
        format!(
            "{}/api/auth/github/callback",
            self.app_base_url.trim_end_matches('/')
        )
    }

    pub fn discord_callback_url(&self) -> String {
        format!(
            "{}/api/auth/discord/callback",
            self.app_base_url.trim_end_matches('/')
        )
    }

    pub fn session_cookie_secure(&self) -> bool {
        self.app_base_url.starts_with("https://")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_secret_prefers_dedicated_secret_over_session_secret() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 4000,
            database_url: "postgres://localhost/test".to_string(),
            dev_user_id: Uuid::nil(),
            dev_user_email: "dev@example.com".to_string(),
            dev_user_username: "dev".to_string(),
            dev_user_display_name: None,
            dev_user_avatar_url: None,
            app_base_url: "http://127.0.0.1:4000".to_string(),
            frontend_base_url: "http://localhost:5173".to_string(),
            app_session_secret: Some("session-secret".to_string()),
            app_notification_secret: Some("notification-secret".to_string()),
            github_client_id: None,
            github_client_secret: None,
            discord_client_id: None,
            discord_client_secret: None,
            admin_api_token: None,
            github_token: None,
            scheduler_enabled: false,
            recompute_interval_secs: 86_400,
            mcp_auth_failure_limit_per_minute: 30,
            mcp_read_limit_per_minute: 120,
            mcp_write_limit_per_hour: 60,
            mcp_log_usage_cooldown_secs: 900,
            mcp_negative_signal_window_hours: 24,
            active_signal_min_reputation: 0.45,
            active_signal_default_consensus: 2,
            active_signal_severe_consensus: 3,
            semantic_search_enabled: false,
        };

        assert_eq!(config.notification_secret(), Some("notification-secret"));
    }
}
