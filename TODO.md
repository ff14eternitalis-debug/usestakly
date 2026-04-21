# UseStakly — TODO MVP

> Version : 4.0 — 2026-04-20
> **Angle produit retenu** : registry qualité-scored pour agents IA — `docs/strategy-quality-scored-registry.md`
> Référence architecture : `docs/plans/mvp-one-shot-blueprint.md` (à reconcilier avec le pivot)
> Référence exécution : `docs/plans/mvp-file-by-file-checklist.md` (idem)

> `UseStakly` est le nom produit retenu. `Project-K` reste l'ancien nom de travail encore présent dans certaines surfaces techniques.

## ✅ Décision stratégique (2026-04-20)

Pivot acté : on part sur le **registry qualité-scored** — les agents IA filtrent par signaux d'usage réel (résolutions, builds, regrets, flags) plutôt que par popularité ou stars.

Implications structurelles :
- **Phases 0–5** : acquis, inchangés.
- **Phase 6 (Quality signals)** : nouvelle phase, prioritaire — capter la data dès le jour 1 est irréversible.
- **Phase 7 (Search / Resolve)** : enrichie d'un param `filter` (auto / strict / explore).
- **Phase 8 (MCP)** : `get_snippet` renvoie `quality_context` natif, provenance signée `slug@v + score@t`.
- **Phase 9 (Safety / Quality gates)** : remonte au cœur produit — politiques de flags toxiques, process modéré security.
- **Phase 10 (Bootstrap corpus)** : seed curé 200–500 snippets + priors externes pour éviter cold start brutal.

### Décisions de scope

Tranchées (2026-04-21) :

- [x] **Annotation code public dès MVP — mode hybride lazy.** Référencement par slug canonique (`npm:request@2.88`, `github:owner/repo@sha`), priors externes (downloads, last_commit, stars) ingérés **à la demande** via MCP puis cachés. Pas de table miroir de tout npm. Débloquer la démo killer Phase 10 sans la machinerie d'ingestion massive.
- [x] **Formule de scoring publique et versionnée.** Poids exacts dans `backend/scoring/formula_v1.toml`, chaque score stocké tagué `formula_version`. Anti-gaming via réputation owner + evidence obligatoire, pas via opacité. Transparence > obscurité — un agent doit pouvoir expliquer pourquoi il rejette un artefact.

Encore à trancher :

- [ ] Extension IDE / CLI en parallèle du MCP, ou tout miser sur MCP ? (reportable en Phase 10)
- [ ] Seed corpus : qui cure les 200 premiers snippets, selon quels critères ? (reportable en Phase 10)

## Mode d'emploi

Cette checklist ne remplace pas les documents détaillés.
Elle sert de feuille de route courte pendant l'implémentation.

Ordre recommandé :
1. `docs/strategy-quality-scored-registry.md` (angle produit)
2. `docs/plans/mvp-one-shot-blueprint.md` (à relire avec lunettes pivot)
3. `docs/plans/mvp-file-by-file-checklist.md` (idem)
4. cette checklist

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
- [x] Retirer `frontend/src/lib/supabase.ts` et la dépendance `@supabase/supabase-js` du `package.json` (décision VPS, plus d'auth SaaS)
- [x] Créer stores Zustand minimaux

## Phase 4 — Auth

> App auto-hébergée sur VPS → **OAuth direct côté backend**, pas de Supabase / SaaS d'auth.

- [x] Implémenter OAuth GitHub (callback + session JWT cookie)
- [x] Implémenter OAuth Discord (callback + session JWT cookie)
- [x] Implémenter synchronisation `users` / `auth_identities`
- [ ] Ajouter UI login GitHub / Discord réelle dans le frontend
- [x] Connecter `GET /api/me` à l'utilisateur courant en mode dev temporaire

## Phase 5 — Libraries / Snippets

- [x] Implémenter modèles métier libraries
- [x] Implémenter CRUD libraries
- [x] Implémenter modèles métier snippets
- [x] Implémenter CRUD snippets
- [x] Implémenter versioning append-only
- [x] Implémenter références canoniques

## Phase 6 — Quality signals (fondation)

> Prérequis de tout le reste — toutes les phases suivantes consomment ce schéma et cette télémétrie.

- [x] Migration `0010_quality_signals.sql` — `external_artifacts`, `quality_signals` polymorphique, `artifact_scores` tagués `formula_version`
- [x] `scoring/formula_v1.toml` — dimensions et poids publics et versionnés
- [x] Domain `quality` + service `capture::record_signal` (evidence enforcement par `SignalKind`)
- [x] Endpoint `POST /api/snippets/:id/signals` — signaux actifs uniquement (les passifs viendront du MCP)
- [x] Service `scoring::recompute_all_scores` (formule pure + agrégation SQL, upsert sur `formula_version`)
- [x] Endpoint admin `POST /api/admin/scoring/recompute` (gardé par `ADMIN_API_TOKEN`)
- [x] Tests unitaires sur les fonctions pures du scoring (5/5)
- [ ] Capture passive au `resolve_reference` — bloquée sur Phase 8 (MCP)
- [ ] Pondération réputation owner (anti-gaming : compte neuf = poids 0) — prévue formula_v2
- [ ] Politique de flags toxiques (`deprecated`, `broken-on-X`, `security-issue`) : evidence + consensus N users + appel auteur

## Phase 7 — Search / Resolve (avec filtres qualité)

- [x] Parser `@library:snippet@version` (`domain::reference::parse_reference`, pur + testé)
- [x] Implémenter `/api/resolve` — JOIN `artifact_scores`, auth optionnelle (public + own)
- [x] Implémenter `/api/search?q=&filter=auto|strict|explore`
  - `auto` : `reliability >= 0.9 AND abandonment <= 0.3 AND flags NOT IN (security-issue, broken)`
  - `strict` : `reliability >= 0.95 AND abandonment <= 0.2 AND overall >= 0.85 AND flags = []`
  - `explore` : aucun filtre, snippets non scorés inclus
- [ ] Recherche hybride (lexical + vector) — vector reporté (fastembed)
- [ ] Embedding `fastembed` (local)
- [ ] Signal `stack-match` calculé au moment de la query (contre `package.json` / `Cargo.toml` client)

## Phase 8 — MCP (avec quality context natif)

- [ ] Ajouter route MCP
- [ ] Implémenter `resolve_reference` — logge outcome pour `resolve_count`
- [ ] Implémenter `search_library` avec param `filter`
- [ ] Implémenter `get_snippet_with_quality_context` — renvoie score multi-dim + flags + provenance signée `slug@v + score@t`
- [ ] Implémenter `check_dependencies`
- [ ] Implémenter `assemble_plan`
- [ ] Implémenter `log_generation` avec outcome post-insertion (T+1h) pour alimenter `build_success_rate` / `regret_rate`

## Phase 9 — Safety / Quality gates (cœur produit)

- [ ] Sanitize de contenu texte
- [ ] Classification de risque
- [ ] Exclure `flagged` / `quarantined` en mode `auto` (par défaut du MCP)
- [ ] Provenance obligatoire dans toute réponse MCP (`// Assemblé depuis: slug@v, score: X.XX`)
- [ ] Process modéré pour `security-issue` (pas de publication avant validation, historique transparent)
- [ ] Graphe Sybil-resistant basé sur OAuth GitHub/Discord (pondération par historique)

## Phase 10 — Bootstrap corpus

> Contre le cold start : sans 6 mois d'usage, les signaux sont faibles. Nécessite une phase de seed soutenue.

- [ ] Curer 200–500 snippets seed high-quality (critères à définir — cf. décision de scope)
- [ ] Import de priors externes (stars GitHub, downloads npm, freshness des repos) comme base de scoring jour 1
- [ ] Démo killer : agent qui refuse `request@2.88` pour cause de `abandonment: 0.92` en mode `auto` (avant/après visible en 30 s)

## Phase 11 — Validation

- [x] Lancer `cargo check`
- [ ] Lancer `cargo test`
- [x] Lancer `npm install`
- [x] Lancer `npm run build`
- [ ] Vérifier le flow local complet (auth → création snippet → signal passif loggé → search avec filter → MCP resolve avec quality_context)
