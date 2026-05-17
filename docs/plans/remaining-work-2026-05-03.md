# UseStakly — Reste à terminer (audit 2026-05-03)

> **Snapshot** : passe complète sur `TODO.md` v5.5 + `docs/plans/*` + état réel du code.
> Les faux positifs détectés (items marqués TODO mais déjà livrés) ont été corrigés directement dans les docs concernées (`TODO.md`, `docs/plans/source-of-truth-oss-radar-plan.md`, `docs/plans/use-case-recommendation-watch-plan.md`, `docs/plans/anti-slop-vitality-v2.md`).
>
> Last reconciled with code: 2026-05-17.
> Cette doc liste **uniquement** ce qui reste vraiment ouvert, organisé par priorité d'exécution.
> À mettre à jour à chaque vague de finition. Source de vérité opérationnelle = `docs/source-of-truth.md` + ce backlog priorisé. `TODO.md` reste une roadmap historique longue.

---

## Priorité 1 — Hardening avant ouverture publique large

Objectifs bloquants identifiés dans `docs/ops-mcp-coolify-hardening.md` et la section "Public beta" de `TODO.md`.

### 1.1 Ops MCP / DB

- [x] **Backup DB Coolify planifié** — livré 2026-05-06. Backup local quotidien sur `usestakly-postgres`, cron `0 2 * * *`, DB `usestakly`, rétention 7 jours / 7 backups, exécution manuelle `success`.
- [x] **Test de restore DB** — validé 2026-05-07 en local via Docker Desktop depuis `pg-dump-usestakly-1778119206.dmp`. Tables critiques restaurées (`users`, `external_artifacts`, `artifact_scores`, `agent_tokens`, `watched_artifacts`, `notifications`, `repo_categories`, `repo_radar_snapshots`) et migrations jusqu'à `22`. Le stockage offsite/S3 reste à décider.
- [ ] **Backup offsite S3** — plan `docs/plans/public-launch-hardening-2026-05-17.md` Task 1 ; local Coolify OK, copie hors VPS + restore test non faits.
- [x] **Hardening `POST /api/repos/{id}/refresh`** — livré 2026-05-17. Session obligatoire, table `repo_refresh_events` (migration 0029), limites DB user/repo, cooldown mémoire secondaire, UI auto-refresh seulement si connecté. Déployer + définir `APP_REPO_REFRESH_USER_LIMIT_PER_HOUR` en prod.
- [x] **Visibilité quota GitHub (logs L1)** — livré 2026-05-17. Logs structurés `x-ratelimit-*` + warnings quota bas / rate-limit ; runbook dans `docs/ops-mcp-coolify-hardening.md`. Endpoint admin quota et statut public dégradé : encore ouverts.
- [x] **Smoke UX public mobile/desktop** — validé manuellement en prod le 2026-05-17 (desktop + mobile, pas d'erreur console visible ni blocage layout signalé).
- [ ] **Email produit réel (watch/digest)** — Task 5 ; test canal OK, livraison inbox produit à prouver.
- [x] **Rate-limit applicative globale `/mcp`** — livré 2026-05-06. Limite par IP pour non-auth/invalides (`APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`) et limite par token valide pour le transport/reads (`APP_MCP_READ_LIMIT_PER_MINUTE`). Les writes gardent le quota par token existant via `agent_token_events`.
- [x] **Alerte externe** — livré 2026-05-07 avec Uptime Kuma : `UseStakly Website`, `UseStakly API Health`, `UseStakly Public Status`, `UseStakly MCP` authentifié avec token monitoring dédié.

### 1.2 Public beta gating

- [x] **Page légale `/legal`** — livrée 2026-05-07 avec mentions beta, MCP, data sources, licence et absence de garantie.
- [x] **Contact officiel affiché** — `contact@usestakly.com` ajouté dans le footer et la page légale. Reste à vérifier côté provider mail que l'adresse ou l'alias existe réellement.

---

## Priorité 2 — Chantiers entamés à finir

Items qui ont déjà du code ou des migrations en place mais ne sont pas terminés.

### 2.1 Notifications use case watches (Lot 3 du plan use-case) ✅ livré 2026-05-12

Migration 0020 + endpoints + UI livrés. Les notifications sont maintenant branchées via le scheduler.

- [x] Étendre `services/scheduler.rs` pour itérer les `use_case_watches.enabled = true`.
- [x] Recalculer les matches via `services/recommendations.rs::recommend_for_use_case`, comparer avec `use_case_watch_matches` persistés.
- [x] Émettre les 4 types de notifications définis dans le plan :
  - `use_case_new_candidate` — nouveau repo entre dans le top N
  - `use_case_best_candidate_changed` — meilleur repo change
  - `use_case_quality_drop` — repo du top baisse de score ≥ 0.10
  - `use_case_flag_added` — repo recommandé prend un flag toxique
- [x] Anti-bruit : max 1 notification batch par watch par jour au MVP, mise à jour de `last_notified_at`.

### 2.2 MCP cohérence intent (Lot 4 du plan use-case + Phase 4 du plan radar) ✅ livré 2026-05-06

- [x] Faire consommer au tool MCP `recommend_github_repos` le **même service** (`services/recommendations.rs`) que `POST /api/use-cases/recommend`, pour aligner les explications intent/categories/topics et la provenance.
- [x] Sortir la réponse en sections séparées dans le tool `recommend_github_repos` :
  - `stable_picks`
  - `emerging_picks`
  - `fallback_candidates`
- [x] Ajouter un tool MCP `watch_use_case` (équivalent du REST `POST /api/use-cases/watch`).

Reste côté MCP : valider un smoke réel avec token prod après déploiement, puis garder la doc d'exemples à jour quand de nouveaux patterns d'agents apparaissent.

### 2.3 Bug capture `last_release_at` (followup formula v2) ✅ livré 2026-05-16

- [x] Investiguer `services/ingestion/github.rs::fetch_releases_summary` : 4/25 repos seulement avaient `last_release_at` populé alors que tokio, vitest, eslint, rust, prisma ont des releases visibles. Livré 2026-05-16 : le fetch releases utilise maintenant un DTO local minimal (`published_at`) via `/repos/{owner}/{repo}/releases?per_page=100`, avec résumé testé et sélection du `published_at` le plus récent.
  - Vérification live GitHub 2026-05-16 sans token : `tokio-rs/tokio` → `2026-05-08T12:53:37Z`, `vitest-dev/vitest` → `2026-05-11T14:38:28Z`, `prisma/prisma` → `2026-04-22T14:19:23Z`.
  - Support ETag helper ajouté côté requêtes conditionnelles ; la persistance DB des ETags reste ouverte pour une future migration si le quota GitHub devient un vrai point chaud.

### 2.4 Phase R1 ingestion — finition

- [x] **Rate-limit handling GitHub** : livré partiellement le 2026-05-16. Les ETags releases/README/events sont persistés, les réponses `304 Not Modified` conservent les valeurs existantes, et les secondary/`Retry-After` courts utilisent un backoff borné. Reste ouvert : dashboard/monitoring du quota GitHub restant.
- [x] **Computation `owner_inactive_days`** côté events API GitHub — livré le 2026-05-16 comme input read-only (`owner_last_activity_at`, `owner_inactive_days`). La règle d'alerte "maintainer silencieux 90j" reste séparée en Phase R3.
- [x] **Cadence refresh corpus entier** : livré 2026-05-06. Le scheduler opt-in refresh les repos watchés + tout repo GitHub dont `priors_fetched_at` est NULL ou vieux de plus de 24 h.
- [ ] **Critère corpus v1 formel** : top N par langage, sur demande, ou via watchlist uniquement ? Aujourd'hui : seed manuel via `seed-public-corpus.ps1`.
- [x] **Tests unitaires parsing GitHub** : ingestion GitHub couvre maintenant parsing releases, conditional headers, rate-limit classification/backoff, et owner activity.

---

## Priorité 3 — Roadmap produit

### 3.1 Phase R3 — Notifications avancées

- [ ] Règle "maintainer silencieux 90j" (dépend de `owner_inactive_days` ci-dessus).
- [ ] Règles d'alerte custom par user (seuils ajustables, mute, digest weekly).
- [ ] Canal v2 email + webhook.
- [ ] Digest email hebdomadaire pour les watchers actifs.

### 3.2 Phase R4 — Trust formula v2

- [x] Pondération réputation owner/reporter formula_v2 : compte neuf = poids 0 pour review active sévère, seuil owner dispute explicite, et notes d'audit enrichies. Livré 2026-05-16 via `[trust]` dans `formula_v2.toml`, `UserReputation::active_signal_review_weight`, `requires_strict_active_review_with_trust`, et notes `active-weight` / `owner-confidence`.
- [ ] Pondération plus riche par historique d'usage prod : surpondérer explicitement les reporters/owners avec signaux d'usage longs et sains dans les décisions de review admin.
- [ ] Graphe Sybil-resistant via OAuth GitHub (followers, contributions, âge compte).

### 3.3 Phase R6 — Polish frontend

- [ ] Page `/account` plus complète : historique contributions, règles d'alerte perso, settings plus riches.
- [ ] UX explication scoring sur `/discover` : barres de dimensions (freshness/adoption/reliability/abandonment/vitality), "pourquoi ce résultat", "pourquoi X exclu en mode auto". L'API expose déjà `GET /api/admin/scoring/explain/{repo_id}` mais n'est pas surfaceé côté UI public.
- [ ] Graph historique score + timeline signaux sur `/repos/$id` (l'historique est en DB mais pas affiché).

### 3.4 Phase R6 — Public positioning (Phase 6 du plan radar)

- [x] Mettre à jour `docs/mcp-examples.md` avec exemples emerging :
  - "Find a reliable testing library."
  - "Find emerging alternatives for auth in TypeScript."
  - "Watch new OSS tools for observability."
- [ ] Vérifier / ajuster `/mcp-guide` côté frontend après la passe MCP du 2026-05-06.
- [ ] Vérifier que `/how-to-read` mentionne explicitement la distinction score (qualité) vs maturity_band (radar).

### 3.5 Phase R2 — Search calibration

- [ ] Affiner le weighting lexical/sémantique/qualité sur corpus plus large et requêtes réelles variées (la calibration initiale a été faite sur ~25 repos).

---

## Priorité 4 — Validation & dette doc

### 4.0 Follow-up Herald 2026-05-06 — vrais signaux à garder

Le rapport `herald_usestakly_20260506_1905.md` contient beaucoup de faux positifs ou de règles de style notées trop sévèrement. Les signaux suivants sont néanmoins réels et doivent rester dans la dette de finition. Ils ne bloquent pas le monitoring externe ni la public beta prudente, mais ils structurent un chantier de maintenabilité.

- [ ] **Refactor maintenabilité des gros fichiers/fonctions** : prioriser `frontend/src/routes/discover.tsx`, `frontend/src/features/repos/components/UseCaseSearchPanel.tsx`, `backend/src/mcp/server.rs`, `backend/src/services/repos.rs`. Objectif : extraire sous-composants/services sans changer le comportement.
- [ ] **Boucles DB potentiellement N+1** : auditer puis batcher si nécessaire `services/use_case_watches.rs`, `services/notifications.rs`, `services/repo_categories.rs`. Priorité moyenne : chemins surtout write/backfill, pas hot path public critique.
- [ ] **Tests ciblés sur zones complexes** : compléter autour des flows MCP metrics/admin, recommandations/use-case watches, ingestion GitHub parsing, et composants frontend complexes.
- [x] **Docs pré-pivot** : retirées des docs actives pour éviter le bruit dans les audits automatiques et les décisions agents.
- [ ] **Dette UI/React à vérifier au fil de l'eau** : garder un oeil sur les listes `.map()` et les composants très conditionnels. Plusieurs alertes `missing key` Herald sont fausses, mais la règle reste utile pendant les refactors.

### 4.1 Phase R7 — E2E

- [x] Smoke public final avant annonce : validé manuellement en prod le 2026-05-17 sur desktop + mobile, sans erreur console visible ni blocage layout signalé.
- [x] Flow local sans mocks API : `npm run test:e2e:real` lance Postgres Docker, backend local, seed SQL, puis couvre landing → discover → repo detail → watchlist → notification → account token → MCP initialize/search.
- [ ] Flow user E2E complet sur stack live : login OAuth réel → search "date picker react" → ouvre profil repo → clique Watch → simule un changement de score → reçoit notif.
- [ ] Flow agent MCP complet : `search_github_repos` → `get_repo_quality_context` → `log_usage` → vérifier que le signal alimente bien `quality_signals` puis `artifact_scores`.
- [ ] Décider si `test:e2e:real` devient un workflow GitHub Actions manuel/nightly avec Postgres, ou reste un release gate local documenté.

### 4.2 Audit parcours utilisateur — phase 2 connectée

Doc existante : `docs/audits/user-journey-audit-phase2-2026-04-24.md`. Reste à couvrir :

- [ ] Flow post-login réel : retour sur page d'origine vs destination par défaut.
- [ ] Flow "watch your first repo" depuis la landing jusqu'à la watchlist réelle.
- [ ] Flow notification → action : depuis le centre de notif, le contexte est-il assez lisible ?
- [ ] États vides connectés : watchlist vide, notifications vides, compte sans token.
- [ ] Erreurs réelles côté UI : échec `POST /api/repos/add`, session expirée, refus auth.
- [ ] Parcours onboarding connecté complet : login OAuth → discover → repo detail → watchlist → notifications → account/tokens.

### 4.3 Passe UX plus respirante

- [ ] Repasser sur les surfaces publiques (`/`, `/discover`, `/repos/$id`, `/how-to-read`, `/mcp-guide`) pour réduire la densité visuelle : moins de blocs explicatifs concurrents, plus d'espace vertical, hiérarchie de titres plus calme, CTA moins nombreux par écran.
- [ ] Simplifier la découverte en gardant le coeur produit visible : score, provenance, besoin/recommandation, radar et MCP, sans transformer chaque écran en documentation.
- [ ] Vérifier mobile et desktop : pas de texte trop serré, pas de formulaires qui dominent le premier écran, footer légal/contact lisible mais discret.
- [ ] Passage mobile / responsive dédié si on veut assumer autre chose que desktop-first.

### 4.3 Doc

- [ ] **Doc tests fonctionnels** : checklist formelle (login OAuth OK, add repo OK, watchlist affiche, notifs se créent, `/api/search` filtres OK, profil repo cohérent). Les docs `dev-workflow.md` et `user-journey.md` couvrent les "comment", manque les "vérifications go/no-go".

---

## Items explicitement déférés (pas dans l'ordre de priorité)

- `R6` : vues libraries/snippets retirées de l'app active ; schéma DB conservé uniquement pour compatibilité historique.
- `R5` token agent : JWT dédié vs OAuth device flow. Implémentation actuelle (`usk_<64 hex>` SHA-256) suffit au MVP.
- "Intuition couche 2" : POC quand ? Jamais, post-MVP, post-traction ?

---

## Mises à jour appliquées dans les docs sources lors de cet audit

Pour traçabilité :

- `TODO.md` L116 : endpoint admin `POST /api/admin/ingest/github` coché (livré).
- `TODO.md` L131 : filtres avancés cochés (`RepoSearchFilters` complet, livré `b608db3`).
- `TODO.md` L230, L232 : doc reproduction tests (`docs/dev-workflow.md`) et doc parcours utilisateur (`docs/user-journey.md`) cochées.
- `docs/plans/source-of-truth-oss-radar-plan.md` : entête "Status 2026-05-03" listant phases livrées (1, 2, 3 hors notifs, 5) et reste (4 MCP, 6 copy).
- `docs/plans/use-case-recommendation-watch-plan.md` : entête "Statut 2026-05-03" listant lots livrés (1, 2, 3 hors notifs, 5) et reste (3 notifs, 4 MCP).
- `docs/plans/anti-slop-vitality-v2.md` : entête "Statut 2026-05-03" marquant le chantier livré + followup `last_release_at`.
