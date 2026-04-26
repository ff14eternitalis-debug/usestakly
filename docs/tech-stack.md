# UseStakly — Stack technique

> Version : 2.0 — 2026-04-26
> Vue à jour de la stack réellement déployée en public beta. Pour les détails de structure, voir `architecture-backend-current.md`.

## Vue d'ensemble

| Couche | Choix | Pourquoi |
|---|---|---|
| Backend core | **Rust 2024**, Axum 0.8, Tokio | Performance MCP + sécurité mémoire + binaire statique |
| ORM / DB | **SQLx 0.8** + PostgreSQL 16 | SQL standard, migrations compile-time, pgvector dispo |
| Vecteurs (optionnel) | **pgvector** + `fastembed` 5 (local) | Zéro clé API, zéro coût variable, OFF par défaut |
| MCP | **rmcp 1.5** Streamable HTTP | Standard MCP, transport HTTP simple à proxy via Coolify |
| Frontend | **React 19** + Vite 7 + TypeScript 5.9 + Tailwind v4 | Stack moderne, SPA suffit |
| Routing | **TanStack Router** + TanStack Query | Type-safe, intégration React Query naturelle |
| State | **Zustand 5** | Minimal, pas de boilerplate Redux |
| CLI MCP | Node ≥18, `node:test` | Léger, distribué via npm registry |
| E2E | **Playwright** | Mocks API → filet anti-régression UI, pas validation backend |
| CI | **GitHub Actions** | Standard, gratuit |
| Auth | **OAuth GitHub + Discord direct** | Pas de provider externe, app auto-hébergée |
| Hébergement | **Coolify sur VPS** | Auto-hébergement, frontend + backend + Postgres sur la même plateforme |

## Backend — Rust

### Dépendances clés (`backend/Cargo.toml`)

```toml
axum = "0.8"
sqlx = { version = "0.8", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
rmcp = { version = "1.5", features = ["server", "transport-streamable-http-server", "macros"] }
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
octocrab = "0.49.7"          # GitHub API (utilisé en complément de reqwest)
fastembed = { version = "5", optional = true }   # feature `semantic-search`
sha2 = "0.10"                # hash MCP tokens
jsonwebtoken = "9"           # session JWT cookie
serde = { version = "1", features = ["derive"] }
toml = "0.8"                 # scoring/formula_v1.toml
```

### Features

- `default = []`
- `semantic-search = ["dep:fastembed"]` — active embeddings + ranking hybride. **OFF par défaut**, OFF en prod (`APP_SEMANTIC_SEARCH_ENABLED=false`).

### Builder Docker

- Rust 1.91 (image officielle)
- Dockerfile copie `scoring/` (TOML formula chargé au runtime)

## Frontend — React + Tailwind

### Dépendances clés (`frontend/package.json`)

```json
{
  "react": "^19.2.0",
  "@tanstack/react-router": "^1.133",
  "@tanstack/react-query": "^5.90",
  "zustand": "^5.0",
  "tailwindcss": "^4.1",
  "vite": "^7.1",
  "typescript": "^5.9",
  "@playwright/test": "^1.59"
}
```

### Build

- `tsc -b && vite build` — type-check inclus, vérifié par la CI.
- E2E : `npm run test:e2e` (Playwright + mocks API). CI installe Chromium, upload `playwright-report/`.

## CLI MCP (`cli/`)

- Package npm public `usestakly-mcp` (v0.1.3 au 2026-04-26)
- Node ≥18, `type: "module"`, ESM
- Tests : `node --test`
- Install agent : `npx usestakly-mcp install` configure Codex / Cursor avec Bearer + endpoint configurable

## Base de données

- PostgreSQL 16 + pgvector
- Image dev : `pgvector/pgvector:pg17`
- Migrations via `sqlx::migrate!` (compile-time, exécutées au boot)
- 17 migrations (1–9 legacy snippets dormantes, 10–17 actives produit GitHub)
- Extension `vector` créée via `ensure_optional_extensions` si présente dans `pg_available_extensions`

## Auth & secrets

- OAuth GitHub + Discord côté backend, session JWT signée via `APP_SESSION_SECRET` dans le cookie `usestakly_session`
- `state` OAuth signé, porte un `return_to` sanitizé contre les open redirects
- Pas de Supabase Auth ni d'autre provider externe
- Secrets injectés en env (jamais commités, `.env` gitignore + `.env.example` checked-in)

## Hébergement

| Composant | Cible |
|---|---|
| Backend Rust | Coolify (Dockerfile dans `backend/`) |
| Frontend | Coolify (Dockerfile dans `frontend/`) |
| PostgreSQL | Coolify managed DB resource |
| DNS / CDN | Cloudflare |
| Statut public | `GET /api/status/public` exposé sans auth |

Coût total dépend du serveur Coolify retenu. Architecture simple à opérer (un VPS, un panel).

## Choix explicitement écartés

| Écarté | Raison |
|---|---|
| Node.js backend | Performance MCP + binaire statique = Rust |
| MongoDB | Données relationnelles (artifacts, scores, signals, watchlists) |
| Vector DB dédiée (Pinecone, Weaviate) | pgvector suffit |
| Next.js App Router | Pas besoin de SSR, SPA Vite suffit |
| Redux / MobX | Zustand couvre nos besoins |
| GraphQL | REST/JSON suffit |
| Embeddings cloud (OpenAI) | `fastembed` local = 0 coût variable |
| Supabase Auth / Auth0 | App auto-hébergée, OAuth direct simple à maintenir |
| Sandpack / Monaco / tree-sitter | Plus dans le scope produit (pivot 2026-04-21) |
| Cron externe (Hangfire, Quartz) | `tokio::spawn` + interval suffit pour le scheduler |
