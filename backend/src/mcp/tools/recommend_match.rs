pub(crate) fn build_use_case_service_query(
    need: &str,
    ecosystem: Option<&str>,
    topics: &[String],
) -> String {
    let mut parts = vec![need.trim().to_string()];
    if let Some(ecosystem) = ecosystem.map(str::trim).filter(|value| !value.is_empty()) {
        parts.push(ecosystem.to_string());
    }
    parts.extend(topics.iter().cloned());
    parts.join(" ")
}

pub(crate) fn repo_matches_topics(
    repo: &crate::domain::repo::RepoSearchResult,
    required: &[String],
) -> bool {
    required.iter().all(|required_topic| {
        repo.topics
            .iter()
            .any(|topic| topic.eq_ignore_ascii_case(required_topic))
            || repo
                .description
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .contains(required_topic)
            || repo.name.to_ascii_lowercase().contains(required_topic)
    })
}

#[cfg(test)]
pub(crate) fn build_recommendation_query(
    need: &str,
    ecosystem: Option<&str>,
    topics: &[String],
    intent: &crate::services::recommendations::UseCaseIntent,
) -> String {
    let mut parts = vec![need.trim().to_string()];
    if let Some(ecosystem) = ecosystem.map(str::trim).filter(|s| !s.is_empty()) {
        parts.push(ecosystem.to_string());
    }
    let inferred_topics;
    let query_topics = if topics.is_empty() {
        inferred_topics = recommendation_query_topics(intent);
        &inferred_topics
    } else {
        topics
    };
    for topic in query_topics {
        parts.push(topic.replace('-', " "));
    }
    parts.join(" ")
}

#[cfg(test)]
pub(crate) fn recommendation_query_topics(
    intent: &crate::services::recommendations::UseCaseIntent,
) -> Vec<String> {
    if intent
        .categories
        .iter()
        .any(|category| category == "testing")
    {
        return vec!["test".to_string(), "testing".to_string()];
    }
    if intent
        .categories
        .iter()
        .any(|category| category == "ui-kit")
    {
        return vec!["components".to_string(), "ui".to_string()];
    }
    if intent.categories.iter().any(|category| category == "auth") {
        return vec!["auth".to_string(), "oauth".to_string()];
    }
    if intent.categories.iter().any(|category| category == "orm") {
        return vec!["orm".to_string(), "database".to_string()];
    }
    intent
        .topics
        .iter()
        .filter(|topic| topic.len() >= 3)
        .take(2)
        .cloned()
        .collect()
}

#[cfg(test)]
pub(crate) fn repo_matches_intent(
    repo: &crate::domain::repo::RepoSearchResult,
    intent: &crate::services::recommendations::UseCaseIntent,
) -> bool {
    if intent.categories.is_empty() {
        return repo_matches_any_topic(repo, &intent.topics);
    }
    repo_matches_any_category(repo, &intent.categories)
        || repo_matches_any_topic(repo, &recommendation_query_topics(intent))
}

#[cfg(test)]
pub(crate) fn repo_matches_any_category(
    repo: &crate::domain::repo::RepoSearchResult,
    categories: &[String],
) -> bool {
    !categories.is_empty()
        && repo.categories.iter().any(|category| {
            categories
                .iter()
                .any(|expected| category.category.eq_ignore_ascii_case(expected))
        })
}

#[cfg(test)]
pub(crate) fn repo_matches_any_topic(
    repo: &crate::domain::repo::RepoSearchResult,
    topics: &[String],
) -> bool {
    topics.iter().any(|topic| {
        repo.topics
            .iter()
            .any(|repo_topic| repo_topic.eq_ignore_ascii_case(topic))
            || repo
                .description
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .contains(topic)
            || repo.name.to_ascii_lowercase().contains(topic)
    })
}
