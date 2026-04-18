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

- Backend : Rust, Axum, SQLx, PostgreSQL, pgvector
- Frontend : React 19, TypeScript, Vite, Tailwind CSS v4
- Auth : Supabase Auth avec GitHub
- Detection : tree-sitter + fastembed local

## Structure

```text
project-k/
├── backend/
├── frontend/
├── docs/
├── docker-compose.yml
└── .github/workflows/
```

## Quickstart local

```bash
docker compose up -d

cd backend
cargo run

cd ../frontend
npm install
npm run dev
```

## Documentation

Le centre de documentation est dans [`docs/`](./docs/).

Points d'entree recommandes :
- [docs/README.md](./docs/README.md)
- [docs/plans/mvp-one-shot-blueprint.md](./docs/plans/mvp-one-shot-blueprint.md)
- [docs/plans/mvp-file-by-file-checklist.md](./docs/plans/mvp-file-by-file-checklist.md)

## Suivi

- checklist historique : [TODO.md](./TODO.md)
- blueprint MVP : [docs/plans/mvp-one-shot-blueprint.md](./docs/plans/mvp-one-shot-blueprint.md)
