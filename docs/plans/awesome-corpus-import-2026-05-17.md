# Awesome Corpus Import Plan

> **For agentic workers:** this is an execution plan for importing a curated GitHub corpus from `sindresorhus/awesome` and related Awesome lists. Keep ingestion bounded, idempotent, and reviewable before writing to production.

**Goal:** Add up to 500 high-signal OSS repositories to UseStakly from Awesome-list sources without flooding the corpus with lists, articles, books, services, or duplicates.

**Source:** [`sindresorhus/awesome`](https://github.com/sindresorhus/awesome) as a root index of Awesome lists, then selected list READMEs at depth 1.

**Key constraint:** cap accepted repos at **500** for the first import. Every step must support dry-run output before ingestion.

**Execution gate:** do not run a production import until Task 4 duplicate short-circuit is implemented and verified. The current `alreadyIndexed` flag alone is not enough if the handler still calls GitHub for already-known repos.

---

## Product Position

This import is for corpus expansion, not a new product surface.

UseStakly should ingest **actual GitHub OSS repos** that developers may depend on, inspect, watch, or ask agents about. Do not ingest the entire Awesome graph recursively, and do not treat every link as a package candidate.

The root `sindresorhus/awesome` README mostly links to curated Awesome lists. Those lists are useful as discovery sources, but the target corpus should be the repos listed inside selected category READMEs.

---

## Current Runtime Facts

- Public/manual add route: `POST /api/repos/add` in `backend/src/handlers/repos_ingestion.rs`.
- Existing ingestion is idempotent at DB level: `ingest_repo` upserts `external_artifacts` with `ON CONFLICT (source, canonical_slug) DO UPDATE`.
- `POST /api/repos/add` already returns `alreadyIndexed` by checking `find_github_artifact_id` before ingestion.
- Existing seed script: `scripts/seed-public-corpus.ps1` posts a static repo list to `/api/repos/add`.
- Existing DB seed binary: `backend/src/bin/seed_github.rs` ingests directly from `backend/seeds/top_repos.toml`; the API path is preferred for this import because it returns `alreadyIndexed`, uses the public add flow, and recomputes the single artifact through `recompute_external_artifact`.
- `GITHUB_TOKEN` is required for ingestion.
- After ingestion, code calls `recompute_external_artifact` for the single artifact.

Risk to close in this plan: ensure user-facing add flow does not create duplicates or misleading "new repo" feedback when the repo already exists.

---

## Scope

### In Scope

- Build an Awesome corpus candidate extractor.
- Limit import to 500 accepted repos.
- Deduplicate by normalized `owner/repo`.
- Filter obvious non-targets.
- Produce dry-run artifacts for review.
- Ingest only approved candidates through existing UseStakly ingestion.
- Add/verify tests that users cannot add duplicate repos as new entries.
- Document the import process and result.

### Out Of Scope

- Deep recursive crawling beyond selected depth 1 Awesome lists.
- Importing packages from npm/crates/PyPI directly.
- Adding a new public UI for corpus import.
- Treating Awesome lists themselves as quality-scored dependency candidates by default.
- Replacing the scheduler or scoring formula.

---

## Task 1 — Candidate Extraction Design

**Goal:** Define exactly what gets collected before writing any script.

**Files to touch:**
- Create: `docs/corpus/awesome-import.md`
- Later script: `scripts/collect-awesome-corpus.mjs`
- Create: `docs/corpus/awesome-lists-allowlist.json`

- [ ] Define source depth:
  - root: `sindresorhus/awesome`
  - depth 1: selected Awesome-list READMEs linked from the root allowlist
  - no depth 2 recursion in the first import
- [ ] Create an allowlist of roughly 15-25 Awesome-list repos before crawling depth 1. Suggested categories:
  - frontend / UI
  - testing
  - Node.js / TypeScript
  - Python
  - Rust
  - Go
  - databases / ORM
  - auth / security
  - observability
  - build/dev tooling
  - machine learning/data tooling
- [ ] Document why each allowlisted Awesome list is included.
- [ ] Define default cap:
  - `maxAcceptedRepos = 500`
  - stop after ranking/filtering, not after raw extraction
- [ ] Define ranking before cap:
  - first dedupe all candidates
  - score candidates by source category relevance and direct repo-link confidence
  - apply round-robin or per-source quotas so one large list cannot fill all 500 slots
  - then take the first 500
- [ ] Define normalized repo key:
  - lowercase `owner/repo`
  - strip `https://github.com/`
  - strip `.git`
  - ignore fragments, query strings, issues, pulls, releases, wiki, discussions, actions, sponsors
- [ ] Define source metadata to keep in dry-run:
  - `owner`
  - `repo`
  - `url`
  - `sourceList`
  - `sourceCategory`
  - `sourceLine`
  - `reason`

**Acceptance criteria:**
- A developer can tell what "up to 500 repos" means before the importer runs.
- The import is bounded and reviewable.
- The exact depth-1 Awesome lists are versioned and reproducible.
- The first 500 repos are chosen after ranking, not by arbitrary README extraction order.

---

## Task 2 — Build Dry-Run Collector

**Goal:** Extract candidates without touching UseStakly ingestion.

**Preferred implementation:** Node script with built-in `fetch`, because README parsing and JSON/CSV output are straightforward and do not require new backend code.

**Create:** `scripts/collect-awesome-corpus.mjs`

**CLI shape:**

```powershell
node scripts/collect-awesome-corpus.mjs `
  --root sindresorhus/awesome `
  --allowlist docs/corpus/awesome-lists-allowlist.json `
  --max 500 `
  --out docs/corpus/awesome-candidates.json `
  --summary docs/corpus/awesome-candidates-summary.md
```

**Collector behavior:**

- [ ] Fetch root README from GitHub raw content.
- [ ] Extract GitHub repo links.
- [ ] Detect likely Awesome-list repos:
  - repo name starts with `awesome`
  - or README line/list text includes `awesome`
  - or source is the root `sindresorhus/awesome`
- [ ] Fetch only depth 1 README files listed in `docs/corpus/awesome-lists-allowlist.json`.
- [ ] Extract direct `github.com/owner/repo` links from those READMEs.
- [ ] Normalize and deduplicate candidate repos.
- [ ] Keep source provenance for every candidate.
- [ ] Emit JSON and markdown summary.
- [ ] Never call `/api/repos/add` in this script.
- [ ] Rate-limit raw README fetches and cache fetched README content locally during one run to avoid repeated GitHub calls while tuning filters.

**Acceptance criteria:**
- Running the collector produces a candidate file and summary only.
- The summary shows raw link count, duplicate count, rejected count, and final candidate count.

---

## Task 3 — Filtering Rules

**Goal:** Avoid importing noisy links that are not useful repo candidates.

**Filtering rules:**

- [ ] Reject non-GitHub links.
- [ ] Reject GitHub URLs that are not repository roots and cannot be safely normalized:
  - `/issues`
  - `/pull`
  - `/pulls`
  - `/releases`
  - `/actions`
  - `/wiki`
  - `/discussions`
  - `/sponsors`
  - `/commit`
- [ ] Normalize valid repo subpaths to the repository root instead of dropping them:
  - `github.com/owner/repo/tree/...` -> `owner/repo`
  - `github.com/owner/repo/blob/...` -> `owner/repo`
- [ ] Reject obvious non-repo GitHub hosts or pseudo paths.
- [ ] Reject root Awesome lists from the final target set by default:
  - keep them as `sourceList`
  - do not ingest them as product candidates unless explicitly allowlisted
- [ ] Reject archived repos only after ingestion metadata is known, not during README parse. The dry-run can mark them as "unknown archival state".
- [ ] Prefer repos with likely developer-tool categories:
  - frontend/ui
  - testing
  - database/orm
  - auth/security
  - backend/web/api
  - observability
  - build/dev tooling
  - data/ML tooling
- [ ] Apply source/category balancing before taking the final 500:
  - per-source cap for very large lists
  - round-robin across source lists or categories
  - stable deterministic ordering for reproducible dry-runs

**Acceptance criteria:**
- The first 500 candidates are varied and useful for UseStakly discovery.
- The output does not mainly contain Awesome-list repos, books, websites, or docs.

---

## Task 4 — Existing Corpus Dedup Check

**Goal:** Avoid ingesting repos already present in UseStakly, avoid unnecessary GitHub calls, and prove user add is idempotent.

**Backend facts to verify:**
- `find_github_artifact_id` checks existing `external_artifacts` by exact `github_owner` + `github_repo`.
- `ingest_repo` upserts by canonical slug.
- `POST /api/repos/add` returns `alreadyIndexed`.

**Files likely touched:**
- `backend/src/handlers/repos_ingestion.rs`
- `backend/src/services/repos/mod.rs`
- `backend/src/services/ingestion/github.rs`
- tests near existing ingestion/handler tests, if available

- [ ] Confirm owner/repo comparison is case-insensitive or normalize before lookup.
- [ ] If not case-insensitive, update `find_github_artifact_id` or caller normalization so `FFmpeg/FFmpeg` and `ffmpeg/ffmpeg` resolve to the same existing artifact.
- [ ] Short-circuit `POST /api/repos/add` when the repo already exists:
  - do **not** call `ingest_repo` for already-indexed repos by default
  - return the existing profile/add response with `alreadyIndexed: true`
  - keep an explicit future refresh path separate from add, instead of refreshing on every duplicate add
- [ ] Add a test for duplicate add behavior:
  - existing repo in DB
  - user/API adds same repo with different casing or GitHub URL format
  - response has `alreadyIndexed: true`
  - no duplicate `external_artifacts` row
  - no GitHub ingestion call is made for the duplicate path, or the logic is structured so this can be proven by a unit/service test
- [ ] Confirm frontend add flow communicates "already indexed" instead of implying a fresh import or refreshed metadata.
- [ ] If current UI copy says "Refreshed metadata" for `alreadyIndexed`, update it to a neutral existing-corpus message before shipping the short-circuit.

**Acceptance criteria:**
- A user cannot create duplicate UseStakly repos for the same GitHub repo.
- Different casing and URL forms are handled.
- Re-adding an existing repo does not spend GitHub quota.

---

## Task 5 — Ingestion Script

**Goal:** Ingest approved candidates safely after dry-run review.

**Create:** `scripts/import-awesome-corpus.ps1`

**CLI shape:**

```powershell
.\scripts\import-awesome-corpus.ps1 `
  -Api "https://mcp.usestakly.com" `
  -Input "docs/corpus/awesome-candidates-approved.json" `
  -Limit 500 `
  -DelayMs 750 `
  -DryRun
```

Then, after review:

```powershell
.\scripts\import-awesome-corpus.ps1 `
  -Api "https://mcp.usestakly.com" `
  -Input "docs/corpus/awesome-candidates-approved.json" `
  -Limit 500 `
  -DelayMs 750
```

**Script behavior:**

- [ ] Read approved candidate JSON.
- [ ] Deduplicate again before sending.
- [ ] Respect `-Limit`.
- [ ] Support `-DryRun`.
- [ ] POST each repo to `/api/repos/add`.
- [ ] Log result per repo:
  - `added`
  - `alreadyIndexed`
  - `failed`
- [ ] Add delay between requests to reduce GitHub pressure.
- [ ] Stop or pause on repeated GitHub rate-limit failures.
- [ ] Emit final summary JSON/MD.
- [ ] Prefer staging/local first. Production import should run only after Task 4 short-circuit is deployed.
- [ ] If a future admin-gated import endpoint exists, prefer it over the public add endpoint for production.

**Acceptance criteria:**
- Import can be reviewed before execution.
- Import can resume safely because `/api/repos/add` is idempotent.
- Existing repos are counted separately from newly ingested repos.
- Duplicate candidates do not trigger GitHub API calls after Task 4 lands.

---

## Task 6 — First Import Runbook

**Goal:** Make the first 500-repo test safe and auditable.

**Create/update:** `docs/corpus/awesome-import.md`

- [ ] Run collector dry-run.
- [ ] Review candidate summary manually.
- [ ] Produce approved JSON with max 500 repos.
- [ ] Run import in staging/local first if possible.
- [ ] Run production import during a quiet window.
- [ ] Monitor backend logs for GitHub rate-limit warnings.
- [ ] Keep the first import at or below 500 repos and consider smaller batches (for example 50-100) if quota warnings appear.
- [ ] Let scheduler/recompute update scores and radar; wait at least 1-2 scheduler cycles after import, or run an admin recompute if intentionally doing a controlled release operation.
- [ ] Watch `APP_INGEST_MAX_REPOS_PER_CYCLE` and scheduler logs after import; 500 new artifacts may take multiple cycles to fully refresh depending on prod limits.
- [ ] Spot-check `/discover` and repo profiles.
- [ ] Record:
  - date
  - source commit/README URL
  - number of candidates
  - number added
  - number already indexed
  - number failed
  - rate-limit events
  - scheduler/recompute follow-up status

**Acceptance criteria:**
- The import result is reproducible.
- There is an audit trail of why these repos entered the corpus.
- The import does not mask GitHub quota pressure or leave operators guessing whether the first scoring/radar pass completed.

---

## Task 7 — Validation

**Goal:** Prove the corpus import and duplicate protection work.

- [ ] Backend tests:
  ```powershell
  cd backend
  cargo test
  cargo clippy --all-targets -- -D warnings
  ```
- [ ] Frontend build:
  ```powershell
  cd frontend
  npm run build
  ```
- [ ] Script dry-run:
  ```powershell
  node scripts/collect-awesome-corpus.mjs --root sindresorhus/awesome --allowlist docs/corpus/awesome-lists-allowlist.json --max 500 --out docs/corpus/awesome-candidates.json --summary docs/corpus/awesome-candidates-summary.md
```
- [ ] Import dry-run:
  ```powershell
  .\scripts\import-awesome-corpus.ps1 -Input docs/corpus/awesome-candidates-approved.json -Limit 500 -DryRun
  ```
- [ ] Duplicate test:
  - import one existing repo twice with different casing/URL format
  - confirm second response reports `alreadyIndexed: true`
  - verify the second add emits no GitHub request

**Acceptance criteria:**
- Tests pass.
- Dry-run produces a bounded, deduped candidate set.
- Duplicate add behavior is proven.

---

## Task 8 — Documentation Sync

**Goal:** Keep docs honest after import tooling lands.

**Files to update:**
- `docs/source-of-truth.md`
- `docs/dev-workflow.md`
- `docs/plans/remaining-work-2026-05-03.md`
- `docs/corpus/awesome-import.md`

- [ ] Document Awesome import as a corpus source, not a new product surface.
- [ ] Document the 500-repo cap.
- [ ] Document dry-run-first workflow.
- [ ] Document duplicate behavior for `POST /api/repos/add`.
- [ ] Run:
  ```powershell
  .\scripts\audit-doc-source-truth.ps1
  git diff --check
  ```

**Acceptance criteria:**
- Future agents know how to repeat or extend the import without flooding the corpus.
- No doc implies users can create duplicate repos.
