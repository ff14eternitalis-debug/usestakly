# GEMINI.md

This file provides guidance to Gemini CLI when working with code in this repository. Reflète l'état au 2026-05-16.

> Ce fichier est miroir de `CLAUDE.md` et `AGENTS.md`. Source de vérité doc : `docs/source-of-truth.md`. Backlog : `docs/plans/remaining-work-2026-05-03.md`.

## Produit

- Nom produit : **UseStakly**. Nom de travail historique : **Project-K** (Komorebi). Pas de renommage spontané — voir `docs/plans/rename-to-usestakly.md`.
- Objectif : **veille GitHub OSS**. UseStakly score des repos GitHub publics pour aider devs et agents IA à choisir leurs dépendances autrement que par les stars.
- État : **public beta exposée et redéployée**. Ops MCP critiques en place (backup DB, Bearer `/mcp`, rate-limit IP/token, Uptime Kuma). Restent surtout : backup offsite/S3, polish release.
- Trois piliers : **discovery qualité-scored** + **watchlist / notifications** + **MCP pour agents** (6 tools, CLI npm `usestakly-mcp`).
- L'ancien produit **bibliothèque de snippets est abandonné** (pivot 2026-04-21). Ne pas réintroduire de surfaces snippets.

## Layout monorepo

- `backend/` — API Rust (Axum 0.8 + SQLx 0.8 + rmcp 1.5). Migrations au boot. Binaire `seed_github`.
- `frontend/` — React 19 + Vite 7 + Tailwind v4 + TanStack Router + Query + Zustand. E2E Playwright.
- `cli/` — package npm `usestakly-mcp` (v0.1.4).
- `docs/` — `docs/source-of-truth.md`, `docs/README.md`, `docs/plans/remaining-work-2026-05-03.md`, `docs/architecture-backend-current.md`, `docs/mcp-protocol.md`. Archives = historique.
- `docker-compose.yml` — uniquement Postgres (`project_k`, `:5432`).

## Commandes

```bash
docker compose up -d
cd backend && cargo run
cd frontend && npm install && npm run dev
cd cli && npm test
```

## Architecture

- `mcp/` — handlers `server.rs`, DTOs `tools/*`. 6 tools : `search_github_repos`, `recommend_github_repos`, `get_repo_quality_context`, `log_usage`, `watch_repo`, `watch_use_case`.
- Séparation : `handler` → `service` → `query` (DB).

## Conventions

- Produit vivant : discovery scorée, profil repo, watchlist, notifications, MCP read/write/recommend/watch-use-case.
- MCP préserve **provenance** (`source`, `formula_version`, `scored_at`).
- Write tools **sous garde** (quota, cooldown, réputation).

## Gotchas

- `sqlx::migrate!` compile-time → recompiler après migration.
- `APP_SESSION_SECRET` requis pour OAuth ; sinon dev user.
- **MCP `/mcp`** : Bearer obligatoire dès `initialize`/`tools/list`.
- **Rate-limit MCP** : writes (0014) + limites IP/token protocole/read (`app/mod.rs`, `APP_MCP_*`).
- **Modération** : réputation v2 + trust formula_v2 livrés ; Sybil OAuth à venir.
- **Semantic search** OFF par défaut en prod.
- **Archived GitHub ≠ abandon** dans `formula_v2`.
