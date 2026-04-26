# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Les deux cohabitent encore dans certains chemins et docs (DB `project_k`, repo). Ne pas faire de renommage spontané — voir `docs/plans/rename-to-usestakly.md`.
- Objectif produit : **veille GitHub OSS**. UseStakly score des repos GitHub publics pour aider des devs et des agents IA à choisir de meilleures dépendances qu'avec les seules stars.
- État au 2026-04-26 : **public beta exposable** (TODO v5.5). Pas encore d'ouverture publique large — l'intention explicite est de déployer pour auditer en conditions réelles, pas pour chasser des users.
- Trois piliers actifs : **discovery qualité-scored** + **watchlist / notifications** + **MCP pour agents IA** (5 tools, CLI npm `usestakly-mcp` publié).
- Le pivot du 2026-04-21 a abandonné l'ancien produit de **bibliothèque de snippets**. Le schéma SQL historique (`libraries`, `snippets`, `snippet_versions`…) reste présent en base, mais **ne pas réintroduire** de surfaces produit snippets sans demande explicite.

## Layout monorepo

- `backend/` — API Rust (Axum 0.8 + SQLx 0.8 + rmcp 1.5). Migrations Postgres dans `backend/migrations/` exécutées automatiquement au boot par `sqlx::migrate!` dans `backend/src/db/mod.rs`. Binaire `seed_github` pour bootstrap corpus.
- `frontend/` — React 19 + Vite 7 + Tailwind v4 + TypeScript. Routing **TanStack Router** (`frontend/src/app/router.tsx`). E2E Playwright (`frontend/e2e/mvp.spec.ts`).
- `cli/` — package npm `usestakly-mcp` (Node ≥18) publié sur registry. `npx usestakly-mcp install` configure Codex/Cursor avec `Authorization: Bearer usk_...` pointant vers le backend.
- `docs/` — docs produit et techniques. Commencer par `docs/README.md`, puis `TODO.md`, `docs/strategy-pivot-2026-04-21.md`, `docs/architecture-backend-current.md` et `docs/mcp-protocol.md`.
- `deploy/coolify/` — cible de déploiement (voir `docs/deployment-coolify.md`, `docs/ops-mcp-coolify-hardening.md`).
- `scripts/seed-public-corpus.ps1` — seed corpus public via API.
- `docker-compose.yml` — uniquement Postgres local (image `pgvector/pgvector:pg17`, DB `project_k`, port 5432).

## Commandes

Avant tout : copier `.env.example` → `.env` à la racine (lu par le backend via `dotenvy`) et lancer la base.

```bash
docker compose up -d              # Postgres + pgvector sur :5432
```

### Backend (`cd backend`)

- `cargo run` — démarre l'API sur `127.0.0.1:4000` et applique les migrations.
- `cargo fmt --check` — vérifié par la CI.
- `cargo clippy --all-targets -- -D warnings` — vérifié par la CI (zéro warning toléré).
- `cargo test` — vérifié par la CI. Un test seul : `cargo test <nom>`.
- `cargo check` — itération rapide.
- Feature `semantic-search` (default OFF) — active `fastembed` pour les embeddings. Build prod sans la feature pour Docker léger.

### Frontend (`cd frontend`)

- `npm install` puis `npm run dev` — Vite sur `:5173`.
- `npm run build` — `tsc -b && vite build` (build type-check inclus, vérifié par la CI).
- `npm run preview` — sert le build.
- `npm run test:e2e` — Playwright avec mocks API. CI installe Chromium et upload le rapport.
- Pas de script lint séparé.

### CLI MCP (`cd cli`)

- `npm test` — `node --test` couvre validation token, écriture config Codex/Cursor, endpoint configurable.

## Architecture backend

Point d'entrée `backend/src/main.rs` → `config::AppConfig::from_env()` → `db::connect()` (pool + migrations + extensions optionnelles `vector`) → `app::build_app()` → `axum::serve`.

Découpage par couches (dans `backend/src/`) :

- `app/` — `Router` + `AppState { config, db }`, CORS strict sur `frontend_base_url`, `TraceLayer`, **middleware d'auth MCP qui rejette tout `/mcp` sans Bearer dès `initialize`/`tools/list`** (livré 2026-04-26), montage du service MCP via `rmcp::StreamableHttpService`.
- `config/` — env (DB, dev user, OAuth, session, GitHub PAT, admin token, scheduler, MCP guards, signaux actifs, semantic search).
- `auth/` — OAuth GitHub + Discord, session JWT cookie `usestakly_session`, `state` OAuth signé avec `return_to` sanitizé contre open redirects (livré 2026-04-24). Fallback dev user via `DEV_USER_*` (overridable par headers `x-debug-user-*`). **Pas de Supabase Auth.**
- `handlers/` — I/O HTTP : `health` (+ `/api/status/public`), `auth`, `me`, `account`, `admin`, `agent_tokens`, `search`, `repos` (re-export de `repos_query`/`repos_ingestion`/`repo_signals`/`repo_viewer`), `watchlist`, `notifications`.
- `services/` — logique métier : `ingestion/github`, `repos`, `watchlist`, `notifications`, `scheduler`, `semantic_search` (feature-gated), `agent_tokens`, sous-domaines `quality/` (`formula`, `compute`, `flags`, `weighting`, `pipeline`, `capture`) et `trust/` (`reputation`, `repo_owners`, `signal_reviews`, `signal_events`, `agent_token_events`, `mcp_metrics`).
- `domain/` — types actifs : `account`, `agent_token`, `quality`, `repo`, `reference`, `watchlist`.
- `db/` — pool + runner migrations + `ensure_optional_extensions`.
- `mcp/` — serveur MCP Streamable HTTP. 5 tools : `search_github_repos`, `recommend_github_repos`, `get_repo_quality_context`, `log_usage`, `watch_repo`. Auth Bearer `usk_<64 hex>` SHA-256.

Séparation à respecter pour tout nouveau code : `handler` (I/O) → `service` (logique) → `query` (DB).

## Architecture frontend

- Entrée `frontend/src/main.tsx` → `AppProviders` → `AppShell`.
- Routing **TanStack Router** (`frontend/src/app/router.tsx`). L'ancien hash routing custom a été retiré.
- Routes actives : `/` (landing), `/discover`, `/repos/$id`, `/watchlist`, `/notifications`, `/account`, `/login`, `/status`, `/privacy`, `/how-to-read`, `/mcp-guide`.
- Garde auth : `/watchlist`, `/notifications`, `/account` redirigent vers `/login` avec `returnTo` signé.
- Organisation par *features* (`src/features/{auth,layout,repos,account}/`). `repo-detail` et `account` sont éclatés en sous-composants (sprint refacto 3).
- État global : Zustand (`src/state/{auth-store,locale-store}.ts`). i18n EN/FR.
- Réseau : `src/lib/api-client.ts` + clients métier `src/lib/api/{account,admin,repos,watchlist}.ts` (`credentials: "include"`, base `VITE_API_BASE_URL`).
- `@tanstack/react-query` câblé dans `frontend/src/app/providers.tsx`.

## CLI MCP (`cli/`)

Package npm public `usestakly-mcp` (v0.1.3 au 2026-04-26).

- `npx usestakly-mcp install` — configure Codex / Cursor avec un token `usk_...` valide et le bon endpoint.
- `npx usestakly-mcp test` — vérifie connectivité et auth.
- Endpoint configurable (plus hardcodé depuis `1524587`).
- Tests `node --test` dans `cli/test/cli.test.mjs`.
- Doc release : `docs/mcp-cli-release.md`.

## Conventions critiques

### Scope produit

- Le produit vivant est : **discovery repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications in-app**, **MCP read + write v1 + recommend**.
- Les tables legacy `libraries` / `snippets` peuvent encore exister en base ; cela **n'autorise pas** à relancer ce produit.
- Si une tâche touche une zone legacy snippets : suppression des surfaces mortes, conservation prudente des migrations et données, protection du produit GitHub actif.
- `TODO.md` v5.5 est la source de vérité d'exécution. Les docs `docs/archive/snippets/` et `docs/archive/business-prepivot/` sont historiques uniquement.

### Principes produit à respecter

- Le **score qualité** et les **flags toxiques** (`deprecated`, `broken`, `security_issue`) sont le cœur du produit. La sélection privilégie les signaux d'usage et la fraîcheur, **pas les stars seules**.
- Les flags toxiques publics passent par **consensus N users distincts × réputation seuil**. `security_issue` reste en `pending` jusqu'à review admin. Les owners GitHub peuvent disputer.
- Toute évolution MCP doit **préserver la provenance** retournée aux agents (`source: "usestakly://..."`, `formula_version`, `scored_at`).
- Les write tools MCP doivent rester **sous garde** (quota, cooldown, fenêtre négatifs, réputation min, refus loggés).
- Ne pas casser les flows discovery, repo-detail, watchlist, notifications, auth, agent tokens, MCP install via CLI.

## CI

`.github/workflows/ci.yml` — trois jobs Ubuntu :

- backend : `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- frontend : `npm install`, `npm run build` (Node 22), Playwright install Chromium, `npm run test:e2e`, upload `playwright-report/` artifact.

Aucun service Postgres provisionné en CI ; tout test DB-bound doit mocker ou être isolé derrière une feature.

## Gotchas

- `sqlx::migrate!` inclut les migrations au compile-time : toute nouvelle migration exige une recompilation du backend.
- Cookies de session exigent `APP_SESSION_SECRET` ; sans lui, `auth_enabled()` renvoie `false` et seul le dev user fonctionne.
- CORS strictement limité à `FRONTEND_BASE_URL` avec `allow_credentials(true)` — changer l'URL frontend casse l'auth.
- `docker-compose.yml` ne démarre **que** Postgres : backend et frontend tournent en local hors Docker.
- **Scheduler** (`services::scheduler::spawn_recompute_loop`) **opt-in** : `APP_SCHEDULER_ENABLED=true` pour activer, `APP_RECOMPUTE_INTERVAL_SECS` pour la cadence (default 86400). Refresh des watchés via `ingest_repo` puis `recompute_all_scores`. Pas de run au boot. Laissé OFF en dev pour ne pas taper l'API GitHub.
- **Serveur MCP** monté à `/mcp` via `rmcp::StreamableHttpService` (`route_service` dans `app::build_app`). **Authorization Bearer requise dès `initialize`/`tools/list`** depuis `5a10ca4` (middleware pré-transport, doc `docs/mcp-endpoint-security.md`). Tokens `usk_<64 hex>` hashés SHA-256 (table `agent_tokens`, migration 0013), plaintext affiché une seule fois à la création via `POST /api/agent-tokens`.
- **Rate-limit MCP par token** sur les writes via `agent_token_events` (migration 0014). Quota global multi-token / par IP **pas encore implémenté** — c'est l'item ops #2 de `docs/ops-mcp-coolify-hardening.md`.
- **Moderation signals** : migrations 0015 (`quality_signal_review`) et 0016 (`quality_signal_events`). Garde-fous v1 (consensus + réputation + review admin + cooldown anti-spam) en place. Réputation v2 runtime livrée. Formula_v2 (compte neuf = poids 0) + Sybil OAuth restent à venir.
- **Scoring formula v1.1** (livré 2026-04-24) : pondération `outcome_weight × reporter_weight × dedup_weight` dans `services/quality/weighting.rs`. Fichier `scoring/formula_v1.toml` section `[weighting]`. Endpoint admin `GET /api/admin/scoring/explain/{repo_id}` pour breakdown signal par signal.
- **Semantic search** (R2b) derrière feature Cargo `semantic-search` (default OFF). Migration 0017 `repo_embeddings`, fastembed + pgvector. **OFF en prod par défaut** (`APP_SEMANTIC_SEARCH_ENABLED=false`) — calibration ranking hybride bundle `17ade16` à valider sur corpus plus large.
- **Archived GitHub ≠ abandon** : ne pas câbler `archived=true` comme trigger unique d'abandon dans `formula_v2`.
- **Public status** : `GET /api/status/public` est exposé sans auth pour la page `/status` du frontend. Renvoie healthcheck + métadonnées beta.
- **Docker prod** : `fastembed` derrière feature `semantic-search`, Rust builder 1.91, Dockerfile copie `scoring/`, pgvector créé via `ensure_optional_extensions` si dispo dans `pg_available_extensions`.
