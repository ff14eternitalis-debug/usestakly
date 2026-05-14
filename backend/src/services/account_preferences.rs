use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    services::{email_templates::EmailLocale, notification_digest::digest_time_for_preset},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPreferences {
    pub digest_time_preset: String,
    pub digest_time_local: String,
    pub timezone: String,
    pub email_locale: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationPreferences {
    pub digest_time_preset: String,
    pub timezone: String,
    pub email_locale: Option<String>,
}

#[derive(FromRow)]
struct PreferencesRow {
    digest_time_local: String,
    timezone: String,
    email_locale: String,
}

pub async fn get(db: &PgPool, user_id: Uuid) -> Result<NotificationPreferences, ApiError> {
    let row: PreferencesRow = sqlx::query_as(
        r#"
        SELECT digest_time_local, timezone, email_locale
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    Ok(NotificationPreferences {
        digest_time_preset: preset_for_time(&row.digest_time_local),
        digest_time_local: row.digest_time_local,
        timezone: row.timezone,
        email_locale: row.email_locale,
    })
}

pub async fn update(
    db: &PgPool,
    user_id: Uuid,
    input: UpdateNotificationPreferences,
) -> Result<NotificationPreferences, ApiError> {
    let digest_time_local = digest_time_for_preset(&input.digest_time_preset)?.to_string();
    let timezone = validate_timezone(&input.timezone)?;
    let email_locale = input
        .email_locale
        .as_deref()
        .map(validate_email_locale)
        .transpose()?;

    let row: PreferencesRow = sqlx::query_as(
        r#"
        UPDATE users
        SET digest_time_local = $2,
            timezone = $3,
            email_locale = COALESCE($4, email_locale),
            updated_at = NOW()
        WHERE id = $1
        RETURNING digest_time_local, timezone, email_locale
        "#,
    )
    .bind(user_id)
    .bind(&digest_time_local)
    .bind(&timezone)
    .bind(email_locale.as_deref())
    .fetch_one(db)
    .await?;

    tracing::info!(
        user_id = %user_id,
        timezone = %row.timezone,
        digest_time_local = %row.digest_time_local,
        email_locale = %row.email_locale,
        "notification preferences updated"
    );

    Ok(NotificationPreferences {
        digest_time_preset: preset_for_time(&row.digest_time_local),
        digest_time_local: row.digest_time_local,
        timezone: row.timezone,
        email_locale: row.email_locale,
    })
}

pub fn validate_timezone(value: &str) -> Result<String, ApiError> {
    let timezone = value.trim();
    timezone
        .parse::<Tz>()
        .map_err(|_| ApiError::bad_request("invalid timezone"))?;
    Ok(timezone.to_string())
}

pub fn validate_email_locale(value: &str) -> Result<String, ApiError> {
    match EmailLocale::parse_lossy(value) {
        EmailLocale::En if value.eq_ignore_ascii_case("en") => Ok("en".to_string()),
        EmailLocale::Fr => Ok("fr".to_string()),
        _ => Err(ApiError::bad_request("invalid email locale")),
    }
}

fn preset_for_time(value: &str) -> String {
    match value {
        "08:00" => "morning",
        "12:00" => "noon",
        "18:00" => "evening",
        "21:00" => "night",
        _ => "morning",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::notification_digest::is_digest_due_now;
    use chrono::{TimeZone, Utc};

    #[test]
    fn validates_real_iana_timezone() {
        assert_eq!(validate_timezone("Europe/Paris").unwrap(), "Europe/Paris");
        assert!(validate_timezone("Paris").is_err());
    }

    #[test]
    fn validates_supported_email_locale() {
        assert_eq!(validate_email_locale("en").unwrap(), "en");
        assert_eq!(validate_email_locale("fr").unwrap(), "fr");
        assert!(validate_email_locale("de").is_err());
    }

    #[test]
    fn preset_times_are_compatible_with_digest_due_check() {
        let now = Utc.with_ymd_and_hms(2026, 5, 8, 10, 3, 0).unwrap();
        let noon = digest_time_for_preset("noon").unwrap();

        assert!(is_digest_due_now(noon, "Europe/Paris", now, 30).unwrap());
    }
}
