# UseStakly — TODO MVP

> Version : 5.0 — 2026-04-21
> **Pivot produit acté** : on abandonne la bibliothèque de snippets.
> Nouveau produit : **outil de veille GitHub qui réduit le bruit des stars et offre un vrai suivi des repos publics OSS**.
> Référence : `docs/strategy-pivot-2026-04-21.md` (scope) et `docs/strategy-quality-scored-registry.md` (moat et principes, toujours valides).
> Business model : voir `docs/business/business-model-exploration.md` (privé, gitignore).

---

## Vision produit recentrée

Deux fonctions noyau :

1. **Discovery qualité-scored** — un dev cherche un outil (ex: « date picker React timezone »), UseStakly retourne les repos GitHub scorés par **usage réel** (reliability, abandonment, freshness), pas par stars. Réduit le bruit de la recherche GitHub où les repos hypés masquent les vrais choix techniques.
2. **Suivi des repos** — un dev met des repos dans sa watchlist et est notifié quand un score bouge significativement (abandonment up, nouveau flag `security-issue`, maintainer silencieux 90 j, etc.). GitHub ne fournit pas ce suivi qualité.

Les agents IA consomment la même data via MCP.

---

## ⚠ Ce qui est retiré du produit

| Retiré | Devenir |
|---|---|
| Libraries / snippets (CRUD, UI, discovery publique) | Schéma DB conservé (tables non droppées), **endpoints + UI dormants**. Réactivables plus tard si un use-case émerge. |
| Tier Team / registry privé | Retiré du roadmap. Le scope est 100 % repos GitHub publics OSS. |
| Couverture npm / crates.io / shadcn | Reporté. MVP = GitHub uniquement. |
| Seed corpus manuel de 200–500 snippets (ancienne Phase 10) | Remplacé par ingestion automatique des repos GitHub. |

## ✅ Acquis techniques réutilisables (phases 0–6)

Ce qui est déjà fait est **agnostique au scope** et reste pertinent :

- Repo + CI + Postgres + Docker compose (Phase 0)
- Backend bootstrap Axum + SQLx + config + migrations (Phase 1)
- Migrations 0001–0009 (users, libraries, snippets, versions, generations) — certaines deviennent dormantes, aucune à rollback
- **Auth OAuth GitHub + Discord avec session JWT cookie** (Phase 4) — essentielle pour le nouveau flow (watchlist, réputation)
- **Migration 0010 `quality_signals`** avec table `external_artifacts`, `quality_signals`, `artifact_scores` — **exactement** ce qu'il faut pour le nouveau produit
- **`scoring/formula_v1.toml`** + service `capture::record_signal` + service `scoring::recompute_all_scores` — gardent leur valeur, cible à ajuster
- **Endpoint `POST /api/snippets/:id/signals`** — sera refactoré pour viser `external_artifacts`
- **Endpoints `/api/resolve` et `/api/search` avec filter auto/strict/explore** — filtres OK, source à repointer sur `external_artifacts`
- **Endpoint admin `/api/admin/scoring/recompute`** — OK
- Audit sécu commit `4e16c0a` validé (voir `docs/security-audit-2026-04-21.md`)

Frontend (Phase 3) : le shell, l'auth, le theme, la providers tree **restent utiles**. Les vues libraries/snippets sont à retirer (Phase R6).

---

## Nouvelles phases (R = refactor vers le pivot)

### Phase R1 — Ingestion GitHub ⏫ PRIORITÉ

Pipeline neuf. C'est le cœur du nouveau produit : sans repos ingérés, rien à scorer.

- [ ] Migration `0011_github_artifacts.sql` — colonnes GitHub-specific sur `external_artifacts` (owner, name, default_branch, stars, forks, license, archived, language, last_commit_at, open_issues_count)
- [ ] Service `ingestion::github` — client REST (octocrab ou reqwest direct), auth via GitHub App (préféré) ou PAT fallback
- [ ] Rate-limit handling : conditional requests (ETags), backoff, quota monitoring
- [ ] Ingestion priors snapshot : stars, forks, subscribers, last_commit_at, open_issues, archived, language, license
- [ ] Computation priors dérivés : `freshness` (via `last_commit_at` + formula), `owner_inactive_days` (via events API)
- [ ] Refresh cadence : daily par défaut, horaire pour repos watchés
- [ ] Critère de corpus v1 : **à trancher** — top N par langage, sur demande, ou quand un user watch ?
- [ ] Endpoint admin `POST /api/admin/ingest/github` pour backfill ciblé
- [ ] Endpoint `POST /api/repos/add` — user propose un repo à ingérer
- [ ] Tests unitaires sur parsing réponses GitHub
- [ ] Mapping `github.com/owner/repo` → UUID `external_artifact_id` (idempotent)

### Phase R2 — Discovery qualité-scored

Remplace la search snippets par la search repos GitHub.

- [ ] `/api/search` repointé : cherche dans `external_artifacts` (repos GitHub), plus dans `snippets`
- [ ] Filtres existants conservés : `filter=auto|strict|explore` (définis dans formula_v1)
- [ ] Filtres nouveaux : langage, license, stars min/max, freshness min
- [ ] Recherche lexicale : ILIKE sur `name` + `description` + topics GitHub
- [ ] Recherche sémantique (Phase R2b) : `fastembed` local, embedding des descriptions, pgvector
- [ ] Ranking combiné : lexical + sémantique + score qualité
- [ ] Endpoint `GET /api/repos/:id` — profil complet (dimensions, flags, historique scores, derniers signaux)
- [ ] UX d'explication : « pourquoi ce repo est proposé, pourquoi request@2.88 est exclu en mode auto »

### Phase R3 — Watchlist & suivi

Le deuxième pilier. C'est ce qui manque sur GitHub aujourd'hui.

- [ ] Migration `0012_watchlists.sql` — `watchlists`, `watched_artifacts`, `notifications`
- [ ] Endpoints `/api/watchlist` — CRUD + ajouter / retirer un repo
- [ ] Détection de changement significatif : diff score T vs T-1, nouveaux flags, dernière activité owner
- [ ] Règles d'alerte défaut : abandonment +0.2, nouveau flag `security-issue` / `broken`, maintainer silencieux 90 j, score `overall` qui descend sous seuil
- [ ] Règles d'alerte custom par user (seuils ajustables, mute, digest weekly)
- [ ] Canal notification v1 : **in-app** seulement (centre de notifications côté frontend)
- [ ] Canal notification v2 : email (via service transactionnel), webhook pour devs avancés
- [ ] Worker cron : job quotidien qui diff les scores et génère les notifications
- [ ] Digest email hebdomadaire pour les watchers actifs

### Phase R4 — Signaux actifs / flags toxiques (cœur produit)

Gardé de Phase 6/9 v4, adapté aux repos GitHub publics.

- [ ] Endpoint `POST /api/repos/:id/signals` — refactor de `/api/snippets/:id/signals`, évidence obligatoire
- [ ] Politique flags toxiques — `deprecated`, `broken-on-X`, `security-issue` : evidence + **consensus N users distincts** avec réputation > seuil
- [ ] Processus modéré pour `security-issue` — publication retardée, appel possible par owner
- [ ] Appel / dispute par l'owner (via OAuth GitHub matching login)
- [ ] Historique transparent public des flags (timeline affichable sur le profil repo)
- [ ] Pondération réputation owner — formula_v2, compte neuf = poids 0, historique d'usage prod = surpondéré
- [ ] Graphe Sybil-resistant via OAuth GitHub (followers, contributions, âge compte)

### Phase R5 — MCP adapté aux repos

Plus des snippets — des repos GitHub.

- [ ] Route MCP SSE / WebSocket
- [ ] Auth agent : token dédié distinct de la session web (pour éviter qu'un client web détourné spamme `build_success`)
- [ ] Outil `search_github_repos(query, filter, language?, stack?)` → candidats scorés
- [ ] Outil `get_repo_quality_context(owner/repo)` → profil complet signé (`repo@sha + score@t + formula_version`)
- [ ] Outil `log_usage(repo, outcome)` → alimente `build_success_rate` et `regret_rate` passifs
- [ ] Outil `watch_repo(repo)` — ajoute à la watchlist du user agent
- [ ] Provenance obligatoire dans toutes les réponses : `// Evalué: github.com/owner/repo@sha, score: 0.92, formula_v1, t=...`

### Phase R6 — Refonte frontend complète

Le frontend actuel est centré snippets. À démolir en grande partie, à rebâtir autour de discovery + watchlist.

- [ ] **Garder** : shell global, providers, theme, auth flow OAuth, layout base
- [ ] **Retirer** (ou mettre en feature flag off) : pages libraries, pages snippets, discovery snippets publics, creator flow snippets
- [ ] **Nouveau** : landing orientée outil de veille — pitch, démo visible, CTA « watch your first repo »
- [ ] **Nouveau** : page recherche / discovery — barre de recherche + résultats scorés + filtres latéraux + explications du scoring
- [ ] **Nouveau** : page profil repo — header avec meta GitHub + barres de dimensions + liste flags + graph historique score + bouton `Watch` + timeline signaux
- [ ] **Nouveau** : dashboard watchlist — grid ou liste avec mini-score + diff récent + accès rapide au profil
- [ ] **Nouveau** : centre de notifications (in-app) — changements scores, nouveaux flags, digests
- [ ] **Nouveau** : page compte — réputation user (basée sur ses signaux validés), historique contributions, règles d'alerte perso
- [ ] TanStack Query à câbler (déjà installé) pour gérer le cache / revalidation des profils repos
- [ ] Router : rester sur hash routing ou basculer TanStack Router ? **à trancher**

### Phase R7 — Validation e2e

- [ ] Flow user : login OAuth → search « date picker react » → voit résultats scorés → ouvre profil repo → clique Watch → (24 h plus tard en simu) reçoit notif de changement
- [ ] Flow agent : MCP search → get_repo_quality_context → log_usage → signal propagé
- [ ] Tests E2E Playwright sur les flows critiques
- [ ] Vérification sécu : l'audit `docs/security-audit-2026-04-21.md` reste valide sur les briques existantes, **refaire un audit** après R1 + R2 + R3 (surface d'attaque étendue : clients GitHub externes, queue de notifications, webhook)

---

## Décisions encore ouvertes

- [ ] **R1** — critère de corpus initial : top N / sur demande / via watchlist uniquement ?
- [ ] **R3** — canal notification v1 : in-app suffit, ou email obligatoire dès le MVP ?
- [ ] **R5** — token agent : JWT dédié généré par l'user, ou OAuth device flow ?
- [ ] **R6** — router frontend : garder hash custom ou migrer TanStack Router ?
- [ ] **R6** — faut-il garder les vues libraries/snippets cachées (réactivables) ou les dégommer net ?
- [ ] **Intuition couche 2** : POC quand ? Jamais, post-MVP, post-traction ?

---

## Ordre d'exécution recommandé

1. **R1** (ingestion GitHub) — sans ça rien ne tourne
2. **R2** (search repos) — débloque la démo killer
3. **R6 partiel** (landing + search UI) — donne un produit visible aux users
4. **R3** (watchlist + notifs) — le deuxième pilier, passer du « je cherche une fois » à « je reviens »
5. **R5** (MCP) — débloque les signaux passifs, alimente le flywheel
6. **R4** (flags toxiques) — avant ouverture publique (sinon review-bombing immédiat)
7. **R6 reste** + **R7** (validation) — polish et launch
