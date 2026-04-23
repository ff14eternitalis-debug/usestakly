# Phase 1 — Fondations

> Version : 1.0 — 2026-04-15
> Durée estimée : 1 semaine
> Dépendances : aucune

## 🎯 Objectif

Poser le **squelette du monorepo**, configurer les outils de base (CI, lint, DB, Docker) et s'assurer que backend + frontend démarrent en local.

## 📋 Livrables

1. Monorepo initialisé avec la structure de [tech-stack.md](../tech-stack.md)
2. `docker-compose.yml` avec PostgreSQL + pgvector
3. Backend Rust : squelette Axum + `sqlx` + route `/health`
4. Frontend React : Vite + Tailwind v4 + page d'accueil
5. CI GitHub Actions : lint + test + build
6. Fichiers de config : `.env.example`, `.gitignore`, `README.md`

## 🔨 Tâches détaillées

### 1.1 Initialisation du dépôt
- [ ] `git init` + `.gitignore` (Node, Rust, OS, .env)
- [ ] Créer l'arborescence : `backend-core/`, `frontend-studio/`, `shared/`, `deploy/`, `docs/`
- [ ] Ajouter un `README.md` racine avec le pitch et un guide de démarrage

### 1.2 Backend Rust
- [ ] `cargo new backend-core --bin`
- [ ] `Cargo.toml` avec les dépendances de [tech-stack.md](../tech-stack.md)
- [ ] Fichier `src/main.rs` : serveur Axum minimal
- [ ] Route `GET /health` → `{"status": "ok", "version": "..."}`
- [ ] Configuration via `.env` (PORT, DATABASE_URL, JWT_SECRET)
- [ ] Logging avec `tracing` + `tracing-subscriber`

### 1.3 Frontend React
- [ ] `pnpm create vite frontend-studio --template react-ts`
- [ ] Installer et configurer Tailwind v4 (`@tailwindcss/vite`)
- [ ] Créer une page d'accueil minimale (logo, tagline)
- [ ] Router via `@tanstack/react-router`
- [ ] Client HTTP : `fetch` wrapper ou `ky`

### 1.4 Base de données
- [ ] `docker-compose.yml` avec service `postgres:16` + image `pgvector/pgvector:pg16`
- [ ] `CREATE EXTENSION vector;` au démarrage (script `init.sql`)
- [ ] Installer `sqlx-cli` : `cargo install sqlx-cli`
- [ ] Créer `migrations/00000000000000_init.sql` (juste un `SELECT 1;`)
- [ ] Connexion testée depuis Rust au démarrage

### 1.5 CI/CD
- [ ] `.github/workflows/ci.yml` : lint (`clippy`, `eslint`) + test + build
- [ ] Cache Cargo + pnpm pour accélérer
- [ ] Badge de build dans le README

### 1.6 Documentation dev
- [ ] `CONTRIBUTING.md` : comment lancer le projet en local
- [ ] Commande `make dev` ou `just dev` qui démarre tout (Docker + backend + frontend)

## ✅ Definition of Done

- [ ] `docker-compose up -d` démarre PostgreSQL avec pgvector installé
- [ ] `cargo run` dans `backend-core` expose `http://localhost:4000/health` → 200
- [ ] `pnpm dev` dans `frontend-studio` ouvre `http://localhost:5173` avec la page d'accueil
- [ ] `cargo test` passe (même vide)
- [ ] `pnpm test` passe (même vide)
- [ ] Un commit sur `main` déclenche la CI et elle passe au vert
- [ ] Un nouveau développeur peut cloner et lancer en < 10 min en suivant le `README.md`

## ⚠️ Pièges à éviter

- ❌ Ne pas installer Tailwind v3 par habitude — la v4 change la config
- ❌ Ne pas oublier `pgvector` dans l'image Docker (sinon les migrations planteront plus tard)
- ❌ Ne pas commit `.env` (ajouter au `.gitignore` dès le départ)
- ❌ Ne pas partir sur un monorepo `turborepo` complexe : `pnpm workspaces` suffit au MVP

## 📚 Références
- [tech-stack.md](../tech-stack.md)
- [data-model.md](../data-model.md) (migrations viendront en phase 2)
