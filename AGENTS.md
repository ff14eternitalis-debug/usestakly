# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Les deux cohabitent encore dans certains chemins et docs. Ne pas faire de renommage spontané — voir `docs/plans/rename-to-usestakly.md`.
- Objectif produit actuel : **veille GitHub OSS**. UseStakly score des repos GitHub publics pour aider des devs et des agents IA à choisir de meilleures dépendances qu'avec les seules stars.
- Deux piliers actifs : **discovery qualité-scored** + **watchlist / notifications**.
- État : MVP en cours (voir `TODO.md`). Le backend, le frontend, l'auth OAuth, le scoring, la watchlist, les notifications et le MCP read-only sont déjà en place.
- L'ancien produit de **bibliothèque de snippets** est **abandonné**. Le schéma SQL historique peut rester présent, mais il ne faut pas réintroduire de nouvelles surfaces produit snippets sans demande explicite.

## Layout monorepo

- `backend/` — API Rust (Axum + SQLx). Les migrations Postgres vivent dans `backend/migrations/` et sont exécutées automatiquement au boot par `sqlx::migrate!` dans `backend/src/db/mod.rs`.
- `frontend/` — React 19 + Vite + Tailwind v4 + TypeScript. Routing avec **TanStack Router**.
- `docs/` — docs produit et techniques. Commencer par `docs/README.md`, puis `TODO.md`, `docs/strategy-pivot-2026-04-21.md` et `docs/mcp-protocol.md`.
- `deploy/coolify/` — cible de déploiement (voir `docs/deployment-coolify.md`).
- `docker-compose.yml` — uniquement Postgres local.

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

- `app/` — assembly du `Router` et `AppState { config, db }`, CORS restreint à `frontend_base_url`, `TraceLayer`.
- `config/` — lecture d'env (DB, dev user, OAuth GitHub/Discord, session secret, token GitHub, token admin, scheduler).
- `auth/` — OAuth direct **GitHub + Discord** avec session JWT dans un cookie `usestakly_session`. Quand `APP_SESSION_SECRET` + un des couples `*_CLIENT_ID/SECRET` est absent, on retombe sur un **dev user** injecté via env `DEV_USER_*` (et surchargeable par headers `x-debug-user-*`). **Supabase Auth n'est pas utilisé**.
- `handlers/` — I/O HTTP pour health, auth, me, search, repos, watchlist, notifications, admin, agent tokens.
- `services/` — logique métier (ingestion GitHub, scoring, repos, watchlist, notifications, scheduler).
- `domain/` — types métier actifs (`repo`, `quality`, `watchlist`, `agent_token`, filtres/représentations communes).
- `db/` — pool + runner de migrations.
- `mcp/` — serveur MCP actif pour la registry GitHub scorée.
- `search/`, `security/` — réserves / dette technique ; ne pas les élargir sans besoin produit clair.

Séparation imposée par `GEMINI.md` : `handler` (I/O) → `service` (logique) → `query` (DB). À respecter pour tout nouveau code.

## Architecture frontend

- Entrée `frontend/src/main.tsx` → `AppProviders` → `AppShell`.
- Le routing passe par **TanStack Router** (`frontend/src/app/router.tsx`).
- L'app active est organisée autour de `discover`, `repo-detail`, `watchlist`, `notifications`, `account`, `login`.
- État global : Zustand pour l'auth / locale, React Query pour la data serveur.
- Réseau : `src/lib/api-client.ts` + clients métier `src/lib/api/*.ts` (`credentials: "include"`, base `VITE_API_BASE_URL`).

## Conventions critiques

### Scope produit

- Le produit vivant est : **recherche de repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications**, **MCP GitHub**.
- Les tables legacy `libraries` / `snippets` peuvent exister encore en base ; cela **n'autorise pas** à relancer ce produit.
- Si une tâche touche une zone legacy snippets, privilégier :
  - suppression des surfaces mortes,
  - conservation prudente des migrations et données,
  - protection du produit GitHub actif.

### Principes produit à respecter

- Le score qualité et les flags sont le cœur du produit.
- La sélection doit privilégier les signaux d'usage et la fraîcheur, pas les stars seules.
- Toute évolution MCP doit préserver la provenance retournée aux agents.
- Ne pas casser les flows discovery, repo detail, watchlist, notifications, auth, agent tokens.

## CI

`.github/workflows/ci.yml` — deux jobs Ubuntu :
- backend : `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- frontend : `npm install`, `npm run build` (Node 22).

Aucun service Postgres n'est provisionné dans la CI ; tout test backend nécessitant la DB doit soit la mocker, soit être isolé derrière une feature.

## Gotchas

- `sqlx::migrate!` inclut les migrations au compile-time : toute nouvelle migration exige une recompilation du backend.
- Les cookies de session exigent `APP_SESSION_SECRET` ; sans lui, `auth_enabled()` renvoie `false` et seul le dev user fonctionne.
- CORS est strictement limité à `FRONTEND_BASE_URL` avec `allow_credentials(true)` — changer l'URL frontend casse l'auth.
- `docker-compose.yml` ne démarre que Postgres : backend et frontend tournent en local hors Docker.
- Scheduler R3 (refresh watched repos + recompute + notifs) est **opt-in** : `APP_SCHEDULER_ENABLED=true` pour l'activer, `APP_RECOMPUTE_INTERVAL_SECS` pour la cadence (default 86400). Spawn via `tokio::spawn` dans `services::scheduler`.
- Serveur MCP monté à `/mcp` via `rmcp::StreamableHttpService`. Auth Bearer via la table `agent_tokens` (migration 0013) — tokens au format `usk_<64 hex>`, hashés SHA-256 en DB, plaintext affiché une seule fois à la création via `POST /api/agent-tokens`.
- `TODO.md` est la source de vérité d'exécution. Beaucoup de docs snippets plus anciens sont historiques uniquement.
