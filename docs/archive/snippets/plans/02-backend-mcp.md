# Phase 2 — Backend & Serveur MCP

> Version : 1.0 — 2026-04-15
> Durée estimée : 2-3 semaines
> Dépendances : Phase 1

## 🎯 Objectif

Développer le **cœur Rust** : authentification, CRUD snippets, serveur MCP avec ses 6 outils, sécurité de base.

## 📋 Livrables

1. Toutes les migrations SQL de [data-model.md](../data-model.md) appliquées
2. Module d'authentification JWT (Argon2id)
3. API REST pour les snippets (CRUD + versions)
4. Serveur MCP JSON-RPC avec 6 outils
5. Middleware : auth, rate-limit, logs, CORS
6. Tests d'intégration couvrant les endpoints critiques

## 🔨 Tâches détaillées

### 2.1 Migrations SQL
- [ ] Créer toutes les migrations depuis [data-model.md](../data-model.md) :
  - [ ] `001_users_and_auth.sql`
  - [ ] `002_snippets_and_versions.sql`
  - [ ] `003_tags.sql`
  - [ ] `004_rule_sets.sql`
  - [ ] `005_projects.sql`
  - [ ] `006_generations.sql`
  - [ ] `007_subscriptions.sql`
  - [ ] `008_community.sql`
  - [ ] `009_indexes_and_triggers.sql`
- [ ] `sqlx migrate run` applique tout sans erreur
- [ ] Peupler `snippet_kinds` au démarrage (seed)

### 2.2 Authentification
- [ ] Module `auth/` : hash Argon2id, vérification, génération JWT Ed25519
- [ ] Endpoints :
  - [ ] `POST /auth/signup` (email + password + username)
  - [ ] `POST /auth/login` → JWT
  - [ ] `POST /auth/refresh`
  - [ ] `GET /auth/me`
- [ ] Middleware `Claims` extractor (Axum)
- [ ] Tests : signup, login, password invalide, token expiré

### 2.3 API REST Snippets
- [ ] `POST /snippets` — créer avec version initiale `1.0.0`
- [ ] `GET /snippets` — liste paginée + filtres (`domain`, `kind`, `language`, `q`)
- [ ] `GET /snippets/:id` — détail + version courante
- [ ] `PATCH /snippets/:id` — modifier les métadonnées (pas le code)
- [ ] `DELETE /snippets/:id`
- [ ] `POST /snippets/:id/versions` — nouvelle version (append-only)
- [ ] `GET /snippets/:id/versions` — historique
- [ ] Calcul automatique du `content_hash` (SHA256)
- [ ] Intégration de l'embedder (stub pour l'instant — la vraie impl en Phase 4)

### 2.4 Serveur MCP (JSON-RPC)
- [ ] Endpoint `POST /mcp/v1/:token` qui reçoit les appels JSON-RPC
- [ ] Implémenter les 6 outils (cf [mcp-protocol.md](../mcp-protocol.md)) :
  - [ ] `search_library`
  - [ ] `get_snippet`
  - [ ] `list_rules`
  - [ ] `check_dependencies`
  - [ ] `propose_new_snippet`
  - [ ] `log_generation`
- [ ] Recherche hybride : embedding cosinus + filtres SQL
- [ ] Contrôle d'accès : un user voit ses snippets + les publics

### 2.5 Sécurité & robustesse
- [ ] Rate limiting (Tower `tower::limit`)
- [ ] CORS configuré proprement (origines whitelistées)
- [ ] Validation des inputs avec `validator` crate
- [ ] Erreurs typées avec `thiserror` → réponses HTTP cohérentes
- [ ] Logs structurés JSON en prod, pretty en dev

### 2.6 Tests
- [ ] Tests unitaires des modules auth, hash
- [ ] Tests d'intégration avec `sqlx::test` (DB éphémère)
- [ ] Tests du serveur MCP avec appels JSON-RPC simulés
- [ ] Snapshot tests pour les réponses API critiques

## ✅ Definition of Done

- [ ] Un utilisateur peut s'inscrire, se connecter, créer un snippet via l'API
- [ ] `POST /mcp/v1/:token` avec un appel `search_library` retourne des résultats pertinents
- [ ] Un snippet privé n'est jamais accessible à un autre utilisateur
- [ ] Un appel sans token (ou token expiré) → 401 JSON bien formé
- [ ] Les tests d'intégration passent en CI sur une DB éphémère
- [ ] Les logs contiennent toutes les infos pour tracer un bug (request ID, user ID, durée)
- [ ] Documentation OpenAPI générée (utipa ou aide-memoire manuel)

## ⚠️ Pièges à éviter

- ❌ Stocker les passwords en clair, même temporairement
- ❌ Utiliser SHA256 pour les passwords (Argon2id obligatoire)
- ❌ Oublier le `ON DELETE CASCADE` — laisse des orphelins
- ❌ Retourner des erreurs détaillées aux clients (fuite d'info sensibles)
- ❌ Mélanger logique métier et SQL dans les handlers → séparer via services

## 📚 Références
- [data-model.md](../data-model.md)
- [mcp-protocol.md](../mcp-protocol.md)
- [tech-stack.md](../tech-stack.md)
