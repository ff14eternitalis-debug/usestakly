# UseStakly

![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)
![Axum](https://img.shields.io/badge/Axum-0.8-111111)
![SQLx](https://img.shields.io/badge/SQLx-0.8-336791)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-4169E1?logo=postgresql&logoColor=white)
![pgvector](https://img.shields.io/badge/pgvector-enabled-4169E1)
![React](https://img.shields.io/badge/React-19-61DAFB?logo=react&logoColor=111111)
![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?logo=typescript&logoColor=white)
![Vite](https://img.shields.io/badge/Vite-7-646CFF?logo=vite&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4-38B2AC?logo=tailwindcss&logoColor=white)
![TanStack Router](https://img.shields.io/badge/TanStack_Router-active-FF4154)
![MCP](https://img.shields.io/badge/MCP-Streamable_HTTP-10B981)
![Coolify](https://img.shields.io/badge/Coolify-deployed-0B0D0E)

> Public beta observatory for GitHub OSS repositories: quality-scored discovery, watchlists, notifications, and MCP tools for coding agents.

UseStakly helps developers and coding agents choose open-source GitHub repositories with a transparent score instead of relying on stars alone.

`UseStakly` is the current product name. `Project-K` is the historical working name and may still appear in paths, migrations, and archived documents.

## Product Scope

UseStakly is no longer a snippets library. The active product is focused on **public GitHub OSS repositories**:

- **Quality-scored discovery** for public GitHub repos
- **Watchlist and notifications** for dependency drift
- **MCP tools** for coding agents
- **Moderated quality signals** such as `deprecated`, `broken`, and `security_issue`

The old snippets product is out of runtime scope. Legacy SQL tables may remain for migration compatibility and technical history, but they are not product surfaces.

## Current Public Beta

Already shipped:

- GitHub and Discord OAuth login
- GitHub repo ingestion via `/api/repos/add`
- repo discovery and repo detail pages
- score dimensions: freshness, adoption, reliability, abandonment
- score provenance on repo detail and MCP responses
- watchlist and in-app notifications
- account page for MCP token creation and revocation
- public beta pages: landing, status, privacy/data, reading guide, MCP guide
- public status endpoint: `/api/status/public`
- local semantic search with `fastembed` and `pgvector`
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

## Data Quality

UseStakly scores are transparent indicators, not official certifications.

- GitHub metadata is real and fetched from GitHub.
- Freshness and abandonment are currently the strongest dimensions.
- Adoption and reliability improve as users and MCP agents report real outcomes through `log_usage`.
- Every score carries formula provenance, currently `v1.1`.

Use the score to guide technical review, not to replace it.

## Stack

Backend:

- Rust 2024
- Axum 0.8
- SQLx 0.8
- PostgreSQL 16 + pgvector
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

## Important Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `APP_BASE_URL`: public backend URL
- `FRONTEND_BASE_URL`: public frontend URL
- `APP_SESSION_SECRET`: required for OAuth sessions
- `GITHUB_TOKEN`: required for GitHub ingestion and owner checks
- `ADMIN_API_TOKEN`: required for admin endpoints
- `APP_MCP_WRITE_LIMIT_PER_HOUR`: MCP write quota per token
- `APP_MCP_LOG_USAGE_COOLDOWN_SECS`: cooldown for repeated `log_usage`
- `APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS`: negative signal cooling window
- `APP_ACTIVE_SIGNAL_MIN_REPUTATION`: trust threshold for active signals

## Security Posture

Already implemented:

- MCP tokens use `usk_<64 hex>` and are hashed server-side
- tokens can be revoked from the account page
- MCP writes are quota-limited per token
- duplicate and repeated negative `log_usage` events are cooled down
- public flags require reputation and consensus
- `security_issue` flows through stricter review
- repo owners can dispute signals
- MCP host validation is configured for the public backend host
- database is private on Coolify, not publicly exposed

Known operations gaps before a wider launch:

- configure scheduled Coolify database backups
- add application-level rate limiting for all `/mcp` calls, including reads and `initialize`
- require Authorization for all `/mcp` requests, including `initialize` and `tools/list`
- add external uptime alerting for `/health`, `/api/status/public`, and a controlled MCP check

See [docs/ops-mcp-coolify-hardening.md](./docs/ops-mcp-coolify-hardening.md).

## Documentation

Start here:

- [docs/README.md](./docs/README.md)
- [TODO.md](./TODO.md)
- [docs/mcp-protocol.md](./docs/mcp-protocol.md)
- [docs/mcp-examples.md](./docs/mcp-examples.md)
- [docs/mcp-cli-release.md](./docs/mcp-cli-release.md)
- [docs/ops-mcp-coolify-hardening.md](./docs/ops-mcp-coolify-hardening.md)
- [docs/deployment-coolify.md](./docs/deployment-coolify.md)
- [docs/architecture-backend-current.md](./docs/architecture-backend-current.md)
- [docs/security-audit-2026-04-21.md](./docs/security-audit-2026-04-21.md)

Archived snippets-era docs live under `docs/archive/snippets/` and are not the source of truth for the current product.
