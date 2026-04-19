# Projet K — MVP File-by-File Checklist

> Version : 1.0 — 2026-04-18
> Statut : checklist d'implémentation détaillée
> Dépendance principale : `docs/plans/mvp-one-shot-blueprint.md`

## But

Cette checklist transforme le blueprint MVP en plan d'exécution **fichier par fichier**.

Chaque entrée répond à 4 questions :
- quel fichier doit exister
- à quoi il sert
- ce qu'il doit contenir
- quand on peut le considérer comme terminé

Ce document est conçu pour servir directement à une implémentation agentique ou humaine.

---

## 1. Racine du repo

### `README.md`
- Rôle : entrée principale du repo
- Doit contenir :
  - pitch produit court
  - stack MVP
  - structure du repo
  - quickstart local
  - liens vers la doc
- Done quand :
  - un nouveau contributeur peut lancer le projet localement en suivant uniquement ce fichier

### `.gitignore`
- Rôle : ignorer artefacts dev/build/secrets
- Doit contenir :
  - `node_modules/`
  - `dist/`
  - `.env`
  - `.env.*`
  - `target/`
  - `.DS_Store`
  - fichiers IDE
- Done quand :
  - aucun secret ni build local n'apparaît dans `git status`

### `.editorconfig`
- Rôle : cohérence de formatage
- Doit contenir :
  - UTF-8
  - LF
  - fin de ligne finale
  - indentation espaces
- Done quand :
  - backend et frontend partagent des règles de base cohérentes

### `.env.example`
- Rôle : documentation centralisée des variables d'environnement
- Doit contenir :
  - variables backend (`DATABASE_URL`, `APP_*`, `APP_SESSION_SECRET`, `GITHUB_CLIENT_*`, `DISCORD_CLIENT_*`)
  - variables frontend (`VITE_API_BASE_URL`)
  - variables du dev user fallback (`DEV_USER_*`)
  - URLs locales par défaut
- Done quand :
  - un dev peut copier ce fichier en `.env` sans deviner une variable

### `docker-compose.yml`
- Rôle : environnement local reproductible
- Doit contenir :
  - PostgreSQL
  - extension pgvector
  - volume persistant
  - port exposé
- Done quand :
  - `docker compose up -d` démarre une DB utilisable par `sqlx`

---

## 2. CI / automatisation

### `.github/workflows/ci.yml`
- Rôle : validation continue
- Doit contenir :
  - job backend
  - job frontend
  - migration / build / tests
- Doit exécuter :
  - Rust fmt
  - Rust clippy
  - Rust test
  - install frontend
  - lint frontend
  - build frontend
  - tests frontend
- Done quand :
  - une PR vide passe sur un projet bootstrapé

---

## 3. Backend — fichiers racine

### `backend/Cargo.toml`
- Rôle : manifeste Rust
- Doit contenir :
  - `axum`
  - `tokio`
  - `sqlx`
  - `serde`
  - `serde_json`
  - `uuid`
  - `tracing`
  - `tower`
  - `tower-http`
  - `anyhow`
  - `thiserror`
  - `jsonwebtoken` ou équivalent JWKS
  - `tree-sitter`
  - `fastembed`
- Done quand :
  - `cargo check` passe

### `backend/Dockerfile`
- Rôle : build image backend
- Doit contenir :
  - build multi-stage
  - cache des dépendances Rust
  - image runtime légère
  - port backend exposé
- Done quand :
  - l'image produit un binaire lançable

---

## 4. Backend — point d'entrée et bootstrap

### `backend/src/main.rs`
- Rôle : point d'entrée du serveur
- Doit contenir :
  - chargement config
  - init tracing
  - init pool DB
  - construction app Axum
  - bind sur host/port
- Done quand :
  - le serveur démarre et répond sur `/health`

### `backend/src/app/mod.rs`
- Rôle : constructeur principal de l'application Axum
- Doit contenir :
  - composition des routers
  - injection du state global
  - middlewares globaux
- Done quand :
  - toutes les routes passent par un point de composition unique

### `backend/src/config/mod.rs`
- Rôle : configuration centralisée
- Doit contenir :
  - struct `AppConfig`
  - lecture env
  - validation minimale des variables critiques
- Done quand :
  - le process refuse de démarrer si une variable essentielle manque

### `backend/src/telemetry/mod.rs`
- Rôle : logs et observabilité
- Doit contenir :
  - init tracing subscriber
  - niveau de log configurable
  - format dev/prod
- Done quand :
  - les logs backend sont lisibles en local et exploitables en CI/prod

---

## 5. Backend — DB et migrations

### `backend/migrations/0001_init_extensions.sql`
- Rôle : extensions PostgreSQL
- Doit contenir :
  - `uuid-ossp`
  - `vector`
  - `pg_trgm`
- Done quand :
  - la DB supporte UUID, embeddings et trigram

### `backend/migrations/0002_users_auth.sql`
- Rôle : identité locale
- Doit contenir :
  - `users`
  - `auth_identities` avec `(provider, provider_user_id)` unique, `provider ∈ {'github', 'discord'}`
- Done quand :
  - un callback OAuth GitHub ou Discord peut upsert l'utilisateur applicatif sans doublon

### `backend/migrations/0003_libraries.sql`
- Rôle : bibliothèques adressables
- Doit contenir :
  - type `visibility`
  - type `trust_level`
  - table `libraries`
- Done quand :
  - une bibliothèque peut être créée avec `slug` unique

### `backend/migrations/0004_snippet_kinds.sql`
- Rôle : dictionnaire des kinds
- Doit contenir :
  - `snippet_domain`
  - `snippet_kinds`
  - seed MVP
- Done quand :
  - les kinds MVP sont disponibles en base

### `backend/migrations/0005_snippets.sql`
- Rôle : entité snippet
- Doit contenir :
  - table `snippets`
  - contraintes `UNIQUE(library_id, slug)`
- Done quand :
  - un snippet ne peut exister que dans une bibliothèque définie

### `backend/migrations/0006_snippet_versions.sql`
- Rôle : versioning append-only
- Doit contenir :
  - table `snippet_versions`
  - `content_hash`
  - `embedding`
  - `risk_level`
- Done quand :
  - un snippet peut avoir plusieurs versions immuables

### `backend/migrations/0007_tags.sql`
- Rôle : tags
- Doit contenir :
  - `tags`
  - `snippet_tags`
- Done quand :
  - les tags sont normalisés et requêtables

### `backend/migrations/0008_rules.sql`
- Rôle : règles d'assemblage
- Doit contenir :
  - `rule_sets`
- Done quand :
  - une bibliothèque peut référencer un rule set

### `backend/migrations/0009_permissions_reports.sql`
- Rôle : permissions et modération
- Doit contenir :
  - `library_permissions`
  - `snippet_reports`
- Done quand :
  - le système peut filtrer l'accès et préparer la modération

### `backend/migrations/0010_generations.sql`
- Rôle : journalisation des assemblages
- Doit contenir :
  - `generations`
- Done quand :
  - une génération peut être historisée avec sa provenance

### `backend/migrations/0011_indexes_views.sql`
- Rôle : performance et vues utiles
- Doit contenir :
  - trigram indexes
  - vector index
  - vue `v_snippets_current`
- Done quand :
  - recherche et résolution utilisent des index exploitables

### `backend/src/db/mod.rs`
- Rôle : point d'accès DB
- Doit contenir :
  - création du pool SQLx
  - helpers transactionnels
- Done quand :
  - tous les handlers backend consomment la DB via ce module

### `backend/src/db/queries/`
- Rôle : requêtes SQL structurées
- Doit contenir :
  - fichiers séparés par domaine (`libraries.rs`, `snippets.rs`, `search.rs`, etc.)
- Done quand :
  - les handlers n'embarquent pas directement de logique SQL complexe

---

## 6. Backend — auth

### `backend/src/auth/mod.rs`
- Rôle : façade auth (OAuth direct GitHub + Discord, session JWT cookie)
- Doit contenir :
  - routes `start` + `callback` pour GitHub et Discord
  - échange `code → access_token` via `reqwest`
  - fetch profil provider (`/user` GitHub, `/users/@me` Discord)
  - upsert `users` + `auth_identities`
  - génération + validation du JWT de session signé avec `APP_SESSION_SECRET`
  - helper `HttpOnly` cookie `usestakly_session`
  - extracteur `CurrentUser` qui lit le cookie et tombe en 401 sinon
  - fallback dev user quand `APP_SESSION_SECRET` + un couple `*_CLIENT_ID/SECRET` est absent
- Done quand :
  - `GET /api/me` peut être protégé proprement
  - un utilisateur peut se connecter via GitHub **ou** Discord sans aucun SDK côté frontend

### `backend/src/auth/jwks.rs`
- Rôle : validation des JWT via JWKS
- Doit contenir :
  - récupération / cache des clés publiques
  - validation `issuer`, `aud`, `sub`
- Done quand :
  - un token invalide est rejeté correctement

### `backend/src/auth/extractor.rs`
- Rôle : extracteur Axum utilisateur
- Doit contenir :
  - extraction bearer token
  - construction `CurrentUser`
- Done quand :
  - les handlers protégés n'ont pas à reparser le token

### `backend/src/services/user_sync_service.rs`
- Rôle : synchronisation utilisateur auth -> modèle local
- Doit contenir :
  - création `users`
  - création / mise à jour `auth_identities`
- Done quand :
  - première connexion GitHub crée le profil applicatif

---

## 7. Backend — domain models

### `backend/src/domain/library.rs`
- Rôle : types métier bibliothèque
- Doit contenir :
  - `Library`
  - `CreateLibraryInput`
  - `UpdateLibraryInput`
  - `LibraryVisibility`
  - `TrustLevel`
- Done quand :
  - la couche bibliothèque a des types centralisés et réutilisables

### `backend/src/domain/snippet.rs`
- Rôle : types métier snippet
- Doit contenir :
  - `Snippet`
  - `SnippetVersion`
  - `CreateSnippetInput`
  - `CreateSnippetVersionInput`
  - `RiskLevel`
- Done quand :
  - les handlers et services consomment les mêmes types

### `backend/src/domain/search.rs`
- Rôle : types recherche / résolution
- Doit contenir :
  - `SearchInput`
  - `ResolveReferenceInput`
  - `SearchScope`
  - `AssemblyMode`
- Done quand :
  - les scopes et modes MCP sont définis au même endroit

### `backend/src/domain/generation.rs`
- Rôle : types de plan d'assemblage et génération
- Doit contenir :
  - `AssemblePlanInput`
  - `AssemblyStep`
  - `GenerationLogInput`
- Done quand :
  - le contrat MCP s'appuie sur des types métier explicites

---

## 8. Backend — services métier

### `backend/src/services/libraries_service.rs`
- Rôle : logique CRUD bibliothèques
- Doit contenir :
  - création
  - édition
  - visibilité
  - gestion bibliothèque par défaut
- Done quand :
  - les règles métier bibliothèque ne vivent pas dans les handlers

### `backend/src/services/snippets_service.rs`
- Rôle : logique CRUD snippets
- Doit contenir :
  - création initiale
  - ajout version
  - calcul hash
  - gestion visibilité
- Done quand :
  - un snippet versionné est manipulable sans logique dupliquée

### `backend/src/services/reference_service.rs`
- Rôle : résolution des références canoniques
- Doit contenir :
  - parsing `@library:snippet@version`
  - résolution DB
  - fallback version courante si version absente
- Done quand :
  - une référence explicite retourne une cible unique ou une erreur claire

### `backend/src/services/search_service.rs`
- Rôle : recherche hybride
- Doit contenir :
  - filtre par scopes
  - filtre par bibliothèques
  - ranking textuel + vectoriel + trust
- Done quand :
  - `POST /api/search` retourne des résultats triés proprement

### `backend/src/services/generation_service.rs`
- Rôle : journalisation des générations
- Doit contenir :
  - persistance des générations
  - format de provenance
- Done quand :
  - chaque assemblage est historisé de manière exploitable

---

## 9. Backend — sécurité / safety

### `backend/src/security/mod.rs`
- Rôle : façade sécurité
- Doit contenir :
  - sanitize texte libre
  - analyse de risque basique
  - policy engine auto
- Done quand :
  - les règles safety MVP passent par un point central

### `backend/src/security/sanitize.rs`
- Rôle : détection de prompt injection textuelle
- Doit contenir :
  - patterns à détecter
  - normalisation de texte
  - score ou flags simples
- Done quand :
  - descriptions / notes dangereuses sont signalées

### `backend/src/security/risk_analysis.rs`
- Rôle : classification de risque snippet
- Doit contenir :
  - règles `safe`
  - règles `review_required`
  - règles `restricted`
- Done quand :
  - un snippet shell ou dangereux n'est pas classé `safe`

### `backend/src/security/policy_engine.rs`
- Rôle : règles d'exclusion pour MCP mode auto
- Doit contenir :
  - exclusion `flagged`
  - exclusion `quarantined`
  - exclusion `restricted` en auto
- Done quand :
  - le mode `auto` ne consomme pas de contenu risqué

---

## 10. Backend — recherche / détection

### `backend/src/search/mod.rs`
- Rôle : façade recherche
- Doit contenir :
  - embedding
  - ranking
  - matching référence / texte
- Done quand :
  - les recherches sont centralisées

### `backend/src/search/embedder.rs`
- Rôle : intégration `fastembed`
- Doit contenir :
  - chargement modèle
  - génération embedding
- Done quand :
  - l'embedding d'un snippet peut être calculé à la création de version

### `backend/src/search/reference_parser.rs`
- Rôle : parser de référence canonique
- Doit contenir :
  - parse bibliothèque
  - parse snippet
  - parse version
- Done quand :
  - les erreurs de format sont détectées proprement

### `backend/src/search/detection.rs`
- Rôle : pipeline de détection
- Doit contenir :
  - détection langage
  - mapping domaine
  - détection kind
  - détection framework
  - suggestion tags
- Done quand :
  - un snippet collé reçoit des suggestions exploitables

---

## 11. Backend — handlers REST

### `backend/src/handlers/health.rs`
- Rôle : santé du service
- Endpoint :
  - `GET /health`
- Done quand :
  - renvoie `200 OK`

### `backend/src/handlers/me.rs`
- Rôle : utilisateur courant
- Endpoint :
  - `GET /api/me`
- Done quand :
  - un utilisateur authentifié récupère son profil local

### `backend/src/handlers/libraries.rs`
- Rôle : endpoints bibliothèques
- Endpoints :
  - `POST /api/libraries`
  - `GET /api/libraries`
  - `GET /api/libraries/:libraryId`
  - `PATCH /api/libraries/:libraryId`
- Done quand :
  - CRUD de base disponible et protégé

### `backend/src/handlers/snippets.rs`
- Rôle : endpoints snippets
- Endpoints :
  - `POST /api/snippets`
  - `GET /api/snippets`
  - `GET /api/snippets/:snippetId`
  - `PATCH /api/snippets/:snippetId`
  - `DELETE /api/snippets/:snippetId`
- Done quand :
  - un snippet peut être créé, lu, édité, archivé

### `backend/src/handlers/snippet_versions.rs`
- Rôle : endpoints versioning
- Endpoints :
  - `POST /api/snippets/:snippetId/versions`
- Done quand :
  - une nouvelle version append-only peut être ajoutée

### `backend/src/handlers/search.rs`
- Rôle : recherche et résolution
- Endpoints :
  - `GET /api/resolve`
  - `POST /api/search`
- Done quand :
  - recherche et résolution sont accessibles au frontend

### `backend/src/handlers/generations.rs`
- Rôle : historique d'assemblage
- Endpoints :
  - `GET /api/generations`
  - `GET /api/generations/:generationId`
- Done quand :
  - l'historique est consultable

---

## 12. Backend — MCP

### `backend/src/mcp/mod.rs`
- Rôle : point d'entrée MCP
- Doit contenir :
  - registry des tools
  - dispatch
- Done quand :
  - le serveur MCP peut router les appels vers les bons handlers

### `backend/src/mcp/router.rs`
- Rôle : route HTTP JSON-RPC MCP
- Doit contenir :
  - endpoint `/mcp/v1`
  - validation requête / réponse
- Done quand :
  - un client MCP peut appeler le serveur localement

### `backend/src/mcp/tools/resolve_reference.rs`
- Rôle : tool `resolve_reference`
- Done quand :
  - une référence canonique est résolue exactement

### `backend/src/mcp/tools/search_library.rs`
- Rôle : tool `search_library`
- Done quand :
  - le tool respecte scopes, bibliothèques, mode et ranking

### `backend/src/mcp/tools/get_snippet.rs`
- Rôle : tool `get_snippet`
- Done quand :
  - code + compatibilité + provenance sont retournés

### `backend/src/mcp/tools/check_dependencies.rs`
- Rôle : tool `check_dependencies`
- Done quand :
  - l'arbre de dépendances est retourné sans ambiguïté

### `backend/src/mcp/tools/assemble_plan.rs`
- Rôle : tool `assemble_plan`
- Done quand :
  - le serveur renvoie un plan d'assemblage structuré

### `backend/src/mcp/tools/list_rules.rs`
- Rôle : tool `list_rules`
- Done quand :
  - le rule set applicable est renvoyé

### `backend/src/mcp/tools/log_generation.rs`
- Rôle : tool `log_generation`
- Done quand :
  - une génération peut être persistée avec provenance

---

## 13. Frontend — fichiers racine

### `frontend/package.json`
- Rôle : manifeste frontend
- Doit contenir :
  - React
  - Vite
  - Tailwind v4
  - TanStack Query
  - router choisi
  - Zustand
  - Monaco
  - test runner
- Done quand :
  - `npm install` ou `pnpm install` puis `build` passe

### `frontend/vite.config.ts`
- Rôle : configuration Vite
- Doit contenir :
  - plugin React
  - plugin Tailwind v4
  - alias éventuels
- Done quand :
  - `npm run dev` démarre correctement

### `frontend/src/main.tsx`
- Rôle : bootstrap frontend
- Doit contenir :
  - mounting React
  - providers globaux
- Done quand :
  - l'app rend correctement la racine

### `frontend/src/app/providers.tsx`
- Rôle : providers frontend
- Doit contenir :
  - QueryClientProvider
  - Auth/session provider
  - Router provider
- Done quand :
  - tous les providers sont centralisés

---

## 14. Frontend — lib / clients

### `frontend/src/lib/api-client.ts`
- Rôle : client HTTP vers API Rust
- Doit contenir :
  - base URL
  - injection bearer token
  - gestion erreurs standard
- Done quand :
  - tous les appels REST passent par ce client

### ~~`frontend/src/lib/supabase.ts`~~ — supprimé du périmètre

Pas de SDK d'auth côté frontend. Le login est déclenché par un simple lien `<a href="{VITE_API_BASE_URL}/api/auth/{github|discord}/start">` et la session est portée par un cookie posé par le backend. Si un fichier stub existe dans le repo pour raisons historiques, il doit être retiré ou vidé de toute dépendance à `@supabase/supabase-js`.

### `frontend/src/lib/query-client.ts`
- Rôle : config TanStack Query
- Doit contenir :
  - retries
  - stale time cohérent
- Done quand :
  - le cache frontend est stable et prévisible

---

## 15. Frontend — state

### `frontend/src/state/auth-store.ts`
- Rôle : état auth local
- Doit contenir :
  - session
  - user
  - token courant si nécessaire
- Done quand :
  - le frontend sait si l'utilisateur est connecté

### `frontend/src/state/ui-store.ts`
- Rôle : état UI local
- Doit contenir :
  - bibliothèque sélectionnée
  - filtres ouverts
  - vue courante
- Done quand :
  - Zustand reste limité à l'état UI non serveur

---

## 16. Frontend — routes

### `frontend/src/routes/login.tsx`
- Rôle : page login
- Doit contenir :
  - CTA GitHub
  - feedback erreur auth
- Done quand :
  - l'utilisateur peut démarrer le flow GitHub

### `frontend/src/routes/dashboard.tsx`
- Rôle : page d'accueil connectée
- Doit contenir :
  - résumé bibliothèques
  - snippets récents
  - générations récentes
- Done quand :
  - le dashboard reflète l'activité de base

### `frontend/src/routes/libraries.tsx`
- Rôle : liste des bibliothèques
- Done quand :
  - l'utilisateur voit et gère ses bibliothèques

### `frontend/src/routes/library-detail.tsx`
- Rôle : détail d'une bibliothèque
- Done quand :
  - on peut voir ses snippets et ses infos

### `frontend/src/routes/snippet-detail.tsx`
- Rôle : détail d'un snippet
- Done quand :
  - version courante, versions, provenance et métadonnées sont visibles

### `frontend/src/routes/search.tsx`
- Rôle : recherche / résolution
- Done quand :
  - recherche texte et champ de référence coexistent

### `frontend/src/routes/generations.tsx`
- Rôle : historique des assemblages
- Done quand :
  - l'utilisateur voit les résultats précédents

---

## 17. Frontend — features

### `frontend/src/features/auth/`
- Fichiers attendus :
  - `api.ts`
  - `hooks.ts`
  - `components/LoginButton.tsx`
- Done quand :
  - tout le flow auth est encapsulé ici

### `frontend/src/features/libraries/`
- Fichiers attendus :
  - `api.ts`
  - `hooks.ts`
  - `components/LibraryForm.tsx`
  - `components/LibraryList.tsx`
  - `components/LibraryVisibilityBadge.tsx`
- Done quand :
  - toute la gestion des bibliothèques est isolée dans cette feature

### `frontend/src/features/snippets/`
- Fichiers attendus :
  - `api.ts`
  - `hooks.ts`
  - `components/SnippetForm.tsx`
  - `components/SnippetEditor.tsx`
  - `components/SnippetMetadataPanel.tsx`
  - `components/CanonicalReference.tsx`
- Done quand :
  - création / édition / affichage d'un snippet sont complets

### `frontend/src/features/search/`
- Fichiers attendus :
  - `api.ts`
  - `hooks.ts`
  - `components/SearchBar.tsx`
  - `components/ReferenceResolveInput.tsx`
  - `components/SearchResults.tsx`
- Done quand :
  - un utilisateur peut chercher ou résoudre explicitement depuis l'UI

### `frontend/src/features/generations/`
- Fichiers attendus :
  - `api.ts`
  - `hooks.ts`
  - `components/GenerationList.tsx`
  - `components/GenerationDetail.tsx`
- Done quand :
  - l'historique d'assemblage est consultable

---

## 18. Frontend — composants structurants

### `frontend/src/components/layout/AppShell.tsx`
- Rôle : layout principal
- Doit contenir :
  - nav principale
  - zone contenu
- Done quand :
  - toutes les pages connectées partagent une structure cohérente

### `frontend/src/components/editor/MonacoCodeEditor.tsx`
- Rôle : éditeur code multi-langage
- Doit contenir :
  - langage dynamique
  - valeur contrôlée
  - options minimales utiles
- Done quand :
  - l'utilisateur peut éditer confortablement le code d'un snippet

### `frontend/src/components/common/TrustBadge.tsx`
- Rôle : afficher le niveau de confiance
- Done quand :
  - l'utilisateur identifie le trust level d'un résultat public

### `frontend/src/components/common/VisibilityBadge.tsx`
- Rôle : afficher visibilité
- Done quand :
  - privé/public est visible partout

### `frontend/src/components/common/ProvenancePanel.tsx`
- Rôle : afficher provenance complète
- Done quand :
  - une brique issue d'une bibliothèque externe n'est jamais opaque

---

## 19. Fichiers de test minimaux

### Backend

#### `backend/tests/health_test.rs`
- vérifie `/health`

#### `backend/tests/auth_me_test.rs`
- vérifie `GET /api/me`

#### `backend/tests/libraries_crud_test.rs`
- vérifie CRUD minimal bibliothèques

#### `backend/tests/snippets_crud_test.rs`
- vérifie CRUD minimal snippets

#### `backend/tests/resolve_reference_test.rs`
- vérifie parsing et résolution `@library:snippet@version`

#### `backend/tests/search_test.rs`
- vérifie recherche texte / filtres / visibilité

#### `backend/tests/mcp_tools_test.rs`
- vérifie tools MCP clés

### Frontend

#### `frontend/src/features/auth/__tests__/login-flow.test.tsx`
- vérifie CTA login

#### `frontend/src/features/libraries/__tests__/library-form.test.tsx`
- vérifie création bibliothèque

#### `frontend/src/features/snippets/__tests__/snippet-form.test.tsx`
- vérifie création snippet

#### `frontend/src/features/search/__tests__/reference-resolve.test.tsx`
- vérifie champ de référence explicite

### E2E

#### `frontend/e2e/mvp-flow.spec.ts`
- vérifie :
  - login
  - create library
  - create snippet
  - publish
  - resolve
  - search

---

## 20. Ordre conseillé de livraison

1. fichiers racine repo
2. backend bootstrap
3. migrations
4. auth backend
5. CRUD libraries
6. CRUD snippets
7. frontend auth
8. frontend libraries
9. frontend snippets
10. resolve/search backend
11. resolve/search frontend
12. MCP
13. safety minimum
14. tests E2E

---

## 21. Definition of Done du document

Cette checklist est utilisable si :
- chaque fichier critique du MVP a un propriétaire logique
- son rôle est explicite
- son contenu attendu est défini
- son critère de fin est vérifiable

Si un fichier du repo MVP n'apparaît pas ici et qu'il est indispensable, il faut ajouter sa ligne avant implémentation.
