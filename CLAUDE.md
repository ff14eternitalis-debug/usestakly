# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Les deux cohabitent encore dans certains chemins et docs. Ne pas faire de renommage spontané — voir `docs/plans/rename-to-usestakly.md`.
- Objectif produit actuel : **veille GitHub OSS**. UseStakly score des repos GitHub publics pour aider des devs et des agents IA à choisir de meilleures dépendances qu'avec les seules stars.
- Deux piliers actifs : **discovery qualité-scored** + **watchlist / notifications**. Les agents IA consomment la même data via MCP.
- État : MVP en cours (voir `TODO.md`). Backend, frontend, auth OAuth, scoring, watchlist, notifications, scheduler, MCP read-only + write v1 sont en place. Reste principalement la finition moderation/réputation v2, l'E2E et la page compte complète.
- L'ancien produit de **bibliothèque de snippets est abandonné**. Le schéma SQL historique (`libraries`, `snippets`, `snippet_versions`…) peut rester présent, mais **ne pas réintroduire** de surfaces produit snippets sans demande explicite.

## Layout monorepo

- `backend/` — API Rust (Axum + SQLx). Les migrations Postgres vivent dans `backend/migrations/` et sont exécutées automatiquement au boot par `sqlx::migrate!` dans `backend/src/db/mod.rs`.
- `frontend/` — React 19 + Vite + Tailwind v4 + TypeScript. Routing avec **TanStack Router** (`frontend/src/app/router.tsx`).
- `docs/` — docs produit et techniques. Commencer par `docs/README.md`, puis `TODO.md`, `docs/strategy-pivot-2026-04-21.md` et `docs/mcp-protocol.md`. Les docs snippets historiques sont archivées dans `docs/archive/snippets/`.
- `deploy/coolify/` — cible de déploiement (voir `docs/deployment-coolify.md`).
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
- `cargo test` — vérifié par la CI. Un test seul : `cargo test <nom_test>` ou `cargo test --test <fichier>`.
- `cargo check` — itération rapide.

### Frontend (`cd frontend`)

- `npm install` puis `npm run dev` — Vite sur `:5173`.
- `npm run build` — `tsc -b && vite build` (build type-check inclus, vérifié par la CI).
- `npm run preview` — sert le build.
- Pas de script lint/test déclaré ; la CI ne lance que `npm install` + `npm run build`.

## Architecture backend

Point d'entrée `backend/src/main.rs` → `config::AppConfig::from_env()` → `db::connect()` (pool + migrations) → `app::build_app()` → `axum::serve`.

Découpage par couches (dans `backend/src/`) :

- `app/` — assembly du `Router` et `AppState { config, db }`, CORS restreint à `frontend_base_url`, `TraceLayer`, montage du service MCP.
- `config/` — lecture d'env (DB, dev user, OAuth GitHub/Discord, session secret, token GitHub d'ingestion, token admin, scheduler).
- `auth/` — OAuth direct **GitHub + Discord** avec session JWT dans un cookie `usestakly_session`. Quand `APP_SESSION_SECRET` + un des couples `*_CLIENT_ID/SECRET` est absent, on retombe sur un **dev user** injecté via env `DEV_USER_*` (et surchargeable par headers `x-debug-user-*`). **Supabase Auth n'est pas et ne sera pas utilisé** — l'app est auto-hébergée sur VPS (Coolify).
- `handlers/` — I/O HTTP : `health`, `auth`, `me`, `account`, `admin`, `agent_tokens`, `search`, `repos` (+ découpage en cours : `repos_query`, `repos_ingestion`, `repo_signals`, `repo_viewer`), `watchlist`, `notifications`.
- `services/` — logique métier : `ingestion` (GitHub REST), `quality` (scoring + formule TOML), `repos`, `watchlist`, `notifications`, `scheduler`, `agent_tokens`, et sous-domaine `trust/` (réputation, repo owners, signal reviews, signal events, agent token events).
- `domain/` — types métier actifs : `account`, `agent_token`, `quality`, `repo`, `reference`, `watchlist`.
- `db/` — pool + runner de migrations.
- `mcp/` — serveur MCP actif pour la registry GitHub scorée (`rmcp::StreamableHttpService`).

Séparation à respecter pour tout nouveau code : `handler` (I/O) → `service` (logique) → `query` (DB).

## Architecture frontend

- Entrée `frontend/src/main.tsx` → `AppProviders` → `AppShell`.
- Le routing passe par **TanStack Router** (`frontend/src/app/router.tsx`). L'ancien hash routing custom a été retiré.
- Routes actives : `/` (landing), `/discover`, `/repo/$owner/$name` (repo-detail), `/watchlist`, `/notifications`, `/account`, `/login`.
- Organisation par *features* (`src/features/{auth,layout}/`) ; le gros de la logique de page vit aujourd'hui dans `src/routes/` et va être extrait en sous-composants (voir `docs/plans/refactor-plan-2026-04-23.md` Sprint 3).
- État global : Zustand (`src/state/{auth-store,locale-store}.ts`). Réseau : fetch direct via `src/lib/api-client.ts` (`credentials: "include"`, base `VITE_API_BASE_URL`).
- `@tanstack/react-query` est installé mais **pas encore câblé** (dette connue).

## Conventions critiques

### Scope produit

- Le produit vivant est : **recherche de repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications**, **MCP GitHub read + write v1**.
- Les tables legacy `libraries` / `snippets` peuvent encore exister en base ; cela **n'autorise pas** à relancer ce produit.
- Si une tâche touche une zone legacy snippets, privilégier la suppression des surfaces mortes, la conservation prudente des migrations et données, la protection du produit GitHub actif.
- `TODO.md` est la source de vérité d'exécution. Les docs `docs/archive/snippets/` sont historiques uniquement.

### Principes produit à respecter

- Le **score qualité** et les **flags toxiques** (`deprecated`, `broken`, `security_issue`) sont le cœur du produit. La sélection doit privilégier les signaux d'usage et la fraîcheur, **pas les stars seules**.
- Les flags toxiques publics passent par du **consensus N users distincts × réputation seuil**. `security_issue` reste en `pending` jusqu'à review admin. Les owners GitHub peuvent disputer.
- Toute évolution MCP doit **préserver la provenance** retournée aux agents (`source: "usestakly://…"`, `formula_version`, `scored_at`).
- Ne pas casser les flows discovery, repo-detail, watchlist, notifications, auth, agent tokens.

## CI

`.github/workflows/ci.yml` — deux jobs Ubuntu :
- backend : `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- frontend : `npm install`, `npm run build` (Node 22).

Aucun service Postgres n'est provisionné dans la CI ; tout test backend nécessitant la DB doit soit la mocker, soit être isolé derrière une feature.

## Gotchas

- `sqlx::migrate!` inclut les migrations au compile-time : toute nouvelle migration exige une recompilation du backend.
- Les cookies de session exigent `APP_SESSION_SECRET` ; sans lui, `auth_enabled()` renvoie `false` et seul le dev user fonctionne.
- CORS est strictement limité à `FRONTEND_BASE_URL` avec `allow_credentials(true)` — changer l'URL frontend casse l'auth.
- `docker-compose.yml` ne démarre **que** Postgres : backend et frontend tournent en local hors Docker.
- **Scheduler R3** (refresh watched repos + recompute + notifs) est **opt-in** : `APP_SCHEDULER_ENABLED=true` pour l'activer, `APP_RECOMPUTE_INTERVAL_SECS` pour la cadence (default 86400). Laissé off en dev pour ne pas taper l'API GitHub au démarrage. Spawn via `tokio::spawn` dans `services::scheduler` — pas de crate cron externe.
- **Serveur MCP** monté à `/mcp` via `rmcp::StreamableHttpService` (Tower service, `route_service` dans `app::build_app`). Auth Bearer via la table `agent_tokens` (migration 0013) — tokens au format `usk_<64 hex>`, hashés SHA-256 en DB, plaintext affiché une seule fois à la création via `POST /api/agent-tokens`. Rate-limit par token via `agent_token_events` (migration 0014). Doc exhaustive : `docs/mcp-protocol.md` v2.
- **Moderation signals** : migrations 0015 (`quality_signal_review`) et 0016 (`quality_signal_events`) portent le workflow review/dispute/timeline des flags toxiques. Les garde-fous v1 (consensus + réputation + review admin + cooldown anti-spam) sont en place ; la pondération réputation v2 est à venir.
- Archived GitHub ≠ abandon : **ne pas** câbler `archived=true` comme trigger unique d'abandon dans `formula_v2`.
