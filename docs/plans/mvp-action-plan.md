# UseStakly — Plan d'Action MVP

> Version : 1.0 — 2026-04-18
> Portée : MVP autonome de `UseStakly`, sans dépendance à `collab-dashboard`

> Note de transition : `UseStakly` est le nom produit retenu. `Project-K` reste l'ancien nom de travail encore visible dans certaines structures techniques.

## 🎯 Objectif du MVP

Livrer une **première version utilisable par un développeur solo** pour :

1. se connecter
2. créer une ou plusieurs bibliothèques
3. créer et classer des snippets dans ces bibliothèques
4. rendre une bibliothèque ou un snippet public si souhaité
5. rechercher des snippets par référence, texte, tags et similarité
6. exposer ces bibliothèques via un serveur MCP
7. assembler du code assisté par IA en réutilisant des snippets privés ou publics existants

Le MVP ne cherche **pas** à couvrir la communauté, la marketplace, la monétisation avancée ni l'intégration avec `collab-dashboard`.

---

## ✅ Décisions structurantes

### 1. Repo GitHub dédié

Le produit doit vivre dans **son propre dépôt GitHub**, distinct de `Project-DK`.

Pourquoi :
- identité produit claire
- historique Git propre
- roadmap, issues et releases centrées sur ce produit
- liberté d'architecture sans dette héritée d'un autre repo

### 2. Structure du repo au démarrage

Pas de monorepo complexe. On garde un repo simple avec deux applications claires :

```text
usestakly/
├── backend/                # Rust + Axum + SQLx
├── frontend/               # React + Vite + Tailwind v4
├── docs/
├── docker-compose.yml
├── .github/workflows/
├── .env.example
└── README.md
```

Le dossier `shared/` est **repoussé** tant qu'on n'a pas une duplication réelle qui justifie son existence.

### 3. Stack technique retenue

#### Backend
- Rust
- Axum
- Tokio
- SQLx
- PostgreSQL
- pgvector
- tower / tower-http
- tracing

#### Frontend
- React 19
- TypeScript
- Vite
- Tailwind CSS v4
- TanStack Query
- Zustand
- Monaco Editor

#### Détection / recherche
- tree-sitter
- fastembed local
- PostgreSQL full-text + `pg_trgm`
- pgvector

#### Infra
- Docker Compose en local
- GitHub Actions pour CI
- déploiement MVP : frontend sur Coolify, backend sur Coolify, base PostgreSQL sur Coolify

### 4. Choix d'authentification MVP

**Choix retenu : OAuth direct `GitHub` + `Discord`, auth portée par le backend Rust (pas de SaaS d'auth).**

Pourquoi ce choix :
- l'app est auto-hébergée sur VPS via Coolify ; un SaaS d'auth externe (Supabase, Auth0, Clerk…) n'apporte aucune valeur et ajoute une dépendance réseau payante
- GitHub cible les développeurs, Discord élargit à la communauté (contributeurs, bibliothèques publiques)
- le backend Axum gère le cycle OAuth complet : `authorize` → `callback` → échange code → fetch profil → upsert `users`/`auth_identities` → session JWT signée dans un cookie `HttpOnly`
- le secret `APP_SESSION_SECRET` est la seule clé à faire tourner pour invalider toutes les sessions
- zéro SDK externe côté frontend : un simple lien `GET /api/auth/{github|discord}/start` déclenche le flow

### 5. Position claire sur GitHub Auth

**GitHub Auth reste un provider pertinent, mais au même titre que Discord — aucun des deux n'est l'identité canonique.**

Décision :
- **MVP** : login via GitHub **ou** Discord, OAuth direct côté backend
- **identité interne** : l'utilisateur est identifié par un `user_id` local (UUID en DB), pas par l'email ni l'ID provider
- **v1.1** : ajouter un fallback `email magic link` si le besoin émerge
- **plus tard** : ajouter une vraie intégration GitHub séparée pour accéder aux repos, commits ou gists

Important :
- le **login GitHub** et l'**intégration GitHub** sont deux sujets différents (scopes, tokens, durée de vie)
- on ne doit pas coupler toute l'identité produit aux permissions repo GitHub
- si on a besoin plus tard d'accéder à des dépôts, on évaluera un **GitHub App** ou un flux OAuth dédié à cette intégration

### 6. Ce qu'on repousse explicitement

- auth maison `JWT + Argon2id`
- multi-provider complexe
- SSO
- organisation / équipes
- billing Stripe
- publication publique et système de stars
- extension VS Code native
- collaboration temps réel
- moteur RULES complet de niveau avancé

---

## 🔐 Design auth concret

## Auth cible au MVP

### Frontend
- boutons `Continuer avec GitHub` / `Continuer avec Discord`
- chaque bouton est un simple lien `<a href="/api/auth/{provider}/start">` — pas de SDK externe
- après callback, le backend pose un cookie `usestakly_session` et redirige vers `FRONTEND_BASE_URL`
- la session est récupérée par `GET /api/me` (le cookie est envoyé automatiquement grâce à `credentials: "include"`)

### Backend Rust
- routes `GET /api/auth/{github|discord}/start` et `GET /api/auth/{github|discord}/callback`
- `start` : génère un `state` JWT court (anti-CSRF) et redirige vers le provider
- `callback` : valide le `state`, échange le `code` contre un access token, récupère le profil, upsert dans `users` + `auth_identities`, signe un JWT de session avec `APP_SESSION_SECRET` et le pose dans un cookie `HttpOnly`
- middleware `CurrentUser` qui décode le cookie à chaque requête et rejette 401 si invalide
- route `POST /api/auth/logout` qui efface le cookie

### Base de données
- table `users` locale : source de vérité (UUID interne)
- table `auth_identities` : `(provider, provider_user_id) → user_id`
- `provider ∈ {'github', 'discord'}` au MVP
- ne jamais prendre l'email provider comme clé primaire métier

## Règles d'implémentation auth

1. Un utilisateur qui se connecte pour la première fois crée ou alimente un `users` local + une ligne `auth_identities`.
2. Le backend ne fait confiance qu'aux JWT de session qu'il a lui-même signés.
3. Le frontend n'accède jamais directement à la base — tout passe par l'API Rust.
4. L'accès MCP exige une session valide (ou, plus tard, un token API dédié).
5. Les futurs tokens API utilisateur seront stockés dans une table séparée, distincte des sessions web.
6. Toute rotation de `APP_SESSION_SECRET` invalide toutes les sessions actives — c'est le bouton d'arrêt d'urgence.

## Variables d'environnement minimales

```env
# frontend
VITE_API_BASE_URL=http://localhost:4000

# backend
DATABASE_URL=
APP_ENV=development
APP_BASE_URL=http://localhost:4000
FRONTEND_BASE_URL=http://localhost:5173
APP_SESSION_SECRET=change-me-to-a-long-random-string
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
```

---

## 🧱 Portée exacte du MVP

## Fonctionnalités incluses

### A. Compte et session
- login GitHub
- logout
- profil minimal utilisateur

### B. Bibliothèques
- créer une bibliothèque
- gérer sa visibilité
- obtenir un `slug` public adressable
- marquer une bibliothèque par défaut

### C. Bibliothèque de snippets
- créer un snippet dans une bibliothèque
- éditer les métadonnées
- créer une nouvelle version
- lister ses snippets
- voir un snippet + sa version courante
- archiver un snippet

### D. Classification
- `domain`
- `kind`
- `category`
- `language`
- `framework`
- tags

### E. Recherche
- résolution par référence explicite
- recherche texte
- filtres par domaine, langage, kind, tags
- filtres par bibliothèque
- recherche vectorielle basique
- tri par pertinence

### F. Détection assistée
- détection du langage
- suggestion du domaine
- suggestion du kind
- suggestion de tags
- validation humaine obligatoire avant sauvegarde

### G. MCP
- `resolve_reference`
- `search_library`
- `get_snippet`
- `assemble_plan`
- `list_rules` version minimale
- `log_generation`

### H. Générations
- enregistrer le prompt
- enregistrer les snippets utilisés
- enregistrer le code produit
- afficher un historique minimal

### I. Visibilité et sécurité minimale
- visibilité `private/public`
- exclusion des contenus signalés en mode auto
- provenance obligatoire pour les résultats MCP

## Fonctionnalités hors MVP

- `unlisted`
- fork, stars, commentaires
- paiement
- organisations / workspaces
- plugin IDE natif
- génération full-project complexe
- repo sync GitHub automatique

---

## 🗺️ Plan d'exécution

## Phase 0 — Cadrage et bootstrap du repo

### Objectif

Créer le repo produit autonome et poser la structure minimale de travail.

### Actions

- créer le dépôt GitHub dédié
- créer la structure `backend/`, `frontend/`, `docs/`
- ajouter `README.md`, `.gitignore`, `.editorconfig`, `.env.example`
- configurer `docker-compose.yml` avec PostgreSQL + pgvector
- configurer CI GitHub Actions :
  - backend : `fmt`, `clippy`, `test`
  - frontend : `lint`, `build`, `test`

### Livrables

- repo prêt à cloner
- CI verte sur un hello world
- base locale qui démarre

### Definition of Done

- `docker compose up` lance Postgres
- `cargo test` passe
- `npm run build` ou `pnpm build` passe
- une PR vide déclenche la CI avec succès

---

## Phase 1 — Backend fondation

### Objectif

Poser un backend Rust stable, observable et sécurisé.

### Actions

- initialiser `backend/` avec Axum
- ajouter :
  - routing
  - config loader
  - pool SQLx
  - middleware d'erreur
  - tracing
  - endpoint `/health`
- préparer les migrations SQLx
- activer les extensions :
  - `uuid-ossp`
  - `vector`
  - `pg_trgm`

### Endpoints minimum

- `GET /health`
- `GET /api/me`

### Definition of Done

- service démarre localement
- connexion DB vérifiée au boot
- logs structurés lisibles
- tests d'intégration de base disponibles

---

## Phase 2 — Auth MVP

### Objectif

Permettre un login GitHub et Discord concret via OAuth direct, sans dépendance à un SaaS d'auth.

### Actions

- créer les OAuth apps GitHub et Discord (dev + prod), noter `client_id` / `client_secret`
- configurer les URLs de callback `{APP_BASE_URL}/api/auth/{provider}/callback` en local et prod
- générer un `APP_SESSION_SECRET` long (≥ 32 bytes aléatoires)
- implémenter les routes `start` + `callback` pour chaque provider (crate `reqwest` pour l'échange code, `jsonwebtoken` pour le state et la session)
- implémenter le middleware `CurrentUser` qui lit/valide le cookie session
- créer le flow complet : bouton frontend → `/start` → provider → `/callback` → cookie → redirect → `GET /api/me`
- upsert dans `users` + `auth_identities` à chaque login
- implémenter `POST /api/auth/logout` (cookie effacé côté serveur)

### Décisions produit

- login par provider OAuth uniquement (GitHub + Discord)
- pas de mot de passe, pas de reset, pas de MFA au MVP
- pas de SDK d'auth côté frontend

### Risques à surveiller

- ne pas dépendre de l'email provider pour l'identité (l'email GitHub peut être privé, celui de Discord changé)
- prévoir des usernames applicatifs indépendants du username provider
- ne pas mélanger session web et futurs API tokens (tables distinctes)
- `APP_SESSION_SECRET` doit être stocké comme un secret, pas dans le repo
- cookie session : `HttpOnly`, `SameSite=Lax`, `Secure` en prod (géré par `session_cookie_secure()`)

### Definition of Done

- un utilisateur peut se connecter avec GitHub ou Discord
- le backend reconnaît l'utilisateur via le cookie session
- `GET /api/me` renvoie un profil cohérent
- `POST /api/auth/logout` invalide la session
- aucune dépendance Supabase dans le code ni dans les env vars

---

## Phase 3 — Modèle de données et CRUD bibliothèques / snippets

### Objectif

Construire le cœur métier utilisable sans IA.

### Actions

- implémenter les tables :
  - `users`
  - `auth_identities`
  - `libraries`
  - `snippets`
  - `snippet_versions`
  - `tags`
  - `snippet_tags`
  - `rule_sets`
  - `generations`
- développer les handlers :
  - `POST /api/libraries`
  - `GET /api/libraries`
  - `PATCH /api/libraries/:id`
  - `POST /api/snippets`
  - `GET /api/snippets`
  - `GET /api/snippets/:id`
  - `POST /api/snippets/:id/versions`
  - `PATCH /api/snippets/:id`
  - `DELETE /api/snippets/:id`
- ajouter validation serveur stricte
- ajouter règles d'autorisation par `owner_id`

### Definition of Done

- un utilisateur authentifié peut gérer plusieurs bibliothèques
- chaque snippet appartient à une bibliothèque
- chaque snippet possède une version courante
- les snippets privés d'un autre utilisateur sont inaccessibles
- les snippets publics sont adressables par `library_slug + snippet_slug`

---

## Phase 4 — Studio frontend MVP

### Objectif

Offrir une interface simple mais solide pour manipuler la bibliothèque.

### Écrans

- login
- dashboard minimal
- liste des bibliothèques
- liste des snippets
- fiche snippet
- formulaire création / édition
- historique des versions

### Actions

- poser une shell UI simple
- intégrer TanStack Query pour les appels API
- utiliser Zustand seulement pour l'état UI local
- intégrer Monaco pour l'édition
- créer l'écran de gestion des bibliothèques
- créer des formulaires :
  - métadonnées
  - code
  - tags
  - classification

### Definition of Done

- on peut créer une bibliothèque depuis l'UI
- on peut créer un snippet dans une bibliothèque
- on peut l'éditer et versionner
- l'interface reste utilisable sans design system complexe

---

## Phase 5 — Détection et recherche hybride

### Objectif

Rendre la bibliothèque réellement utile avant même la génération IA.

### Actions

- intégrer `tree-sitter` côté backend
- détecter le langage à l'upload
- proposer :
  - domaine
  - kind
  - framework
  - tags
- intégrer `fastembed`
- générer et stocker l'embedding à la création de version
- ajouter la résolution par `library_slug + snippet_slug`
- implémenter recherche hybride :
  - `ILIKE` / trigram
  - filtres SQL
  - similarité vectorielle

### Definition of Done

- un snippet collé reçoit une suggestion exploitable
- la recherche retrouve correctement des snippets proches
- une référence explicite résout un snippet sans ambiguïté
- la validation finale reste humaine

---

## Phase 6 — Serveur MCP minimal

### Objectif

Exposer la bibliothèque à un client IA avec un périmètre minimal mais réel.

### Actions

- créer la route MCP dédiée
- implémenter :
  - `resolve_reference`
  - `search_library`
  - `get_snippet`
  - `assemble_plan`
  - `list_rules`
  - `log_generation`
- ajouter audit logs
- ajouter quotas simples par utilisateur si nécessaire
- ajouter scopes de recherche
- ajouter policy engine minimal pour exclure les contenus `flagged` / `quarantined`

### Contraintes MVP

- pas de création automatique de snippet par l'IA
- pas d'orchestration complexe multi-domaines
- pas de moteur RULES avancé au premier jet
- pas d'exécution automatique de snippets publics

### Definition of Done

- un client MCP authentifié peut résoudre une référence exacte
- un client MCP authentifié peut chercher et lire des snippets
- un client MCP peut préparer un plan d'assemblage simple
- les générations sont tracées

---

## Phase 7 — Boucle de valeur MVP

### Objectif

Valider que l'outil fait gagner du temps avant d'ajouter des couches produit.

### Actions

- afficher l'historique des générations
- mesurer :
  - snippets créés
  - recherches effectuées
  - générations déclenchées
- interroger les premiers utilisateurs
- identifier les 3 frictions majeures

### Definition of Done

- 3 à 5 utilisateurs testent le produit
- on sait quels usages sont réellement adoptés
- on sait si l'étape suivante est :
  - meilleure recherche
  - meilleure UX
  - intégration GitHub
  - extension IDE

---

## 📅 Ordre recommandé sur 6 semaines

### Semaine 1
- phase 0
- phase 1

### Semaine 2
- phase 2

### Semaine 3
- phase 3

### Semaine 4
- phase 4

### Semaine 5
- phase 5

### Semaine 6
- phase 6 puis phase 7 en début de validation utilisateur

---

## 🧪 Stratégie de tests

## Backend
- unit tests sur la logique de classification
- tests d'intégration SQLx pour les handlers critiques
- tests auth pour les endpoints protégés

## Frontend
- tests unitaires ciblés sur les composants critiques
- tests de flux sur login, création snippet, versioning

## E2E minimum

1. login GitHub
2. création d'une bibliothèque
3. création d'un snippet
4. édition d'une version
5. résolution par référence explicite
6. recherche d'un snippet
7. appel MCP `search_library`

---

## 📌 Backlog post-MVP

À considérer seulement après validation usage :

- email magic link en fallback auth
- import GitHub Gist / repo
- GitHub App pour intégrations repo
- publication publique
- stars / forks
- billing
- workspace / équipe
- extension Cursor / VS Code

---

## Verdict produit / technique

Le MVP doit optimiser **la vitesse d'apprentissage**, pas la complétude.

La bonne stratégie est :
- repo dédié
- backend Rust robuste
- frontend React simple
- auth gérée
- GitHub login pertinent mais non exclusif à long terme
- intégration GitHub métier séparée du login

Si une décision doit être protégée coûte que coûte, c'est celle-ci :

> **ne pas construire une auth maison ni une architecture trop large avant d'avoir validé la boucle "je stocke mes snippets -> je les retrouve -> l'IA les réutilise".**
