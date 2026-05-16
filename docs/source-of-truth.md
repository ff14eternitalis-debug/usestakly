# UseStakly Source Of Truth

> Last reconciled: 2026-05-16
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

`proof_tier` (`corpus_only` | `usage_limited` | `community_backed`) is a **label for agents and UI**, not a replacement for the scoring formula. Radar may mark large active OSS as `corpus_backed` / established before community proof exists; strict MCP filters still require usage where configured.

Background refresh: `POST /api/repos/{id}/refresh` re-ingests GitHub metadata and recomputes **one** artifact (`recompute_external_artifact`). Cooldown: `APP_REPO_REFRESH_COOLDOWN_SECS` (default 900). Stale structural signals: `APP_STRUCTURAL_STALE_SECS` (default 172800 = 48h).

## Legacy Boundary

The snippets product is abandoned.
Legacy tables may remain for migration compatibility, but they are not product guidance.

Never add or revive snippets/libraries UI, API, MCP tools, or roadmap items unless the user explicitly asks for that legacy product.

## Drift Audit

Run `.\scripts\audit-doc-source-truth.ps1` after documentation edits. A failure means an active doc may still contain a stale legacy reference.
