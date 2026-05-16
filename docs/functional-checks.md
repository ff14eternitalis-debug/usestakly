# UseStakly — Checklist tests fonctionnels

> Version : 1.0 — 2026-05-03
> Usage : check-list go / no-go avant un déploiement, après une refacto sensible, ou pour valider une instance locale fraîchement ingérée. Couvre les flows visibles utilisateur et MCP. Pour le détail "comment lancer la stack", voir `docs/dev-workflow.md`. Pour la lecture conceptuelle des parcours, voir `docs/user-journey.md`.

Format : chaque check est un test minimal **un seul outil ou une seule action**, avec une condition de réussite observable. Si tu coches tout, le produit est fonctionnel sur ses promesses publiques. Un échec n'est pas forcément bloquant — annoter dans la PR concernée.

---

## A. Stack & santé

- [ ] **A1 — Backend up** : `curl http://localhost:4000/health` renvoie `200` avec un JSON contenant `status: "ok"`.
- [ ] **A2 — Status public** : `curl http://localhost:4000/api/status/public` renvoie `200` avec `seedRepoCount > 0` et `formulaVersion = "v2.0"`.
- [ ] **A3 — Frontend build** : `npm run build` côté `frontend/` termine sans erreur TypeScript.
- [ ] **A4 — Frontend up** : `http://localhost:5173/` charge la landing et affiche le CTA `Discover`.
- [ ] **A5 — Migrations appliquées** : `psql ... -c "SELECT version FROM _sqlx_migrations ORDER BY version DESC LIMIT 5"` renvoie au moins jusqu'à la migration `0022`.

## B. Auth & dev user

- [ ] **B1 — Dev user actif sans secrets** : sans `APP_SESSION_SECRET` ni OAuth credentials, `GET /api/me` renvoie `usestakly-dev`.
- [ ] **B2 — OAuth GitHub round trip** (si configuré) : clic `Sign in with GitHub` → consent → retour sur `/` avec session `usestakly_session` posée et `GET /api/me` renvoie l'identité GitHub.
- [ ] **B3 — Return-to signé** : visite `/watchlist` non connecté → redirection `/login?return_to=...`, login → atterrissage sur `/watchlist` (et non sur `/`).
- [ ] **B4 — Sign out** : depuis `/account`, clic sign out → cookie effacé, `/api/me` rebascule sur dev user.

## C. Discover

- [ ] **C1 — Search lexicale** : `/discover` mode Reliable, query `react table` retourne au moins un repo avec score affiché et badge formula version.
- [ ] **C2 — Filtres** : combinaison `language=TypeScript` + `stars_min=1000` + `filter=auto` retourne uniquement des repos TS avec ≥ 1000 stars, sans flags sévères.
- [ ] **C3 — Mode Radar emerging** : toggle Radar → resultats incluent des repos avec `maturity_band` ∈ {`emerging`, `experimental`} et `sort=trend`.
- [ ] **C4 — Use case search** : taper "ORM TypeScript fiable" dans `UseCaseSearchPanel` → l'intention détectée affiche `categories: [database, orm]` ou similaire, et la shortlist contient au moins un repo connu (Prisma, Drizzle, TypeORM).
- [ ] **C5 — Add repo** : connecté, soumettre `owner/repo` → ingestion réussie, redirection `/repos/$id`, score visible.

## D. Repo detail

- [ ] **D1 — Profil chargé** : `/repos/{id}` d'un repo seedé affiche les 5 dimensions (freshness, adoption, reliability, abandonment, vitality) avec valeurs numériques.
- [ ] **D2 — Vitality breakdown** : la tuile vitality affiche les 4 sous-signaux (contributors 90d, commits 30d, CI, dernier release) **OU** la mention "not yet captured" si `structural_signals_at` est NULL.
- [ ] **D3 — Provenance** : le bloc score affiche `formula_version = "v2.0"`, `scored_at`, et `source: usestakly://...`.
- [ ] **D4 — Radar maturity** : le `RepoHeader` affiche la bande maturity (established / emerging / experimental / stale / noisy) avec une explication courte.
- [ ] **D5 — Watch toggle** : connecté, clic Watch → repo apparaît dans `/watchlist` section Repos.

## E. Watchlist

- [ ] **E1 — Liste repos** : `/watchlist` connecté affiche les repos watchés avec score actuel et tendance.
- [ ] **E2 — Section Besoins** : si au moins une use case watch créée, la section `Besoins` apparaît avec le label et le top match.
- [ ] **E3 — Mute / Unmute** : toggle mute sur un repo persiste après refresh.
- [ ] **E4 — Confirm remove** : remove demande confirmation puis retire le repo (`E1` ne le liste plus).
- [ ] **E5 — État vide cohérent** : sans aucun watch, l'écran montre l'empty state avec CTA `Discover` (pas un faux empty sur 401).

## F. Notifications

- [ ] **F1 — Liste** : `/notifications` connecté affiche les notifs unread + read en deux groupes.
- [ ] **F2 — Mark-read on click** : cliquer le lien repo d'une notif unread → `markRead` est déclenché avant la nav, le badge unread décrémente.
- [ ] **F3 — Génération sur changement** : déclencher `POST /api/admin/scoring/recompute` après une modif de signal sévère sur un repo watché → une notif `score_drop` (ou équivalent) est créée.
- [ ] **F4 — Erreur réseau** : couper le backend, recharger `/notifications` → message d'erreur dédié + bouton retry, pas de faux empty state.

## G. Account & MCP tokens

- [ ] **G1 — Création token** : `/account` connecté, créer un token avec label `smoke-test` → plaintext `usk_<64 hex>` affiché une fois.
- [ ] **G2 — Liste tokens** : le nouveau token apparaît avec son label et sa date.
- [ ] **G3 — Révocation** : delete sur le token → disparaît de la liste, `GET /api/agent-tokens` ne le renvoie plus.
- [ ] **G4 — Réputation** : la `ReputationCard` affiche le score réputation user et le seuil min pour signaux actifs.
- [ ] **G5 — Modération admin** (avec `x-admin-token`) : la file pending/disputed se charge et permet review.

## H. MCP

- [ ] **H1 — Auth requise** : `curl -X POST http://localhost:4000/mcp -d '{"jsonrpc":"2.0","id":1,"method":"initialize",...}'` **sans** Bearer renvoie `401` ou équivalent (middleware pré-transport).
- [ ] **H2 — Initialize avec token** : même requête avec `Authorization: Bearer usk_...` valide renvoie une réponse MCP `result` avec `serverInfo`.
- [ ] **H3 — `tools/list`** : retourne 6 tools : `search_github_repos`, `recommend_github_repos`, `get_repo_quality_context`, `log_usage`, `watch_repo`, `watch_use_case`.
- [ ] **H4 — `search_github_repos`** : paramètres `{query: "react table", limit: 5}` renvoie une shortlist avec `quality_overall`, `radar` (si snapshot), et `provenance.formula_version = "v2.0"`.
- [ ] **H5 — `get_repo_quality_context`** : `{owner: "TanStack", name: "table"}` (ou un repo seedé) renvoie le profil complet avec dimensions, flags, recent_signals.
- [ ] **H6 — `recommend_github_repos`** : `{need: "TypeScript ORM"}` renvoie au moins un candidat avec `reasons`, `caveats`, `stable_picks` / `emerging_picks` et `fallback_candidates`.
- [ ] **H7 — `log_usage`** : `{owner, name, outcome: "build_success"}` réussit, retourne le score recalculé, et un `quality_signal` est persisté côté DB.
- [ ] **H8 — `watch_repo`** : `{owner, name}` ajoute le repo à la watchlist du user propriétaire du token, visible ensuite dans `/watchlist`.
- [ ] **H9 — `watch_use_case`** : `{need: "testing tools for TypeScript"}` crée une veille d'intention visible ensuite dans `/watchlist` section `Besoins`.
- [ ] **H10 — Quota write** : 30 `log_usage` rapides avec le même token → les derniers sont refusés (`mcp_guard_rejection` dans `agent_token_events`).

## I. CLI MCP

- [ ] **I1 — Install** : `npx usestakly-mcp install` propose la liste des clients (Codex / Cursor) et écrit le bon header `Authorization: Bearer usk_...`.
- [ ] **I2 — Endpoint configurable** : passer `--endpoint http://localhost:4000/mcp` écrit la bonne URL (pas le hardcoded prod).
- [ ] **I3 — Test** : `npx usestakly-mcp test` réussit avec un token valide et échoue clairement avec un token bidon.

## J. Garde-fous trust / signaux

- [ ] **J1 — Signal non-éligible rejeté** : un user avec réputation sous le seuil tente `POST /api/repos/:id/signals` sur `security_issue` → réponse `pending` ou refus.
- [ ] **J2 — Consensus N users** : un seul signal `deprecated` sur un repo n'apparaît pas dans `artifact_scores.flags` ; après N signaux distincts éligibles, il s'y trouve.
- [ ] **J3 — Dispute owner** : un OAuth GitHub matchant l'owner d'un repo flag peut soumettre une dispute (transition logguée dans `quality_signal_events`).
- [ ] **J4 — Audit endpoint** : `GET /api/admin/scoring/explain/{repo_id}` (avec admin token) renvoie le breakdown signal par signal avec `outcome_weight × reporter_weight × dedup_weight`.

## K. Live / staging

Réutilise les sections **A–J** ci-dessus en remplaçant `http://localhost:4000` par l'URL API cible (ex. `https://api.usestakly.com`) et le frontend par l'URL publique (ex. `https://usestakly.com`). Gate ordonné : [`docs/validation/live-release-checklist.md`](validation/live-release-checklist.md).

### Variables d'environnement (cible deploy)

- [ ] **K0 — Secrets présents** : `APP_SESSION_SECRET`, `FRONTEND_BASE_URL` (CORS), credentials OAuth GitHub si auth publique, `GITHUB_TOKEN` pour ingestion, token monitoring `usk_…` (création via **G1** sur l'env cible).
- [ ] **K1 — Health prod** : équivalent **A1–A2** sur l'API déployée (`/health`, `/api/status/public` avec `seedRepoCount > 0`, `formulaVersion = "v2.0"`).
- [ ] **K2 — OAuth prod** (si activé) : **B2–B4** dans le navigateur sur l'URL frontend de l'environnement.
- [ ] **K3 — Discover / repo** (spot) : **C1**, **D1–D3** sur au moins un repo du corpus seedé.
- [ ] **K4 — MCP smoke automatisé** : `.\scripts\mcp-live-smoke.ps1 -Endpoint "https://api…/mcp" -Token "usk_…"` — couvre **H2**, **H4**, **H5** ; **H7** uniquement avec `-WriteSignal` (écrit un vrai signal).
- [ ] **K5 — MCP manuel complémentaire** (optionnel) : **H3**, **H6**, **H8–H10** et CLI **I1–I3** si le release touche MCP ou le package npm.
- [ ] **K6 — Données après deploy** (staging ou prod contrôlée) : après `log_usage` volontaire, ligne dans `quality_signals` ; après `POST /api/admin/scoring/recompute`, `artifact_scores.computed_at` avance pour le repo concerné.

Local full-stack sans ce script MCP : `cd frontend && npm run test:e2e:real` — voir [`docs/dev-workflow.md`](dev-workflow.md#e2e-réel-local-sans-mocks).

---

## Quand cette checklist tombe en faux positif

Si un check rate alors que le code n'a pas changé sur ce flow, vérifier dans l'ordre :

1. Migrations bien jouées (`A5`).
2. Env vars : `APP_SESSION_SECRET`, `GITHUB_TOKEN`, `APP_SCHEDULER_ENABLED`, `APP_SEMANTIC_SEARCH_ENABLED`.
3. Corpus seedé (`A2` `seedRepoCount > 0`).
4. Cookie session présent (DevTools → Application → Cookies).
5. Pour MCP : token actif et non révoqué dans `agent_tokens`.

Cette checklist est volontairement **non automatisée** sauf **K4** (smoke MCP) et le parcours local `test:e2e:real`. Pour les tests E2E CI, voir `frontend/e2e/mvp.spec.ts` (couverture partielle — Phase R7 dans `docs/plans/remaining-work-2026-05-03.md`).
