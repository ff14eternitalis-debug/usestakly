# UseStakly

> GitHub observability for OSS repos: discovery, watchlist, notifications, MCP, and moderated quality signals.

`UseStakly` est le nom produit actuel. `Project-K` reste le nom historique encore visible dans certains chemins, migrations et documents.

## Produit vivant

UseStakly n'est plus une bibliotheque de snippets. Le produit vivant est maintenant centre sur des **repos GitHub publics OSS** avec quatre briques principales :

- **discovery qualite-scored**
- **watchlist + notifications**
- **MCP pour agents**
- **signaux actifs moderes** (`deprecated`, `broken`, `security_issue`, etc.)

L'ancien produit snippets a ete retire du runtime. Le schema SQL historique existe encore seulement pour compatibilite de migration et comme archive technique.

## Ce qui est deja livre

- auth OAuth GitHub + Discord
- ingestion GitHub (`/api/repos/add` + endpoint admin de backfill)
- recherche de repos et page detail
- watchlist + notifications in-app
- MCP read + write (`search_github_repos`, `get_repo_quality_context`, `log_usage`, `watch_repo`)
- tokens agents en UI (`/account`)
- reputation utilisateur v1
- consensus multi-users avant exposition des flags publics
- review admin pour `security_issue`
- dispute owner GitHub avec audit trail

## Stack

- Backend : Rust 2024, Axum 0.8, SQLx 0.8, PostgreSQL
- Frontend : React 19, TypeScript, Vite 7, Tailwind v4, TanStack Router, TanStack Query, Zustand
- Auth : OAuth GitHub + Discord cote backend, cookie session `usestakly_session`
- MCP : `rmcp` Streamable HTTP sur `/mcp`
- Scoring : formule locale `scoring/formula_v1.toml`

## Quickstart local

Prerequis : Docker, Rust stable, Node 22, npm.

```bash
cp .env.example .env
docker compose up -d

cd backend
cargo run

cd ../frontend
npm install
npm run dev
```

Le backend tourne sur `http://127.0.0.1:4000` et le frontend Vite sur `http://localhost:5173`.

Sans `APP_SESSION_SECRET` ni `*_CLIENT_ID/SECRET`, l'OAuth est desactive et le backend retombe sur le dev user `DEV_USER_*`.

## Variables importantes

- `GITHUB_TOKEN` : requis pour l'ingestion GitHub et certaines verifications owner org
- `ADMIN_API_TOKEN` : requis pour les endpoints admin et la moderation
- `APP_MCP_WRITE_LIMIT_PER_HOUR` : quota write MCP par token
- `APP_ACTIVE_SIGNAL_MIN_REPUTATION` : seuil minimal avant signaux actifs
- `APP_ACTIVE_SIGNAL_DEFAULT_CONSENSUS` : consensus pour flags actifs standards
- `APP_ACTIVE_SIGNAL_SEVERE_CONSENSUS` : consensus pour flags severes

## Commandes utiles

| Contexte | Commande |
|---|---|
| Backend check rapide | `cd backend && cargo check` |
| Backend lint strict | `cd backend && cargo fmt --check && cargo clippy --all-targets -- -D warnings` |
| Backend tests | `cd backend && cargo test` |
| Frontend build | `cd frontend && npm run build` |

## Etat de securite

Le produit a maintenant :

- tokens MCP hashés
- quotas et cooldowns sur les writes MCP
- reputation minimale avant signaux actifs
- review admin pour `security_issue`
- dispute owner sans suppression silencieuse
- audit trail des transitions de signal

Voir [docs/security-audit-2026-04-21.md](./docs/security-audit-2026-04-21.md).

Limite actuelle importante :

- les repos d'organisation GitHub sont supportes pour la dispute owner via **membership public** seulement
- les memberships prives / roles fins d'organisation ne sont pas encore supportes

## Documentation

Pour reprendre rapidement le projet :

- [docs/README.md](./docs/README.md)
- [TODO.md](./TODO.md)
- [docs/mcp-protocol.md](./docs/mcp-protocol.md)
- [docs/security-audit-2026-04-21.md](./docs/security-audit-2026-04-21.md)

Les anciennes docs snippets ont ete deplacees dans `docs/archive/snippets/` et ne sont plus la source de verite du produit actuel.
