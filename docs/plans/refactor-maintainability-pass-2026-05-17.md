# UseStakly Maintainability Refactor Pass Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce maintainability risk in the largest active backend/frontend files without changing runtime behavior, public API contracts, scoring, MCP schemas, or product copy.

**Architecture:** This is a behavior-preserving refactor. Keep the existing boundaries: HTTP handlers stay I/O-only, business logic stays in `services/`, MCP server handlers remain the auth/serialization boundary, and frontend routes delegate reusable UI to feature components. Split by responsibility, keep public function names stable where possible, and verify after every task.

**Tech Stack:** Rust 2024, Axum, SQLx/Postgres, rmcp Streamable HTTP, React 19, Vite 7, TypeScript, TanStack Router/Query, PowerShell scripts.

---

## Context

Current size hotspots from the 2026-05-17 audit:

| File | Lines | Refactor urgency | Reason |
|---|---:|---|---|
| `backend/src/services/ingestion/github.rs` | ~1383 | High | Mixes GitHub HTTP, quota/ETag, structural signals, DB persistence, URL parsing, and tests. This is the highest-risk active file after corpus import. |
| `backend/src/mcp/server.rs` | ~1129 | Medium | Tool handlers are still long, but DTO/mappers already live under `backend/src/mcp/tools/*`. Refactor when touching MCP next. |
| `backend/src/services/notification_channels.rs` | ~869 | High | Mixes DB store, email SMTP, Discord webhook, encryption, validation, and alert message formatting. Relevant to launch Task 5 email proof. |
| `backend/src/services/recommendations.rs` | ~756 | Low/Medium | Long heuristic file, but internally coherent. Do not split until recommendation calibration resumes. |
| `frontend/src/routes/index.tsx` | ~614 | Medium | Landing page is visual/content-heavy. Split only if the public polish pass continues. |
| `frontend/src/routes/repo-detail.tsx` | ~477 | Low | Already delegates to feature components. |
| `frontend/src/i18n/en.ts` | ~897 | Low | Large because it is copy. Keep as-is while only EN is active. |

Already improved:

- `backend/src/services/repos.rs` was split into `backend/src/services/repos/*`.
- `backend/src/mcp/tools/*` already contains many MCP DTO/mapping helpers.
- `frontend/src/routes/discover.tsx` is no longer the top frontend hotspot.

Current unrelated working tree note:

- `.gitignore` has a local change adding `spitch.md`. Do not revert it.
- Treat this note as informational only: always inspect `git status` at execution time because other user changes may exist.

---

## Non-Goals

- Do not change scoring semantics, formula weights, radar classification, proof tiers, or dimensions.
- Do not change MCP tool names, parameter shapes, output JSON fields, or provenance fields.
- Do not change REST routes or response DTOs.
- Do not remove legacy SQL migrations for snippets/libraries.
- Do not rename product concepts or change visible marketing copy except if a compile-time extraction requires moving JSX unchanged.
- Do not add Redis, queues, or new infrastructure.

---

## File Structure Target

### GitHub Ingestion

Create a module directory and move focused responsibilities out of the monolith:

```text
backend/src/services/ingestion/github/
  mod.rs              # public API: build_client, fetch_repo, ingest_repo, parse_github_repo_input, shared structs
  client.rs           # Octocrab client, conditional requests, ETag headers, GitHub rate-limit classification/logging
  repo.rs             # fetch_repo/fetch_repo_with_state and GitHubRepoMetadata assembly
  structural.rs       # commits, releases, README, owner activity, StructuralSignals
  persist.rs          # load_existing_ingestion_state, upsert_github_artifact, ingest_repo orchestration
  parse.rs            # parse_github_repo_input
```

Keep the public module path stable:

```rust
use crate::services::ingestion::github::{build_client, ingest_repo, parse_github_repo_input};
```

Keep `backend/src/services/ingestion/github_quota.rs` as a sibling module. Do not merge quota persistence into `github/client.rs` during this refactor. The split `client.rs` should continue to call `crate::services::ingestion::github_quota::{record_headers_snapshot, record_limit_hit}` where the current monolith does, and `backend/src/services/ingestion/mod.rs` should keep `pub mod github_quota;`.

### Notification Channels

Create a module directory and isolate providers/security/storage:

```text
backend/src/services/notification_channels/
  mod.rs              # public API: list_for_user, upsert, delete, send_test, deliver_watch_alert
  model.rs            # NotificationChannelType, DTOs, row structs
  store.rs            # SQL reads/writes and delivery error persistence
  crypto.rs           # encrypt/decrypt/cipher helpers
  email.rs            # SMTP message sending, test email, watch alert email
  discord.rs          # Discord webhook validation, masking, test/watch alert delivery
  message.rs          # WatchAlertMessage and watch_alert_message
```

Keep the public service path stable:

```rust
crate::services::notification_channels::send_test(...)
crate::services::notification_channels::deliver_watch_alert(...)
```

Keep crate-visible helpers used by the digest service available:

```rust
pub(crate) use crypto::decrypt_webhook_url;
pub(crate) use email::send_email;
```

`backend/src/services/notification_digest.rs` currently imports these helpers from `notification_channels`, so the refactor must not hide them.

### MCP Server

Keep `#[tool_router]` and `#[tool]` declarations in:

```text
backend/src/mcp/server.rs
```

Move handler bodies into:

```text
backend/src/mcp/tools/search_handler.rs
backend/src/mcp/tools/recommend_handler.rs
backend/src/mcp/tools/context_handler.rs
backend/src/mcp/tools/log_usage_handler.rs
backend/src/mcp/tools/watch_handler.rs
```

Each helper should accept `&AppState`, params, and `Parts`, then return the same output type:

```rust
pub async fn handle_search_github_repos(
    state: &AppState,
    p: SearchReposParams,
    parts: Parts,
) -> Result<SearchReposOutput, ErrorData>
```

The tool method remains a thin wrapper:

```rust
async fn search_github_repos(
    &self,
    Parameters(p): Parameters<SearchReposParams>,
    Extension(parts): Extension<Parts>,
) -> Result<Json<SearchReposOutput>, ErrorData> {
    handle_search_github_repos(&self.state, p, parts).await.map(Json)
}
```

Avoid naming confusion with the existing DTO/helper modules:

- existing `backend/src/mcp/tools/search.rs`, `recommend.rs`, `context.rs`, `write.rs`, and related files remain DTO/mapping/helper modules;
- new `*_handler.rs` files contain the former `server.rs` tool bodies;
- do not rename the existing modules or duplicate DTO definitions.

Target file size after splitting: prefer files under ~200 lines for new code modules. If a file would exceed that because tests are colocated or SQL is large, either split one level deeper, for example `structural/readme.rs` and `structural/commits.rs`, or document the exception in the commit message.

### Landing Page

Only split if more visual polish lands:

```text
frontend/src/features/landing/
  HeroSection.tsx
  LiveRepositoryPanel.tsx
  LandingMetrics.tsx
  LandingPrinciples.tsx
  LandingCta.tsx
  index.ts
```

`frontend/src/routes/index.tsx` should become route composition, not a style rewrite.

---

## Execution Order

1. Notification channels split, because Task 5 email proof is still launch-relevant.
2. GitHub ingestion split, because ingestion/quota is the largest active backend risk.
3. MCP server thin-handler split, only after Tasks 1-2 are merged and verified.
4. Landing page split, only if continuing frontend polish.
5. Docs/backlog cleanup after code is stable.

Each task should be a separate commit.

---

## Task 0: Baseline Verification

**Files:**
- Read only.

- [ ] **Step 1: Confirm working tree and preserve unrelated changes**

Run:

```powershell
git status --short --branch
```

Expected:

```text
current branch is visible
any modified/untracked files are understood before editing
```

Do not assume only `.gitignore` is modified. If unrelated files appear, inspect them before editing and do not revert user changes.

- [ ] **Step 2: Run backend baseline checks**

Run:

```powershell
cd backend
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Expected:

```text
cargo fmt --check exits 0
cargo test exits 0
cargo clippy exits 0
```

- [ ] **Step 3: Run frontend baseline check**

Run:

```powershell
cd frontend
npm run build
```

Expected:

```text
vite build exits 0
```

- [ ] **Step 4: Commit nothing**

This task is a checkpoint only.

---

## Task 1: Split Notification Channel Providers

**Goal:** Make notification delivery easier to debug before proving real product email delivery.

**Files:**
- Modify: `backend/src/services/mod.rs`
- Replace file with module directory:
  - Move from: `backend/src/services/notification_channels.rs`
  - Create: `backend/src/services/notification_channels/mod.rs`
  - Create: `backend/src/services/notification_channels/model.rs`
  - Create: `backend/src/services/notification_channels/store.rs`
  - Create: `backend/src/services/notification_channels/crypto.rs`
  - Create: `backend/src/services/notification_channels/email.rs`
  - Create: `backend/src/services/notification_channels/discord.rs`
  - Create: `backend/src/services/notification_channels/message.rs`
- Test: existing tests currently inside `notification_channels.rs`, moved to the relevant modules.

- [ ] **Step 1: Move model types first**

Move these items into `model.rs`:

```rust
NotificationChannelType
NotificationChannelSummary
UpsertNotificationChannel
ChannelRow
ChannelSecretRow
ExistingChannelSecretRow
DeliveryChannelRow
WatchAlertDelivery
```

Expose only what other modules need:

```rust
pub enum NotificationChannelType { ... }
pub struct NotificationChannelSummary { ... }
pub struct UpsertNotificationChannel { ... }
pub struct WatchAlertDelivery<'a> { ... }
pub(crate) struct ChannelRow { ... }
pub(crate) struct ChannelSecretRow { ... }
pub(crate) struct ExistingChannelSecretRow { ... }
pub(crate) struct DeliveryChannelRow { ... }
```

- [ ] **Step 2: Create `mod.rs` with stable public exports**

`mod.rs` should declare:

```rust
mod crypto;
mod discord;
mod email;
mod message;
mod model;
mod store;

pub use model::{
    NotificationChannelSummary, NotificationChannelType, UpsertNotificationChannel,
    WatchAlertDelivery,
};

pub use discord::{mask_discord_webhook_url, validate_discord_webhook_url};
pub use email::validate_notification_email;
pub(crate) use crypto::decrypt_webhook_url;
pub(crate) use email::send_email;

use crate::app::error::ApiError;
use crate::config::AppConfig;
use sqlx::PgPool;
use uuid::Uuid;
```

Keep these public function names in `mod.rs`:

```rust
pub async fn list_for_user(...)
pub async fn upsert(...)
pub async fn delete(...)
pub async fn send_test(...)
pub async fn deliver_watch_alert(...)
```

- [ ] **Step 3: Move crypto helpers**

Move into `crypto.rs`:

```rust
pub(crate) fn encrypt_webhook_url(secret: &str, plaintext: &str) -> Result<String, String>
pub(crate) fn decrypt_webhook_url(secret: &str, ciphertext: &str) -> Result<String, String>
fn cipher_from_secret(secret: &str) -> Result<Aes256Gcm, String>
```

Move the existing encryption roundtrip test into `crypto.rs`.

- [ ] **Step 4: Move Discord helpers**

Move into `discord.rs`:

```rust
pub fn validate_discord_webhook_url(value: &str) -> Result<String, ApiError>
pub fn mask_discord_webhook_url(value: &str) -> String
pub(crate) async fn post_discord_test_message(webhook_url: &str) -> Result<(), anyhow::Error>
pub(crate) async fn post_discord_watch_alert(...) -> Result<(), anyhow::Error>
```

Move these tests into `discord.rs`:

```rust
discord_webhook_accepts_only_discord_webhook_urls
discord_webhook_url_is_masked_without_leaking_secret
```

- [ ] **Step 5: Move email helpers**

Move into `email.rs`:

```rust
pub fn validate_notification_email(value: &str) -> Result<String, ApiError>
pub(crate) async fn post_email_test_message(config: &AppConfig, to: &str) -> Result<(), anyhow::Error>
pub(crate) async fn post_email_watch_alert(...) -> Result<(), anyhow::Error>
pub(crate) async fn send_email(config: &AppConfig, to: &str, subject: &str, html: String, text: String) -> Result<(), anyhow::Error>
```

Move this test into `email.rs`:

```rust
notification_email_rejects_invalid_address
```

`send_email` must remain reachable as `crate::services::notification_channels::send_email` for `notification_digest.rs`.

- [ ] **Step 6: Move watch alert message formatting**

Move into `message.rs`:

```rust
pub(crate) struct WatchAlertMessage { ... }
pub(crate) fn watch_alert_message(...) -> WatchAlertMessage
```

Move this test into `message.rs`:

```rust
watch_alert_message_explains_score_drop
```

- [ ] **Step 7: Move SQL store helpers**

Move into `store.rs`:

```rust
pub(crate) async fn existing_channel_secret(...)
pub(crate) async fn update_delivery_error(...)
```

If `list_for_user`, `upsert`, `delete`, `send_test`, and `deliver_watch_alert` become too large in `mod.rs`, move their SQL-only parts into `store.rs`, but keep public API functions in `mod.rs`.

- [ ] **Step 8: Run focused tests**

Run:

```powershell
cd backend
cargo test services::notification_channels
```

Expected:

```text
all notification_channels tests pass
```

- [ ] **Step 9: Run backend checks**

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Expected: all pass.

- [ ] **Step 10: Commit**

Run:

```powershell
git add backend/src/services/notification_channels.rs backend/src/services/notification_channels backend/src/services/mod.rs
git commit -m "refactor: split notification channel providers"
```

---

## Task 2: Split GitHub Ingestion Into Focused Modules

**Goal:** Make GitHub ingestion safer to change after the Awesome corpus import and quota work.

**Files:**
- Replace file with module directory:
  - Move from: `backend/src/services/ingestion/github.rs`
  - Create: `backend/src/services/ingestion/github/mod.rs`
  - Create: `backend/src/services/ingestion/github/client.rs`
  - Create: `backend/src/services/ingestion/github/repo.rs`
  - Create: `backend/src/services/ingestion/github/structural.rs`
  - Create: `backend/src/services/ingestion/github/persist.rs`
  - Create: `backend/src/services/ingestion/github/parse.rs`
- Keep: `backend/src/services/ingestion/structural_extras.rs`
- Test: move existing tests to modules near the functions they cover.

- [ ] **Step 1: Create module shell without changing public API**

Create `github/mod.rs` with:

```rust
mod client;
mod parse;
mod persist;
mod repo;
mod structural;

pub use client::build_client;
pub(crate) use client::github_get_json_with_etag;
pub use parse::parse_github_repo_input;
pub use persist::{ingest_repo, upsert_github_artifact};
pub use repo::{fetch_repo, GitHubRepoMetadata};
pub use structural::{GitHubIngestionMetadata, StructuralSignals};
```

Use `pub(crate)` for helpers that tests or sibling modules need, not `pub` by default.

Keep `backend/src/services/ingestion/github_quota.rs` outside this directory split. `client.rs` should import it from the sibling module rather than moving or duplicating quota logic.

- [ ] **Step 2: Move parse logic first**

Move `parse_github_repo_input` and its tests into `parse.rs`:

```rust
pub fn parse_github_repo_input(input: &str) -> Result<(String, String), ApiError>
```

Move these tests:

```rust
parses_owner_repo
parses_url_with_query_and_git_suffix
rejects_extra_segments
```

Run:

```powershell
cd backend
cargo test services::ingestion::github::parse
```

- [ ] **Step 3: Move client, rate-limit, ETag helpers**

Move into `client.rs`:

```rust
pub fn build_client(token: &str) -> Result<Octocrab, ApiError>
pub(crate) fn classify_rate_limit(...)
pub(crate) fn conditional_request_headers(...)
pub(crate) fn github_api_failure_with_headers(...)
pub(crate) fn github_api_failure(...)
pub(crate) async fn github_get_json_with_etag<T>(...)
```

Move related structs/enums:

```rust
GitHubRateLimitKind
```

Move these tests:

```rust
github_rate_limit_headers_detect_primary_limit
github_rate_limit_body_detects_secondary_limit
github_api_failure_maps_secondary_limit_with_context
github_api_failure_maps_access_denied_with_status_context
conditional_headers_include_etag_when_present
conditional_headers_skip_blank_etag
backoff_delay_uses_retry_after_before_default_secondary_delay
```

Run:

```powershell
cd backend
cargo test services::ingestion::github::client
```

After moving this code, verify `client.rs` still records quota snapshots and rate-limit hits through `github_quota.rs`. Search for:

```powershell
rg -n "record_headers_snapshot|record_limit_hit|github_quota" backend/src/services/ingestion
```

Expected: calls still exist from the split GitHub client path, and `backend/src/services/ingestion/mod.rs` still exposes `github_quota`.

- [ ] **Step 4: Move structural signal logic**

Move into `structural.rs`:

```rust
pub struct StructuralSignals { ... }
pub struct GitHubIngestionMetadata { ... }
pub(crate) struct ExistingGitHubIngestionState { ... }
pub(crate) async fn fetch_structural_signals(...)
pub(crate) async fn fetch_latest_default_branch_commit_at(...)
pub(crate) async fn fetch_commits_since(...)
pub(crate) async fn fetch_readme_text_with_etag(...)
pub(crate) async fn fetch_owner_activity_summary(...)
pub(crate) fn summarize_releases(...)
```

Move related structs:

```rust
CommitSummary
CommitTally
GitHubReadmeResponse
GitHubRepoEvent
GitHubEventActor
OwnerActivitySummary
OwnerActivityFetch
ReadmeFetch
```

Move these tests:

```rust
decodes_base64_readme_content_for_classification
limits_readme_text_used_for_classification
tally_counts_distinct_authors_and_30d_window
tally_handles_empty_input
tally_solo_dev_high_cadence_is_one_contributor
tally_30d_boundary_is_inclusive
release_summary_selects_newest_published_release
release_summary_handles_empty_releases
etag_not_modified_preserves_existing_release_values
owner_activity_ignores_bots_and_counts_same_day_activity
owner_activity_returns_none_for_empty_events
```

Run:

```powershell
cd backend
cargo test services::ingestion::github::structural
```

- [ ] **Step 5: Move repo metadata assembly**

Move into `repo.rs`:

```rust
pub struct GitHubRepoMetadata { ... }
pub async fn fetch_repo(...)
pub(crate) async fn fetch_repo_with_state(...)
```

`repo.rs` should call:

```rust
structural::fetch_structural_signals(...)
structural::fetch_latest_default_branch_commit_at(...)
client::github_api_failure(...)
```

Run:

```powershell
cd backend
cargo check
```

- [ ] **Step 6: Move persistence and ingest orchestration**

Move into `persist.rs`:

```rust
pub async fn upsert_github_artifact(...)
pub async fn ingest_repo(...)
pub(crate) async fn load_existing_ingestion_state(...)
```

`persist.rs` should call:

```rust
repo::fetch_repo_with_state(...)
repo_categories::upsert_repo_categories_with_readme(...)
semantic_search::upsert_repo_embedding_if_enabled(...)
```

Keep SQL text unchanged except for imports.

- [ ] **Step 7: Run focused ingestion tests**

Run:

```powershell
cd backend
cargo test services::ingestion::github
```

Expected: all moved ingestion tests pass.

- [ ] **Step 8: Run backend checks**

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Expected: all pass.

- [ ] **Step 9: Commit**

Run:

```powershell
git add backend/src/services/ingestion/github.rs backend/src/services/ingestion/github backend/src/services/ingestion/mod.rs
git commit -m "refactor: split github ingestion modules"
```

---

## Task 3: Thin MCP Server Tool Handlers

**Goal:** Reduce `server.rs` to MCP registration and thin wrappers while preserving tool schemas exactly.

**Files:**
- Modify: `backend/src/mcp/server.rs`
- Modify: `backend/src/mcp/tools/mod.rs`
- Create: `backend/src/mcp/tools/search_handler.rs`
- Create: `backend/src/mcp/tools/recommend_handler.rs`
- Create: `backend/src/mcp/tools/context_handler.rs`
- Create: `backend/src/mcp/tools/log_usage_handler.rs`
- Create: `backend/src/mcp/tools/watch_handler.rs`
- Test: existing tests in `backend/src/mcp/server.rs`, moved only if they directly test moved helpers.

- [ ] **Step 1: Extract search handler**

Create `search_handler.rs`:

```rust
use http::request::Parts;
use rmcp::ErrorData;

use crate::app::AppState;
use crate::mcp::auth::verify_bearer;
use crate::mcp::tools::{SearchReposOutput, SearchReposParams};

pub async fn handle_search_github_repos(
    state: &AppState,
    p: SearchReposParams,
    parts: Parts,
) -> Result<SearchReposOutput, ErrorData> {
    verify_bearer(&state.db, &parts).await?;
    // Move the existing search_github_repos body here, returning SearchReposOutput.
}
```

Update `server.rs` wrapper:

```rust
handle_search_github_repos(&self.state, p, parts).await.map(Json)
```

- [ ] **Step 2: Extract recommendation handler**

Create `recommend_handler.rs` with:

```rust
pub async fn handle_recommend_github_repos(
    state: &AppState,
    p: RecommendReposParams,
    parts: Parts,
) -> Result<RecommendReposOutput, ErrorData>
```

Move the existing body of `recommend_github_repos` unchanged except for imports.

- [ ] **Step 3: Extract context handler**

Create `context_handler.rs` with:

```rust
pub async fn handle_get_repo_quality_context(
    state: &AppState,
    p: RepoContextParams,
    parts: Parts,
) -> Result<RepoContextOutput, ErrorData>
```

Move the existing body of `get_repo_quality_context` unchanged except for imports.

- [ ] **Step 4: Extract log usage handler**

Create `log_usage_handler.rs` with:

```rust
pub async fn handle_log_usage(
    state: &AppState,
    p: LogUsageParams,
    parts: Parts,
) -> Result<LogUsageOutput, ErrorData>
```

Move the existing body of `log_usage` unchanged except for imports.

- [ ] **Step 5: Extract watch handlers**

Create `watch_handler.rs` with:

```rust
pub async fn handle_watch_repo(
    state: &AppState,
    p: WatchRepoParams,
    parts: Parts,
) -> Result<WatchRepoOutput, ErrorData>

pub async fn handle_watch_use_case(
    state: &AppState,
    p: WatchUseCaseParams,
    parts: Parts,
) -> Result<WatchUseCaseOutput, ErrorData>
```

Move existing bodies unchanged except for imports.

- [ ] **Step 6: Export handlers from `tools/mod.rs`**

Add:

```rust
pub use context_handler::handle_get_repo_quality_context;
pub use log_usage_handler::handle_log_usage;
pub use recommend_handler::handle_recommend_github_repos;
pub use search_handler::handle_search_github_repos;
pub use watch_handler::{handle_watch_repo, handle_watch_use_case};
```

- [ ] **Step 7: Preserve tool metadata**

Verify this command shows the same six tool names:

```powershell
rg -n "name = \"(search_github_repos|recommend_github_repos|get_repo_quality_context|log_usage|watch_repo|watch_use_case)\"" backend/src/mcp/server.rs
```

Expected: six matches.

- [ ] **Step 8: Run MCP tests**

Run:

```powershell
cd backend
cargo test mcp
```

Expected: all MCP tests pass.

- [ ] **Step 9: Run full backend checks**

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Expected: all pass.

- [ ] **Step 10: Optional live smoke after deployment only**

Do not run against prod unless this refactor has been deployed.

After deployment, run:

```powershell
.\scripts\mcp-live-smoke.ps1
```

Expected: initialize/tools/search/context/log_usage flow passes.

- [ ] **Step 11: Commit**

Run:

```powershell
git add backend/src/mcp/server.rs backend/src/mcp/tools
git commit -m "refactor: thin mcp tool handlers"
```

---

## Task 4: Split Landing Page Sections If Front Polish Continues

**Goal:** Make the landing route easier to iterate visually without changing the current design.

**Files:**
- Modify: `frontend/src/routes/index.tsx`
- Create: `frontend/src/features/landing/HeroSection.tsx`
- Create: `frontend/src/features/landing/LiveRepositoryPanel.tsx`
- Create: `frontend/src/features/landing/LandingMetrics.tsx`
- Create: `frontend/src/features/landing/LandingPrinciples.tsx`
- Create: `frontend/src/features/landing/LandingCta.tsx`
- Create: `frontend/src/features/landing/index.ts`

- [ ] **Step 1: Extract `LiveRepositoryPanel` first**

Move only the live repository card JSX and related local formatting helpers from `index.tsx` into:

```tsx
export function LiveRepositoryPanel() {
  return (
    // existing JSX copied without visual changes
  );
}
```

Use the same class names.

- [ ] **Step 2: Run frontend build**

Run:

```powershell
cd frontend
npm run build
```

Expected: build succeeds.

- [ ] **Step 3: Extract `HeroSection`**

Move hero heading, body copy, CTAs, and panel composition into:

```tsx
export function HeroSection() {
  return (
    // existing JSX copied without visual changes
  );
}
```

`index.tsx` should render:

```tsx
<HeroSection />
```

- [ ] **Step 4: Extract metrics and principles sections**

Move metrics into `LandingMetrics.tsx`:

```tsx
export function LandingMetrics() {
  return (
    // existing metrics JSX copied without visual changes
  );
}
```

Move principles/content rows into `LandingPrinciples.tsx`:

```tsx
export function LandingPrinciples() {
  return (
    // existing principles JSX copied without visual changes
  );
}
```

- [ ] **Step 5: Export components**

Create `frontend/src/features/landing/index.ts`:

```ts
export { HeroSection } from "./HeroSection";
export { LandingCta } from "./LandingCta";
export { LandingMetrics } from "./LandingMetrics";
export { LandingPrinciples } from "./LandingPrinciples";
export { LiveRepositoryPanel } from "./LiveRepositoryPanel";
```

- [ ] **Step 6: Verify no visible copy changed unintentionally**

Run:

```powershell
git diff -- frontend/src/routes/index.tsx frontend/src/features/landing
```

Expected: mostly moved JSX. No new product copy except import/export changes.

- [ ] **Step 7: Run frontend verification**

Run:

```powershell
cd frontend
npm run build
```

Expected: build succeeds.

If a local dev server is already running, use it. Otherwise:

```powershell
cd frontend
npm run dev
```

Open `/` and verify:

- navbar wordmark still displays `UseStakly`;
- hero heading still starts with `Choose GitHub OSS`;
- live repository panel renders;
- footer is not visually broken on desktop.

- [ ] **Step 8: Commit**

Run:

```powershell
git add frontend/src/routes/index.tsx frontend/src/features/landing
git commit -m "refactor: split landing page sections"
```

---

## Task 5: Documentation And Backlog Cleanup

**Goal:** Align docs with the refactor so future agents do not chase old file paths.

**Files:**
- Modify: `docs/source-of-truth.md`
- Modify: `docs/architecture-backend-current.md`
- Modify: `docs/plans/remaining-work-2026-05-03.md`
- Modify: `AGENTS.md`
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update backend architecture paths**

In `docs/architecture-backend-current.md`, update references:

```text
backend/src/services/ingestion/github.rs
```

to:

```text
backend/src/services/ingestion/github/*
```

and:

```text
backend/src/services/notification_channels.rs
```

to:

```text
backend/src/services/notification_channels/*
```

- [ ] **Step 2: Update MCP source-of-truth wording only if Task 3 shipped**

If Task 3 shipped, update `docs/source-of-truth.md` from:

```text
MCP tool handlers: backend/src/mcp/server.rs
DTO/mappers: backend/src/mcp/tools/*
```

to:

```text
MCP tool registration: backend/src/mcp/server.rs
MCP handler implementations: backend/src/mcp/tools/*_handler.rs
MCP DTO/mappers/helpers: backend/src/mcp/tools/*
```

- [ ] **Step 3: Mark refactor backlog items accurately**

In `docs/plans/remaining-work-2026-05-03.md`, update the maintainability item with actual completions:

```markdown
- [x] `backend/src/services/notification_channels.rs` split into `backend/src/services/notification_channels/*` (date, commit).
- [x] `backend/src/services/ingestion/github.rs` split into `backend/src/services/ingestion/github/*` (date, commit).
- [x] `backend/src/mcp/server.rs` thinned into `backend/src/mcp/tools/*` handlers (date, commit).
- [ ] Frontend landing/discover polish refactors remain opportunistic.
```

Only mark items completed if the matching task shipped.

- [ ] **Step 4: Clean stale agent docs while touching them**

If editing `AGENTS.md` or `CLAUDE.md`, avoid stale claims:

```text
Formula v2 is the runtime scoring formula. Formula v1/v1.1 docs and TOML are historical audit references.
```

Do not remove the warning that snippets/libraries are legacy and not product scope.

- [ ] **Step 5: Run doc source audit**

Run:

```powershell
.\scripts\audit-doc-source-truth.ps1
```

Expected:

```text
passed
```

- [ ] **Step 6: Run diff check**

Run:

```powershell
git diff --check
```

Expected: no whitespace errors. CRLF warnings are acceptable on Windows if no diff-check error is emitted.

- [ ] **Step 7: Commit**

Run:

```powershell
git add docs/source-of-truth.md docs/architecture-backend-current.md docs/plans/remaining-work-2026-05-03.md AGENTS.md CLAUDE.md
git commit -m "docs: update refactor source of truth"
```

---

## Final Verification

Run after all implemented tasks:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

cd ..\frontend
npm run build

cd ..
.\scripts\audit-doc-source-truth.ps1
git diff --check
git status --short --branch
```

Expected:

```text
backend fmt/clippy/tests pass
frontend build passes
doc audit passes
diff check passes
working tree only contains intentional files
```

---

## Rollback Strategy

Because this plan is behavior-preserving:

- If a task fails after moving code, stop and restore only the files touched by that task from the previous commit.
- Do not revert unrelated user files such as `.gitignore` or local-only docs.
- Prefer one commit per task so rollback is `git revert <task-commit>` instead of manual surgery.

---

## Acceptance Criteria

- `notification_channels` and `github` are module directories with focused responsibilities.
- Public Rust import paths used elsewhere in the app still compile.
- MCP tool names and JSON schemas remain unchanged.
- REST routes and frontend API types remain unchanged.
- Existing tests pass.
- No product behavior changes are intentionally introduced.
- Docs no longer point agents at obsolete refactor targets.
