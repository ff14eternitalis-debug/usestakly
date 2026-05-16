# UseStakly Next Useful Work Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Finish the next high-leverage UseStakly workstreams: safer GitHub ingestion, stronger scoring/trust, clearer discovery UX, live validation, and gradual maintainability refactors.

**Architecture:** Keep the current product boundaries: HTTP handlers stay I/O-only, business logic stays in `services/`, MCP remains a serialization/auth boundary, and frontend pages delegate complex UI to focused components. Each task should ship independently with tests and a small commit.

**Tech Stack:** Rust 2024, Axum, SQLx/Postgres, GitHub REST via `reqwest`, React 19, Vite, TanStack Router/Query, Playwright, npm CLI tests.

---

## Context

UseStakly is already in a healthy public beta state. Local verification on 2026-05-16 passed:

- `cd backend && cargo fmt --check`
- `cd backend && cargo clippy --all-targets -- -D warnings`
- `cd backend && cargo test`
- `cd frontend && npm run build`
- `cd frontend && npm run test:e2e`
- `cd cli && npm test`

The remaining work should improve production confidence and product clarity, not restart broad architecture.

## Execution Order

1. GitHub ingestion reliability.
2. Scoring/trust formula v2 completion.
3. Discovery and repo-detail explanations.
4. Live validation gates.
5. Maintainability refactors.

This order improves data quality first, then trust weighting, then exposes better explanations in the UI, then validates the live paths, then reduces code size risk.

---

## Task 1: GitHub Ingestion Reliability

**Goal:** Make ingestion more robust against GitHub API rate limits and improve release metadata capture.

**Files:**
- Modify: `backend/src/services/ingestion/github.rs`
- Modify: `backend/src/config/mod.rs`
- Test: `backend/src/services/ingestion/github.rs`
- Docs: `docs/plans/remaining-work-2026-05-03.md`

- [x] **Step 1: Add focused tests for GitHub rate-limit response classification**

Add tests in `backend/src/services/ingestion/github.rs` near the existing ingestion helper tests.

Required cases:

```rust
#[test]
fn github_rate_limit_headers_detect_primary_limit() {
    let headers = github_rate_limit_headers("0", "1778791697", None);
    let limit = classify_rate_limit(&headers, 403, "");
    assert!(matches!(limit, Some(GitHubRateLimitKind::Primary { .. })));
}

#[test]
fn github_rate_limit_body_detects_secondary_limit() {
    let headers = github_rate_limit_headers("42", "1778791697", None);
    let limit = classify_rate_limit(
        &headers,
        403,
        "You have exceeded a secondary rate limit. Please wait a few minutes.",
    );
    assert!(matches!(limit, Some(GitHubRateLimitKind::Secondary)));
}
```

Run:

```powershell
cd backend
cargo test services::ingestion::github::tests::github_rate_limit
```

Expected before implementation: compile failure because helpers/types do not exist.

- [x] **Step 2: Implement rate-limit classification helpers**

Add a small internal enum and helpers in `backend/src/services/ingestion/github.rs`.

Acceptance:

- Primary limit is detected when `x-ratelimit-remaining = 0`.
- Secondary limit is detected from GitHub's secondary-limit message.
- The helper returns retry timing when `x-ratelimit-reset` is available.
- No production logging leaks tokens or request authorization headers.

Run:

```powershell
cd backend
cargo test services::ingestion::github::tests::github_rate_limit
```

Expected: tests pass.

- [x] **Step 3: Add conditional request support with ETags**

Extend ingestion fetch helpers to accept optional ETag metadata where useful.

Recommended minimal scope:

- Releases endpoint.
- Repo metadata endpoint if the current code has a single reusable request helper.

Store ETag only if there is already a natural DB field or low-risk metadata JSON location. If no existing persistence slot exists, keep this step to request helper support and document persistence as a later migration.

Run:

```powershell
cd backend
cargo test services::ingestion::github
cargo clippy --all-targets -- -D warnings
```

Expected: all tests pass, no warnings.

- [x] **Step 4: Fix `last_release_at` capture**

Investigate `fetch_releases_summary` in `backend/src/services/ingestion/github.rs`.

Acceptance:

- Public repos with releases populate `last_release_at` from the newest release `published_at`.
- Empty releases return `None` without failing ingestion.
- Non-200 GitHub responses produce a typed ingestion error with status context.
- Add a fixture-style unit test using a small JSON payload with two releases where the newest release is selected.

Run:

```powershell
cd backend
cargo test services::ingestion::github::tests::release
```

Expected: release parsing tests pass.

- [x] **Step 5: Update docs and commit**

Update `docs/plans/remaining-work-2026-05-03.md`:

- Mark `last_release_at` follow-up complete only after verifying against at least three real repos.
- Keep ETag persistence open if helper-only support was shipped.

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Commit:

```powershell
git add backend/src/services/ingestion/github.rs backend/src/config/mod.rs docs/plans/remaining-work-2026-05-03.md
git commit -m "Harden GitHub ingestion metadata refresh"
```

---

## Task 2: Scoring And Trust Formula V2

**Goal:** Finish reporter/owner trust weighting so new or unproven accounts cannot meaningfully steer severe signals.

**Files:**
- Modify: `backend/scoring/formula_v2.toml`
- Modify: `backend/src/services/quality/weighting.rs`
- Modify: `backend/src/services/trust/reputation.rs`
- Modify: `backend/src/services/trust/signal_reviews.rs`
- Test: `backend/src/services/quality/weighting.rs`
- Test: `backend/src/services/trust/reputation.rs`
- Test: `backend/src/services/trust/signal_reviews.rs`
- Docs: `docs/trust-model-v1.md`
- Docs: `docs/plans/remaining-work-2026-05-03.md`

- [ ] **Step 1: Add tests for new-account reporter weight**

Add tests proving:

- A reporter with no real usage gets weight `0.0` for active severe signals.
- A reporter with healthy real usage gets a non-zero weight.
- Existing passive usage weights remain compatible with formula v1.1 expectations.

Run:

```powershell
cd backend
cargo test services::quality::weighting::tests::new_account
```

Expected before implementation: failing test.

- [ ] **Step 2: Add formula v2 trust thresholds**

Add explicit config keys to `backend/scoring/formula_v2.toml`.

Required values:

```toml
[trust]
new_account_active_signal_weight = 0.0
min_real_usage_for_active_weight = 2
owner_dispute_min_reputation = 0.35
severe_signal_low_trust_review = true
```

Wire these through the formula loader if the `Formula` type does not already include `trust`.

Run:

```powershell
cd backend
cargo test services::quality::formula
```

Expected: formula v1 and v2 load.

- [ ] **Step 3: Apply reporter weighting in active signal review**

In `backend/src/services/trust/signal_reviews.rs`, make severe active signals from low-trust reporters go to review instead of auto-accept paths.

Acceptance:

- `security_issue` is always pending until admin review.
- `broken` and `doesnt_match_claim` from low-trust reporters become pending.
- Non-severe signals keep current behavior unless reporter weight is zero and consensus cannot be met.

Run:

```powershell
cd backend
cargo test services::trust::signal_reviews
```

Expected: tests pass.

- [ ] **Step 4: Apply owner dispute weighting**

Enhance owner dispute review context so owner trust influences queue priority and decision support, without auto-rejecting valid owners.

Acceptance:

- GitHub owner identity still matters first.
- Low-reputation owner dispute is accepted into the queue but not treated as high-confidence.
- High-reputation owner dispute is surfaced with stronger trust context.

Run:

```powershell
cd backend
cargo test services::trust
```

Expected: tests pass.

- [ ] **Step 5: Update trust docs and commit**

Update `docs/trust-model-v1.md` with:

- New-account weight behavior.
- Difference between reporter trust and owner trust.
- Why Sybil-resistant GitHub graph remains deferred.

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Commit:

```powershell
git add backend/scoring/formula_v2.toml backend/src/services/quality/weighting.rs backend/src/services/trust/reputation.rs backend/src/services/trust/signal_reviews.rs docs/trust-model-v1.md docs/plans/remaining-work-2026-05-03.md
git commit -m "Finish formula v2 trust weighting"
```

---

## Task 3: Discovery And Repo Detail Explanations

**Goal:** Help users understand why a repo appears, why it is excluded, and how its score changed.

**Files:**
- Modify: `backend/src/services/repos.rs`
- Modify: `backend/src/handlers/repos_query.rs`
- Modify: `frontend/src/lib/types.ts`
- Modify: `frontend/src/components/RepoCard.tsx`
- Modify: `frontend/src/routes/discover.tsx`
- Modify: `frontend/src/routes/repo-detail.tsx`
- Create: `frontend/src/features/repos/components/RepoRecommendationExplanation.tsx`
- Create: `frontend/src/features/repos/components/RepoScoreHistory.tsx`
- Create: `frontend/src/features/repos/components/RepoSignalTimeline.tsx`
- Test: `frontend/e2e/mvp.spec.ts`

- [ ] **Step 1: Add backend explanation fields**

Extend repo search/profile DTOs with fields like:

```ts
explanation: {
  includedBecause: string[];
  caveats: string[];
  excludedByAuto?: string[];
}
```

Backend source should derive these from existing fields:

- Quality dimensions.
- Flags.
- Radar maturity.
- Filter mode.
- Lexical/topic match.

Run:

```powershell
cd backend
cargo test services::repos
```

Expected: existing repo service tests pass, new explanation tests pass.

- [ ] **Step 2: Add score history endpoint data to repo profile**

Use existing DB history if already available through `artifact_scores` snapshots or score records.

Acceptance:

- Repo detail response includes chronological score points.
- Empty history returns an empty array.
- Dates use ISO strings.

Run:

```powershell
cd backend
cargo test handlers::repos_query services::repos
```

Expected: tests pass.

- [ ] **Step 3: Add frontend types and explanation component**

Update `frontend/src/lib/types.ts`.

Create `RepoRecommendationExplanation.tsx` with props:

```ts
type RepoRecommendationExplanationProps = {
  includedBecause: string[];
  caveats: string[];
  excludedByAuto?: string[];
};
```

Acceptance:

- Shows included reasons first.
- Shows caveats in quieter styling.
- Does not render empty sections.

Run:

```powershell
cd frontend
npm run build
```

Expected: TypeScript build passes.

- [ ] **Step 4: Add score history and signal timeline components**

Create:

- `RepoScoreHistory.tsx`: compact trend list or simple SVG-free chart using div bars.
- `RepoSignalTimeline.tsx`: chronological list of signal events with kind, status, and date.

Acceptance:

- Components handle empty states.
- No nested cards inside cards.
- Text fits mobile width.

Run:

```powershell
cd frontend
npm run build
```

Expected: build passes.

- [ ] **Step 5: Add E2E coverage**

Update `frontend/e2e/mvp.spec.ts` mocked API data to include explanation, history, and signal timeline fields.

Acceptance:

- Discovery shows at least one explanation reason.
- Repo detail shows score history section.
- Repo detail shows signal timeline or empty timeline state.

Run:

```powershell
cd frontend
npm run test:e2e
```

Expected: Playwright tests pass.

- [ ] **Step 6: Commit**

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

cd ../frontend
npm run build
npm run test:e2e
```

Commit:

```powershell
git add backend/src/services/repos.rs backend/src/handlers/repos_query.rs frontend/src/lib/types.ts frontend/src/components/RepoCard.tsx frontend/src/routes/discover.tsx frontend/src/routes/repo-detail.tsx frontend/src/features/repos/components/RepoRecommendationExplanation.tsx frontend/src/features/repos/components/RepoScoreHistory.tsx frontend/src/features/repos/components/RepoSignalTimeline.tsx frontend/e2e/mvp.spec.ts
git commit -m "Explain repo recommendations and score history"
```

---

## Task 4: Live Validation Gates

**Goal:** Add repeatable live validation for real OAuth and real MCP behavior without making fast CI fragile.

**Files:**
- Create: `docs/validation/live-release-checklist.md`
- Create: `scripts/mcp-live-smoke.ps1`
- Modify: `frontend/scripts/run-real-e2e.mjs`
- Modify: `docs/dev-workflow.md`

- [ ] **Step 1: Write the live release checklist**

Create `docs/validation/live-release-checklist.md`.

Required sections:

- Environment variables needed.
- OAuth flow: login, return-to behavior, logout.
- Discovery flow: search, repo detail, watch.
- Notifications flow: watchlist alert or seeded notification.
- MCP flow: `initialize`, `tools/list`, `search_github_repos`, `get_repo_quality_context`, `log_usage`.
- Data check: signal appears in `quality_signals`, score recompute updates `artifact_scores`.
- Rollback notes: frontend-only vs backend deploy.

- [ ] **Step 2: Add MCP smoke script**

Create `scripts/mcp-live-smoke.ps1`.

Required parameters:

```powershell
param(
  [Parameter(Mandatory=$true)][string]$Endpoint,
  [Parameter(Mandatory=$true)][string]$Token,
  [string]$Repo = "vitejs/vite"
)
```

Acceptance:

- Fails fast if token is missing.
- Calls MCP initialize with Bearer token.
- Calls `search_github_repos`.
- Calls `get_repo_quality_context`.
- Optionally calls `log_usage` only when `-WriteSignal` is passed, to avoid accidental production writes.

- [ ] **Step 3: Document local and live usage**

Update `docs/dev-workflow.md` with:

```powershell
.\scripts\mcp-live-smoke.ps1 -Endpoint "https://api.usestakly.com/mcp" -Token "usk_..."
```

Also document the write-signal mode separately and warn that it records a real signal.

- [ ] **Step 4: Commit**

Run:

```powershell
cd frontend
npm run test:e2e

cd ../cli
npm test
```

Commit:

```powershell
git add docs/validation/live-release-checklist.md scripts/mcp-live-smoke.ps1 docs/dev-workflow.md frontend/scripts/run-real-e2e.mjs
git commit -m "Add live validation gates"
```

---

## Task 5: Maintainability Refactors

**Goal:** Reduce risk in the largest files without changing behavior.

**Files:**
- Modify: `backend/src/mcp/server.rs`
- Create: `backend/src/mcp/tools/search.rs`
- Create: `backend/src/mcp/tools/recommend.rs`
- Create: `backend/src/mcp/tools/context.rs`
- Create: `backend/src/mcp/tools/write.rs`
- Modify: `backend/src/mcp/tools/mod.rs`
- Modify: `backend/src/services/repos.rs`
- Create: `backend/src/services/repos/search.rs`
- Create: `backend/src/services/repos/profile.rs`
- Modify: `frontend/src/routes/discover.tsx`
- Create: `frontend/src/features/repos/components/DiscoverFilters.tsx`
- Create: `frontend/src/features/repos/components/DiscoverResults.tsx`
- Create: `frontend/src/features/repos/components/UseCaseSearchPanel.tsx` if not already extracted there

- [ ] **Step 1: Capture baseline behavior**

Run:

```powershell
cd backend
cargo test mcp::server services::repos

cd ../frontend
npm run build
npm run test:e2e
```

Expected: all pass before refactor.

- [ ] **Step 2: Split MCP pure mappers first**

Move serialization helpers and output mapping functions from `backend/src/mcp/server.rs` into focused files under `backend/src/mcp/tools/`.

Acceptance:

- Tool names and schemas do not change.
- Existing MCP tests pass without snapshot rewrites.

Run:

```powershell
cd backend
cargo test mcp::server
```

Commit:

```powershell
git add backend/src/mcp/server.rs backend/src/mcp/tools
git commit -m "Refactor MCP tool mapping helpers"
```

- [ ] **Step 3: Split repo service search/profile helpers**

Move pure search normalization, sorting, and profile assembly helpers out of `backend/src/services/repos.rs`.

Acceptance:

- Public service API remains stable for handlers and MCP.
- Tests for `services::repos` still pass.

Run:

```powershell
cd backend
cargo test services::repos
cargo clippy --all-targets -- -D warnings
```

Commit:

```powershell
git add backend/src/services/repos.rs backend/src/services/repos
git commit -m "Refactor repo service search and profile helpers"
```

- [ ] **Step 4: Split Discover page UI**

Extract from `frontend/src/routes/discover.tsx`:

- Filters panel.
- Results list.
- Empty/loading/error states.
- Use-case search panel only if it is still embedded in the route.

Acceptance:

- Route behavior and URL query behavior remain unchanged.
- Mocked E2E tests still pass.
- No visible copy changes beyond necessary component boundaries.

Run:

```powershell
cd frontend
npm run build
npm run test:e2e
```

Commit:

```powershell
git add frontend/src/routes/discover.tsx frontend/src/features/repos/components
git commit -m "Refactor discovery route components"
```

- [ ] **Step 5: Final full verification**

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

cd ../frontend
npm run build
npm run test:e2e

cd ../cli
npm test
```

Expected: all pass.

---

## Completion Criteria

This plan is complete when:

- GitHub ingestion handles rate-limit signals and release metadata reliably.
- Formula v2 trust behavior prevents fresh accounts from steering severe active signals.
- Discovery and repo detail explain recommendation reasons, caveats, score history, and signal timeline.
- Live OAuth and MCP validation have documented, repeatable commands.
- The largest files are reduced through behavior-preserving extractions.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm run build`, `npm run test:e2e`, and `cli npm test` pass.

## Deferred Work

- Full Sybil-resistant graph from GitHub followers/contributions/account age.
- Nightly GitHub Actions workflow with Postgres for `test:e2e:real`.
- Large-scale search calibration on a broader corpus.
- Full public visual redesign pass for every marketing/documentation page.
