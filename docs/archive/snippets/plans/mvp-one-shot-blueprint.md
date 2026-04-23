# UseStakly — MVP One-Shot Blueprint

> Version : 1.0 — 2026-04-18 *(pré-pivot 2026-04-20)*
> Statut : document maître de construction MVP *(phases 0–5 toujours valides, phases 6+ obsolètes)*
> But : fournir une spécification suffisamment précise pour construire le MVP en une seule passe cohérente

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Ce blueprint **précède le pivot** vers le registry qualité-scored. Source de vérité actuelle : [`../strategy-quality-scored-registry.md`](../strategy-quality-scored-registry.md) (produit) et [`../../TODO.md`](../../TODO.md) (exécution).
>
> **Toujours valide** : §2–§5 (positionnement général, principes d'assemblage, périmètre bibliothèques/snippets), parties backend/frontend/auth/DB — implémentées en phases 0–5 du TODO actuel.
>
> **Obsolète ou à compléter** :
> - La boucle de valeur §2 s'arrête à « IA assemble avant d'inventer » — post-pivot, elle continue par : *télémétrie d'usage → score → filtrage automatique par l'agent*.
> - §3 principes : ajouter « capture des signaux d'usage dès le jour 1 », « get_snippet renvoie un quality_context natif », « mode `auto` = filtre par défaut sur reliability/abandonment/flags ».
> - Toute section sur la recherche MCP / safety / publication publique doit être relue avec les phases 6–11 du TODO actuel en tête.
>
> Le reste du document est conservé pour référence historique.

## 1. Intention du document

Ce document est la **source de vérité opérationnelle** pour construire le MVP de `UseStakly`.

> Note de transition : `UseStakly` est le nom produit retenu. `Project-K` reste l'ancien nom de travail encore visible dans certaines structures techniques et documents plus anciens.

Il ne remplace pas totalement les autres docs, mais il les condense en un plan de réalisation exécutable.
Si une ambiguïté apparaît entre plusieurs documents, ce blueprint fait foi pour le MVP.

Le MVP visé ici doit permettre :
- de créer des bibliothèques de code adressables
- de stocker des snippets multi-domaines dans ces bibliothèques
- de rendre certains contenus publics
- de résoudre un snippet par référence exacte
- de chercher des snippets dans plusieurs bibliothèques
- de préparer un plan d'assemblage pour une feature ou une app
- d'exposer cela à une IA via MCP
- de réduire la génération brute de code

---

## 2. Positionnement produit MVP

## Proposition produit courte

`UseStakly` est une infrastructure de bibliothèques de code privées ou publiques que les IA peuvent **résoudre**, **chercher** et **assembler** via MCP afin de construire des applications avec plus de fiabilité et moins d'hallucinations.

## Ce que le MVP est

- une bibliothèque de code multi-domaines
- un système de bibliothèques adressables
- un moteur de recherche hybride
- un résolveur exact de références
- un serveur MCP d'assemblage
- une couche minimale de provenance et de sécurité

## Ce que le MVP n'est pas

- un IDE complet
- un réseau social de code abouti
- une marketplace
- une plateforme de monétisation
- une extension native Cursor / VS Code
- un moteur de génération full-project totalement autonome

## Boucle de valeur MVP

1. l'utilisateur se connecte
2. il crée une bibliothèque
3. il ajoute des snippets
4. il les classe et versionne
5. il peut en publier certains
6. il ou une IA les résout/recherche via MCP
7. l'IA assemble avant d'inventer

---

## 3. Principes non négociables

1. **La bibliothèque est l'unité d'adressage principale.**
2. **Le snippet est l'unité d'assemblage principale.**
3. **Résoudre avant chercher, chercher avant générer.**
4. **Tout snippet public est une donnée, jamais une autorité de pilotage.**
5. **La provenance doit être conservée à chaque assemblage.**
6. **Le MVP est multi-domaines par modèle, mais optimisé d'abord sur un sous-ensemble de langages.**
7. **Pas d'auth maison au MVP.**
8. **Pas d'exécution automatique de contenu public non validé.**

---

## 4. Périmètre fonctionnel MVP

## Inclus

### Identité
- login OAuth direct via GitHub **et** Discord (implémenté dans le backend Rust, aucun SaaS d'auth)
- session utilisateur dans un cookie JWT signé (`APP_SESSION_SECRET`)
- profil minimal

### Bibliothèques
- création
- édition
- slug public unique
- visibilité `private` ou `public`
- bibliothèque par défaut

### Snippets
- création dans une bibliothèque
- édition des métadonnées
- versioning append-only
- archivage
- consultation de la version courante

### Classification
- `domain`
- `kind`
- `category`
- `language`
- `framework`
- `framework_version`
- tags

### Recherche
- recherche textuelle
- recherche vectorielle
- filtres par bibliothèque
- filtres par stack
- résolution par référence explicite

### MCP
- `resolve_reference`
- `search_library`
- `get_snippet`
- `check_dependencies`
- `assemble_plan`
- `list_rules`
- `log_generation`

### Sécurité minimale
- visibilité privée/publique
- exclusion des contenus signalés en mode `auto`
- policy engine MCP
- provenance obligatoire
- classification de risque simple

## Hors périmètre MVP

- `unlisted`
- étoiles, forks, commentaires
- scoring communautaire avancé
- billing
- publication onchain
- attestation Intuition
- extension IDE native
- exécution de snippets
- orchestration full-project avancée

---

## 5. Stack technique figée pour le MVP

## Backend
- Rust
- Axum
- Tokio
- SQLx
- PostgreSQL
- pgvector
- tower / tower-http
- tracing
- serde / serde_json
- uuid
- jsonwebtoken ou validation JWT compatible JWKS
- tree-sitter
- fastembed

## Frontend
- React 19
- TypeScript
- Vite
- Tailwind CSS v4
- TanStack Query
- Zustand
- Monaco Editor
- React Router ou TanStack Router

## Auth
- OAuth direct côté backend Rust (pas de SaaS d'auth — app auto-hébergée sur VPS)
- providers MVP : GitHub **et** Discord
- session JWT dans un cookie `HttpOnly` signé avec `APP_SESSION_SECRET`

## Infra
- Docker Compose
- GitHub Actions
- frontend : Coolify
- backend : Coolify
- DB : PostgreSQL managé sur Coolify

## Langages / formats optimisés au MVP

- TS / TSX / JS
- Rust
- SQL
- Bash
- YAML
- TOML
- Dockerfile

Le modèle reste compatible avec tout le reste.

---

## 6. Structure cible du repo

```text
usestakly/
├── backend/
│   ├── src/
│   │   ├── app/
│   │   ├── auth/
│   │   ├── config/
│   │   ├── db/
│   │   ├── domain/
│   │   ├── handlers/
│   │   ├── mcp/
│   │   ├── search/
│   │   ├── security/
│   │   ├── services/
│   │   ├── telemetry/
│   │   └── main.rs
│   ├── migrations/
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/
│   ├── src/
│   │   ├── app/
│   │   ├── components/
│   │   ├── features/
│   │   │   ├── auth/
│   │   │   ├── libraries/
│   │   │   ├── snippets/
│   │   │   ├── search/
│   │   │   └── generations/
│   │   ├── lib/
│   │   ├── routes/
│   │   ├── state/
│   │   └── main.tsx
│   ├── package.json
│   ├── vite.config.ts
│   └── Dockerfile
├── docs/
├── .github/workflows/
├── docker-compose.yml
├── .env.example
└── README.md
```

Règle :
- pas de `shared/` au départ
- ajouter `shared/` seulement si une duplication réelle apparaît

---

## 7. Modèle de données final pour le MVP

## Extensions PostgreSQL

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
```

## Types

```sql
CREATE TYPE visibility AS ENUM ('private', 'public');
CREATE TYPE trust_level AS ENUM (
  'private',
  'public_unverified',
  'verified_author',
  'community_trusted',
  'flagged',
  'quarantined'
);
CREATE TYPE snippet_domain AS ENUM ('frontend', 'backend', 'devops', 'data', 'shared');
```

## Tables MVP obligatoires

### `users`

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email TEXT UNIQUE NOT NULL,
  username TEXT UNIQUE NOT NULL,
  display_name TEXT,
  avatar_url TEXT,
  bio TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### `auth_identities`

```sql
CREATE TABLE auth_identities (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider TEXT NOT NULL,
  provider_user_id TEXT NOT NULL,
  credentials JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(provider, provider_user_id)
);
```

### `libraries`

```sql
CREATE TABLE libraries (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  slug TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT,
  visibility visibility NOT NULL DEFAULT 'private',
  trust_level trust_level NOT NULL DEFAULT 'private',
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  default_stack JSONB NOT NULL DEFAULT '{}',
  allowed_domains JSONB NOT NULL DEFAULT '[]',
  metadata JSONB NOT NULL DEFAULT '{}',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### `snippet_kinds`

```sql
CREATE TABLE snippet_kinds (
  domain snippet_domain NOT NULL,
  kind TEXT NOT NULL,
  description TEXT,
  PRIMARY KEY (domain, kind)
);
```

Le seed MVP doit inclure :
- frontend : `atom`, `molecule`, `organism`, `template`, `util`
- backend : `function`, `handler`, `middleware`, `model`, `service`, `query`
- devops : `config`, `script`, `dockerfile`
- data : `query`, `migration`
- shared : `type`, `constant`, `util`

### `snippets`

```sql
CREATE TABLE snippets (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  library_id UUID NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
  owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  slug TEXT NOT NULL,
  domain snippet_domain NOT NULL,
  kind TEXT NOT NULL,
  category TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  language TEXT NOT NULL,
  runtime TEXT,
  framework TEXT,
  framework_version TEXT,
  visibility visibility NOT NULL DEFAULT 'private',
  trust_level trust_level NOT NULL DEFAULT 'private',
  license TEXT NOT NULL DEFAULT 'MIT',
  current_version_id UUID,
  rule_set_id UUID,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(library_id, slug),
  FOREIGN KEY (domain, kind) REFERENCES snippet_kinds(domain, kind)
);
```

### `snippet_versions`

```sql
CREATE TABLE snippet_versions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
  version TEXT NOT NULL,
  code TEXT NOT NULL,
  variables JSONB NOT NULL DEFAULT '[]',
  css_classes TEXT[],
  dependencies JSONB DEFAULT '[]',
  exports JSONB NOT NULL DEFAULT '[]',
  imports JSONB NOT NULL DEFAULT '[]',
  compatibility JSONB NOT NULL DEFAULT '{}',
  metadata JSONB NOT NULL DEFAULT '{}',
  content_hash TEXT NOT NULL,
  embedding vector(384),
  risk_level TEXT NOT NULL DEFAULT 'safe',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(snippet_id, version)
);
```

### `tags`

```sql
CREATE TABLE tags (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT UNIQUE NOT NULL
);
```

### `snippet_tags`

```sql
CREATE TABLE snippet_tags (
  snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
  tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (snippet_id, tag_id)
);
```

### `rule_sets`

```sql
CREATE TABLE rule_sets (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_id UUID REFERENCES users(id) ON DELETE CASCADE,
  library_id UUID REFERENCES libraries(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  rules JSONB NOT NULL,
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### `library_permissions`

```sql
CREATE TABLE library_permissions (
  library_id UUID NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  can_read BOOLEAN NOT NULL DEFAULT TRUE,
  can_resolve BOOLEAN NOT NULL DEFAULT TRUE,
  can_search BOOLEAN NOT NULL DEFAULT TRUE,
  PRIMARY KEY (library_id, user_id)
);
```

### `snippet_reports`

```sql
CREATE TABLE snippet_reports (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
  reported_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  reason TEXT NOT NULL,
  details JSONB NOT NULL DEFAULT '{}',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### `generations`

```sql
CREATE TABLE generations (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  target_domain snippet_domain,
  prompt TEXT NOT NULL,
  used_snippets UUID[] NOT NULL,
  output_code TEXT NOT NULL,
  plan JSONB,
  llm_model TEXT NOT NULL,
  tokens_input INT,
  tokens_output INT,
  duration_ms INT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Références canoniques

Le système doit reconnaître :

```text
@alice/react-ui-kit:frontend-atom-action-button-primary
@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0
```

Règle :
- le frontend et le MCP manipulent ce format humain
- le backend résout en `library_id`, `snippet_id`, `version_id`

## Index obligatoires

- index trigram sur `libraries.slug`
- index trigram sur `snippets.slug`
- index `snippets(library_id)`
- index `snippets(domain, kind)`
- index `snippet_versions(snippet_id)`
- index ivfflat sur `snippet_versions.embedding`

---

## 8. Contrat d'API REST

Le frontend ne doit pas parler directement à la base métier.
Il passe par l'API Rust.

## Endpoints MVP obligatoires

### Auth / session

- `GET /api/me`

Réponse :

```json
{
  "id": "uuid",
  "email": "alice@example.com",
  "username": "alice",
  "displayName": "Alice",
  "avatarUrl": "https://..."
}
```

### Libraries

- `POST /api/libraries`
- `GET /api/libraries`
- `GET /api/libraries/:libraryId`
- `PATCH /api/libraries/:libraryId`

Payload `POST /api/libraries` :

```json
{
  "name": "React UI Kit",
  "slug": "@alice/react-ui-kit",
  "description": "Bibliothèque React/Tailwind",
  "visibility": "private",
  "defaultStack": {
    "frontend": {
      "framework": "react",
      "styling": "tailwind",
      "language": "tsx"
    }
  }
}
```

### Snippets

- `POST /api/snippets`
- `GET /api/snippets`
- `GET /api/snippets/:snippetId`
- `PATCH /api/snippets/:snippetId`
- `DELETE /api/snippets/:snippetId`
- `POST /api/snippets/:snippetId/versions`

Payload `POST /api/snippets` :

```json
{
  "libraryId": "uuid",
  "slug": "frontend-atom-action-button-primary",
  "name": "Primary Button",
  "domain": "frontend",
  "kind": "atom",
  "category": "action",
  "language": "tsx",
  "framework": "react",
  "frameworkVersion": "19",
  "visibility": "private",
  "description": "Bouton primaire Tailwind",
  "tags": ["button", "tailwind", "react"],
  "initialVersion": {
    "version": "1.0.0",
    "code": "export function PrimaryButton() { return <button /> }"
  }
}
```

### Search / resolve

- `GET /api/resolve?ref=...`
- `POST /api/search`

Payload `POST /api/search` :

```json
{
  "query": "primary button",
  "scope": "selected_libraries_only",
  "librarySlugs": ["@alice/react-ui-kit"],
  "domain": "frontend",
  "kind": "atom",
  "framework": "react",
  "mode": "guided",
  "limit": 10
}
```

### Generations

- `GET /api/generations`
- `GET /api/generations/:generationId`

## Validation

Chaque endpoint write doit valider :
- présence des champs
- format du slug
- cohérence `domain / kind`
- cohérence `visibility`
- contrôle d'accès
- absence de dépassement de taille sur le code et les métadonnées

---

## 9. Contrat MCP

## Outils obligatoires

### `resolve_reference`

Entrée :

```json
{
  "reference": "@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0",
  "requester_scope": "own_plus_public"
}
```

Usage :
- obtenir une brique exacte
- éviter l'ambiguïté

### `search_library`

Entrée :

```json
{
  "query": "button",
  "scope": "selected_libraries_only",
  "library_slugs": ["@alice/react-ui-kit"],
  "domain": "frontend",
  "kind": "atom",
  "framework": "react",
  "mode": "strict",
  "limit": 10
}
```

Usage :
- chercher dans une ou plusieurs bibliothèques
- filtrer par stack

### `get_snippet`

Entrée :

```json
{
  "id": "uuid",
  "version": "1.2.0"
}
```

Usage :
- récupérer le code, la compatibilité, les dépendances, la provenance

### `check_dependencies`

Entrée :

```json
{
  "snippet_id": "uuid",
  "version": "1.2.0"
}
```

Usage :
- résoudre l'arbre des dépendances

### `assemble_plan`

Entrée :

```json
{
  "goal": "Construire une app de gestion de tâches",
  "scope": "own_plus_public",
  "mode": "guided",
  "library_slugs": ["@alice/react-ui-kit", "@bob/rust-api-primitives"],
  "frontend_stack": {
    "framework": "react",
    "styling": "tailwind",
    "language": "tsx"
  },
  "backend_stack": {
    "language": "rust",
    "framework": "axum",
    "database": "postgres"
  }
}
```

Usage :
- préparer l'assemblage
- lister les briques retenues
- déclarer les trous éventuels

### `list_rules`

Usage :
- récupérer les règles applicables

### `log_generation`

Usage :
- journaliser l'assemblage et la sortie finale

## Politique MCP obligatoire

L'agent doit suivre cet ordre :

1. `resolve_reference` si une référence explicite est donnée
2. `search_library` si la référence n'est pas complète
3. `get_snippet`
4. `check_dependencies`
5. `assemble_plan`
6. génération de fallback uniquement si nécessaire
7. `log_generation`

## Scopes

- `private_only`
- `own_plus_public`
- `public_only`
- `selected_libraries_only`

## Modes

- `strict`
- `guided`
- `auto`

Règles :
- `strict` n'utilise que les bibliothèques indiquées
- `guided` privilégie les bibliothèques indiquées et complète si permis
- `auto` explore le public compatible mais exclut tout contenu douteux

---

## 10. Trust & Safety MVP

## Menaces à couvrir dès le MVP

- prompt injection dans descriptions / README / commentaires
- snippets publics malveillants
- faux snippets “compatibles”
- dépendances toxiques
- snippets shell dangereux

## Règles minimales obligatoires

### Données, pas instructions

Un snippet public est toujours traité comme une **donnée**.
Jamais comme une instruction système.

### Sanitization

Scanner au minimum :
- descriptions
- README
- notes
- tags libres

Signaux à bloquer ou marquer :
- `ignore previous instructions`
- `reveal secrets`
- `send env`
- `execute this command`
- `system override`

### Analyse statique initiale

Marquer comme `review_required` ou `restricted` si on détecte :
- `curl | sh`
- reverse shell
- suppression destructrice
- accès secret / env / filesystem sensible
- commandes réseau suspectes

### Politique MCP

- pas d'exécution automatique
- pas de shell implicite
- pas de migration destructive implicite
- exclusion `flagged` et `quarantined` en mode `auto`
- provenance obligatoire sur tous les résultats

### Provenance minimale à conserver

Pour chaque brique assemblée :
- `library_slug`
- `snippet_slug`
- `snippet_version`
- `content_hash`
- `owner_id` ou identité publique

---

## 11. UX du MVP

## Écrans obligatoires

### 1. Login

- boutons `Continuer avec GitHub` et `Continuer avec Discord`
- chaque bouton = lien direct vers `/api/auth/{provider}/start` (pas de SDK côté frontend)
- après callback, le backend pose le cookie session et redirige vers le frontend

### 2. Dashboard

- résumé du compte
- bibliothèques récentes
- snippets récents
- dernières générations

### 3. Libraries

- liste des bibliothèques
- création
- édition
- visibilité
- slug public

### 4. Snippets list

- filtres
- recherche
- badges domaine / langage / visibilité

### 5. Snippet detail

- métadonnées
- code courant
- versions
- dépendances
- provenance

### 6. Snippet create/edit

- code editor Monaco
- métadonnées
- suggestions de classification
- aperçu de la référence canonique

### 7. Search / resolve

- champ de recherche
- champ de référence explicite
- filtres de bibliothèque / domaine / stack

### 8. Generations

- historique
- prompt
- snippets utilisés
- résultat

## UX rules

- l'identifiant de bibliothèque doit être visible partout
- la référence canonique d'un snippet doit être copiables facilement
- la visibilité doit être explicite
- l'utilisateur doit voir quand un résultat est `public_unverified`
- l'utilisateur doit voir la provenance avant d'utiliser une brique

---

## 12. Détection et recherche

## Détection

Pipeline MVP :
1. détection du langage
2. mapping vers domaine
3. détection du kind
4. détection du framework
5. suggestion de tags
6. embedding

Règle :
- toujours humaine en validation finale

## Recherche hybride

Combinaison de :
- trigram / full-text
- filtres structurés
- similarité vectorielle

Ordre recommandé de ranking :
1. exact match référence
2. slug exact
3. match textuel fort
4. compatibilité de stack
5. similarité vectorielle
6. trust level

---

## 13. Auth détaillée

## Choix

- OAuth direct côté backend Rust, aucun SaaS d'auth externe
- providers MVP : GitHub **et** Discord
- session : JWT signé avec `APP_SESSION_SECRET`, stocké dans un cookie `HttpOnly`

## Pourquoi

- app auto-hébergée sur VPS via Coolify → un SaaS d'auth externe n'apporte pas de valeur et ajoute une dépendance réseau payante
- GitHub cible les développeurs, Discord élargit à la communauté
- `APP_SESSION_SECRET` est le bouton d'arrêt d'urgence : sa rotation invalide toutes les sessions
- zéro SDK frontend à maintenir

## Flow détaillé

1. **Start** — `GET /api/auth/{provider}/start`
   - génère un `state` JWT court (anti-CSRF) signé avec `APP_SESSION_SECRET`
   - redirige vers `https://github.com/login/oauth/authorize` ou `https://discord.com/oauth2/authorize`
2. **Callback** — `GET /api/auth/{provider}/callback?code=...&state=...`
   - valide le `state`
   - échange `code` contre un access token via `reqwest`
   - récupère le profil (`GET /user` pour GitHub, `GET /users/@me` pour Discord)
   - upsert dans `users` + `auth_identities`
   - signe un JWT de session (sub = `user_id`, exp = 7 jours)
   - pose le cookie `usestakly_session` (`HttpOnly`, `SameSite=Lax`, `Secure` si `APP_BASE_URL` est en `https://`)
   - redirige vers `FRONTEND_BASE_URL`
3. **Usage** — chaque requête API renvoie le cookie ; un middleware le décode et injecte `CurrentUser`
4. **Logout** — `POST /api/auth/logout` efface le cookie

## Règles

- ne jamais utiliser l'email provider comme clé métier
- `user_id` applicatif = UUID local, stable et unique
- stocker la relation dans `auth_identities (provider, provider_user_id → user_id)`
- séparer les sessions web et futurs API tokens (tables distinctes)
- rotation de `APP_SESSION_SECRET` = kill switch global

## Variables d'environnement

### Frontend

```env
VITE_API_BASE_URL=http://localhost:4000
```

### Backend

```env
DATABASE_URL=
APP_ENV=development
APP_HOST=0.0.0.0
APP_PORT=4000
APP_BASE_URL=https://api.usestakly.com
FRONTEND_BASE_URL=https://usestakly.com
APP_SESSION_SECRET=
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
RUST_LOG=info
```

---

## 14. Build order exact

## Étape 0 — Bootstrap repo

Livrer :
- repo initial
- backend hello world
- frontend hello world
- docker compose postgres
- CI verte

## Étape 1 — Backend foundation

Livrer :
- config
- tracing
- `/health`
- connexion DB
- migrations initiales

## Étape 2 — Auth

Livrer :
- login GitHub
- `GET /api/me`
- synchronisation `users` / `auth_identities`

## Étape 3 — Libraries

Livrer :
- tables `libraries`
- CRUD libraries
- slug unique
- visibilité

## Étape 4 — Snippets

Livrer :
- tables snippets / versions / tags
- CRUD snippets
- versioning
- références canoniques

## Étape 5 — Frontend métier

Livrer :
- écran libraries
- écran snippets
- création / édition
- Monaco

## Étape 6 — Détection + recherche

Livrer :
- pipeline de détection
- embeddings
- `/api/resolve`
- `/api/search`

## Étape 7 — MCP

Livrer :
- tools MVP
- scopes
- modes
- audit logs
- assemble plan

## Étape 8 — Safety minimum

Livrer :
- sanitize contenu
- classification de risque
- exclusion auto `flagged` / `quarantined`

## Étape 9 — Validation end-to-end

Livrer :
- tests E2E
- parcours réel
- vérification de provenance

---

## 15. Definition of Done globale MVP

Le MVP est considéré comme terminé si et seulement si :

1. un utilisateur peut se connecter avec GitHub
2. il peut créer une bibliothèque
3. il peut y ajouter un snippet versionné
4. il peut rendre une bibliothèque ou un snippet public
5. un snippet public peut être résolu par référence canonique
6. un utilisateur peut chercher des snippets dans une ou plusieurs bibliothèques
7. un client MCP peut appeler `resolve_reference`, `search_library`, `get_snippet`, `assemble_plan`
8. chaque génération journalise les snippets utilisés
9. les contenus publics douteux ne sont pas consommés en mode `auto`
10. la provenance est visible et conservée

---

## 16. Test plan minimal obligatoire

## Backend

- tests unitaires sur la résolution de référence
- tests unitaires sur le filtrage de visibilité
- tests unitaires sur la policy de trust
- tests d'intégration SQLx sur CRUD libraries
- tests d'intégration SQLx sur CRUD snippets
- tests d'intégration SQLx sur recherche
- tests d'intégration auth sur `GET /api/me`

## Frontend

- login flow
- création de bibliothèque
- création de snippet
- édition de version
- affichage de la référence canonique
- recherche

## E2E

1. login GitHub
2. create library
3. create snippet
4. publish snippet
5. resolve by canonical reference
6. search by library
7. MCP `assemble_plan`

---

## 17. Décisions techniques imposées

Pour éviter les dérives pendant la construction :

- pas d'auth maison
- pas de GraphQL pour le produit MVP
- pas de vector DB externe
- pas de Next.js pour le frontend MVP
- pas de monorepo complexe
- pas d'exécution automatique
- pas de génération libre avant résolution/recherche

---

## 18. Prompt d'implémentation pour une équipe ou un agent

Si ce document sert de base à une implémentation agentique, l'agent doit suivre ces règles :

1. construire d'abord la structure de repo et les migrations
2. implémenter le backend avant le raffinement frontend
3. ne jamais sauter les couches de validation / auth / safety
4. respecter les contrats d'API et MCP décrits ici
5. ne pas inventer de feature hors périmètre
6. conserver les noms, types et scopes définis ici sauf contradiction technique démontrée

---

## 19. Résumé exécutable

Construire :
- un backend Rust + Axum + SQLx + Postgres
- un frontend React + Vite + Tailwind
- une auth OAuth directe (GitHub + Discord) portée par le backend
- un modèle `libraries -> snippets -> versions`
- une résolution canonique `@library:snippet@version`
- une recherche hybride
- un MCP avec `resolve`, `search`, `get`, `dependencies`, `assemble_plan`, `log`
- une policy de sécurité minimale

Et valider le tout avec un parcours complet :

> login → create library → create snippet → publish → resolve → search → assemble plan → log generation
