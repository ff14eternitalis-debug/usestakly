# UseStakly — Stack Technique

> Version : 1.0 — 2026-04-15

## 🎯 Résumé

| Couche | Choix | Raison principale |
|---|---|---|
| Backend core | **Rust** + Axum + Tokio | Performance MCP + sécurité mémoire |
| Base de données | **PostgreSQL** + pgvector | SQL standard + recherche sémantique |
| Recherche vectorielle | **pgvector** + `fastembed` (local) | Zéro clé API, zéro coût variable |
| Frontend | **React 19** + **Tailwind CSS v4** + TypeScript | Écosystème UI + typage strict |
| Build frontend | **Vite** | Rapidité, standard moderne |
| State | **Zustand** | Minimaliste, pas de boilerplate Redux |
| Code editor | **Monaco** ou **Sandpack** | Sandpack pour preview live |
| Monorepo | **pnpm workspaces** + **Cargo workspaces** | Gestion multi-langages |
| Containerisation | **Docker** + **docker-compose** | Dev local reproductible |
| CI/CD | **GitHub Actions** | Gratuit, standard |
| Auth | **GitHub + Supabase Auth** (MVP) | Onboarding développeur rapide sans auth maison |

## 🦀 Backend — Rust

### Pourquoi Rust ?
- Le serveur MCP analyse potentiellement des milliers de snippets → **latence minimale**
- Manipulation de code source sensible → **sécurité mémoire** native
- `tree-sitter`, `fastembed`, `sqlx` : écosystème complet pour notre usage
- Compilation statique → déploiement simple (un binaire)

### Dépendances clés
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
sqlx = { version = "0.8", features = ["postgres", "uuid", "chrono", "runtime-tokio-rustls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
tree-sitter = "0.22"
fastembed = "4"
argon2 = "0.5"
jsonwebtoken = "9"
tracing = "0.1"
anyhow = "1"
thiserror = "1"
```

## 🎨 Frontend — React + Tailwind

### Pourquoi React + Tailwind ?
- **Tailwind v4** : utility-first, zéro CSS custom à maintenir, parfait pour enforcer les RULES
- **React 19** : Server Components, Actions, écosystème le plus large
- **TypeScript strict** : les types partagés (`shared-type-*`) sont la source de vérité

### Dépendances clés
```json
{
  "dependencies": {
    "react": "^19",
    "react-dom": "^19",
    "zustand": "^5",
    "@tanstack/react-query": "^5",
    "@tanstack/react-router": "^1",
    "@codesandbox/sandpack-react": "^2",
    "@monaco-editor/react": "^4",
    "lucide-react": "^0"
  },
  "devDependencies": {
    "vite": "^6",
    "typescript": "^5.6",
    "tailwindcss": "^4",
    "@vitejs/plugin-react": "^4",
    "vitest": "^2"
  }
}
```

## 🗄️ Base de données — PostgreSQL + pgvector

- Standard, portable, hébergeable partout (local, Supabase, Neon, self-host)
- `pgvector` : recherche sémantique dans la même DB que le reste — pas de vector DB séparée à maintenir
- `JSONB` pour les règles et métadonnées évolutives
- Migrations via `sqlx migrate`

## 🏗️ Structure monorepo

```
PROJET_K/
├── backend-core/            # Rust
│   ├── src/
│   │   ├── mcp_server/
│   │   ├── parser/
│   │   ├── storage/
│   │   └── main.rs
│   ├── migrations/          # sqlx
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend-studio/         # React + Tailwind
│   ├── src/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── services/
│   │   └── App.tsx
│   ├── package.json
│   └── vite.config.ts
├── shared/
│   ├── types/               # TS + Rust via Serde
│   ├── rules/               # JSON
│   └── prompts/             # Prompts MCP versionnés
├── deploy/
│   ├── docker-compose.yml
│   └── github-actions/
├── docs/
├── TODO.md
└── README.md
```

## 🔑 Authentification & secrets

- **GitHub + Supabase Auth** pour le MVP
- **JWT validés côté backend Rust** au lieu d'une auth maison
- Secrets en variables d'environnement via `.env` (jamais commité) + `dotenvy` en dev
- Production : secrets injectés par le runtime Coolify

## 🚀 Hébergement cible (MVP)

| Composant | Hébergeur suggéré | Coût estimé |
|---|---|---|
| Backend Rust | Coolify | dépend du serveur |
| PostgreSQL | Coolify PostgreSQL managé | dépend du serveur |
| Frontend | Coolify | dépend du serveur |
| DNS + CDN | Cloudflare | 0 $ |

Total MVP : dépend surtout du serveur Coolify retenu, avec une architecture plus simple à opérer.

## ❌ Choix explicitement écartés

| Écarté | Raison |
|---|---|
| Node.js pour le backend | Moins performant, typage moins strict que Rust |
| MongoDB | Les relations snippets/versions/tags sont relationnelles par nature |
| Vector DB dédiée (Pinecone, Weaviate) | pgvector suffit et évite un service en plus |
| Next.js App Router | Pas besoin de SSR complexe, Vite SPA suffit pour le studio |
| Redux / MobX | Zustand couvre nos besoins avec moins de boilerplate |
| GraphQL | REST/JSON suffit au MVP, moins de complexité |
| OpenAI embeddings (cloud) | `fastembed` local = 0 coût + 0 dépendance externe |
