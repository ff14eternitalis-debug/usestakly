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

### Plan revision (2026-05-16, post-merge Tasks 1–2)

Tasks 1–2 merged on `main`. Tasks 3–5 were revised against the real codebase:

- Do **not** reuse the field name `explanation` on repo DTOs — `RepoRadarSnapshot.explanation` already exists (radar JSON).
- `artifact_scores` is **one row per `(repo, formula_version)`** (UPSERT) — not a score time series without a migration or an explicit MVP scope.
- `RepoProfile.recent_signals` + `RepoSignalsList` already cover much of the signal timeline — extend, do not duplicate blindly.
- Search responses only include **included** repos — `excludedByAuto` belongs on the **search response meta**, not per card, unless scope is narrowed.
- Reuse patterns from `services/recommendations.rs` (`reason`), admin explain logic (shared service, not a public clone of the admin endpoint).
- Task 4 should **extend** `docs/functional-checks.md`, not fork a second checklist.
- Task 5 **repo/MCP splits run after Task 3** (or split `repos.rs` before Task 3 backend if you prefer less merge pain).

**Branch for Task 3+:** `feat/repo-explanations` from updated `main`.

## Execution Order

1. GitHub ingestion reliability. ✅ merged
2. Scoring/trust formula v2 completion. ✅ merged
3. Discovery and repo-detail explanations.
4. Live validation gates (can run in parallel with Task 3 — docs/scripts only).
5. Maintainability refactors (**after Task 3**, especially `services/repos.rs`).

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

- [x] **Step 1: Add tests for new-account reporter weight**

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

- [x] **Step 2: Add formula v2 trust thresholds**

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

- [x] **Step 3: Apply reporter weighting in active signal review**

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

- [x] **Step 4: Apply owner dispute weighting**

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

- [x] **Step 5: Update trust docs and commit**

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

**Implementation note (merged):** Runtime paths are `services/trust/reputation.rs` (`active_signal_review_weight`, `requires_strict_active_review_with_trust`), `handlers/repo_signals.rs`, and `services/quality/formula.rs` (`TrustWeights` loader). Tests: `cargo test services::trust::reputation` (not `weighting::tests::new_account`). Owner dispute notes use `owner-confidence` in admin review context.

---

## Task 3: Discovery And Repo Detail Explanations

**Goal:** Help users understand why a repo appears in **this** search or profile context, with honest score-change UX — without duplicating radar explanations, use-case `reason` strings, or the admin-only scoring breakdown.

**Aligns with:** `docs/plans/remaining-work-2026-05-03.md` §3.3 (public UX; admin explain stays admin).

**Files:**
- Modify: `backend/src/domain/repo.rs` (new DTO types)
- Modify: `backend/src/services/repos.rs` (or `backend/src/services/repos/search.rs` + `profile.rs` if split early)
- Optional new: `backend/src/services/repos/explain.rs` (pure builders shared by search + profile)
- Modify: `backend/src/handlers/repos_query.rs`
- Modify: `frontend/src/lib/types.ts`
- Modify: `frontend/src/components/RepoCard.tsx`
- Modify: `frontend/src/routes/discover.tsx`
- Modify: `frontend/src/routes/repo-detail.tsx`
- Create: `frontend/src/features/repos/components/RepoRecommendationExplanation.tsx`
- Create: `frontend/src/features/repos/components/RepoScoreHistory.tsx` (MVP scope — see Step 2)
- Modify: `frontend/src/features/repos/components/RepoSignalsList.tsx` **or** thin `RepoSignalTimeline.tsx` wrapper (see Step 4)
- Test: `backend/src/services/repos.rs` (explanation builders)
- Test: `frontend/e2e/mvp.spec.ts`

### Naming and API shape (required)

- Use **`recommendationExplanation`** (camelCase in JSON), not `explanation`, to avoid clashing with `radar.explanation`.
- Shape:

```ts
recommendationExplanation: {
  includedBecause: string[];
  caveats: string[];
}
```

- **`excludedByAuto`** — only on **`RepoSearchResponse`** when `filter === "auto"`, not on each item:

```ts
// RepoSearchResponse (search only)
filterSummary?: {
  excludedCount?: number;
  excludedExamples?: string[]; // short human labels, cap at 3
};
```

Per-repo `excludedByAuto` on cards is **out of scope** unless you add a debug/admin mode later.

- **Use-case panel:** keep `recommendations[].reason` from `services/recommendations.rs`; optionally map the same builder for lexical search rows so copy feels consistent.

### Reuse (do not rebuild)

| Existing | Use for |
|----------|---------|
| `RepoMetricsPanel` + `ScoreBar` on repo detail | Dimension bars — do not duplicate on discover cards |
| `radar.explanation` + `radarSummary()` on `RepoCard` | Radar copy — link, do not merge into `recommendationExplanation` |
| `GET /api/admin/scoring/explain/{repo_id}` | Admin audit only — extract **public-safe** bullet builders in `services/`, not expose admin route |
| `RepoProfile.recent_signals` | Signal history source |

- [x] **Step 1: Backend explanation builders + search/profile fields**

Add `RecommendationExplanation` in `domain/repo.rs`. Implement pure builders (suggested module `services/repos/explain.rs`) from data already loaded in search/profile:

- Active filter mode (`auto` / `strict` / `explore`) and whether quality gates passed.
- Top 1–2 quality dimensions vs thresholds (e.g. reliability, abandonment).
- Public flags and radar `maturity_band` (reference radar separately in UI).
- Lexical/topic match when `q` or topics are present.

Attach `recommendationExplanation` to each `RepoSearchResult` and to `RepoProfile.repo`.

For `filter === "auto"`, add optional `filterSummary` on `RepoSearchResponse` (counts or short examples of repos filtered out — implement only if a cheap SQL-side or post-filter hook exists; otherwise document `excludedCount: null` and defer).

Run:

```powershell
cd backend
cargo test services::repos explain
```

Expected: new unit tests on builders; existing `services::repos` tests pass.

- [x] **Step 2: Score change UX on repo profile (pick one MVP)** — shipped **option A** (snapshot + optional v1.1 delta)

`artifact_scores` stores **one current row** per `(external_artifact_id, formula_version)` via UPSERT — there is **no** chronological history table today.

**Pick exactly one for this task:**

| Option | Work | User sees |
|--------|------|-----------|
| **A (recommended MVP)** | No migration | `scoreSnapshot`: current overall + dimensions + `computedAt` + optional delta vs previous formula version row if v1.1 row exists |
| **B** | Migration `artifact_score_snapshots` + write on recompute | `scoreHistory: { scoredAt, overall }[]` |
| **C** | Defer chart | Only `computedAt` + link to `/how-to-read` |

If **B**: add migration, append in `quality/pipeline.rs` on recompute, query last N points in profile.

Acceptance (all options):

- Repo profile JSON documents which option shipped.
- Empty history → `scoreHistory: []` or omitted per option.
- Dates are ISO 8601 strings.

Run:

```powershell
cd backend
cargo test handlers::repos_query services::repos
```

- [x] **Step 3: Frontend types + `RepoRecommendationExplanation`**

Update `frontend/src/lib/types.ts` with `recommendationExplanation` and search `filterSummary`.

Create `RepoRecommendationExplanation.tsx`:

```ts
type RepoRecommendationExplanationProps = {
  includedBecause: string[];
  caveats: string[];
};
```

Wire on `RepoCard` (discover) and repo detail header area. Do not render empty sections. Keep radar block unchanged.

Run:

```powershell
cd frontend
npm run build
```

- [x] **Step 4: Score history + signals UI** — `RepoScoreHistory` + extended `RepoSignalsList` events

- **`RepoScoreHistory.tsx`:** implement per Step 2 option (bars/list or snapshot-only).
- **Signals:** prefer extending `RepoSignalsList` (show more events, clearer timeline) over a parallel list. Add `RepoSignalTimeline.tsx` only if it stays a thin wrapper (under 80 lines).

Acceptance:

- Empty states for no history / no signals.
- No nested cards inside cards.
- Mobile-friendly text width.

Run:

```powershell
cd frontend
npm run build
```

- [x] **Step 5: E2E coverage**

Update `frontend/e2e/mvp.spec.ts` mocks with `recommendationExplanation`, score field(s) from Step 2, and existing `recentSignals` shape.

Acceptance:

- Discover: at least one `includedBecause` visible on a result card.
- Repo detail: score section matches chosen MVP; signals section visible or empty state.

Run:

```powershell
cd frontend
npm run test:e2e
```

- [ ] **Step 6: Commit** (à faire par l'utilisateur)

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

cd ../frontend
npm run build
npm run test:e2e
```

```powershell
git add backend/src/domain/repo.rs backend/src/services/repos.rs backend/src/handlers/repos_query.rs `
  frontend/src/lib/types.ts frontend/src/components/RepoCard.tsx `
  frontend/src/routes/discover.tsx frontend/src/routes/repo-detail.tsx `
  frontend/src/features/repos/components/RepoRecommendationExplanation.tsx `
  frontend/src/features/repos/components/RepoScoreHistory.tsx `
  frontend/e2e/mvp.spec.ts
# plus explain.rs, migration, RepoSignalsList — if touched
git commit -m "Explain repo recommendations and score snapshot on profile"
```

---

## Task 4: Live Validation Gates

**Goal:** Add repeatable **live/staging** validation for OAuth and MCP without making fast CI fragile. **Extend** the existing checklist — do not maintain two divergent lists.

**Base doc:** `docs/functional-checks.md` (sections A–J). This task adds automation + prod-oriented pointers.

**Files:**
- Modify: `docs/functional-checks.md` (new section **K. Live / staging** or annotate H/I for prod URLs)
- Create: `docs/validation/live-release-checklist.md` (**thin wrapper**: env vars, prod base URLs, link to functional-checks, rollback notes only)
- Create: `scripts/mcp-live-smoke.ps1`
- Modify: `frontend/scripts/run-real-e2e.mjs` (cross-link only, unless a flag is missing)
- Modify: `docs/dev-workflow.md`

- [ ] **Step 1: Extend functional checks + thin live wrapper**

In `docs/functional-checks.md`, add a short **Live / staging** section:

- Required env vars (`APP_SESSION_SECRET`, OAuth, `usk_` monitoring token, prod `FRONTEND_BASE_URL`).
- Pointer: OAuth B1–B4, Discover C*, MCP H1–H10, CLI I1–I3 already defined above — re-run against prod/staging URLs.
- Data check after deploy: signal in `quality_signals`, `artifact_scores.computed_at` moves on recompute.

Create `docs/validation/live-release-checklist.md` as a **one-page deploy gate** (not a duplicate of A–J):

- Base URLs (`https://api…`, `https://…/mcp`).
- Ordered go/no-go (health → status public → MCP smoke script → manual OAuth if needed).
- Rollback: frontend-only vs backend vs DB migration.

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
- Maps to functional-checks **H2, H4, H5** (and H7 only with `-WriteSignal`).

- [ ] **Step 3: Document local and live usage**

Update `docs/dev-workflow.md` with:

```powershell
.\scripts\mcp-live-smoke.ps1 -Endpoint "https://api.usestakly.com/mcp" -Token "usk_..."
```

Document `frontend/scripts/run-real-e2e.mjs` + `npm run test:e2e:real` (Postgres + `e2e/real-api.spec.ts`) as the **local** full-stack path; smoke script as the **remote MCP** path.

Warn: `-WriteSignal` records a real signal in the target environment.

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
git add docs/functional-checks.md docs/validation/live-release-checklist.md scripts/mcp-live-smoke.ps1 docs/dev-workflow.md frontend/scripts/run-real-e2e.mjs
git commit -m "Add live validation gates and MCP smoke script"
```

---

## Task 5: Maintainability Refactors

**Goal:** Reduce risk in the largest files without changing behavior.

**When:** **After Task 3 is merged** (avoids merge conflicts on `services/repos.rs` and `discover.tsx`). Exception: you may split `services/repos.rs` **before** Task 3 Step 1 if you want explanation code to land directly in `repos/search.rs` + `repos/profile.rs`.

**Size baseline (2026-05-16):** `backend/src/mcp/server.rs` ~1920 lines, `backend/src/services/repos.rs` ~750 lines, `frontend/src/routes/discover.tsx` ~690 lines. Target: ≤200 lines per new file (workspace rule).

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
- `frontend/src/features/repos/components/UseCaseSearchPanel.tsx` — **already extracted**; do not recreate

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

Move pure search normalization, sorting, profile assembly, and **explanation builders** (from Task 3) out of `backend/src/services/repos.rs` into `backend/src/services/repos/{mod,search,profile,explain}.rs`.

Acceptance:

- Public service API remains stable for handlers and MCP (`search_github_repos`, `get_repo_profile` re-exports unchanged).
- Tests for `services::repos` and explanation builders still pass.

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

- GitHub ingestion handles rate-limit signals and release metadata reliably (ETag **persistence** and 429 backoff remain optional follow-ups in `remaining-work` §2.4).
- Formula v2 trust behavior prevents fresh accounts from steering severe active signals (Sybil graph still deferred).
- Discovery and repo detail expose `recommendationExplanation`; repo profile exposes the **chosen** score MVP (snapshot and/or history); signals UX is improved without conflicting with `radar.explanation`.
- Live/staging validation is documented via `functional-checks` + `mcp-live-smoke.ps1` + dev-workflow cross-links.
- `mcp/server.rs`, `services/repos/*`, and `discover.tsx` are split with no tool/schema/URL behavior change.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm run build`, `npm run test:e2e`, and `cli npm test` pass.

## Deferred Work

- Full Sybil-resistant graph from GitHub followers/contributions/account age.
- Nightly GitHub Actions workflow with Postgres for `test:e2e:real`.
- Large-scale search calibration on a broader corpus.
- Full public visual redesign pass for every marketing/documentation page.
- GitHub ingestion: ETag DB persistence, exponential backoff on 429, quota monitoring (`remaining-work` §2.4).
- Per-repo `excludedByAuto` without search-level `filterSummary` (debug/admin only if ever needed).
