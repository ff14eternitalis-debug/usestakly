use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::Mailbox,
    message::{MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use validator::ValidateEmail;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::watchlist::NotificationKind,
    services::email_templates::{
        EmailField, EmailTemplate, render_test_email, render_watch_alert_email,
    },
};

use super::message::watch_alert_message;

pub fn validate_notification_email(value: &str) -> Result<String, ApiError> {
    let email = value.trim().to_ascii_lowercase();
    if !email.validate_email()
        || !email
            .split('@')
            .nth(1)
            .is_some_and(|domain| domain.contains('.'))
    {
        return Err(ApiError::bad_request("invalid email address"));
    }
    Ok(email)
}

pub(crate) async fn post_email_test_message(
    config: &AppConfig,
    to: &str,
) -> Result<(), anyhow::Error> {
    send_email(config, to, &render_test_email()).await
}

pub(crate) async fn post_email_watch_alert(
    config: &AppConfig,
    to: &str,
    repo_full_name: &str,
    repo_url: Option<&str>,
    kind: NotificationKind,
    payload: &serde_json::Value,
) -> Result<(), anyhow::Error> {
    let message = watch_alert_message(repo_full_name, kind, payload);
    let fields = message
        .fields
        .iter()
        .filter_map(|field| {
            let name = field
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("Detail");
            let value = field
                .get("value")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            (!value.is_empty()).then(|| EmailField {
                name: name.to_string(),
                value: value.to_string(),
            })
        })
        .collect::<Vec<_>>();
    let email = render_watch_alert_email(
        &format!("[UseStakly] {}", message.title),
        &message.title,
        &message.content,
        &message.description,
        repo_url,
        &fields,
    );

    send_email(config, to, &email).await
}

pub(crate) async fn send_email(
    config: &AppConfig,
    to: &str,
    email: &EmailTemplate,
) -> Result<(), anyhow::Error> {
    let (Some(username), Some(password)) = (
        config.email_smtp_username.as_deref(),
        config.email_smtp_password.as_deref(),
    ) else {
        anyhow::bail!("email SMTP is not configured");
    };

    let from: Mailbox = Mailbox::new(
        Some(config.email_from_name.clone()),
        config.email_from_address.parse()?,
    );
    let to: Mailbox = to.parse()?;
    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(&email.subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(email.text.clone()))
                .singlepart(SinglePart::html(email.html.clone())),
        )?;
    let creds = Credentials::new(username.to_string(), password.to_string());
    let transport = if config.email_smtp_port == 465 {
        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.email_smtp_host)?
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.email_smtp_host)?
    };
    let mailer = transport
        .port(config.email_smtp_port)
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_email_rejects_invalid_address() {
        assert!(validate_notification_email("dev@example.com").is_ok());
        assert!(validate_notification_email("not-an-email").is_err());
        assert!(validate_notification_email("dev@localhost").is_err());
    }
}
