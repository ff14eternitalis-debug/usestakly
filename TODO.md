# UseStakly — TODO MVP

> Version : 2.0 — 2026-04-18
> Référence principale : `docs/plans/mvp-one-shot-blueprint.md`
> Référence exécution : `docs/plans/mvp-file-by-file-checklist.md`

> `UseStakly` est le nom produit retenu. `Project-K` reste l'ancien nom de travail encore présent dans certaines surfaces techniques.

## Mode d'emploi

Cette checklist ne remplace pas les documents détaillés.
Elle sert de feuille de route courte pendant l'implémentation.

Ordre recommandé :
1. `docs/plans/mvp-one-shot-blueprint.md`
2. `docs/plans/mvp-file-by-file-checklist.md`
3. cette checklist

---

## Phase 0 — Repo

- [x] Doc centralisée dans `docs/`
- [x] Ajouter `.gitignore`
- [x] Ajouter `.editorconfig`
- [x] Ajouter `.env.example`
- [x] Ajouter `docker-compose.yml`
- [x] Ajouter CI GitHub Actions initiale
- [x] Mettre à jour le `README.md`

## Phase 1 — Backend bootstrap

- [x] Créer `backend/Cargo.toml`
- [x] Créer `backend/src/main.rs`
- [x] Créer `backend/src/app/mod.rs`
- [x] Créer `backend/src/config/mod.rs`
- [x] Créer `backend/src/db/mod.rs`
- [x] Créer `backend/src/telemetry/mod.rs`
- [x] Créer route `GET /health`
- [x] Créer route `GET /api/me`

## Phase 2 — Base de données

- [x] Ajouter migration `0001_init_extensions.sql`
- [x] Ajouter migration `0002_users_auth.sql`
- [x] Ajouter migration `0003_libraries.sql`
- [x] Ajouter migration `0004_snippet_kinds.sql`
- [x] Ajouter migration `0005_snippets.sql`
- [x] Ajouter migration `0006_snippet_versions.sql`
- [x] Ajouter migration `0007_tags_and_rule_sets.sql`
- [x] Ajouter migration `0008_permissions_and_reports.sql`
- [x] Ajouter migration `0009_generations_and_indexes.sql`

## Phase 3 — Frontend bootstrap

- [x] Créer `frontend/package.json`
- [x] Créer `frontend/vite.config.ts`
- [x] Créer `frontend/tsconfig.json`
- [x] Créer `frontend/index.html`
- [x] Créer `frontend/src/main.tsx`
- [x] Créer `frontend/src/app/providers.tsx`
- [x] Créer `frontend/src/components/layout/AppShell.tsx`
- [x] Créer `frontend/src/lib/api-client.ts`
- [x] Créer `frontend/src/lib/supabase.ts`
- [x] Créer stores Zustand minimaux

## Phase 4 — Auth

- [ ] Implémenter validation JWT Supabase côté backend
- [ ] Implémenter synchronisation `users` / `auth_identities`
- [ ] Ajouter UI login GitHub réelle
- [x] Connecter `GET /api/me` à l'utilisateur courant en mode dev temporaire

## Phase 5 — Libraries / Snippets

- [x] Implémenter modèles métier libraries
- [x] Implémenter CRUD libraries
- [x] Implémenter modèles métier snippets
- [x] Implémenter CRUD snippets
- [x] Implémenter versioning append-only
- [x] Implémenter références canoniques

## Phase 6 — Search / Resolve

- [ ] Parser `@library:snippet@version`
- [ ] Implémenter `/api/resolve`
- [ ] Implémenter `/api/search`
- [ ] Ajouter recherche hybride
- [ ] Ajouter embedding `fastembed`

## Phase 7 — MCP

- [ ] Ajouter route MCP
- [ ] Implémenter `resolve_reference`
- [ ] Implémenter `search_library`
- [ ] Implémenter `get_snippet`
- [ ] Implémenter `check_dependencies`
- [ ] Implémenter `assemble_plan`
- [ ] Implémenter `log_generation`

## Phase 8 — Safety

- [ ] Ajouter sanitize de contenu texte
- [ ] Ajouter classification de risque
- [ ] Exclure `flagged` / `quarantined` en mode `auto`
- [ ] Ajouter provenance obligatoire dans les résultats MCP

## Phase 9 — Validation

- [x] Lancer `cargo check`
- [ ] Lancer `cargo test`
- [x] Lancer `npm install`
- [x] Lancer `npm run build`
- [ ] Vérifier le flow local complet
