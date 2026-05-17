# UseStakly Source Of Truth

> Last reconciled: 2026-05-17
> Scope: current runtime and documentation routing for agents.

## Current Product

UseStakly is a GitHub OSS discovery, scoring, watchlist, notification, and MCP product.
It is not a snippets library.

Active pillars:

- quality-scored GitHub repository discovery
- repo profiles with score provenance and signal history
- watchlist, notifications, notification channels, and use-case watches
- MCP tools for agent discovery, context, passive usage, and watches

## Canonical Docs

Read in this order:

1. `AGENTS.md` (and mirror: `CLAUDE.md`, `GEMINI.md`)
2. `README.md`
3. `docs/architecture-backend-current.md`
4. `docs/mcp-protocol.md`
5. `docs/trust-model-v1.md`
6. `docs/plans/remaining-work-2026-05-03.md`

## Validation (live / staging)

- `docs/validation/live-release-checklist.md` — ordered go/no-go after deploy
- `scripts/mcp-live-smoke.ps1` — remote MCP smoke (no Cursor MCP required)

## Completed plans (historical only)

- `docs/plans/next-useful-work-2026-05-16.md` — merged on `main`; do not use for open work

## Runtime Truth Beats Docs

If docs disagree with code, verify code first:

- backend routes: `backend/src/app/mod.rs`
- MCP tool handlers: `backend/src/mcp/server.rs` (`#[tool_router]`, `build_service`)
- MCP DTOs/mappers: `backend/src/mcp/tools/*`
- config/env: `backend/src/config/mod.rs` and `.env.example`
- scoring formula: `backend/scoring/formula_v2.toml`
- migrations: `backend/migrations/`
- frontend routes: `frontend/src/app/router.tsx`
- CLI version: `cli/package.json`

## Corpus vs community proof (display layer)

Two truths coexist on every repo profile:

- **GitHub corpus (observable)** — freshness, vitality, CI, releases, contributor cadence. Ingested into `external_artifacts` and surfaced via `dimension_states` with `source: github_metadata` and `ingestion_status`.
- **UseStakly community (decisional)** — adoption and reliability from passive MCP `log_usage` and weighted signals. Surfaced via `dimension_states` with `source: usage_signals` or `neutral_default` until `min_sample` builds in `formula_v2.toml`.

`quality.overall` remains the formula v2 score. The display layer adds five explicit dimension states (`freshness`, `adoption`, `reliability`, `abandonment`, `vitality`) and `proof_tier` (`corpus_only` | `usage_limited` | `community_backed`) as a **label for agents and UI**, not a replacement score.

Where to read this truth:

- REST profile: `GET /api/repos/{id}` returns `dimensionStates`, `proofTier`, `ingestionStatus`, plus compatibility fields `quality` and `vitalityInputs`.
- MCP context: `get_repo_quality_context` returns `proof_tier`, `dimension_states`, `ingestion_status`, and normal provenance.
- UI profile: repo detail renders `RepoMetricsPanel`, `DimensionScoreRow`, and `StructuralRefreshBanner` from `dimensionStates` + `proofTier`.
- Discovery/search cards still expose the classic score bars only. `GET /api/repos/search`, `/api/search`, and MCP `search_github_repos` do **not** expose `dimensionStates`.

Radar may mark large active OSS as `corpus_backed` / established before community proof exists; strict MCP filters and recommendations still keep their existing proof/community gates where configured.

Background refresh: `POST /api/repos/{id}/refresh` requires an authenticated session, re-ingests GitHub metadata, and recomputes **one** artifact (`recompute_external_artifact`). Limits: DB-backed `repo_refresh_events` (`APP_REPO_REFRESH_USER_LIMIT_PER_HOUR` default 10 completed/hour/user, `APP_REPO_REFRESH_COOLDOWN_SECS` default 900 per repo window) plus a best-effort in-memory cooldown. Throttled calls return the cached profile (`refreshed: false`). Repo profile reads stay public. UI auto-refresh on repo detail runs only when signed in. Stale structural signals: `APP_STRUCTURAL_STALE_SECS` (default 172800 = 48h).

Known non-fields: `ingestionStatus` does not currently include `lastIngestError`.

## Legacy Boundary

The snippets product is abandoned.
Legacy tables may remain for migration compatibility, but they are not product guidance.

Never add or revive snippets/libraries UI, API, MCP tools, or roadmap items unless the user explicitly asks for that legacy product.

## Drift Audit

Run `.\scripts\audit-doc-source-truth.ps1` after documentation edits. A failure means an active doc may still contain a stale legacy reference.
