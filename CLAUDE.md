# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Les deux cohabitent dans le code, les docs et les chemins. Ne pas faire de renommage spontané — voir `docs/plans/rename-to-usestakly.md` pour la stratégie de transition.
- Objectif : bibliothèques adressables de snippets de code (privés/publics), exposées à une IA via MCP pour qu'elle **assemble** des briques existantes au lieu d'en inventer.
- État : MVP en cours (voir `TODO.md`). Backend + frontend bootstrappés, auth OAuth opérationnelle, search/resolve/MCP **pas encore implémentés** (phases 6–8 ouvertes).

## Layout monorepo

- `backend/` — API Rust (Axum + SQLx). Les migrations Postgres vivent dans `backend/migrations/` et sont exécutées automatiquement au boot par `sqlx::migrate!` dans `backend/src/db/mod.rs`.
- `frontend/` — React 19 + Vite + Tailwind v4 + TypeScript. Routing par `hash` (pas TanStack Router pour l'instant malgré la dépendance).
- `docs/` — **source de vérité** pour architecture, nomenclature, protocole MCP, plans. Commencer par `docs/README.md`, puis `docs/plans/mvp-one-shot-blueprint.md` et `docs/plans/mvp-file-by-file-checklist.md`.
- `deploy/coolify/` — cible de déploiement (voir `docs/deployment-coolify.md`).
- `docker-compose.yml` — uniquement la base : image `pgvector/pgvector:pg17`, DB `project_k`, port 5432.

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
- `config/` — lecture d'env (DB, dev user, OAuth GitHub/Discord, session secret).
- `auth/` — OAuth direct **GitHub + Discord** avec session JWT dans un cookie `usestakly_session`. Quand `APP_SESSION_SECRET` + un des couples `*_CLIENT_ID/SECRET` est absent, on retombe sur un **dev user** injecté via env `DEV_USER_*` (et surchargeable par headers `x-debug-user-*`). **Supabase Auth n'est pas et ne sera pas utilisé** — l'app est auto-hébergée sur VPS (Coolify), pas de dépendance à un SaaS d'auth externe. Les anciens plans dans `docs/plans/mvp-*.md` qui décrivent une intégration Supabase Auth sont obsolètes sur ce point.
- `handlers/` — I/O HTTP uniquement (health, auth, me, libraries, snippets).
- `services/` — logique métier (actuellement stub).
- `domain/` — types métier (`library`, `snippet`, `generation`, `search`).
- `db/` — pool + runner de migrations.
- `mcp/`, `search/`, `security/` — squelettes ; implémentation à venir (phases 6–8).

Séparation imposée par `GEMINI.md` : `handler` (I/O) → `service` (logique) → `query` (DB). À respecter pour tout nouveau code.

## Architecture frontend

- Entrée `frontend/src/main.tsx` → `AppProviders` → `AppShell`.
- `AppShell.tsx` gère le routing par `window.location.hash` (parse/rebuild en local) — pas de router lib active malgré `@tanstack/react-router` dans les deps.
- Organisation par *features* (`src/features/{app,auth,libraries,snippets,workspace,search,generations}/`) avec un sous-dossier `components/` ; le dossier `components/` de haut niveau est réservé au layout et aux primitives communes.
- État global : Zustand (`src/state/{auth-store,ui-store}.ts`). Réseau : fetch direct via `src/lib/api-client.ts` (`credentials: "include"`, base `VITE_API_BASE_URL`).
- `@tanstack/react-query` est installé mais pas encore câblé.

## Conventions critiques

### Nomenclature des snippets (voir `docs/nomenclature.md`)

Format **obligatoire** pour tout slug de snippet :
```
{domain}-{kind}-{category}-{name}-{variant?}
```
- kebab-case, ASCII uniquement, anglais, pas de version dans le nom.
- Regex : `^(frontend|backend|devops|data|shared)-[a-z]+-[a-z]+-[a-z0-9-]+$`.
- Le `kind` doit exister dans la table `snippet_kinds` pour le `domain` correspondant.
- Versioning via `snippet_versions` (append-only, semver, `current_version_id` sur `snippets`).

### Principes produit à respecter

- **Interdit d'inventer** : quand l'IA propose du code dans un flow MCP, elle doit d'abord chercher un snippet existant (`search_library`) et inclure un commentaire de provenance `// Assemblé depuis: <slug>@<version>`.
- Détection / embeddings : **100% local** (tree-sitter + fastembed). Pas de dépendance à une API payante pour ces briques.

## CI

`.github/workflows/ci.yml` — deux jobs Ubuntu :
- backend : `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- frontend : `npm install`, `npm run build` (Node 22).

Aucun service Postgres n'est provisionné dans la CI ; tout test backend nécessitant la DB doit soit la mocker, soit être isolé derrière une feature.

## Gotchas

- `sqlx::migrate!` inclut les migrations au compile-time : toute nouvelle migration exige une recompilation du backend.
- Les cookies de session exigent `APP_SESSION_SECRET` ; sans lui, `auth_enabled()` renvoie `false` et seul le dev user fonctionne.
- CORS est strictement limité à `FRONTEND_BASE_URL` avec `allow_credentials(true)` — changer l'URL frontend casse l'auth.
- `docker-compose.yml` ne démarre **que** Postgres : le backend et le frontend tournent en local hors Docker.
