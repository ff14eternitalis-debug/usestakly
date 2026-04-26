# GEMINI.md

This file provides guidance to Gemini CLI when working with code in this repository. Reflète l'état au 2026-04-26.

> Ce fichier est miroir de `CLAUDE.md` et `AGENTS.md`. Tout ce qui est durable côté projet doit y rester aligné.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Pas de renommage spontané — voir `docs/plans/rename-to-usestakly.md`.
- Objectif : **veille GitHub OSS**. UseStakly score des repos GitHub publics pour aider devs et agents IA à choisir leurs dépendances autrement que par les stars.
- État : **public beta exposable** (TODO v5.5). Pas d'ouverture publique large tant que les ops MCP (backup DB, rate-limit globale, alerte externe) et le légal ne sont pas finis.
- Trois piliers : **discovery qualité-scored** + **watchlist / notifications** + **MCP pour agents** (5 tools, CLI npm `usestakly-mcp`).
- L'ancien produit **bibliothèque de snippets est abandonné** (pivot 2026-04-21). Schéma SQL legacy reste en base, ne pas réintroduire de surfaces produit snippets.

## Layout monorepo

- `backend/` — API Rust (Axum 0.8 + SQLx 0.8 + rmcp 1.5). Migrations dans `backend/migrations/` exécutées au boot. Binaire `seed_github`.
- `frontend/` — React 19 + Vite 7 + Tailwind v4 + TypeScript. **TanStack Router** + TanStack Query + Zustand. E2E Playwright.
- `cli/` — package npm `usestakly-mcp` publié (Node ≥18). Point d'installation MCP pour agents externes.
- `docs/` — commencer par `docs/README.md`, `TODO.md`, `docs/strategy-pivot-2026-04-21.md`, `docs/architecture-backend-current.md`, `docs/mcp-protocol.md`. Archives sous `docs/archive/`.
- `deploy/coolify/` — déploiement (`docs/deployment-coolify.md`, `docs/ops-mcp-coolify-hardening.md`).
- `scripts/seed-public-corpus.ps1` — seed corpus public.
- `docker-compose.yml` — uniquement Postgres local (`pgvector/pgvector:pg17`, DB `project_k`, `:5432`).

## Commandes

Avant tout : copier `.env.example` → `.env` et lancer la base.

```bash
docker compose up -d              # Postgres + pgvector
```

### Backend (`cd backend`)

- `cargo run` — démarre l'API sur `127.0.0.1:4000`.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo check`.
- Feature `semantic-search` (default OFF) pour `fastembed` + pgvector.

### Frontend (`cd frontend`)

- `npm install` puis `npm run dev` — Vite sur `:5173`.
- `npm run build` (CI), `npm run test:e2e` (Playwright avec mocks API).

### CLI MCP (`cd cli`)

- `npm test` — `node --test`.

## Architecture

### Backend

`main.rs` → `config::AppConfig::from_env()` → `db::connect()` → `app::build_app()` → `axum::serve`.

Couches :

- `app/` — Router + AppState, CORS strict, **middleware MCP qui rejette tout `/mcp` sans Bearer dès `initialize`/`tools/list`**.
- `auth/` — OAuth GitHub + Discord, session JWT cookie `usestakly_session`, `state` OAuth signé avec `return_to` sanitizé.
- `handlers/` — I/O HTTP : `health`, `auth`, `me`, `account`, `admin`, `agent_tokens`, `search`, `repos` (split en `repos_query`/`repos_ingestion`/`repo_signals`/`repo_viewer`), `watchlist`, `notifications`.
- `services/` — `ingestion/github`, `repos`, `watchlist`, `notifications`, `scheduler`, `semantic_search` (feature-gated), `agent_tokens`, sous-domaines `quality/` (`formula`, `compute`, `flags`, `weighting`, `pipeline`, `capture`) et `trust/` (`reputation`, `repo_owners`, `signal_reviews`, `signal_events`, `agent_token_events`, `mcp_metrics`).
- `domain/` — types métier actifs.
- `mcp/` — Streamable HTTP server, 5 tools MCP, auth Bearer `usk_<64 hex>` SHA-256.

Séparation stricte : `handler` (I/O) → `service` (logique) → `query` (DB).

### Frontend

- `frontend/src/main.tsx` → `AppProviders` → `AppShell`.
- Routes actives : `/`, `/discover`, `/repos/$id`, `/watchlist`, `/notifications`, `/account`, `/login`, `/status`, `/privacy`, `/how-to-read`, `/mcp-guide`.
- Garde auth sur routes privées avec `returnTo` signé.
- Features dans `src/features/{auth,layout,repos,account}/`. `repo-detail` et `account` éclatés en sous-composants.
- API métier dans `src/lib/api/{account,admin,repos,watchlist}.ts`.

### MCP tools

| Tool | Type |
|---|---|
| `search_github_repos` | read |
| `recommend_github_repos` | read |
| `get_repo_quality_context` | read |
| `log_usage` | write (retourne le score recalculé) |
| `watch_repo` | write |

Garde-fous write configurés via env (`APP_MCP_*`, `APP_ACTIVE_SIGNAL_*`).

## Conventions

### Scope produit

- Produit vivant : **discovery repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications in-app**, **MCP read + write + recommend**.
- Tables legacy snippets/libraries en base mais sans surface produit.
- `TODO.md` v5.5 = source de vérité d'exécution.

### Principes

- **Score qualité** + **flags toxiques** = cœur produit, pas les stars.
- Flags publics par **consensus × réputation**, `security_issue` modéré.
- MCP préserve la **provenance** (`source`, `formula_version`, `scored_at`).
- Write tools MCP **sous garde** (quota, cooldown, fenêtre, réputation, refus loggés).

## CI

`.github/workflows/ci.yml` :

- backend : `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` + `cargo test`.
- frontend : `npm install` + `npm run build` (Node 22) + Playwright + upload artifact.

Aucun Postgres provisionné en CI — tests DB-bound mockés ou feature-gated.

## Gotchas

- `sqlx::migrate!` au compile-time → toute nouvelle migration recompile le backend.
- `APP_SESSION_SECRET` requis pour les cookies session ; sans lui, fallback dev user.
- CORS strict sur `FRONTEND_BASE_URL` avec `allow_credentials(true)`.
- `docker-compose.yml` ne démarre que Postgres.
- **Scheduler** opt-in via `APP_SCHEDULER_ENABLED=true`, cadence `APP_RECOMPUTE_INTERVAL_SECS` (default 86400).
- **MCP `/mcp`** : Authorization Bearer obligatoire dès `initialize`/`tools/list` (middleware pré-transport, `docs/mcp-endpoint-security.md`). Tokens `usk_<64 hex>` SHA-256.
- **Rate-limit MCP** par token sur writes via `agent_token_events` (migration 0014). Pas encore de rate-limit globale multi-token/IP.
- **Modération** : migrations 0015/0016. Réputation v2 runtime livrée. Formula_v2 + Sybil OAuth à venir.
- **Scoring v1.1** : pondération `outcome × reporter × dedup` dans `services/quality/weighting.rs`. Endpoint admin `GET /api/admin/scoring/explain/{repo_id}`.
- **Semantic search** feature OFF par défaut, OFF en prod (`APP_SEMANTIC_SEARCH_ENABLED=false`).
- **Archived GitHub ≠ abandon** dans `formula_v2`.
- **`/api/status/public`** exposé sans auth.
- **Docker prod** : `fastembed` feature-gated, Rust 1.91, pgvector via `ensure_optional_extensions`.
