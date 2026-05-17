mod client;
mod parse;
mod persist;
mod repo;
mod structural;

pub use client::build_client;
pub(crate) use client::github_get_json_with_etag;
pub use parse::parse_github_repo_input;
pub use persist::{ingest_repo, upsert_github_artifact};
pub use repo::{GitHubRepoMetadata, fetch_repo};
pub use structural::{GitHubIngestionMetadata, StructuralSignals};

pub(crate) use client::{GitHubJsonResponse, GitHubReleaseSummary, summarize_releases};
