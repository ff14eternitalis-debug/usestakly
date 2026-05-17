use crate::app::error::ApiError;

pub fn parse_github_repo_input(input: &str) -> Result<(String, String), ApiError> {
    let trimmed = input.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return Err(ApiError::bad_request(
            "repo is required (GitHub URL or owner/repo)",
        ));
    }

    let candidate = trimmed
        .strip_prefix("https://github.com/")
        .or_else(|| trimmed.strip_prefix("http://github.com/"))
        .or_else(|| trimmed.strip_prefix("github.com/"))
        .unwrap_or(trimmed)
        .split(['?', '#'])
        .next()
        .unwrap_or(trimmed)
        .trim_end_matches(".git");

    let mut parts = candidate.split('/').filter(|p| !p.trim().is_empty());
    let owner = parts
        .next()
        .ok_or_else(|| ApiError::bad_request("repo must include owner/repo"))?;
    let name = parts
        .next()
        .ok_or_else(|| ApiError::bad_request("repo must include owner/repo"))?;

    if parts.next().is_some() {
        return Err(ApiError::bad_request(
            "repo must be a GitHub URL or owner/repo",
        ));
    }
    if owner.contains(' ') || name.contains(' ') {
        return Err(ApiError::bad_request("repo must not contain whitespace"));
    }

    Ok((owner.to_string(), name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::parse_github_repo_input;

    #[test]
    fn parses_owner_repo() {
        let (owner, repo) = parse_github_repo_input("openai/gpt").unwrap();
        assert_eq!(owner, "openai");
        assert_eq!(repo, "gpt");
    }

    #[test]
    fn parses_url_with_query_and_git_suffix() {
        let (owner, repo) =
            parse_github_repo_input("https://github.com/openai/gpt.git?tab=readme").unwrap();
        assert_eq!(owner, "openai");
        assert_eq!(repo, "gpt");
    }

    #[test]
    fn rejects_extra_segments() {
        assert!(parse_github_repo_input("openai/gpt/issues").is_err());
    }
}
