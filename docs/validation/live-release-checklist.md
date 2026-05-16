# UseStakly — Live / staging release gate

> Version : 1.0 — 2026-05-16  
> One-page go/no-go after a backend or full-stack deploy. Detailed steps live in [`docs/functional-checks.md`](../functional-checks.md).

## URLs (adjust per environment)

| Surface | Example (production) | Example (local) |
|---------|----------------------|-----------------|
| Website | `https://usestakly.com` | `http://localhost:5173` |
| API health | `https://api.usestakly.com/health` | `http://localhost:4000/health` |
| Public status | `https://api.usestakly.com/api/status/public` | `http://localhost:4000/api/status/public` |
| MCP | `https://api.usestakly.com/mcp` | `http://localhost:4000/mcp` |

## Secrets (never commit)

- `usk_…` agent token (monitoring or short-lived smoke token from `/account`)
- `APP_ADMIN_TOKEN` only if running admin checks (section J)
- OAuth: GitHub app credentials on the target environment for manual B2–B4

Prefer [Infisical](https://infisical.com/docs/documentation/getting-started/overview) or your team vault for distribution — not chat or git.

## Ordered gate (≈15 min)

1. **Health** — section **A** in `functional-checks.md` against the target API base URL.
2. **Public status** — **A2** shows `seedRepoCount > 0`, `formulaVersion = "v2.0"`.
3. **MCP smoke (automated)** — from repo root:
   ```powershell
   .\scripts\mcp-live-smoke.ps1 -Endpoint "https://api.usestakly.com/mcp" -Token $env:USESTAKLY_MCP_TOKEN
   ```
   Do **not** pass `-WriteSignal` on production unless you intend to persist a real `log_usage` row.
4. **OAuth** (if enabled on env) — **B2–B4** in the browser on the target frontend URL.
5. **Discover + repo detail** — **C1**, **D1–D3** on one known seeded repo.
6. **Watchlist / notifications** (optional) — **E1**, **F1** when auth is configured.
7. **Data spot-check** (optional, DB read-only):
   - After a controlled `log_usage` (staging only): new row in `quality_signals`.
   - After admin recompute: `artifact_scores.computed_at` updated for the repo.

Full MCP matrix: section **H** in `functional-checks.md`. CLI install: section **I**.

## Local full-stack gate (no production)

Uses Docker Postgres + backend + Playwright (no MCP script required):

```powershell
cd frontend
npm run test:e2e:real
```

See [`docs/dev-workflow.md`](../dev-workflow.md#e2e-réel-local-sans-mocks).

## Rollback

| Change type | Action |
|-------------|--------|
| Frontend only | Redeploy previous frontend image / static build; no DB migration revert. |
| Backend only | Redeploy previous backend image; run migrations only forward — do not delete migration files. |
| DB migration | Restore from latest Coolify backup if migration broke schema; see `docs/ops-mcp-coolify-hardening.md`. |

## Cursor / IDE MCP (optional)

This gate does **not** require adding UseStakly to Cursor. Use the smoke script or `npx usestakly-mcp test` for endpoint validation.

To let **Cursor agents** call UseStakly during development:

```powershell
npx usestakly-mcp install --client cursor --endpoint https://api.usestakly.com/mcp --token-env USESTAKLY_MCP_TOKEN
```

Then reload MCP servers in Cursor settings. See [`docs/mcp-guide`](/mcp-guide) on the site and `docs/mcp-cli-release.md`.
