# Projet K — Nomenclature des Snippets

> Version : 1.0 — 2026-04-15

## 🎯 Règle fondamentale

```
{domain}-{kind}-{category}-{name}-{variant?}
```

Tout en **kebab-case**, ASCII uniquement, pas d'accent, pas d'espace.

## 🔤 Les 5 composants

### 1. `domain` (obligatoire)

| Valeur | Description |
|---|---|
| `frontend` | UI visible par l'utilisateur final |
| `backend` | Logique serveur, API, traitement |
| `devops` | Infra, CI/CD, conteneurisation |
| `data` | Requêtes, migrations, transformations |
| `shared` | Ressources communes front/back |

### 2. `kind` (obligatoire, dépend du domain)

Valeurs autorisées définies dans la table `snippet_kinds`. Voir [architecture.md §2](./architecture.md).

### 3. `category` (obligatoire)

Libre mais **normalisée** par domaine. Exemples courants :

**Frontend** : `action`, `input`, `display`, `layout`, `feedback`, `nav`, `form`, `data-viz`
**Backend** : `auth`, `crud`, `logging`, `validation`, `notification`, `payment`
**DevOps** : `deploy`, `monitor`, `build`, `secret`
**Data** : `analytics`, `migration`, `seed`, `export`

### 4. `name` (obligatoire)

Un nom descriptif en kebab-case. Préférer **concret** à **abstrait** :
- ✅ `button`, `login-form`, `user-profile-card`
- ❌ `widget-1`, `helper`, `thing`

### 5. `variant` (optionnel)

Pour différencier des versions fonctionnellement équivalentes :
- Visuelles : `primary`, `secondary`, `ghost`, `compact`, `expanded`
- Techniques : `jwt`, `oauth`, `email-only`, `with-icon`

## ✅ Exemples canoniques

```
frontend-atom-action-button-primary
frontend-atom-input-text-bordered
frontend-molecule-form-login-email
frontend-organism-nav-header-sticky
frontend-template-layout-dashboard

backend-handler-auth-login-jwt
backend-middleware-auth-jwt-verify
backend-function-validator-email
backend-service-user-create
backend-query-users-find-by-email
backend-model-user-dto

devops-config-docker-compose-dev
devops-script-deploy-rolling
devops-dockerfile-rust-musl

data-migration-users-add-roles
data-query-analytics-daily-signups

shared-type-user-profile
shared-constant-http-status
```

## ❌ À éviter

| Mauvais | Bon | Pourquoi |
|---|---|---|
| `Btn_Primary_V1` | `frontend-atom-action-button-primary` | Pas de version dans le nom (semver en DB) |
| `loginForm` | `frontend-molecule-form-login-email` | camelCase interdit, domain manquant |
| `utils1` | `backend-function-validator-email` | Sémantique nulle |
| `frontend-btn` | `frontend-atom-action-button-primary` | Abréviations interdites |
| `frontend-atom-bouton-primaire` | `frontend-atom-action-button-primary` | Toujours en anglais (lingua franca du code) |

## 🔒 Validation côté application

Regex de validation :
```
^(frontend|backend|devops|data|shared)-[a-z]+-[a-z]+-[a-z0-9-]+$
```

La validation du `kind` se fait en plus contre la table `snippet_kinds(domain, kind)`.

## 🔁 Versioning

Le nom **ne contient jamais** de version. Le versioning se fait via :
- Table `snippet_versions` (append-only)
- Champ `version` en **semver** (`1.0.0`, `1.2.3`)
- `current_version_id` sur la table `snippets` pointe vers la version active

## 🆔 Identifiants

| Champ | Usage |
|---|---|
| `id` (UUID) | Identifiant interne stable (liens DB, générations, API) |
| `slug` (texte) | Nom humain, unique par utilisateur |
| `content_hash` (SHA256) | Empreinte immuable du code (intégrité + futur Web3) |

**Le slug peut changer** (renommage). **L'UUID ne change jamais.**

## 🌐 Unicité

- Unique par `(owner_id, slug)` → chaque user peut avoir son propre `atom-button-primary`
- Les snippets publics conservent leur slug d'origine ; résolution via `owner_username/slug`

## 🏷️ Tags (orthogonaux au slug)

Les tags sont une **dimension libre** stockée séparément :
- `dark-mode`, `accessible`, `mobile-first`, `a11y`, `seo`, `experimental`, `deprecated`
- Indexés pour recherche hybride (tag + embedding sémantique)
- Peuvent être suggérés par la détection automatique
