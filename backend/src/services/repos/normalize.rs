pub(crate) fn tokenize_query(query: Option<&str>) -> Vec<String> {
    let mut tokens = Vec::new();
    for token in query
        .unwrap_or_default()
        .split(|c: char| !c.is_alphanumeric())
        .map(|token| token.trim().to_lowercase())
        .filter(|token| token.len() >= 2)
    {
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    tokens
}

pub(crate) fn normalize_topics(topics: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    for topic in topics {
        let topic = topic
            .trim()
            .trim_start_matches('#')
            .to_ascii_lowercase()
            .replace('_', "-");
        if !topic.is_empty() && !normalized.contains(&topic) {
            normalized.push(topic);
        }
    }
    normalized
}

pub(crate) fn normalize_public_signal(signal: &str) -> String {
    match signal {
        "security_issue" => "security-issue".to_string(),
        other => other.to_string(),
    }
}

pub(crate) fn normalize_maturity_bands(bands: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    for band in bands {
        let band = band.trim().to_ascii_lowercase().replace('_', "-");
        let known = matches!(
            band.as_str(),
            "established" | "emerging" | "experimental" | "stale" | "noisy"
        );
        if known && !normalized.contains(&band) {
            normalized.push(band);
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::{normalize_maturity_bands, normalize_topics, tokenize_query};

    #[test]
    fn tokenize_query_normalizes_and_deduplicates() {
        let tokens = tokenize_query(Some("React UI, react  typescript"));
        assert_eq!(tokens, vec!["react", "ui", "typescript"]);
    }

    #[test]
    fn tokenize_query_drops_short_empty_tokens() {
        let tokens = tokenize_query(Some("a / c++ / rpc"));
        assert_eq!(tokens, vec!["rpc"]);
    }

    #[test]
    fn normalize_topics_deduplicates_and_normalizes() {
        let topics = normalize_topics(&[
            "#React".to_string(),
            "data_grid".to_string(),
            "react".to_string(),
        ]);
        assert_eq!(topics, vec!["react", "data-grid"]);
    }

    #[test]
    fn normalize_maturity_bands_keeps_known_values_only() {
        let bands = normalize_maturity_bands(&[
            "Emerging".to_string(),
            "experimental".to_string(),
            "unknown".to_string(),
            "emerging".to_string(),
        ]);
        assert_eq!(bands, vec!["emerging", "experimental"]);
    }
}
