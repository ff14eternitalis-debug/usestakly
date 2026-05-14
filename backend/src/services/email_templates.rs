#[derive(Debug, Clone, PartialEq)]
pub struct EmailTemplate {
    pub subject: String,
    pub text: String,
    pub html: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmailField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmailSection {
    pub title: String,
    pub items: Vec<String>,
}

const LOGO_URL: &str = "https://usestakly.com/usestackly-logo-white-lime.png";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailLocale {
    En,
    Fr,
}

impl EmailLocale {
    pub fn parse_lossy(value: &str) -> Self {
        if value.eq_ignore_ascii_case("fr") {
            Self::Fr
        } else {
            Self::En
        }
    }

    pub fn from_accept_language(value: Option<&str>) -> Self {
        let Some(value) = value else {
            return Self::En;
        };
        if value
            .split(',')
            .any(|part| part.trim_start().to_ascii_lowercase().starts_with("fr"))
        {
            Self::Fr
        } else {
            Self::En
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
        }
    }
}

pub fn render_test_email(locale: EmailLocale) -> EmailTemplate {
    let subject = match locale {
        EmailLocale::En => "UseStakly notification channel connected",
        EmailLocale::Fr => "Canal de notification UseStakly connecté",
    };
    let body = match locale {
        EmailLocale::En => "UseStakly can now send critical watch alerts to this email address.",
        EmailLocale::Fr => {
            "UseStakly peut désormais envoyer les alertes de veille critiques à cette adresse email."
        }
    };
    let title = match locale {
        EmailLocale::En => "Notification channel connected",
        EmailLocale::Fr => "Canal de notification connecté",
    };
    let eyebrow = match locale {
        EmailLocale::En => "Channel ready",
        EmailLocale::Fr => "Canal prêt",
    };
    branded_email(locale, subject, title, eyebrow, body, &[], None)
}

pub fn render_watch_alert_email(
    locale: EmailLocale,
    subject: &str,
    title: &str,
    intro: &str,
    description: &str,
    repo_url: Option<&str>,
    fields: &[EmailField],
) -> EmailTemplate {
    let mut text = format!("{intro}\n\n{description}");
    if let Some(url) = repo_url {
        let label = match locale {
            EmailLocale::En => "Repository",
            EmailLocale::Fr => "Dépôt",
        };
        text.push_str(&format!("\n\n{label}: {url}"));
    }
    for field in fields {
        if !field.value.is_empty() {
            text.push_str(&format!("\n\n{}: {}", field.name, field.value));
        }
    }

    let eyebrow = match locale {
        EmailLocale::En => "Watch alert",
        EmailLocale::Fr => "Alerte de veille",
    };
    branded_email(locale, subject, title, eyebrow, &text, fields, repo_url)
}

pub fn render_digest_email(locale: EmailLocale, sections: &[EmailSection]) -> EmailTemplate {
    let mut text = match locale {
        EmailLocale::En => "UseStakly daily watch digest.",
        EmailLocale::Fr => "Résumé quotidien de veille UseStakly.",
    }
    .to_string();
    for section in sections.iter().filter(|section| !section.items.is_empty()) {
        text.push_str(&format!(
            "\n\n{}\n{}",
            section.title,
            section
                .items
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    let fields = sections
        .iter()
        .filter(|section| !section.items.is_empty())
        .map(|section| EmailField {
            name: section.title.clone(),
            value: section
                .items
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
        })
        .collect::<Vec<_>>();

    branded_email(
        locale,
        match locale {
            EmailLocale::En => "UseStakly daily watch digest",
            EmailLocale::Fr => "Résumé quotidien de veille UseStakly",
        },
        match locale {
            EmailLocale::En => "Daily watch digest",
            EmailLocale::Fr => "Résumé quotidien de veille",
        },
        match locale {
            EmailLocale::En => "Watch summary",
            EmailLocale::Fr => "Résumé de veille",
        },
        &text,
        &fields,
        None,
    )
}

fn branded_email(
    locale: EmailLocale,
    subject: &str,
    title: &str,
    eyebrow: &str,
    text: &str,
    fields: &[EmailField],
    action_url: Option<&str>,
) -> EmailTemplate {
    let html_fields = fields
        .iter()
        .filter(|field| !field.value.is_empty())
        .map(|field| {
            format!(
                r#"<tr>
  <td style="padding:12px 0;border-top:1px solid #1d1f25;">
    <div style="font:600 11px/1.4 ui-monospace,SFMono-Regular,Cascadia Code,Menlo,monospace;letter-spacing:0.16em;text-transform:uppercase;color:#6b6e77;">{}</div>
    <div style="margin-top:6px;font:400 15px/1.55 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif;color:#f5f6f7;white-space:pre-line;">{}</div>
  </td>
</tr>"#,
                escape_html(&field.name),
                escape_html(&field.value)
            )
        })
        .collect::<String>();

    let action = action_url.map_or_else(String::new, |url| {
        format!(
            r#"<div style="margin-top:22px;">
  <a href="{}" style="display:inline-block;border:1px solid #b6ff3c;border-radius:6px;padding:11px 15px;font:600 14px/1 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif;color:#0a1304;background:#b6ff3c;text-decoration:none;">Open repository</a>
</div>"#,
            escape_attr(url)
        )
    });

    let html = format!(
        r#"<!doctype html>
<html lang="{}">
  <body style="margin:0;padding:0;background:#08090b;color:#f5f6f7;">
    <div style="display:none;max-height:0;overflow:hidden;color:#08090b;" translate="no" class="notranslate">{}</div>
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" translate="no" class="notranslate" style="background:#08090b;margin:0;padding:32px 16px;">
      <tr>
        <td align="center">
          <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="max-width:620px;border-collapse:collapse;">
            <tr>
              <td style="padding:0 0 16px 0;">
                <table role="presentation" cellspacing="0" cellpadding="0">
                  <tr>
                    <td style="vertical-align:middle;">
                      <img src="{}" width="42" height="40" alt="UseStakly" translate="no" class="notranslate" style="display:block;width:42px;height:40px;border:0;outline:none;text-decoration:none;" />
                    </td>
                    <td translate="no" class="notranslate" style="padding-left:12px;font:700 20px/1.2 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif;color:#f5f6f7;">UseStakly</td>
                  </tr>
                </table>
              </td>
            </tr>
            <tr>
              <td style="border:1px solid #1d1f25;border-radius:8px;background:#111216;padding:28px;">
                <div style="font:600 11px/1.4 ui-monospace,SFMono-Regular,Cascadia Code,Menlo,monospace;letter-spacing:0.18em;text-transform:uppercase;color:#b6ff3c;">{}</div>
                <h1 style="margin:10px 0 16px 0;font:700 28px/1.12 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif;color:#f5f6f7;">{}</h1>
                <div style="border:1px solid #1d1f25;border-radius:8px;background:#0c0d10;padding:18px;font:400 16px/1.6 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif;color:#a7a9b0;white-space:pre-line;">{}</div>
                {}
                <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="margin-top:18px;border-collapse:collapse;">{}</table>
              </td>
            </tr>
            <tr>
              <td translate="no" class="notranslate" style="padding:18px 4px 0 4px;font:400 12px/1.6 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif;color:#6b6e77;">
                {}
              </td>
            </tr>
          </table>
        </td>
      </tr>
    </table>
  </body>
</html>"#,
        locale.code(),
        escape_html(text.lines().next().unwrap_or(subject)),
        LOGO_URL,
        escape_html(eyebrow),
        escape_html(title),
        escape_html(text),
        action,
        html_fields,
        escape_html(match locale {
            EmailLocale::En =>
                "UseStakly public beta · OSS quality radar for developers and agents.",
            EmailLocale::Fr =>
                "Beta publique UseStakly · Radar de qualité OSS pour développeurs et agents.",
        })
    );

    EmailTemplate {
        subject: subject.to_string(),
        text: text.to_string(),
        html,
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_attr(value: &str) -> String {
    escape_html(value).replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_email_has_branded_html_and_plain_fallback() {
        let email = render_test_email(EmailLocale::En);

        assert_eq!(email.subject, "UseStakly notification channel connected");
        assert!(
            email
                .text
                .contains("UseStakly can now send critical watch alerts")
        );
        assert!(email.html.contains("UseStakly"));
        assert!(email.html.contains("#08090b"));
        assert!(email.html.contains("#111216"));
        assert!(email.html.contains("#1d1f25"));
        assert!(email.html.contains("#b6ff3c"));
        assert!(
            email
                .html
                .contains("https://usestakly.com/usestackly-logo-white-lime.png")
        );
        assert!(email.html.contains("alt=\"UseStakly\""));
        assert!(email.html.contains("<html lang=\"en\">"));
        assert!(email.html.contains("translate=\"no\""));
        assert!(email.html.contains("class=\"notranslate\""));
        assert!(email.html.contains("public beta"));
    }

    #[test]
    fn test_channel_email_can_render_in_french() {
        let email = render_test_email(EmailLocale::Fr);

        assert_eq!(email.subject, "Canal de notification UseStakly connecté");
        assert!(email.text.contains("alertes de veille critiques"));
        assert!(email.html.contains("<html lang=\"fr\">"));
        assert!(email.html.contains("Canal de notification connecté"));
        assert!(email.html.contains("Beta publique UseStakly"));
    }

    #[test]
    fn watch_alert_email_keeps_repo_link_fields_and_escapes_html() {
        let email = render_watch_alert_email(
            EmailLocale::En,
            "[UseStakly] owner/repo: score drop",
            "owner/<repo>: score drop",
            "UseStakly alert: owner/<repo> quality score dropped.",
            "A watched repository crossed the score-drop alert threshold.",
            Some("https://github.com/owner/repo"),
            &[EmailField {
                name: "New <score>".to_string(),
                value: "0.42 & falling".to_string(),
            }],
        );

        assert!(
            email
                .text
                .contains("Repository: https://github.com/owner/repo")
        );
        assert!(email.text.contains("New <score>: 0.42 & falling"));
        assert!(email.html.contains("owner/&lt;repo&gt;"));
        assert!(email.html.contains("New &lt;score&gt;"));
        assert!(email.html.contains("0.42 &amp; falling"));
        assert!(email.html.contains("Open repository"));
    }

    #[test]
    fn digest_email_groups_sections_in_text_and_html() {
        let email = render_digest_email(
            EmailLocale::En,
            &[
                EmailSection {
                    title: "Repos to watch".to_string(),
                    items: vec!["owner/a".to_string(), "owner/b".to_string()],
                },
                EmailSection {
                    title: "New flags".to_string(),
                    items: vec!["owner/c".to_string()],
                },
            ],
        );

        assert_eq!(email.subject, "UseStakly daily watch digest");
        assert!(email.text.contains("Repos to watch\nowner/a\nowner/b"));
        assert!(email.text.contains("New flags\nowner/c"));
        assert!(email.html.contains("Daily watch digest"));
        assert!(email.html.contains("Repos to watch"));
        assert!(email.html.contains("owner/a\nowner/b"));
    }
}
