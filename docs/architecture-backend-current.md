# Architecture backend actuelle

> Version : 1.3
> Dernière mise à jour : 2026-05-08
> Portée : backend vivant de **UseStakly** (public beta exposable)

## Vue d'ensemble

Le backend UseStakly est une API Rust/Axum centrée sur cinq capacités produit :

- découverte de repos GitHub publics scorés (lexical + sémantique optionnel + qualité)
- watchlist + notifications in-app + canaux sortants configurables + digest Discord quotidien
- signaux qualité modérés (consensus, réputation, review, dispute)
- exposition MCP Streamable HTTP pour agents IA (6 tools, auth Bearer)
- observabilité MCP et statut public pour la beta

Le point d'entrée :

`main.rs` → `config::AppConfig::from_env()` → `db::connect()` (pool + migrations + extensions optionnelles) → `app::build_app()` → `axum::serve`

## Sous-domaines actifs

### `app/`

Assemble le `Router` et `AppState { config, db }`. Configure :

- CORS strict sur `frontend_base_url` avec `allow_credentials(true)`
- `TraceLayer`
- middleware d'authentification MCP qui rejette toute requête `/mcp` sans `Authorization: Bearer ...` valide **dès `initialize` et `tools/list`**
- montage du service MCP via `rmcp::StreamableHttpService` à `/mcp`

### `config/`

Lecture d'env (`.env` chargé via `dotenvy`). Couvre DB, dev user, OAuth GitHub/Discord, session JWT, GitHub PAT pour ingestion, admin token, scheduler, garde-fous MCP, signaux actifs, semantic search.

### `auth/`

OAuth direct **GitHub + Discord**. Session JWT signée stockée dans le cookie `usestakly_session`. Le `state` OAuth est signé et porte un `return_to` sanitizé contre les open redirects (livré 2026-04-24).

Quand `APP_SESSION_SECRET` ou les couples OAuth sont absents, retombe sur un dev user injecté via `DEV_USER_*` (overridable par headers `x-debug-user-*`). `APP_NOTIFICATION_SECRET` chiffre les destinations sensibles des canaux de notification, séparément du secret de session. **Supabase Auth n'est ni utilisé ni prévu** — l'app est auto-hébergée sur VPS via Coolify.

### `handlers/`

Responsabilité : I/O HTTP seulement.

- `health` — `/health` + `/api/status/public` (status enrichi public beta)
- `auth` — callbacks OAuth GitHub / Discord avec `return_to` signé
- `me`, `account` — profil user, settings
- `admin` — endpoints admin gated par `ADMIN_API_TOKEN` (recompute, MCP metrics, scoring explain, embeddings backfill, signal review queue…)
- `agent_tokens` — CRUD tokens MCP (`POST/GET /api/agent-tokens`, `DELETE /api/agent-tokens/{id}`)
- `search` — recherche discovery publique
- `repos` — re-export des handlers spécialisés repo
- `repos_query` — profil repo détaillé + filtres avancés discover
- `repos_ingestion` — `POST /api/repos/add`
- `repo_signals` — création de signaux et dispute owner
- `repo_viewer` — état viewer-spécifique d'un repo
- `watchlist`, `notifications`, `notification_channels`

### `services/`

Responsabilité : logique métier.

- `ingestion/github.rs` — client GitHub REST direct (reqwest), normalisation repo, ingestion priors (stars, forks, last_commit_at, archived, language, license)
- `repos.rs` — agrégation profils repo, réponses discovery, score provenance
- `watchlist.rs`, `notifications.rs`, `notification_channels.rs`, `notification_digest.rs`
- `scheduler.rs` — boucle opt-in `tokio::spawn` refresh quotidien des priors GitHub stale (> 24 h) + repos watchés, puis recompute + emit notifs
- `semantic_search.rs` — embeddings repo + ranking hybride lexical/sémantique/qualité (derrière feature `semantic-search`)
- `agent_tokens.rs` — création, hash SHA-256, lookup, révocation
- `quality/`
- `trust/`

### `services/quality/`

Le scoring est éclaté pour rester testable sans DB autant que possible.

- `formula.rs` — chargement TOML `scoring/formula_v1.toml` + types
- `compute.rs` — calcul pur du score à partir des dimensions agrégées
- `flags.rs` — consensus, normalisation et résolution des flags publics
- `weighting.rs` — agrégation pondérée des signaux passifs (formula v1.1) : `outcome_weight × reporter_weight × dedup_weight` ; expose `aggregate_weighted_counts` et `explain_signals`
- `pipeline.rs` — chargement DB, `recompute_externals_with_config`, upsert `artifact_scores`, émission notifs
- `capture.rs` — enregistrement de signaux qualité

`compute.rs`, `flags.rs`, `weighting.rs` couverts par tests purs (sans DB).

### `services/trust/`

Réputation, modération, ownership, observabilité MCP.

- `reputation.rs` — réputation v2 runtime (usage réel + outcomes positifs + reliability + pénalité regret + ancienneté)
- `repo_owners.rs` — détection owner GitHub (direct, membre public d'org, membre privé via PAT, collaborateur/maintainer si l'API le confirme)
- `signal_reviews.rs` — workflow review admin (pending / accepted / rejected / disputed)
- `signal_events.rs` — timeline transparente des transitions de signal
- `agent_token_events.rs` — log des appels MCP et refus guards
- `mcp_metrics.rs` — agrégations SQL pour `/api/admin/mcp/metrics?window=24h|7d|30d`

### `mcp/`

Serveur MCP Streamable HTTP monté à `/mcp`.

- `auth.rs` — `verify_bearer` : SHA-256 du token contre `agent_tokens.token_hash`, lookup user, retour `AgentTokenContext`
- `server.rs` — handlers des 6 tools (voir ci-dessous)
- `tools/` — réservé aux helpers de tool partagés à venir

Tools exposés :

| Tool | Type | Description |
|---|---|---|
| `search_github_repos` | read | recherche scorée filtrable (filter, language, stars_min, limit) |
| `recommend_github_repos` | read | recommandations haut niveau (search + filter + provenance) |
| `get_repo_quality_context` | read | profil complet d'un repo (dimensions, flags, signals, provenance) |
| `log_usage` | write | crée un `quality_signal` passif. Retourne le score recalculé pour feedback agent immédiat |
| `watch_repo` | write | ajoute le repo à la watchlist du user propriétaire du token |
| `watch_use_case` | write | crée une veille d'intention/radar sur un besoin naturel |

Garde-fous write (config via env) :

- quota par token : `APP_MCP_WRITE_LIMIT_PER_HOUR`
- cooldown anti-doublon : `APP_MCP_LOG_USAGE_COOLDOWN_SECS`
- fenêtre négatifs : `APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS`
- limite MCP globale par IP pour auth absente/invalide : `APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`
- limite MCP transport/reads par token valide : `APP_MCP_READ_LIMIT_PER_MINUTE`
- réputation min : `APP_ACTIVE_SIGNAL_MIN_REPUTATION`
- consensus actif : `APP_ACTIVE_SIGNAL_DEFAULT_CONSENSUS` / `APP_ACTIVE_SIGNAL_SEVERE_CONSENSUS`
- refus enregistrés en `agent_token_events` avec `kind='mcp_guard_rejection'`

### `db/`

Pool SQLx, runner de migrations (`sqlx::migrate!` au boot), `ensure_optional_extensions` qui crée `vector` si présent dans `pg_available_extensions` (pgvector reste optionnel pour la prod).

### `domain/`

Types métier actifs : `account`, `agent_token`, `quality`, `repo`, `reference`, `watchlist`.

### `bin/`

- `seed_github` — binaire d'amorçage corpus manuel via `backend/seeds/top_repos.toml`

## Migrations

| # | Sujet | Statut |
|---|---|---|
| 0001–0009 | snippets/libraries/users/auth/permissions/generations | dormantes — produit retiré |
| 0010 | `quality_signals`, `external_artifacts`, `artifact_scores` | actif |
| 0011 | `external_artifacts` GitHub-specific (owner, name, stars, license, archived…) | actif |
| 0012 | `watchlists`, `watched_artifacts`, `notifications` | actif |
| 0013 | `agent_tokens` (auth MCP) | actif |
| 0014 | `agent_token_events` (rate-limit + observability) | actif |
| 0015 | `quality_signal_review` (workflow admin) | actif |
| 0016 | `quality_signal_events` (timeline) | actif |
| 0017 | `repo_embeddings` (pgvector, optionnel) | actif si feature `semantic-search` |
| 0023 | `notification_channels` (email destination + Discord webhook chiffré) | actif |
| 0024 | `digest_time_local`, `timezone`, `notification_digest_deliveries` | actif |
| 0025 | `use_case_*` notification kinds + flags persistés sur `use_case_watch_matches` | actif |

## Flux principaux

### Discovery

1. requête HTTP `/api/search` ou `/api/repos/...`
2. service `repos` interroge `external_artifacts` + `artifact_scores`
3. enrichissement filtres avancés (langage, license, stars min, freshness)
4. blend lexical / sémantique (si feature ON) / score qualité
5. réponse avec score provenance (`formula_version`, `scored_at`, `source: usestakly://...`)

### Recompute qualité

1. ingestion ou refresh d'un repo (`ingest_repo`)
2. chargement de la formule TOML
3. agrégation pondérée des signaux passifs via `weighting::aggregate_weighted_counts`
4. calcul des dimensions (`compute_score`)
5. résolution flags publics (`flags::resolve`) par consensus + réputation
6. upsert `artifact_scores` avec snapshot précédent
7. diff seuils → émission de notifications watchers in-app + livraison Discord webhook si configurée
8. réévaluation des `use_case_watches` actives par le scheduler : nouveau candidat, meilleur candidat changé, quality drop, nouveau flag, avec cooldown 24 h par watch

### Digest quotidien

1. l'utilisateur choisit un créneau simple dans `/account` (`morning`, `noon`, `evening`, `night`) et son fuseau IANA
2. le scheduler digest tourne par défaut toutes les 30 minutes (`APP_DIGEST_INTERVAL_SECS`)
3. il sélectionne les canaux `daily_digest_enabled = TRUE`, dont l'heure locale tombe dans la fenêtre courante
4. `notification_digest_deliveries` garantit un seul digest par canal et par date locale
5. aucun message n'est envoyé si aucune alerte repo ou candidat radar important n'existe sur les dernières 24 h

### Signal actif modéré

1. `POST /api/repos/:id/signals` avec evidence
2. contrôle réputation user, dispatch :
   - signaux sévères (`broken`, `doesnt_match_claim`, `security_issue`) à reporter faible → review pending
   - sinon acceptation conditionnelle au consensus
3. exposition publique seulement après seuils trust + review admin si nécessaire
4. dispute owner via OAuth GitHub matching (login direct, membre org, collaborateur via API)
5. timeline persistée dans `quality_signal_events`

### Observabilité MCP

1. chaque appel MCP loggé dans `agent_token_events`
2. refus guards loggés `kind='mcp_guard_rejection'` avec payload `{tool, reason, ...}`
3. agrégations admin via `GET /api/admin/mcp/metrics?window=...` (totaux, distribution outcomes, breakdown refus, top repos, top users, daily volume)
4. panel `AdminMcpObservabilityPanel` côté frontend dans `/account` derrière gate admin

## Frontières à préserver

- `handler` reste I/O — pas de logique métier
- `quality` calcule et orchestre le scoring ; la confiance sociale appartient à `trust`
- `repos` agrège l'expérience produit ; ne doit pas devenir un fourre-tout de modération
- `mcp` doit conserver la provenance dans chaque output (`source`, `formula_version`, `scored_at`)
- les tables legacy snippets restent en base mais n'ont aucune surface runtime active

## Tests et CI

- 32+ tests purs côté backend (compute, flags, weighting, reputation, MCP server, ingestion parsing, mcp_metrics window parser)
- aucun service Postgres dans la CI : tout test DB-bound est mocké ou isolé derrière une feature
- E2E Playwright frontend : `frontend/e2e/mvp.spec.ts` (mocks API, filet anti-régression UI)
- E2E réel local : `frontend/e2e/real-api.spec.ts` via `npm run test:e2e:real` (Postgres Docker + backend local + seed SQL, sans mocks API)
- CI installe Chromium et upload `playwright-report/` en artifact

## Dette restante

- pas de couverture intégration DB en CI (Postgres non provisionné)
- rate-limit applicative globale `/mcp` livrée en local runtime ; la couverture CI reste limitée aux tests unitaires/purs
- backoff + ETags sur ingestion GitHub à venir (R1 reste)
- `owner_inactive_days` non calculé → règle "maintainer silencieux 90 j" R3 bloquée
- réputation v2 runtime livrée ; formula_v2 (compte neuf = poids 0) + Graphe Sybil OAuth GitHub à venir
- doc reproduction tests + tests fonctionnels acceptée comme dette pré-ouverture externe
