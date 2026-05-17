use reqwest::Url;
use serde_json::json;

use crate::{app::error::ApiError, domain::watchlist::NotificationKind};

use super::message::watch_alert_message;

pub fn validate_discord_webhook_url(value: &str) -> Result<String, ApiError> {
    let trimmed = value.trim();
    let parsed = Url::parse(trimmed).map_err(|_| ApiError::bad_request("invalid webhook URL"))?;
    if parsed.scheme() != "https" {
        return Err(ApiError::bad_request("webhook URL must use HTTPS"));
    }
    let host = parsed.host_str().unwrap_or_default();
    if host != "discord.com" && host != "discordapp.com" {
        return Err(ApiError::bad_request("only Discord webhooks are supported"));
    }
    let segments: Vec<_> = parsed
        .path_segments()
        .map(|segments| segments.collect())
        .unwrap_or_default();
    if segments.len() < 4
        || segments[0] != "api"
        || segments[1] != "webhooks"
        || segments[2].is_empty()
        || segments[3].is_empty()
    {
        return Err(ApiError::bad_request("invalid Discord webhook URL"));
    }
    Ok(trimmed.to_string())
}

pub fn mask_discord_webhook_url(value: &str) -> String {
    let parsed = match Url::parse(value) {
        Ok(parsed) => parsed,
        Err(_) => return "discord webhook ...".to_string(),
    };
    let segments: Vec<_> = parsed
        .path_segments()
        .map(|segments| segments.collect())
        .unwrap_or_default();
    let id = segments.get(2).copied().unwrap_or("unknown");
    let token = segments.get(3).copied().unwrap_or("");
    let tail = token
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    format!("discord webhook {id}/...{tail}")
}

pub(crate) async fn post_discord_test_message(webhook_url: &str) -> Result<(), anyhow::Error> {
    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&json!({
            "username": "UseStakly",
            "content": "UseStakly notification test.",
            "allowed_mentions": { "parse": [] },
            "embeds": [{
                "title": "Notification channel connected",
                "description": "UseStakly can now send critical watch alerts to this Discord channel.",
                "color": 8900331
            }]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Discord webhook returned HTTP {}", response.status());
    }
    Ok(())
}

pub(crate) async fn post_discord_watch_alert(
    webhook_url: &str,
    repo_full_name: &str,
    repo_url: Option<&str>,
    kind: NotificationKind,
    payload: &serde_json::Value,
) -> Result<(), anyhow::Error> {
    let message = watch_alert_message(repo_full_name, kind, payload);
    let mut embed = json!({
        "title": message.title,
        "description": message.description,
        "color": message.color,
        "fields": message.fields
    });
    if let Some(url) = repo_url {
        embed["url"] = json!(url);
    }

    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&json!({
            "username": "UseStakly",
            "content": message.content,
            "allowed_mentions": { "parse": [] },
            "embeds": [embed]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Discord webhook returned HTTP {}", response.status());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discord_webhook_accepts_only_discord_webhook_urls() {
        assert!(
            validate_discord_webhook_url(
                "https://discord.com/api/webhooks/123456789012345678/token-value"
            )
            .is_ok()
        );
        assert!(
            validate_discord_webhook_url(
                "https://discordapp.com/api/webhooks/123456789012345678/token-value"
            )
            .is_ok()
        );
        assert!(validate_discord_webhook_url("https://example.com/webhook").is_err());
    }

    #[test]
    fn discord_webhook_url_is_masked_without_leaking_secret() {
        let masked = mask_discord_webhook_url(
            "https://discord.com/api/webhooks/123456789012345678/abcdefghijklmnopqrstuvwxyz",
        );

        assert_eq!(masked, "discord webhook 123456789012345678/...wxyz");
    }
}
