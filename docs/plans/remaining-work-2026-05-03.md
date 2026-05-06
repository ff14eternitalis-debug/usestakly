# UseStakly — Reste à terminer (audit 2026-05-03)

> **Snapshot** : passe complète sur `TODO.md` v5.5 + `docs/plans/*` + état réel du code.
> Les faux positifs détectés (items marqués TODO mais déjà livrés) ont été corrigés directement dans les docs concernées (`TODO.md`, `docs/plans/source-of-truth-oss-radar-plan.md`, `docs/plans/use-case-recommendation-watch-plan.md`, `docs/plans/anti-slop-vitality-v2.md`).
>
> Cette doc liste **uniquement** ce qui reste vraiment ouvert, organisé par priorité d'exécution.
> À mettre à jour à chaque vague de finition. Source de vérité opérationnelle = `TODO.md` ; ce fichier en est la vue priorisée à l'instant T.

---

## Priorité 1 — Hardening avant ouverture publique large

Objectifs bloquants identifiés dans `docs/ops-mcp-coolify-hardening.md` et la section "Public beta" de `TODO.md`.

### 1.1 Ops MCP / DB

- [ ] **Backup DB Coolify planifié** — quotidien, rétention 7 jours minimum, test de restore. Vérification : `coolify database backup list z3xzjc0sy03kr6mpv8xvka7l --format json` retourne au moins une config.
- [x] **Rate-limit applicative globale `/mcp`** — livré 2026-05-06. Limite par IP pour non-auth/invalides (`APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`) et limite par token valide pour le transport/reads (`APP_MCP_READ_LIMIT_PER_MINUTE`). Les writes gardent le quota par token existant via `agent_token_events`.
- [ ] **Alerte externe** (UptimeRobot / Better Stack / Grafana Cloud) sur `GET /health`, `GET /api/status/public`, et un test MCP contrôlé avec token monitoring dédié.

### 1.2 Public beta gating

- [ ] **Page légale `/legal` ou `/terms`** — aujourd'hui seul `/privacy` existe (`frontend/src/routes/privacy.tsx`).
- [ ] **Email contact officiel** — domaine public stable livré (`https://www.usestakly.com` + `https://mcp.usestakly.com`). Reste à ajouter une adresse contact officielle sur landing/footer/privacy.

---

## Priorité 2 — Chantiers entamés à finir

Items qui ont déjà du code ou des migrations en place mais ne sont pas terminés.

### 2.1 Notifications use case watches (Lot 3 du plan use-case)

Migration 0020 + endpoints + UI livrés. **Aucune notification n'est jamais émise** sur ces watches.

- [ ] Étendre `services/scheduler.rs` pour itérer les `use_case_watches.enabled = true`.
- [ ] Recalculer les matches via `services/recommendations.rs::recommend_for_use_case`, comparer avec `use_case_watch_matches` persistés.
- [ ] Émettre les 4 types de notifications définis dans le plan via `services/notifications.rs` :
  - `use_case_new_candidate` — nouveau repo entre dans le top N
  - `use_case_best_candidate_changed` — meilleur repo change
  - `use_case_quality_drop` — repo du top baisse de score ≥ 0.10
  - `use_case_flag_added` — repo recommandé prend un flag toxique
- [ ] Anti-bruit : max 1 notification par watch par jour au MVP, mettre à jour `last_notified_at`.

### 2.2 MCP cohérence intent (Lot 4 du plan use-case + Phase 4 du plan radar) ✅ livré 2026-05-06

- [x] Faire consommer au tool MCP `recommend_github_repos` le **même service** (`services/recommendations.rs`) que `POST /api/use-cases/recommend`, pour aligner les explications intent/categories/topics et la provenance.
- [x] Sortir la réponse en sections séparées dans le tool `recommend_github_repos` :
  - `stable_picks`
  - `emerging_picks`
  - `fallback_candidates`
- [x] Ajouter un tool MCP `watch_use_case` (équivalent du REST `POST /api/use-cases/watch`).

Reste côté MCP : valider un smoke réel avec token prod après déploiement, puis garder la doc d'exemples à jour quand de nouveaux patterns d'agents apparaissent.

### 2.3 Bug capture `last_release_at` (followup formula v2)

- [ ] Investiguer `services/ingestion/github.rs::fetch_releases_summary` : 4/25 repos seulement ont `last_release_at` populé alors que tokio, vitest, eslint, rust, prisma ont des releases visibles. Probablement pagination ou parsing du payload `/releases`. Pas bloquant (vitality utilise neutre 0.5 quand NULL) mais améliore la qualité du score.

### 2.4 Phase R1 ingestion — finition

- [ ] **Rate-limit handling GitHub** : ETags / conditional requests, backoff sur 429, monitoring du quota restant. Aujourd'hui un 429 lève une `forbidden` brute (`services/ingestion/github.rs`).
- [ ] **Computation `owner_inactive_days`** côté events API GitHub — préalable à la règle d'alerte "maintainer silencieux 90j" (Phase R3).
- [x] **Cadence refresh corpus entier** : livré 2026-05-06. Le scheduler opt-in refresh les repos watchés + tout repo GitHub dont `priors_fetched_at` est NULL ou vieux de plus de 24 h.
- [ ] **Critère corpus v1 formel** : top N par langage, sur demande, ou via watchlist uniquement ? Aujourd'hui : seed manuel via `seed-public-corpus.ps1`.
- [ ] **Tests unitaires parsing GitHub** : aucun test sur `services/ingestion/github.rs` au-delà des helpers `decode_readme_content` et `parse_github_repo_input`.

---

## Priorité 3 — Roadmap produit

### 3.1 Phase R3 — Notifications avancées

- [ ] Règle "maintainer silencieux 90j" (dépend de `owner_inactive_days` ci-dessus).
- [ ] Règles d'alerte custom par user (seuils ajustables, mute, digest weekly).
- [ ] Canal v2 email + webhook.
- [ ] Digest email hebdomadaire pour les watchers actifs.

### 3.2 Phase R4 — Trust formula v2

- [ ] Pondération réputation owner/reporter formula_v2 : compte neuf = poids 0, historique d'usage prod surpondéré. Le fichier `formula_v2.toml` existe et la pipeline charge `load_v2()` par défaut, mais le facteur "compte neuf = poids 0" reste à implémenter dans `weighting.rs`.
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

### 4.1 Phase R7 — E2E

- [ ] Flow user E2E complet sur stack live : login OAuth réel → search "date picker react" → ouvre profil repo → clique Watch → simule un changement de score → reçoit notif. Aujourd'hui `frontend/e2e/mvp.spec.ts` couvre des fixtures mockées (~80 lignes).
- [ ] Flow agent MCP E2E : `search_github_repos` → `get_repo_quality_context` → `log_usage` → vérifier que le signal alimente bien `quality_signals` puis `artifact_scores`.
- [ ] Étendre Playwright sur les flows critiques connectés (login OAuth, watch, notif).

### 4.2 Audit parcours utilisateur — phase 2 connectée

Doc existante : `docs/audits/user-journey-audit-phase2-2026-04-24.md`. Reste à couvrir :

- [ ] Flow post-login réel : retour sur page d'origine vs destination par défaut.
- [ ] Flow "watch your first repo" depuis la landing jusqu'à la watchlist réelle.
- [ ] Flow notification → action : depuis le centre de notif, le contexte est-il assez lisible ?
- [ ] États vides connectés : watchlist vide, notifications vides, compte sans token.
- [ ] Erreurs réelles côté UI : échec `POST /api/repos/add`, session expirée, refus auth.
- [ ] Parcours onboarding connecté complet : login OAuth → discover → repo detail → watchlist → notifications → account/tokens.
- [ ] Passage mobile / responsive dédié si on veut assumer autre chose que desktop-first.

### 4.3 Doc

- [ ] **Doc tests fonctionnels** : checklist formelle (login OAuth OK, add repo OK, watchlist affiche, notifs se créent, `/api/search` filtres OK, profil repo cohérent). Les docs `dev-workflow.md` et `user-journey.md` couvrent les "comment", manque les "vérifications go/no-go".

---

## Items explicitement déférés (pas dans l'ordre de priorité)

- `R6` : faut-il garder les vues libraries/snippets cachées (réactivables) ou les dégommer net ? Schéma DB conservé en l'état.
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
