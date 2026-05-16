# UseStakly — TODO MVP / Public Beta

> **Document historique** — conserve la roadmap détaillée v5.x. Pour le backlog priorisé actuel, utiliser `docs/plans/remaining-work-2026-05-03.md` et `docs/source-of-truth.md`.

> Version : 5.6 — 2026-05-06 (+ radar MCP, watch_use_case)
> **Pivot produit acté** : on abandonne la bibliothèque de snippets.
> Nouveau produit : **outil de veille GitHub qui réduit le bruit des stars et offre un vrai suivi des repos publics OSS**.
> Référence : `docs/strategy-pivot-2026-04-21.md` (scope) et `docs/strategy-quality-scored-registry.md` (moat et principes, toujours valides).
> Business model : voir `docs/business/business-model-exploration.md` (privé, gitignore).
>
> **État au 2026-05-08** : MVP public beta exposable. Discovery, repo detail, watchlist, notifications, OAuth, MCP read/write/recommend/watch-use-case, CLI npm, status public, privacy/data, guide de lecture, guide MCP, rate-limit MCP, backup DB Coolify quotidien, test de restore DB, monitoring externe Uptime Kuma, canaux de notification configurables et digest Discord quotidien sont en place. Restent surtout : stockage backup offsite/S3, E2E complet, provider email d'envoi et notifications de veilles d'intention.

---

## Vision produit recentrée

Deux fonctions noyau :

1. **Discovery qualité-scored** — un dev cherche un outil (ex: « date picker React timezone »), UseStakly retourne les repos GitHub scorés par **usage réel** (reliability, abandonment, freshness), pas par stars. Réduit le bruit de la recherche GitHub où les repos hypés masquent les vrais choix techniques.
2. **Suivi des repos** — un dev met des repos dans sa watchlist et est notifié quand un score bouge significativement (abandonment up, nouveau flag `security-issue`, maintainer silencieux 90 j, etc.). GitHub ne fournit pas ce suivi qualité.

Les agents IA consomment la même data via MCP.

Cap produit à venir : **UseStakly = source de vérité qualité + radar OSS anti-bruit**. Le score reste la base, mais le produit doit aussi distinguer les choix établis des projets émergents utiles pour un besoin dev. Plan d'action : `docs/plans/source-of-truth-oss-radar-plan.md`.

---

## État public beta — 2026-04-26

- [x] Landing publique orientée promesse produit
- [x] Page `Lire UseStakly`
- [x] Page `Privacy / Données`
- [x] Page `Status / Beta`
- [x] Endpoint public `GET /api/status/public`
- [x] Corpus public initial élargi par catégories via `scripts/seed-public-corpus.ps1`
- [x] Guide MCP public avec installation `npx usestakly-mcp install`
- [x] Package npm `usestakly-mcp` publié
- [x] MCP validé dans Codex : search, detail, log_usage, watch_repo
- [x] Tool MCP haut niveau `recommend_github_repos`
- [x] Tool MCP `watch_use_case`
- [x] Doc exemples MCP : `docs/mcp-examples.md`
- [x] Page légale courte (`/legal`)
- [x] Domaine public stable et contact officiel affiché (`contact@usestakly.com`)

---

## Priorité ops / sécurité MCP avant ouverture plus large

Voir doc dédiée : `docs/ops-mcp-coolify-hardening.md`.

- [x] **Configurer un backup DB Coolify planifié**
  - Risque principal actuel : perte de données Postgres.
  - Livré 2026-05-06 : backup local Coolify activé sur `usestakly-postgres` (`z3xzjc0sy03kr6mpv8xvka7l`), cron `0 2 * * *`, DB `usestakly`, rétention locale 7 jours / 7 backups.
  - Vérifié via API Coolify `GET /api/v1/databases/z3xzjc0sy03kr6mpv8xvka7l/backups` : config `n12jqb2qn56mcmiqrwnjbh1z` active, exécution manuelle `success`.
  - Test de restore validé 2026-05-07 en local via Docker Desktop depuis `pg-dump-usestakly-1778119206.dmp` : tables critiques et migrations restaurées avec succès.
  - Reste à faire avant ouverture large : stockage distant/offsite.

- [x] **Ajouter une rate-limit applicative sur `/mcp`**
  - Couvrir `initialize`, `tools/list`, read tools et write tools.
  - Livré 2026-05-06 : limite par IP pour non-auth/invalides (`APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`) et limite par token valide pour le transport/reads (`APP_MCP_READ_LIMIT_PER_MINUTE`).
  - Les write tools gardent leurs quotas/cooldowns existants via `agent_token_events`.

- [x] **Forcer Authorization sur toute route `/mcp`**
  - Même `initialize` et `tools/list` ne doivent pas exposer gratuitement le catalogue.
  - Garder la validation DB token dans chaque tool.
  - Ajouter un middleware pré-transport MCP pour refuser missing/invalid Bearer.

- [x] **Ajouter une alerte externe**
  - Checks recommandés :
    - `GET /health`
    - `GET /api/status/public`
    - test MCP contrôlé avec token dédié monitoring
  - Livré 2026-05-07 avec Uptime Kuma : `UseStakly Website`, `UseStakly API Health`, `UseStakly Public Status`, `UseStakly MCP` authentifié avec token dédié monitoring.

---

## ⚠ Ce qui est retiré du produit

| Retiré | Devenir |
|---|---|
| Libraries / snippets (CRUD, UI, discovery publique) | Produit retiré. Schéma DB conservé uniquement pour compatibilité historique ; aucune surface runtime active à réintroduire sans demande explicite. |
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
- **Endpoint signals pré-pivot (snippets era)** — refactoré vers `POST /api/repos/:id/signals` + `external_artifacts`
- **Endpoints `/api/resolve` et `/api/search` avec filter auto/strict/explore** — filtres OK, source à repointer sur `external_artifacts`
- **Endpoint admin `/api/admin/scoring/recompute`** — OK
- Audit sécu commit `4e16c0a` validé (voir `docs/security-audit-2026-04-21.md`)

Frontend (Phase 3) : le shell, l'auth, le theme, la providers tree **restent utiles**. Les vues libraries/snippets sont à retirer (Phase R6).

---

## Nouvelles phases (R = refactor vers le pivot)

### Phase R1 — Ingestion GitHub ✅ LIVRÉE (commit 69cb5ac)

Pipeline neuf. C'est le cœur du nouveau produit : sans repos ingérés, rien à scorer.

- [x] Migration `0011_github_artifacts.sql` — colonnes GitHub-specific sur `external_artifacts` (owner, name, default_branch, stars, forks, license, archived, language, last_commit_at, open_issues_count)
- [x] Service `ingestion::github` — client REST (reqwest direct), auth via PAT
- [x] Ingestion priors snapshot : stars, forks, last_commit_at, open_issues, archived, language, license
- [x] Endpoint `POST /api/repos/add` — user propose un repo à ingérer (`handlers::repos`)
- [x] Mapping `github.com/owner/repo` → UUID `external_artifact_id` (idempotent)
- [x] Binary `seed_github` pour bootstrap corpus manuel
- [x] **Livré 2026-05-16** : ingestion GitHub fiabilisée (ETags releases/README/events, backoff borné, classification rate-limit) — migration `0027`, voir `remaining-work` §2.4 pour monitoring quota optionnel.
- [x] **Livré 2026-05-16** : `owner_inactive_days` et champs ETag persistés — migration `0027`.
- [x] Cadence refresh automatique : scheduler opt-in, default 24 h, refresh des repos watchés + corpus GitHub stale (`priors_fetched_at` NULL ou > 24 h) via `services::scheduler`.
- [ ] **Reste à faire** : critère corpus v1 formel — **à trancher** entre top N par langage / sur demande / via watchlist uniquement
- [x] Endpoint admin `POST /api/admin/ingest/github` pour backfill ciblé (`backend/src/handlers/admin.rs::ingest_github_repo`, route câblée dans `app/mod.rs`)
- [ ] **Reste à faire** : tests unitaires sur parsing réponses GitHub

### Phase R2 — Discovery qualité-scored ✅ LIVRÉE (commit 8e4e1f7, fix f053f79)

Remplace la search snippets par la search repos GitHub.

- [x] `/api/search` repointé : cherche dans `external_artifacts` (repos GitHub)
- [x] Filtres existants conservés : `filter=auto|strict|explore` (définis dans formula_v1)
- [x] Recherche lexicale : ILIKE sur `name` + `description` + topics GitHub
- [x] Endpoint `GET /api/repos/:id` — profil complet (dimensions, flags, historique scores)
- [x] **R2b** : recherche sémantique locale branchée — `fastembed` + `pgvector`, embeddings des repos GitHub à l'ingestion, query embedding au search, ranking hybride avec le score qualité
- [x] Endpoint admin de backfill embeddings corpus existant — `POST /api/admin/embeddings/backfill`
- [x] Calibration initiale du ranking hybride sur corpus local réel — `auto` ne vide plus artificiellement les résultats, score lexical tokenisé + blend lexical / sémantique / qualité branché
- [ ] **Reste à faire** : affiner encore le weighting lexical / sémantique / score qualité sur corpus plus large et requêtes réelles variées
- [x] Filtres avancés (langage, license, stars min/max, freshness, topics, score_min, abandonment_max, include_archived) — `RepoSearchFilters` complet dans `services/repos.rs`, livré 2026-04-26 (`b608db3`)
- [ ] **Reste à faire** : UX d'explication « pourquoi ce repo est proposé, pourquoi X est exclu en mode auto »

### Phase R3 — Watchlist & suivi ✅ LIVRÉE partiellement (commit 8750ea8)

Le deuxième pilier. C'est ce qui manque sur GitHub aujourd'hui.

- [x] Migration `0012_watchlists.sql` — `watchlists`, `watched_artifacts`, `notifications`
- [x] Endpoints `/api/watchlist` — CRUD + ajouter / retirer un repo (`handlers::watchlist`)
- [x] Détection de changement significatif : diff score T vs T-1 dans `services::notifications` (`fetch_prev_snapshot` + seuils)
- [x] Règles d'alerte défaut : abandonment +0.20, nouveau flag `security-issue` / `broken`, score `overall` qui chute de ≥ 0.10
- [x] Canal notification v1 : **in-app** (endpoints `/api/notifications`, route frontend `notifications.tsx`)
- [x] Canaux sortants v1 : préférences `/account`, email de destination enregistré, Discord webhook chiffré en base via `APP_NOTIFICATION_SECRET`, test webhook et livraison des alertes critiques repo watchlist.
- [x] Résumé quotidien Discord : choix horaire utilisateur (`morning/noon/evening/night`), timezone IANA, scheduler 30 min, anti-doublon par canal/jour, pas d'envoi si rien d'important.
- [x] Worker scheduler autonome : `services::scheduler::spawn_recompute_loop` — tokio::spawn + interval, opt-in via `APP_SCHEDULER_ENABLED`, cadence via `APP_RECOMPUTE_INTERVAL_SECS` (default 24 h). Refresh des repos watchés + repos GitHub dont les priors ont plus de 24 h via `ingest_repo`, puis `recompute_all_scores` (qui émet les notifs). Pas de run au boot.
- [x] Notifications des veilles d'intention : le scheduler réévalue les `use_case_watches`, compare avec `use_case_watch_matches`, émet `use_case_new_candidate`, `use_case_best_candidate_changed`, `use_case_quality_drop`, `use_case_flag_added`, et applique un cooldown MVP de 24 h par watch.
- [ ] **Reste à faire** : règle « maintainer silencieux 90 j » (dépend de `owner_inactive_days` côté R1)
- [ ] **Reste à faire** : règles d'alerte custom par user (seuils ajustables, mute, digest weekly)
- [x] Provider d'envoi email transactionnel Brevo : SMTP configuré en prod, canal email ajouté depuis `/account`, bouton test validé avec réception réelle.
- [ ] **Reste à faire** : tester une vraie notification sortante email, pas seulement le test de canal : alerte watchlist ou digest envoyé par le scheduler.

### Phase R4 — Signaux actifs / flags toxiques (cœur produit)

Gardé de Phase 6/9 v4, adapté aux repos GitHub publics.

- [x] Endpoint `POST /api/repos/:id/signals` — remplace l'endpoint signals legacy pré-pivot, évidence obligatoire
- [x] Politique flags toxiques v1 — `deprecated`, `broken`, `security_issue` : evidence + **consensus N users distincts** avec réputation > seuil avant exposition publique
- [x] Processus modéré v1 pour `security_issue` — création en `pending`, publication retardée jusqu'à review admin
- [x] Appel / dispute par l'owner (via OAuth GitHub matching login)
- [x] Support owner v1 pour org GitHub : membre public de l'organisation reconnu comme owner éligible à la dispute
- [x] Historique transparent v1 des transitions de signal (submitted / reviewed / disputed) affiché sur le profil repo
- [x] Réputation utilisateur v1 exposée en API/UI compte + seuil minimal avant signals actifs
- [x] Consensus v1 sur les flags publics : seuls les signaux actifs venant de users éligibles comptent dans `artifact_scores.flags`
- [x] Réputation utilisateur v2 runtime — score pondéré par usage réel, outcomes positifs, fiabilité build et pénalité regret ; éligibilité active exige désormais un minimum de vrai usage
- [x] Pondération réputation reporter v2 dans le workflow de modération — reporters faibles sur signaux actifs sévères (`broken`, `doesnt_match_claim`, `security_issue`) passent en review stricte/pending au lieu d'une acceptation automatique
- [x] Pondération trust owner v1 dans les disputes/reviews — la file admin remonte maintenant aussi le contexte trust du compte owner qui conteste, et les signaux acceptés puis disputés reviennent dans la boucle de review
- [x] Pondération réputation owner / reporter v2 dans les reviews sensibles — livré 2026-05-16 via `[trust]` dans `formula_v2.toml` (compte neuf poids actif 0 pour signaux sévères).
- [ ] Graphe Sybil-resistant via OAuth GitHub (followers, contributions, âge compte)

### Phase R5 — MCP adapté aux repos et besoins

Plus des snippets — des repos GitHub. Split en R5a (read-only, livré 2026-04-23) / R5b (write tools + signaux passifs, livré partiellement le 2026-04-23).

**R5a — livrée 2026-04-23**

- [x] Transport Streamable HTTP via `rmcp` 1.5 monté à `/mcp` (`mcp::server::build_service`)
- [x] Migration `0013_agent_tokens.sql` — table + index hot-path
- [x] Auth agent : Bearer token `usk_<64 hex>` hashé SHA-256, lookup via `mcp::auth::verify_bearer`
- [x] Endpoints REST de gestion : `POST/GET /api/agent-tokens`, `DELETE /api/agent-tokens/{id}`
- [x] Outil `search_github_repos(query, filter, language?, stars_min?, limit?)` → candidats scorés
- [x] Outil `get_repo_quality_context(owner, name)` → profil complet (dimensions, flags, signals)
- [x] Provenance dans chaque output : `{ source: "usestakly://registry/github[/owner/name]", formula_version, scored_at }`
- [x] Doc v2 post-pivot dans `docs/mcp-protocol.md` (l'ancienne v1 pré-pivot est remplacée)

**R5b — livré partiellement 2026-04-23**

- [x] Outil `log_usage(repo, outcome)` → crée un `quality_signal` passif avec `user_id` du token
- [x] Outil `watch_repo(repo)` — ajoute à la watchlist du user propriétaire du token
- [x] Outil `watch_use_case(need, risk_tolerance?)` — crée une veille d'intention/radar du user propriétaire du token
- [x] `recommend_github_repos` aligné sur `services::recommendations` et réponse structurée en `stable_picks` / `emerging_picks` / `fallback_candidates`
- [x] Ingestion à la volée d'un repo manquant avant `log_usage` / `watch_repo` si `GITHUB_TOKEN` est configuré
- [x] Rate-limit par token (quota write/heure configurable) via `agent_token_events` (migration `0014`)
- [x] Garde-fous anti-spam sur `log_usage` : cooldown par token + fenêtre de refroidissement sur outcomes négatifs répétés
- [x] UI de gestion des tokens côté frontend : page `/account` (création, liste, révocation)
- [x] Poisoning-resistance v2 partielle sur `log_usage` / trust : réputation user explicite, pondération par ancienneté + usage réel + regret/build reliability
- [x] Support owner GitHub v2 best effort : owner direct, membre public d'org, membre privé d'org si `GITHUB_TOKEN` le permet, ou collaborateur/maintainer repo si l'API GitHub peut confirmer le rôle
- [x] Poisoning-resistance avancée v2 sur `log_usage` : outcomes négatifs désormais filtrés par réputation trust, historique d'usage sain et notes minimales pour les cas les plus sensibles
- [x] Observabilité MCP v1 : refus des guards enregistrés comme `mcp_guard_rejection` dans `agent_token_events`, endpoint admin `GET /api/admin/mcp/metrics?window=24h|7d|30d` (totaux, distribution outcomes, breakdown refus par tool/raison, top repos, top users, daily volume), panel `AdminMcpObservabilityPanel` dans `/account`
- [x] Pondération encore plus fine par type d'outcome / historique par repo sur `log_usage` (formula **v1.1** — 2026-04-24) : chaque signal passif est pondéré `outcome_weight × review_weight(reporter) × 1/(1 + k · n_prev_same_user_same_repo)` avant d'alimenter `compute_score`. Poids par outcome : resolve 1.0, build_success/failure 1.2, re_resolve 1.5, regret 2.0 ; `dedup_k = 0.25`. Paramètres dans `scoring/formula_v1.toml` section `[weighting]`. Endpoint admin `GET /api/admin/scoring/explain/{repo_id}` pour tracer la décomposition signal par signal. Les counts bruts restent persistés pour audit.

### Phase R6 — Refonte frontend complète ✅ LIVRÉE partiellement (commits b1221c8, 9fec584)

Le frontend actuel est centré snippets. À démolir en grande partie, à rebâtir autour de discovery + watchlist.

- [x] **Gardé** : shell global, providers, theme, auth flow OAuth, layout base
- [x] **Retiré** : vues libraries/snippets supprimées de la navigation (routes ayant disparu du dossier `routes/`)
- [x] **Nouveau** : landing orientée outil de veille (`routes/index.tsx`, refait par commit 9fec584 « redo design »)
- [x] **Nouveau** : page recherche / discovery (`routes/discover.tsx`)
- [x] **Nouveau** : page profil repo (`routes/repo-detail.tsx`)
- [x] **Nouveau** : dashboard watchlist (`routes/watchlist.tsx`)
- [x] **Nouveau** : centre de notifications in-app (`routes/notifications.tsx`)
- [x] Composant i18n EN/FR livré (`LocaleSwitch`, `locale-store`)
- [x] Page compte v1 utile — tokens MCP, réputation user, file de modération admin légère
- [x] Page compte : canaux de notification configurables (email + Discord webhook, avec test webhook).
- [ ] **Reste à faire** : page compte plus complète — historique contributions, règles d'alerte perso, settings plus riches
- [x] UI v1 de modération légère : file pending/disputed dans `/account`, review admin, dispute owner, timeline locale sur le profil repo
- [x] TanStack Query câblé via `frontend/src/app/providers.tsx` et utilisé sur les routes actives
- [x] Router tranché : TanStack Router en usage sur l'app active
- [ ] **Reste à faire** : UX d'explication du scoring sur la page discovery (barres de dimensions, flags, « pourquoi ce résultat »)
- [ ] **Reste à faire** : graph historique score + timeline signaux sur le profil repo
- [x] Correctifs UX v1 post-audit : garde auth avant montage du routeur, CTA `add repo` moins technique, CTA repo non-auth orienté bénéfice, hiérarchie CTA landing resserrée

### Phase R7 — Validation e2e

- [ ] Smoke public final : landing → Lire UseStakly → Explorer → repo detail → Guide MCP → Privacy → Legal → Status, avec vérification responsive et console propre.
- [x] Flow local sans mocks API : landing → discover → repo detail → watchlist → notification → account token → MCP initialize/search via `frontend/e2e/real-api.spec.ts` et `npm run test:e2e:real`.
- [x] Flow user live OAuth : login OAuth → search → repo detail → watchlist → notifications → account token → MCP test validé en prod.
- [ ] Flow agent complet : MCP search → get_repo_quality_context → log_usage → vérifier que le signal alimente `quality_signals` puis `artifact_scores`.
- [ ] Brancher éventuellement `test:e2e:real` en workflow manuel/nightly avec service Postgres, pas dans la CI rapide tant que le coût/temps n'est pas stabilisé.
- [x] Vérification sécu : audit mis à jour post-pivot dans `docs/security-audit-2026-04-21.md`

### Phase R8 — Passe UX plus respirante

- [ ] Réduire la densité des pages publiques après finalisation du hardening : plus d'espace, moins de blocs concurrents, texte plus court.
- [ ] Prioriser les signaux coeur produit à l'écran : score, provenance, recommandation par besoin, radar, MCP.
- [ ] Vérifier `/`, `/discover`, `/repos/$id`, `/how-to-read`, `/mcp-guide` en desktop et mobile.

---

## 📄 Documentation à livrer (dette acceptée 2026-04-23)

Reportée pour ne pas casser le flow actuel, mais à faire avant ouverture externe :

- [x] **Doc reproduction tests** — `docs/dev-workflow.md` couvre démarrage stack, env vars, recherche sémantique, commandes courantes
- [ ] **Doc tests fonctionnels** — check-list : login OAuth OK, add repo OK, watchlist affiche, notifs se créent quand un score bouge, `/api/search` filtre auto/strict/explore, profil repo cohérent
- [x] **Doc parcours utilisateur** — `docs/user-journey.md` v2.0 (5 flows : découverte, connexion/watch, agent MCP, signal modéré, admin)

## 🧭 Audit parcours utilisateur — phase 1 faite, phase 2 à faire

Un premier audit réel a été mené et documenté dans `docs/audits/user-journey-audit-2026-04-23.md`.

Points déjà traités suite à cet audit :

- [x] garde auth corrigée : `/watchlist`, `/notifications`, `/account` redirigent désormais proprement vers `/login` pour un user anonyme
- [x] CTA discovery `add repo` clarifié (`Ingest repo` retiré)
- [x] CTA repo non-auth clarifié avec promesse de valeur
- [x] hiérarchie CTA landing resserrée
- [x] pattern login unifié côté shell actif : le header passe aussi par `/login`

Reste à couvrir dans un second audit, connecté cette fois :

- [ ] auditer le flow post-login réel : retour sur page d'origine ou destination par défaut
- [ ] flow « watch your first repo » depuis la landing jusqu'à la watchlist réelle
- [ ] flow notification → action : depuis le centre de notif, arrive-t-on sur un contexte assez lisible
- [ ] états vides connectés : watchlist vide, notifications vides, compte sans token
- [ ] erreurs réelles côté UI : échec `POST /api/repos/add`, session expirée, refus auth
- [x] parcours onboarding connecté complet : login OAuth → discover → repo detail → watchlist → notifications → account token → MCP test
- [ ] passage mobile / responsive dédié si on veut assumer autre chose que desktop-first

---

## Décisions encore ouvertes

- [ ] **R1** — critère de corpus initial : top N / sur demande / via watchlist uniquement ?
- [x] **R3** — canal notification v1 : in-app + Discord webhook configurable ; email enregistré mais envoi réel dépend encore d'un provider transactionnel.
- [ ] **R5** — token agent : JWT dédié généré par l'user, ou OAuth device flow ?
- [x] **R6** — router frontend : TanStack Router retenu pour l'app active
- [x] **R6** — vues libraries/snippets retirées de l'app active ; ne pas les réactiver sans demande explicite.
- [ ] **Intuition couche 2** : POC quand ? Jamais, post-MVP, post-traction ?

---

## Ordre d'exécution recommandé

~~1. **R1** (ingestion GitHub)~~ ✅
~~2. **R2** (search repos)~~ ✅
~~3. **R6 partiel** (landing + search UI)~~ ✅
~~4. **R3** (watchlist + notifs)~~ ✅ (sauf email / règles custom)

**Prochain à trancher** (dans l'ordre réel de priorité) :

~~1. **R3 finition — worker cron quotidien**~~ ✅ livré 2026-04-23 — scheduler tokio::spawn dans `services::scheduler`, opt-in via `APP_SCHEDULER_ENABLED`.
~~2. **R5a — MCP read-only**~~ ✅ livré 2026-04-23 — transport Streamable HTTP + 2 tools (`search_github_repos`, `get_repo_quality_context`) + auth Bearer via `agent_tokens`. Doc : `docs/mcp-protocol.md` v2.

1. **R4/R5b finition (reputation v2 + moderation plus fine)** — avant ouverture publique large
   Les garde-fous v1 sont là (quota MCP, réputation, consensus, review admin, dispute owner, audit trail). La vraie suite est la pondération réputation plus riche et le support des memberships GitHub privés / rôles fins.

2. **Second audit parcours utilisateur (connecté)** — qualité visible, secondaire pré-launch
   Le premier audit public est fait et ses frictions principales ont été corrigées. Il reste à valider le vrai parcours connecté de bout en bout.

3. **R2b approfondi (recherche sémantique)** — calibration sur corpus plus large
   La brique est maintenant en place et calibrée une première fois. La suite utile n'est plus de la brancher, mais de l'affiner sur plus de données.

4. **R6 reste** (compte user plus riche, explication scoring) + **R7** (validation E2E) — polish et launch
