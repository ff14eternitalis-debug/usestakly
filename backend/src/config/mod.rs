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
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub discord_client_id: Option<String>,
    pub discord_client_secret: Option<String>,
    pub admin_api_token: Option<String>,
    pub github_token: Option<String>,
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
        let github_client_id = env::var("GITHUB_CLIENT_ID").ok();
        let github_client_secret = env::var("GITHUB_CLIENT_SECRET").ok();
        let discord_client_id = env::var("DISCORD_CLIENT_ID").ok();
        let discord_client_secret = env::var("DISCORD_CLIENT_SECRET").ok();
        let admin_api_token = env::var("ADMIN_API_TOKEN").ok();
        let github_token = env::var("GITHUB_TOKEN")
            .ok()
            .filter(|s| !s.trim().is_empty());

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
            github_client_id,
            github_client_secret,
            discord_client_id,
            discord_client_secret,
            admin_api_token,
            github_token,
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
