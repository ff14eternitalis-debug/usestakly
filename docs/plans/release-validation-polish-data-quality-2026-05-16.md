# UseStakly Release Validation, Public Polish, And Data Quality Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the completed next-useful-work batch into a safer public-beta release by validating live flows, polishing the public UX, and finishing the most important GitHub ingestion reliability gaps.

**Architecture:** Keep the first two tasks mostly outside core business logic: validation uses scripts, docs, and targeted E2E/manual gates; UX polish stays in frontend route/components/i18n without API churn. Data quality changes stay in `services/ingestion/github.rs` plus a narrow migration, preserving the handler -> service -> query boundary.

**Tech Stack:** Rust 2024, Axum, SQLx/Postgres migrations, Octocrab/GitHub REST, React 19, Vite, TanStack Router/Query, Playwright, PowerShell MCP smoke script.

---

## Context

The previous plan `docs/plans/next-useful-work-2026-05-16.md` is complete and merged on `main`.

Known state after that plan:

- Backend verification passed: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Frontend verification passed: `npm run build`, `npm run test:e2e`.
- CLI verification passed: `npm test`.
- Residual Vite warning: main JS bundle is around `529 kB` after minification.
- `scripts/mcp-live-smoke.ps1` exists and covers MCP initialize, `search_github_repos`, `get_repo_quality_context`, and optional `log_usage`.
- Remaining product risks are live validation, public UX density/readability, and GitHub ingestion reliability (`ETag` persistence, 429/backoff/quota, `owner_inactive_days`).

## Execution Order

1. **Release validation gate**: prove live/staging auth + MCP + recompute flows before changing product behavior.
2. **Public beta UX polish**: make the exposed pages calmer, clearer, and lighter.
3. **GitHub data quality hardening**: persist request freshness metadata and add maintainer inactivity input for future notifications.

Do not combine Task 3 with frontend polish in the same commit. The ingestion work touches migrations and should be reviewed independently.

---

## Task 1: Release Validation Gate

**Goal:** Produce a repeatable go/no-go release gate for public beta using existing docs/scripts, then record the result in `docs/validation/`.

**Files:**

- Modify: `docs/functional-checks.md`
- Modify: `docs/validation/live-release-checklist.md`
- Create: `docs/validation/live-release-report-2026-05-16.md`
- Modify: `scripts/mcp-live-smoke.ps1` only if a live run exposes a real parsing/session bug
- Optional modify: `frontend/e2e/real-api.spec.ts` only if the local full-stack flow is flaky for a reproducible product reason

- [ ] **Step 1: Capture current validation baseline**

Run from repo root:

```powershell
git status --short --branch
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

Expected:

- Git branch is clean before validation changes.
- Backend checks pass.
- Frontend build passes. If Vite still warns about chunk size, record it as a follow-up, not a failure.
- E2E mocked suite passes.
- CLI tests pass.

- [ ] **Step 2: Run local full-stack E2E if the environment is available**

Prerequisites:

- Docker is running.
- `.env` exists from `.env.example`.
- Local backend can connect to Postgres.

Run:

```powershell
docker compose up -d
cd frontend
npm run test:e2e:real
```

Expected:

- `frontend/e2e/real-api.spec.ts` covers landing -> discover -> repo detail -> watchlist -> notifications -> account token -> MCP initialize/search.
- If unavailable because local secrets or Postgres are not ready, record `SKIPPED` with the exact blocker in `docs/validation/live-release-report-2026-05-16.md`.

- [ ] **Step 3: Run live/staging MCP smoke without writes**

Set variables in the shell used for validation:

```powershell
$env:USESTAKLY_MCP_ENDPOINT = "https://api.usestakly.com/mcp"
$env:USESTAKLY_MCP_TOKEN = "usk_<64 hex monitoring token>"
.\scripts\mcp-live-smoke.ps1 -Endpoint $env:USESTAKLY_MCP_ENDPOINT -Token $env:USESTAKLY_MCP_TOKEN -Repo "vitejs/vite"
```

Expected:

- `initialize` returns a valid MCP result and session id.
- `search_github_repos` returns at least one repo.
- `get_repo_quality_context` returns `provenance.formula_version`.
- Script exits with code `0`.

Do not run `-WriteSignal` on production unless explicitly doing the controlled write check in Step 5.

- [ ] **Step 4: Manually verify OAuth live flow**

Use the target frontend URL and follow `docs/functional-checks.md` sections B, C, D, E, G:

```text
landing -> /login -> GitHub OAuth -> return_to -> /discover -> repo detail -> watchlist -> notifications -> /account
```

Expected:

- OAuth sets `usestakly_session`.
- `GET /api/me` returns the OAuth identity, not the dev user.
- `returnTo` lands on the originally requested page.
- Watchlist action succeeds.
- Account page can show or create an MCP token.

- [ ] **Step 5: Run controlled MCP write and recompute check on staging or approved prod**

Only run this if the target environment can accept a real `log_usage` signal.

```powershell
.\scripts\mcp-live-smoke.ps1 `
  -Endpoint $env:USESTAKLY_MCP_ENDPOINT `
  -Token $env:USESTAKLY_MCP_TOKEN `
  -Repo "vitejs/vite" `
  -WriteSignal
```

Then verify via DB/admin tooling:

```sql
SELECT source, signal_type, outcome, created_at
FROM quality_signals
ORDER BY created_at DESC
LIMIT 5;

SELECT formula_version, computed_at
FROM artifact_scores
ORDER BY computed_at DESC
LIMIT 5;
```

Expected:

- New `quality_signals` row exists for the controlled repo.
- After recompute, `artifact_scores.computed_at` advances.
- If recompute is manual/admin-only, record the exact command or endpoint used.

- [ ] **Step 6: Write the validation report**

Create `docs/validation/live-release-report-2026-05-16.md` with this structure:

```markdown
# UseStakly Live Release Report - 2026-05-16

## Target

- Frontend:
- API:
- MCP:
- Commit:

## Automated Checks

| Check | Result | Notes |
| --- | --- | --- |
| backend fmt/clippy/test | PASS | |
| frontend build | PASS | Vite chunk warning if still present |
| frontend mocked E2E | PASS | |
| cli tests | PASS | |
| local real E2E | PASS/SKIPPED/FAIL | |
| MCP smoke read | PASS | |
| MCP smoke write | PASS/SKIPPED | |

## Manual Checks

| Flow | Result | Notes |
| --- | --- | --- |
| OAuth GitHub | PASS/SKIPPED/FAIL | |
| Discover -> repo detail | PASS | |
| Watchlist -> notifications | PASS | |
| Account token | PASS | |
| Public responsive smoke | PASS/SKIPPED/FAIL | |

## Decision

Go/no-go:

## Follow-ups

- 
```

Acceptance:

- Every `SKIPPED` includes a concrete reason.
- Every `FAIL` includes a file/issue pointer or proposed next fix.

- [ ] **Step 7: Commit validation gate**

Run:

```powershell
git status --short
git add docs/functional-checks.md docs/validation/live-release-checklist.md docs/validation/live-release-report-2026-05-16.md scripts/mcp-live-smoke.ps1 frontend/e2e/real-api.spec.ts
git commit -m "Record live release validation gate"
```

Only include `scripts/mcp-live-smoke.ps1` or `frontend/e2e/real-api.spec.ts` if they changed.

---

## Task 2: Public Beta UX Polish

**Goal:** Make public pages easier to trust and scan before a wider beta announcement, while reducing the current frontend bundle warning if the fix is low-risk.

**Files:**

- Modify: `frontend/src/app/router.tsx`
- Modify: `frontend/src/routes/index.tsx`
- Modify: `frontend/src/routes/discover.tsx`
- Modify: `frontend/src/routes/repo-detail.tsx`
- Modify: `frontend/src/routes/how-to-read.tsx`
- Modify: `frontend/src/routes/mcp-guide.tsx`
- Modify: `frontend/src/components/RepoCard.tsx`
- Modify: `frontend/src/features/repos/components/DiscoverResults.tsx`
- Modify: `frontend/src/features/repos/components/RepoRecommendationExplanation.tsx`
- Modify: `frontend/src/features/repos/components/RepoScoreHistory.tsx`
- Modify: `frontend/src/i18n/en.ts`
- Modify: `frontend/src/i18n/fr.ts`
- Test: `frontend/e2e/mvp.spec.ts`

- [ ] **Step 1: Add a public smoke E2E path**

Extend `frontend/e2e/mvp.spec.ts` with a smoke that navigates:

```text
/ -> /how-to-read -> /discover -> first repo detail -> /mcp-guide -> /privacy -> /legal -> /status
```

Test shape:

```ts
test("public beta pages are reachable", async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") consoleErrors.push(msg.text());
  });

  await page.goto("/");
  await expect(page.getByRole("link", { name: /how to read|lire/i })).toBeVisible();

  await page.goto("/how-to-read");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  await page.goto("/discover");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  await page.goto("/mcp-guide");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  await page.goto("/privacy");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  await page.goto("/legal");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  await page.goto("/status");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  expect(consoleErrors).toEqual([]);
});
```

Adjust accessible names to match the current EN/FR copy rather than forcing English-only labels.

Run:

```powershell
cd frontend
npm run test:e2e
```

Expected before polish: the test may pass already. Keep it as a regression guard.

- [ ] **Step 2: Calm dense public copy and hierarchy**

Review these pages in desktop and mobile:

- `/`
- `/discover`
- `/repos/$id`
- `/how-to-read`
- `/mcp-guide`

Acceptance:

- No section has multiple competing primary CTAs in the same viewport.
- Explanatory blocks are shorter than the repo/data panels they support.
- `/how-to-read` explicitly distinguishes quality score from `maturity_band`.
- `/mcp-guide` reflects the current CLI and endpoint guidance from `docs/mcp-cli-release.md` and `docs/mcp-endpoint-security.md`.
- Discovery keeps the product first: score, radar, recommendation explanation, provenance.

Run:

```powershell
cd frontend
npm run build
```

- [ ] **Step 3: Improve recommendation/exclusion display without API changes**

Use existing fields:

- `repo.recommendationExplanation`
- `searchResponse.filterSummary`
- `repo.radar.explanation`
- `repo.scoreSnapshot`

Acceptance:

- `RepoRecommendationExplanation` does not render empty headings.
- Discover cards show at most 2 included reasons and 1 caveat unless expanded copy already exists.
- `DiscoverResults` shows `filterSummary` as a short filter note, not a warning banner.
- Repo detail shows recommendation explanation near the repo identity, and score snapshot near score/provenance.

Run:

```powershell
cd frontend
npm run build
npm run test:e2e
```

- [ ] **Step 4: Address the Vite chunk warning with route-level lazy loading**

If the bundle warning still appears, convert heavy route imports in `frontend/src/app/router.tsx` to lazy-loaded route components.

Target pattern:

```ts
const DiscoverPage = lazyRouteComponent(() =>
  import("../routes/discover").then((module) => ({ default: module.DiscoverPage }))
);
```

Use the TanStack Router-supported lazy route pattern already compatible with the installed version. If `lazyRouteComponent` is not exported by the current package, use `React.lazy` plus a local route wrapper.

Acceptance:

- Initial main chunk drops below Vite's 500 kB warning or the remaining warning is documented with the exact largest module.
- Route preloading still works well enough for normal navigation.
- No blank page during route load; use an existing app-level loading style if needed.

Run:

```powershell
cd frontend
npm run build
npm run test:e2e
```

- [ ] **Step 5: Browser verification on desktop and mobile**

Start the dev server:

```powershell
cd frontend
npm run dev
```

Verify:

- Desktop viewport: `/`, `/discover`, one `/repos/$id`, `/how-to-read`, `/mcp-guide`.
- Mobile viewport: same pages.
- No overlapping text.
- No console errors.
- Buttons/links fit their containers.

If using automated browser tooling, capture screenshots for at least `/discover` and `/repos/$id`.

- [ ] **Step 6: Commit public UX polish**

Run:

```powershell
cd frontend
npm run build
npm run test:e2e

git status --short
git add frontend/src/app/router.tsx frontend/src/routes frontend/src/components frontend/src/features/repos/components frontend/src/i18n frontend/e2e/mvp.spec.ts
git commit -m "Polish public beta pages and route loading"
```

---

## Task 3: GitHub Data Quality Hardening

**Goal:** Finish the high-value ingestion reliability gaps: persist ETags, apply retry/backoff on rate limits, expose quota context in logs/admin docs, and capture `owner_inactive_days` for future maintainer-silence notifications.

**Files:**

- Create: `backend/migrations/0020_github_ingestion_reliability.sql`
- Modify: `backend/src/domain/repo.rs`
- Modify: `backend/src/services/ingestion/github.rs`
- Modify: `backend/src/services/repos/rows.rs`
- Modify: `backend/src/services/repos/profile.rs`
- Modify: `backend/src/services/quality/formula.rs` only if formula v2 consumes `owner_inactive_days` immediately
- Modify: `docs/plans/remaining-work-2026-05-03.md`
- Modify: `docs/architecture-backend-current.md`
- Test: `backend/src/services/ingestion/github.rs`

- [ ] **Step 1: Add DB columns for ingestion freshness metadata**

Create `backend/migrations/0020_github_ingestion_reliability.sql`:

```sql
-- Persist GitHub conditional-request and maintainer-inactivity metadata.
-- Existing external_artifacts.etag remains the repo metadata ETag.

ALTER TABLE external_artifacts
  ADD COLUMN github_releases_etag TEXT,
  ADD COLUMN github_readme_etag TEXT,
  ADD COLUMN github_events_etag TEXT,
  ADD COLUMN github_rate_limit_reset_at TIMESTAMPTZ,
  ADD COLUMN github_last_rate_limit_at TIMESTAMPTZ,
  ADD COLUMN owner_last_activity_at TIMESTAMPTZ,
  ADD COLUMN owner_inactive_days INT;

CREATE INDEX idx_external_artifacts_owner_inactive_days
  ON external_artifacts(owner_inactive_days DESC NULLS LAST)
  WHERE owner_inactive_days IS NOT NULL;
```

Acceptance:

- No existing column is repurposed ambiguously.
- Nullable columns preserve current ingestion behavior for existing rows.

- [ ] **Step 2: Add pure tests for ETag metadata and 304 behavior**

In `backend/src/services/ingestion/github.rs`, add pure helper tests before wiring live requests:

```rust
#[test]
fn structural_signals_preserve_existing_values_on_not_modified() {
    let existing = StructuralSignals {
        releases_count: Some(4),
        last_release_at: Some(parse_dt("2026-05-01T00:00:00Z")),
        ..StructuralSignals::default()
    };
    let refreshed = existing.clone().merge_not_modified();
    assert_eq!(refreshed.releases_count, existing.releases_count);
    assert_eq!(refreshed.last_release_at, existing.last_release_at);
}

#[test]
fn backoff_delay_uses_retry_after_before_default_secondary_delay() {
    let headers = github_rate_limit_headers("12", "1778791697", Some("7"));
    let limit = classify_rate_limit(StatusCode::TOO_MANY_REQUESTS, &headers, "");
    assert_eq!(retry_delay(&limit).map(|d| d.as_secs()), Some(7));
}
```

If helper names differ, keep the behavior exactly:

- 304 means reuse existing DB values.
- `Retry-After` wins when available.
- Secondary rate limit gets a bounded default delay.

Run:

```powershell
cd backend
cargo test services::ingestion::github::tests::etag
cargo test services::ingestion::github::tests::backoff
```

Expected before implementation: compile failure for new helpers.

- [ ] **Step 3: Thread existing DB metadata into ingestion**

Add a small DB-loaded context for an existing GitHub artifact:

```rust
#[derive(Debug, Clone, Default)]
struct ExistingGitHubIngestionState {
    repo_etag: Option<String>,
    releases_etag: Option<String>,
    readme_etag: Option<String>,
    events_etag: Option<String>,
    releases_count: Option<i32>,
    last_release_at: Option<DateTime<Utc>>,
    owner_last_activity_at: Option<DateTime<Utc>>,
    owner_inactive_days: Option<i32>,
}
```

Acceptance:

- `ingest_repo` fetches this state by `(source='github', canonical_slug)` before network calls when a row exists.
- Missing rows use `ExistingGitHubIngestionState::default()`.
- ETags are sent with `If-None-Match` for releases/readme/events where supported.
- 304 responses preserve existing values instead of clearing columns.

Run:

```powershell
cd backend
cargo test services::ingestion::github
```

- [ ] **Step 4: Implement bounded backoff for rate-limit responses**

Add a small retry wrapper around GitHub calls that can hit secondary limits:

- releases
- README
- events
- commits if the current Octocrab flow can be wrapped without invasive changes

Acceptance:

- Maximum one retry per endpoint per ingestion run.
- Use `Retry-After` when provided.
- Use a short bounded default for secondary rate limits, for example 2 seconds in tests/configurable path, not an unbounded sleep.
- Primary rate limit with reset far in the future does not block a request thread for minutes; it records reset context and returns a typed error.
- Logs include owner/repo and endpoint, not tokens.

Run:

```powershell
cd backend
cargo test services::ingestion::github::tests::backoff
cargo clippy --all-targets -- -D warnings
```

- [ ] **Step 5: Capture maintainer inactivity from GitHub events**

Add an events fetcher:

```text
GET /repos/{owner}/{repo}/events?per_page=100
```

MVP rule:

- Consider maintainer activity as events where `actor.login` matches `github_owner`, or where event type is one of `PushEvent`, `ReleaseEvent`, `PullRequestEvent`, `IssuesEvent` and actor is not an obvious bot.
- `owner_last_activity_at` is the newest matching event `created_at`.
- `owner_inactive_days = floor(now - owner_last_activity_at)`.
- Empty or failed events fetch leaves both fields `NULL` and does not mark the repo abandoned.

Add DTO:

```rust
#[derive(Debug, Deserialize)]
struct GitHubRepoEvent {
    #[serde(rename = "type")]
    event_type: String,
    actor: GitHubEventActor,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct GitHubEventActor {
    login: String,
}
```

Run:

```powershell
cd backend
cargo test services::ingestion::github::tests::owner_activity
```

Acceptance:

- Bot actors ending with `[bot]` do not count.
- Owner activity computes `0` days for same-day activity.
- No events returns `(None, None)`.

- [ ] **Step 6: Persist new fields and expose them read-only**

Update `upsert_github_artifact` to write:

- `github_releases_etag`
- `github_readme_etag`
- `github_events_etag`
- `github_rate_limit_reset_at`
- `github_last_rate_limit_at`
- `owner_last_activity_at`
- `owner_inactive_days`

Update repo read models only where useful:

- `backend/src/domain/repo.rs`: add optional `owner_inactive_days` to `RepoSignals` or `RepoProfile` metadata, whichever matches existing structure.
- `backend/src/services/repos/rows.rs`: map the column.
- `backend/src/services/repos/profile.rs`: include the field for repo detail.

Acceptance:

- Frontend does not need to render the field in this task unless the existing signal panel can show it without new UX work.
- API remains backward-compatible because the fields are optional.

Run:

```powershell
cd backend
cargo test services::repos services::ingestion::github
cargo clippy --all-targets -- -D warnings
```

- [ ] **Step 7: Update docs and remaining-work state**

Update:

- `docs/plans/remaining-work-2026-05-03.md`
- `docs/architecture-backend-current.md`

Required notes:

- ETag persistence is now stored per endpoint where implemented.
- Backoff is bounded and does not sleep until primary reset.
- `owner_inactive_days` exists as a formula/notification input, but the "maintainer silent 90j" notification rule remains a separate task unless implemented here.
- Quota monitoring is either done via logs/admin metrics or remains explicitly open.

- [ ] **Step 8: Full backend verification and commit**

Run:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Commit:

```powershell
git status --short
git add backend/migrations/0020_github_ingestion_reliability.sql backend/src/services/ingestion/github.rs backend/src/domain/repo.rs backend/src/services/repos docs/plans/remaining-work-2026-05-03.md docs/architecture-backend-current.md
git commit -m "Persist GitHub ingestion freshness metadata"
```

---

## Completion Criteria

This plan is complete when:

- A live/staging validation report exists and clearly says go/no-go.
- MCP read smoke has been run against the target environment.
- OAuth live flow has been verified or explicitly skipped with a concrete blocker.
- Public pages have an E2E reachability smoke and no console errors in the tested path.
- `/how-to-read` explains quality score vs `maturity_band`; `/mcp-guide` matches current MCP install guidance.
- Vite bundle warning is fixed or documented with an exact follow-up.
- GitHub ingestion persists endpoint ETags where implemented, handles 304 without data loss, applies bounded backoff, and stores `owner_inactive_days`.
- Backend, frontend, and CLI verification commands pass for the tasks they touch.

## Deferred Work

- Nightly/manual GitHub Actions workflow for `npm run test:e2e:real` with Postgres.
- Full quota dashboard for GitHub API consumption if logs are not enough.
- Notification rule: "maintainer silent 90j" using `owner_inactive_days`.
- Sybil-resistant GitHub graph.
- Larger search calibration on a broader corpus.
- Email provider for notification digests if Discord/webhook is not enough.
