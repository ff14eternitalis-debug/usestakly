# UseStakly

![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)
![Axum](https://img.shields.io/badge/Axum-0.8-111111)
![SQLx](https://img.shields.io/badge/SQLx-0.8-336791)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-17-4169E1?logo=postgresql&logoColor=white)
![pgvector](https://img.shields.io/badge/pgvector-enabled-4169E1)
![React](https://img.shields.io/badge/React-19-61DAFB?logo=react&logoColor=111111)
![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?logo=typescript&logoColor=white)
![Vite](https://img.shields.io/badge/Vite-7-646CFF?logo=vite&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4-38B2AC?logo=tailwindcss&logoColor=white)
![TanStack Router](https://img.shields.io/badge/TanStack_Router-active-FF4154)
![MCP](https://img.shields.io/badge/MCP-Streamable_HTTP-10B981)
![Coolify](https://img.shields.io/badge/Coolify-deployed-0B0D0E)
![Source Available: BSL 1.1](https://img.shields.io/badge/Source_Available-BSL_1.1-blue.svg)

> Public beta observatory for GitHub OSS repositories: quality-scored discovery, watchlists, in-app notifications, and MCP tools for coding agents.

UseStakly helps developers and coding agents choose open-source GitHub repositories with a transparent score instead of relying on stars alone.

`UseStakly` is the only active product name.

## Product Scope

UseStakly focuses on **public GitHub OSS repositories**:

- **Quality-scored discovery** for public GitHub repos
- **Watchlist and in-app notifications** for dependency drift
- **MCP tools** for coding agents
- **Moderated quality signals** such as `deprecated`, `broken`, and `security_issue`

## Current Public Beta

Already shipped:

- GitHub and Discord OAuth login
- GitHub repo ingestion via `/api/repos/add`
- repo discovery and repo detail pages
- score dimensions: freshness, adoption, reliability, abandonment, and vitality
- score provenance plus `dimensionStates` / `proofTier` on repo detail and MCP context
- watchlist and in-app notifications
- notification channels and daily digest plumbing; one real outbound product email is still a launch-hardening proof item
- account page for MCP token creation and revocation
- public beta pages: landing, status, privacy/data, reading guide, MCP guide
- public status endpoint: `/api/status/public`
- optional local semantic search with `fastembed` and `pgvector` behind the `semantic-search` feature
- Awesome-list corpus import tooling and the first bounded production import
- MCP Streamable HTTP endpoint at `/mcp`
- npm installer for agents:

```bash
npx usestakly-mcp install
```

Available MCP tools:

- `recommend_github_repos`
- `search_github_repos`
- `get_repo_quality_context`
- `log_usage`
- `watch_repo`
- `watch_use_case`

## Data Quality

UseStakly scores are transparent indicators, not official certifications.

- GitHub metadata is real and fetched from GitHub.
- Freshness and abandonment are currently the strongest dimensions.
- Adoption and reliability improve as users and MCP agents report real outcomes through `log_usage`.
- Every score carries formula provenance, currently `v2.0`.
- Repo profile display separates GitHub corpus evidence from UseStakly community proof with dimension states and proof tiers.

Use the score to guide technical review, not to replace it.

## Stack

Backend:

- Rust 2024
- Axum 0.8
- SQLx 0.8
- PostgreSQL 17 + pgvector
- `rmcp` Streamable HTTP server
- OAuth handled server-side with a `usestakly_session` cookie

Frontend:

- React 19
- TypeScript
- Vite 7
- Tailwind CSS v4
- TanStack Router
- TanStack Query
- Zustand

Deployment:

- Coolify
- backend Dockerfile in `backend/`
- frontend Dockerfile in `frontend/`
- PostgreSQL/pgvector managed as a Coolify database resource

## Local Quickstart

Requirements:

- Docker
- Rust stable
- Node 22+
- npm

```bash
cp .env.example .env
docker compose up -d

cd backend
cargo run

cd ../frontend
npm install
npm run dev
```

Backend runs on `http://127.0.0.1:4000`.

Frontend runs on `http://localhost:5173`.

If `APP_SESSION_SECRET` and OAuth client secrets are missing, OAuth is disabled and the backend falls back to the `DEV_USER_*` development user.

## Useful Commands

| Context | Command |
|---|---|
| Backend quick check | `cd backend && cargo check` |
| Backend format | `cd backend && cargo fmt --check` |
| Backend lint | `cd backend && cargo clippy --all-targets -- -D warnings` |
| Backend tests | `cd backend && cargo test` |
| Frontend build | `cd frontend && npm run build` |
| CLI tests | `cd cli && npm test` |
| Public corpus seed | `./scripts/seed-public-corpus.ps1` |
| Awesome corpus collect | `node scripts/collect-awesome-corpus.mjs --max 500` |
| Awesome corpus import | `./scripts/import-awesome-corpus.ps1 -DryRun` |

## Important Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `APP_BASE_URL`: public backend URL
- `FRONTEND_BASE_URL`: public frontend URL
- `APP_SESSION_SECRET`: required for OAuth sessions
- `GITHUB_TOKEN`: required for GitHub ingestion and owner checks
- `ADMIN_API_TOKEN`: required for admin endpoints
- `APP_NOTIFICATION_SECRET`: encrypts notification channel destinations
- `APP_REPO_REFRESH_USER_LIMIT_PER_HOUR`: per-user repo refresh guard
- `APP_MCP_WRITE_LIMIT_PER_HOUR`: MCP write quota per token
- `APP_MCP_LOG_USAGE_COOLDOWN_SECS`: cooldown for repeated `log_usage`
- `APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS`: negative signal cooling window
- `APP_ACTIVE_SIGNAL_MIN_REPUTATION`: trust threshold for active signals

## Security Posture

Already implemented:

- MCP tokens use `usk_<64 hex>` and are hashed server-side
- tokens can be revoked from the account page
- MCP writes are quota-limited per token
- repo profile refresh requires a session and is DB rate-limited per user/repo
- duplicate and repeated negative `log_usage` events are cooled down
- public flags require reputation and consensus
- `security_issue` flows through stricter review
- repo owners can dispute signals
- MCP host validation is configured for the public backend host
- database is private on Coolify, not publicly exposed
- Authorization Bearer is required for all `/mcp` requests, including `initialize` and `tools/list` (HTTP entrypoint middleware, see [docs/mcp-endpoint-security.md](./docs/mcp-endpoint-security.md))

Known operations gaps before a wider launch:

- prove one real outbound product email notification or digest (channel test alone is not enough)
- configure offsite/S3 database backups when budget allows (local Coolify backups and restore test are in place)
- run the live release checklist again after future deployments that change runtime behavior (`docs/validation/live-release-checklist.md`, `scripts/mcp-live-smoke.ps1`)

See [docs/ops-mcp-coolify-hardening.md](./docs/ops-mcp-coolify-hardening.md).

## Documentation

Start here:

- [docs/source-of-truth.md](./docs/source-of-truth.md)
- [docs/README.md](./docs/README.md)
- [docs/plans/remaining-work-2026-05-03.md](./docs/plans/remaining-work-2026-05-03.md)
- [TODO.md](./TODO.md) (historical roadmap)
- [docs/mcp-protocol.md](./docs/mcp-protocol.md)
- [docs/mcp-examples.md](./docs/mcp-examples.md)
- [docs/mcp-cli-release.md](./docs/mcp-cli-release.md)
- [docs/ops-mcp-coolify-hardening.md](./docs/ops-mcp-coolify-hardening.md)
- [docs/deployment-coolify.md](./docs/deployment-coolify.md)
- [docs/architecture-backend-current.md](./docs/architecture-backend-current.md)
- [docs/security-audit-2026-04-21.md](./docs/security-audit-2026-04-21.md)

## License

UseStakly is **source-available** under the **Business Source License 1.1** (BSL 1.1).

The repository is public so developers can inspect how the product works, audit the scoring logic, and give feedback. The main platform is not OSI open source today: production use for competing scoring, registry, discovery, white-label, or user-redirection services is restricted until the Change Date.

On **May 1st, 2030**, this version of UseStakly automatically converts to the **Apache License, Version 2.0**.

The MCP CLI in `cli/` (`usestakly-mcp`) is distributed separately under the **MIT License** to make agent installation easy.

For full terms and conditions, please see the [LICENSE](./LICENSE) file.
