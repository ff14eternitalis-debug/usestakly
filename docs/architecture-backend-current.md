# Architecture backend actuelle

> Version : 1.0
> Dernière mise à jour : 2026-04-23
> Portée : backend vivant de **UseStakly** post-pivot GitHub OSS

## Vue d'ensemble

Le backend UseStakly est une API Rust/Axum centrée sur quatre capacités produit :

- découverte de repos GitHub publics scorés
- watchlist + notifications
- signaux qualité modérés
- exposition MCP pour agents

Le point d'entrée suit la chaîne :

`main.rs` → `config::AppConfig::from_env()` → `db::connect()` → `app::build_app()` → `axum::serve`

## Sous-domaines actifs

### `handlers/`

Responsabilité : I/O HTTP seulement.

Handlers principaux :

- `search` — recherche discovery publique
- `repos_query` — profil repo + recherche détaillée
- `repos_ingestion` — `POST /api/repos/add`
- `repo_signals` — création de signaux et dispute owner
- `repo_viewer` — état viewer spécifique au repo
- `watchlist`
- `notifications`
- `account`
- `admin`
- `agent_tokens`

Le fichier `handlers/repos.rs` n'est plus un gros handler monolithique ; il re-exporte les handlers spécialisés liés au domaine repo.

### `services/`

Responsabilité : logique métier.

Sous-domaines principaux :

- `ingestion/github` — ingestion GitHub REST et normalisation repo
- `repos` — agrégation des profils repo et réponses discovery
- `watchlist`
- `notifications`
- `scheduler` — boucle opt-in refresh + recompute
- `quality/`
- `trust/`

### `services/quality/`

Le scoring a été découpé pour éviter le fichier-orchestrateur unique.

- `formula.rs` — chargement TOML + types de formule
- `compute.rs` — calcul pur du score
- `flags.rs` — consensus et normalisation des flags publics
- `pipeline.rs` — chargement DB, recompute, upsert, notifications
- `capture.rs` — enregistrement des signaux qualité

Règle utile : `compute.rs` et `flags.rs` doivent rester testables sans DB dès que possible.

### `services/trust/`

Responsabilité : réputation, modération, ownership et garde-fous MCP.

- `reputation.rs`
- `repo_owners.rs`
- `signal_reviews.rs`
- `signal_events.rs`
- `agent_token_events.rs`

Cette zone porte les règles de confiance produit. Toute nouvelle logique de modération devrait y entrer plutôt que se disperser dans `repos` ou `quality`.

### `mcp/`

Responsabilité : serveur MCP Streamable HTTP monté à `/mcp`.

- auth Bearer via `agent_tokens`
- tools read : recherche + contexte repo
- tools write : log d'usage + watch repo

## Flux principaux

### Discovery

1. recherche HTTP
2. récupération des repos depuis `external_artifacts`
3. enrichissement qualité via `artifact_scores`
4. réponse frontend ou MCP

### Recompute qualité

1. ingestion/refresh éventuel d'un repo
2. chargement de la formule
3. calcul pur des dimensions
4. résolution des flags publics par consensus
5. upsert `artifact_scores`
6. émission éventuelle de notifications

### Signal actif modéré

1. création du signal repo
2. contrôle réputation / éligibilité
3. review admin si nécessaire (`security_issue`)
4. exposition publique seulement après consensus + règles trust
5. dispute owner possible sans suppression silencieuse

## Frontières à préserver

- `handler` ne doit pas porter la logique métier
- `quality` calcule et orchestre le scoring, mais la confiance sociale reste dans `trust`
- `repos` agrège l'expérience produit repo ; il ne doit pas devenir un fourre-tout de modération
- les tables legacy snippets peuvent rester en base, mais ne sont plus une surface runtime active

## Dette restante

- documentation SQL/data model encore partiellement legacy
- pas encore de doc technique dédiée au parcours notifications scheduler
- réputation v2 et ownership org GitHub privé restent ouverts
