# UseStakly

> Infrastructure de bibliothèques de code privées ou publiques que les IA peuvent resoudre, chercher et assembler via MCP.

> `UseStakly` est le nom produit retenu. `Project-K` reste le nom de travail historique encore present dans certaines structures techniques.

## En bref

`UseStakly` permet de :
- creer des bibliotheques de code adressables
- stocker des snippets multi-domaines et multi-langages
- publier certaines briques si souhaite
- resoudre un snippet par reference exacte
- permettre a une IA d'assembler avant d'inventer

## Stack MVP

- Backend : Rust (edition 2024), Axum 0.8, SQLx 0.8, PostgreSQL + pgvector
- Frontend : React 19, TypeScript, Vite 7, Tailwind CSS v4, Zustand
- Auth : OAuth direct GitHub + Discord, session JWT en cookie `usestakly_session` (app auto-hébergée sur VPS, aucun SaaS d'auth — Supabase Auth volontairement écarté)
- Detection (prevu) : tree-sitter + fastembed, 100 % local

## Structure

```text
project-k/
├── backend/            # API Rust (Axum) + migrations SQLx
│   ├── src/            # app, auth, config, db, domain, handlers, mcp, search, security, services, telemetry
│   └── migrations/     # 0001..0009 appliquees au boot
├── frontend/           # React 19 + Vite + Tailwind v4
│   └── src/            # app, components, features, lib, state, routes
├── docs/               # source de verite (architecture, plans, nomenclature, MCP)
├── deploy/coolify/     # cible d'hebergement
├── docker-compose.yml  # Postgres + pgvector uniquement
└── .github/workflows/  # CI (fmt, clippy, test, build)
```

## Quickstart local

Prerequis : Docker, Rust stable, Node 22, npm.

```bash
cp .env.example .env            # ajuster DATABASE_URL, DEV_USER_*, OAuth si besoin
docker compose up -d            # demarre Postgres + pgvector sur :5432

cd backend
cargo run                       # API sur http://127.0.0.1:4000, migrations auto

cd ../frontend
npm install
npm run dev                     # UI sur http://localhost:5173
```

Sans `APP_SESSION_SECRET` ni `*_CLIENT_ID/SECRET`, l'auth OAuth est desactivee et le backend utilise le **dev user** defini par les variables `DEV_USER_*`.

## Etat

Ce que le code couvre aujourd'hui (voir `TODO.md` pour le detail) :

- [x] Bootstrap backend + frontend, CI
- [x] Migrations 0001..0009 (users, libraries, snippets, versions, tags, rules, permissions, generations)
- [x] CRUD libraries + snippets + versioning append-only
- [x] OAuth GitHub et Discord
- [ ] Parsing `@library:snippet@version`, `/api/search`, `/api/resolve`
- [ ] Serveur MCP (`resolve_reference`, `search_library`, `assemble_plan`...)
- [ ] Couche Trust / Safety (sanitize, classification, provenance obligatoire)

## Commandes utiles

| Contexte | Commande |
|---|---|
| Backend format + lint strict | `cd backend && cargo fmt --check && cargo clippy --all-targets -- -D warnings` |
| Backend tests | `cd backend && cargo test` |
| Frontend build (type-check inclus) | `cd frontend && npm run build` |

## Documentation

Le centre de documentation est dans [`docs/`](./docs/).

Points d'entree recommandes :
- [docs/README.md](./docs/README.md)
- [docs/plans/mvp-one-shot-blueprint.md](./docs/plans/mvp-one-shot-blueprint.md)
- [docs/plans/mvp-file-by-file-checklist.md](./docs/plans/mvp-file-by-file-checklist.md)

## Suivi

- checklist historique : [TODO.md](./TODO.md)
- blueprint MVP : [docs/plans/mvp-one-shot-blueprint.md](./docs/plans/mvp-one-shot-blueprint.md)
