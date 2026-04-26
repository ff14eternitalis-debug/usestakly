# AGENTS.md

This file provides guidance to Codex (and other CLI agents) when working with code in this repository. Reflète l'état au 2026-04-26.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Les deux cohabitent encore dans certains chemins (DB `project_k`, repo). Pas de renommage spontané — voir `docs/plans/rename-to-usestakly.md`.
- Objectif : **veille GitHub OSS**. UseStakly score des repos GitHub publics pour aider devs et agents IA à choisir leurs dépendances autrement que par les stars.
- État : **public beta exposable** (TODO v5.5). Pas d'ouverture publique large tant que les ops MCP (backup DB, rate-limit globale, alerte externe) et le légal ne sont pas finis.
- Trois piliers actifs : **discovery qualité-scored** + **watchlist / notifications** + **MCP pour agents** (5 tools, CLI npm `usestakly-mcp`).
- L'ancien produit **bibliothèque de snippets est abandonné**. Schéma SQL historique (libraries/snippets/...) reste en base, mais ne pas réintroduire de surfaces produit snippets sans demande explicite.

## Layout monorepo

- `backend/` — API Rust (Axum 0.8 + SQLx 0.8 + rmcp 1.5). Migrations dans `backend/migrations/` exécutées au boot par `sqlx::migrate!`. Binaire `seed_github` pour bootstrap corpus.
- `frontend/` — React 19 + Vite 7 + Tailwind v4 + TypeScript. Routing **TanStack Router**. E2E Playwright (`frontend/e2e/mvp.spec.ts`).
- `cli/` — package npm `usestakly-mcp` publié, point d'installation MCP pour agents externes.
- `docs/` — commencer par `docs/README.md`, puis `TODO.md`, `docs/strategy-pivot-2026-04-21.md`, `docs/architecture-backend-current.md`, `docs/mcp-protocol.md`. Archives sous `docs/archive/`.
- `deploy/coolify/` — cible de déploiement (voir `docs/deployment-coolify.md`, `docs/ops-mcp-coolify-hardening.md`).
- `scripts/seed-public-corpus.ps1` — seed corpus public via API.
- `docker-compose.yml` — uniquement Postgres local (`pgvector/pgvector:pg17`, DB `project_k`, `:5432`).

## Commandes

Avant tout : copier `.env.example` → `.env` (lu par `dotenvy`) et lancer la base.

```bash
docker compose up -d              # Postgres + pgvector
```

### Backend (`cd backend`)

- `cargo run` — démarre l'API sur `127.0.0.1:4000`, applique les migrations.
- `cargo fmt --check` — CI.
- `cargo clippy --all-targets -- -D warnings` — CI (zéro warning).
- `cargo test` — CI.
- `cargo check` — itération rapide.
- Feature `semantic-search` (default OFF) : active `fastembed`. Build prod sans la feature pour Docker léger.

### Frontend (`cd frontend`)

- `npm install` puis `npm run dev` — Vite sur `:5173`.
- `npm run build` — `tsc -b && vite build` (CI).
- `npm run test:e2e` — Playwright avec mocks API.
- Pas de script lint séparé.

### CLI MCP (`cd cli`)

- `npm test` — couvre validation token, écriture configs, endpoint configurable.

## Architecture backend

`main.rs` → `config::AppConfig::from_env()` → `db::connect()` → `app::build_app()` → `axum::serve`.

- `app/` — `Router` + `AppState`, CORS strict, `TraceLayer`, **middleware MCP qui rejette tout `/mcp` sans Bearer dès `initialize`/`tools/list`**.
- `config/` — env (DB, OAuth, session, GitHub PAT, admin, scheduler, MCP guards, signaux actifs, semantic search).
- `auth/` — OAuth GitHub + Discord, session JWT cookie `usestakly_session`, `state` OAuth signé avec `return_to` sanitizé. Fallback dev user via `DEV_USER_*`. **Pas de Supabase Auth.**
- `handlers/` — `health` (+ `/api/status/public`), `auth`, `me`, `account`, `admin`, `agent_tokens`, `search`, `repos` (split `repos_query`/`repos_ingestion`/`repo_signals`/`repo_viewer`), `watchlist`, `notifications`.
- `services/` — `ingestion/github`, `repos`, `watchlist`, `notifications`, `scheduler`, `semantic_search` (feature-gated), `agent_tokens`, sous-domaines `quality/` (`formula`, `compute`, `flags`, `weighting`, `pipeline`, `capture`) et `trust/` (`reputation`, `repo_owners`, `signal_reviews`, `signal_events`, `agent_token_events`, `mcp_metrics`).
- `domain/` — `account`, `agent_token`, `quality`, `repo`, `reference`, `watchlist`.
- `db/` — pool, migrations, `ensure_optional_extensions` (pgvector optionnel).
- `mcp/` — Streamable HTTP server. 5 tools : `search_github_repos`, `recommend_github_repos`, `get_repo_quality_context`, `log_usage`, `watch_repo`.

Séparation : `handler` (I/O) → `service` (logique) → `query` (DB). À respecter pour tout nouveau code.

## Architecture frontend

- `frontend/src/main.tsx` → `AppProviders` → `AppShell`.
- **TanStack Router** (`frontend/src/app/router.tsx`).
- Routes actives : `/`, `/discover`, `/repos/$id`, `/watchlist`, `/notifications`, `/account`, `/login`, `/status`, `/privacy`, `/how-to-read`, `/mcp-guide`.
- Garde auth : `/watchlist`, `/notifications`, `/account` redirigent vers `/login` avec `returnTo` signé.
- Features : `src/features/{auth,layout,repos,account}/`. `repo-detail` et `account` éclatés en sous-composants (sprint refacto 3).
- État : Zustand (`auth-store`, `locale-store`). i18n EN/FR.
- Réseau : `src/lib/api-client.ts` + `src/lib/api/{account,admin,repos,watchlist}.ts` (`credentials: "include"`, base `VITE_API_BASE_URL`).
- React Query câblé dans `frontend/src/app/providers.tsx`.

## CLI MCP (`cli/`)

Package npm `usestakly-mcp` (v0.1.3 au 2026-04-26).

- `npx usestakly-mcp install` — configure Codex / Cursor avec Bearer + endpoint.
- `npx usestakly-mcp test` — vérifie connectivité et auth.
- Endpoint configurable, plus hardcodé.
- Doc release : `docs/mcp-cli-release.md`.

## Conventions

### Scope produit

- Produit vivant : **discovery repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications in-app**, **MCP read + write + recommend**.
- Tables legacy `libraries` / `snippets` peuvent exister en base. Cela **n'autorise pas** à relancer ce produit.
- Toucher zone legacy snippets = privilégier suppression de surfaces mortes, conservation prudente des migrations, protection du produit GitHub actif.

### Principes produit à respecter

- **Score qualité** + **flags toxiques** (`deprecated`, `broken`, `security_issue`) = cœur produit. Pas les stars seules.
- Flags toxiques publics : **consensus N users distincts × réputation seuil**. `security_issue` reste `pending` jusqu'à review admin. Owners GitHub peuvent disputer.
- Toute évolution MCP doit préserver la **provenance** (`source: "usestakly://..."`, `formula_version`, `scored_at`).
- Write tools MCP toujours **sous garde** (quota, cooldown, fenêtre négatifs, réputation min, refus loggés).
- Ne pas casser : discovery, repo-detail, watchlist, notifications, auth, agent tokens, install MCP via CLI.

## CI

`.github/workflows/ci.yml` :

- backend : `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- frontend : `npm install`, `npm run build` (Node 22), Playwright Chromium + `npm run test:e2e`, upload `playwright-report/`.

Aucun service Postgres provisionné — tests DB-bound mockés ou feature-gated.

## Gotchas

- `sqlx::migrate!` au compile-time → toute nouvelle migration exige une recompilation backend.
- Cookies session exigent `APP_SESSION_SECRET`. Sans lui, `auth_enabled()` = `false`, seul le dev user fonctionne.
- CORS strict sur `FRONTEND_BASE_URL` avec `allow_credentials(true)` — changer l'URL casse l'auth.
- `docker-compose.yml` ne démarre **que** Postgres.
- **Scheduler opt-in** : `APP_SCHEDULER_ENABLED=true`, cadence `APP_RECOMPUTE_INTERVAL_SECS` (default 86400). Pas de run au boot. OFF en dev par défaut.
- **MCP `/mcp`** monté via `rmcp::StreamableHttpService`. **Authorization Bearer requise dès `initialize`/`tools/list`** (middleware pré-transport, doc `docs/mcp-endpoint-security.md`). Tokens `usk_<64 hex>` SHA-256 dans `agent_tokens` (migration 0013).
- **Rate-limit MCP par token** sur writes via `agent_token_events` (migration 0014). Pas encore de rate-limit globale multi-token / par IP — item ops #2 dans `docs/ops-mcp-coolify-hardening.md`.
- **Modération** : migrations 0015/0016 (review + events). Garde-fous v1 en place. Réputation v2 runtime livrée. Formula_v2 + Sybil OAuth à venir.
- **Scoring v1.1** (2026-04-24) : pondération `outcome × reporter × dedup` dans `services/quality/weighting.rs`. Endpoint admin `GET /api/admin/scoring/explain/{repo_id}`.
- **Semantic search** (R2b) feature `semantic-search` OFF par défaut. Migration 0017 `repo_embeddings`, fastembed + pgvector. OFF en prod (`APP_SEMANTIC_SEARCH_ENABLED=false`).
- **Archived GitHub ≠ abandon** : pas câbler `archived=true` comme trigger unique d'abandon dans `formula_v2`.
- **Public status** : `GET /api/status/public` exposé sans auth pour la page `/status`.
- **Docker prod** : `fastembed` derrière feature `semantic-search`, Rust builder 1.91, pgvector via `ensure_optional_extensions`.
