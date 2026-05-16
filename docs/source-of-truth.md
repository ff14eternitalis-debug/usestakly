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

## Legacy Boundary

The snippets product is abandoned.
Legacy tables may remain for migration compatibility, but they are not product guidance.

Never add or revive snippets/libraries UI, API, MCP tools, or roadmap items unless the user explicitly asks for that legacy product.

## Drift Audit

Run `.\scripts\audit-doc-source-truth.ps1` after documentation edits. A failure means an active doc may still contain a stale legacy reference.
