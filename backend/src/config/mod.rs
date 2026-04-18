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
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

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

        Ok(Self {
            host,
            port,
            database_url,
            dev_user_id,
            dev_user_email,
            dev_user_username,
            dev_user_display_name,
            dev_user_avatar_url,
        })
    }
}
