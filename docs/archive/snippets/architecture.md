# Projet K — Architecture & Classification Multi-Domaines

> Document de référence pour l'architecture du Projet K (Komorebi).
> Version : 1.0 — 2026-04-15 *(pré-pivot 2026-04-20)*

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Le modèle de classification multi-domaines (`frontend / backend / devops / data / shared`) et la nomenclature `{domain}-{kind}-{category}-{name}` **restent la fondation**. Post-pivot, ils sont **complétés** par une couche qualité-scored (cf. [`strategy-quality-scored-registry.md`](./strategy-quality-scored-registry.md)) qui devient le filtre principal consommé par les agents IA.
>
> Section 7 et 8 de ce doc (détection automatique, évolution détection) : **secondaires** post-pivot. La détection peut alimenter certains signaux d'usage mais n'est plus un composant bloquant du MVP. Voir TODO.md phases 6–11 pour la priorisation actuelle.

---

## Sommaire

1. [Contexte & changement d'approche](#1-contexte--changement-dapproche)
2. [Modèle de classification à deux axes](#2-modèle-de-classification-à-deux-axes)
3. [Nomenclature universelle des snippets](#3-nomenclature-universelle-des-snippets)
4. [Schéma PostgreSQL](#4-schéma-postgresql)
5. [Prompt système MCP](#5-prompt-système-mcp)
6. [Structure des projets générés](#6-structure-des-projets-générés)
7. [Système de détection automatique — Plan zéro coût](#7-système-de-détection-automatique--plan-zéro-coût)
8. [Plan d'évolution de la détection](#8-plan-dévolution-de-la-détection)
9. [Tableau récapitulatif des impacts](#9-tableau-récapitulatif-des-impacts)

---

## 1. Contexte & changement d'approche

### Ce qui change

Le Projet K n'est **pas** uniquement un système Atomic Design pour UI React.
C'est une **bibliothèque universelle de snippets de code multi-langages, multi-domaines**, avec deux grandes sections applicatives :

- **Frontend** : HTML, CSS, JS, TS, React, Vue, Svelte, Tailwind…
- **Backend** : Rust, Python, Go, Node, SQL, Bash, Docker, YAML…

### Conséquence

L'**Atomic Design ne peut plus être le modèle universel** — il ne s'applique qu'aux UI.
Un handler HTTP n'est pas un bouton : parler d'"atome backend" n'a aucun sens technique.

Il faut donc une **classification adaptative** qui garde l'Atomic Design là où il a du sens (frontend) tout en offrant une taxonomie pertinente aux autres domaines (backend, devops, data).

---

## 2. Modèle de classification à deux axes

### Axe 1 — Domaine (obligatoire)

```
frontend  |  backend  |  devops  |  data  |  shared
```

### Axe 2 — Type de snippet (adaptatif selon domaine)

**Frontend/UI — Atomic Design conservé :**

| Kind | Description |
|---|---|
| `atom` | Composant UI indivisible (Button, Input, Icon) |
| `molecule` | Combinaison d'atomes (SearchBar) |
| `organism` | Section fonctionnelle (Header, Footer) |
| `template` | Layout sans données |
| `util` | Helper frontend (hook, formatter) |

**Backend / Data / DevOps — Taxonomie dédiée :**

| Kind | Description |
|---|---|
| `function` | Fonction pure réutilisable (validateur, formatter) |
| `handler` | Endpoint, route, controller |
| `middleware` | Interceptor, guard, auth layer |
| `model` | Schéma, entity, DTO |
| `service` | Classe de service métier |
| `query` | SQL, ORM, requête DB |
| `config` | Fichier de configuration (yaml, toml, env) |
| `script` | Bash, CI/CD, migration |
| `pattern` | Pattern architectural (repository, factory) |
| `dockerfile` | Image conteneur |
| `migration` | Migration de schéma |

**Transverse (tous domaines) :**

| Kind | Description |
|---|---|
| `util` | Helper générique |
| `constant` | Constante / enum |
| `type` | Type ou interface partagé |

---

## 3. Nomenclature universelle des snippets

### Format

```
{domain}-{kind}-{category}-{name}-{variant?}
```

### Exemples

| Slug | Signification |
|---|---|
| `frontend-atom-action-button-primary` | Bouton primaire (Atomic Design) |
| `frontend-molecule-form-login-email` | Formulaire login complet |
| `backend-handler-auth-login-jwt` | Endpoint de login JWT |
| `backend-middleware-auth-jwt-verify` | Vérification token JWT |
| `backend-function-validator-email` | Validateur d'email pur |
| `backend-query-users-find-by-email` | Requête SQL |
| `devops-script-deploy-rolling` | Script de déploiement |
| `shared-type-user-profile` | Type User partagé front/back |

### Règles invariantes

- **UUID stable** derrière chaque snippet → le slug peut être renommé sans casser les liens
- **Versioning semver** (`1.2.0`) géré par le système, **pas** dans le nom
- **Tags libres** dans une table séparée pour recherche hybride
- Le préfixe `domain` rend impossible toute confusion entre un `atom` UI et une fonction backend

---

## 4. Schéma PostgreSQL

### Types & table principale

```sql
CREATE TYPE snippet_domain AS ENUM ('frontend', 'backend', 'devops', 'data', 'shared');

DROP TYPE IF EXISTS atomic_level CASCADE;

CREATE TABLE snippets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug TEXT NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Classification
    domain snippet_domain NOT NULL,
    kind TEXT NOT NULL,
    category TEXT NOT NULL,

    name TEXT NOT NULL,
    description TEXT,

    -- Langage & stack
    language TEXT NOT NULL,              -- 'rust','python','tsx','sql'…
    runtime TEXT,                        -- 'node','deno','bun','python3.12'
    framework TEXT,                      -- 'react','axum','fastapi', null
    framework_version TEXT,              -- '19.0','0.7'

    visibility visibility NOT NULL DEFAULT 'private',
    license TEXT NOT NULL DEFAULT 'MIT',
    current_version_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(owner_id, slug)
);
```

### Table de référence des kinds valides

```sql
CREATE TABLE snippet_kinds (
    domain snippet_domain NOT NULL,
    kind TEXT NOT NULL,
    description TEXT,
    PRIMARY KEY (domain, kind)
);

INSERT INTO snippet_kinds (domain, kind, description) VALUES
    ('frontend', 'atom',       'Composant UI indivisible'),
    ('frontend', 'molecule',   'Combinaison d''atomes'),
    ('frontend', 'organism',   'Section fonctionnelle'),
    ('frontend', 'template',   'Layout sans données'),
    ('frontend', 'util',       'Helper frontend (hook, format)'),
    ('backend',  'function',   'Fonction pure réutilisable'),
    ('backend',  'handler',    'Endpoint / route / controller'),
    ('backend',  'middleware', 'Middleware / guard / interceptor'),
    ('backend',  'model',      'Schéma / entity / DTO'),
    ('backend',  'service',    'Classe de service métier'),
    ('backend',  'query',      'Requête SQL / ORM'),
    ('devops',   'config',     'Fichier de configuration'),
    ('devops',   'script',     'Script shell / CI'),
    ('devops',   'dockerfile', 'Image conteneur'),
    ('data',     'query',      'SQL / analytics'),
    ('data',     'migration',  'Migration de schéma'),
    ('shared',   'type',       'Type / interface partagé'),
    ('shared',   'constant',   'Constante / enum');
```

### Projets multi-domaines

```sql
ALTER TABLE projects
    ALTER COLUMN stack TYPE JSONB USING stack::jsonb;
```

Exemple de `stack` :

```json
{
  "frontend": {"framework": "react", "styling": "tailwind", "language": "tsx"},
  "backend":  {"framework": "axum", "language": "rust", "database": "postgres"},
  "devops":   {"container": "docker", "ci": "github-actions"}
}
```

---

## 5. Prompt système MCP

Deux approches possibles. **Option A recommandée pour démarrer.**

### Option A — Prompt unique avec branchement (recommandé)

```markdown
# RÔLE
Tu es l'Architecte du Projet K. Tu assembles des snippets multi-domaines
(frontend, backend, devops) depuis la bibliothèque de l'utilisateur.

# DOMAINE DE TRAVAIL ACTUEL
Domaine : {{request.target_domain}}   # frontend | backend | devops | fullstack
Stack   : {{project.stack | json}}

# CONTRAINTES COMMUNES (tous domaines)
1. Appelle `search_library` AVANT de coder, filtré par domain + language
2. Interdiction d'inventer : noms, classes, fonctions hors librairie
3. Respecte la stack déclarée (pas de nouvelles dépendances)
4. Chaque composant : commentaire de provenance
   (ex: `// Assemblé depuis: backend-function-validator-email@1.0.2`)

# CONTRAINTES SPÉCIFIQUES PAR DOMAINE

## Si domain = frontend
- Applique l'Atomic Design strictement (atom → molecule → organism)
- Priorité aux atomes existants ≥80% du besoin
- Classes CSS exclusivement depuis `css_classes` des snippets

## Si domain = backend
- Sépare : handler (I/O) → service (logique) → query (DB)
- Un handler n'accède JAMAIS à la DB directement → passe par un service
- Valide les entrées au niveau handler (appelle des `function` validators)
- Types partagés front/back : utilise `shared-type-*` (jamais dupliqués)

## Si domain = devops
- Réutilise les `config` et `script` existants
- Variables d'environnement : référencées, jamais hardcodées

## Si domain = fullstack
- Génère dans l'ordre : shared/types → backend → frontend
- Les types `shared-type-*` sont la source de vérité
- Les appels API frontend doivent matcher les handlers backend générés

# PROCÉDURE
ÉTAPE 1 — Décompose la demande par domaine
ÉTAPE 2 — `search_library` pour chaque besoin (filtré par domain + language)
ÉTAPE 3 — Plan d'assemblage explicite (liste des snippets, justifications)
ÉTAPE 4 — Génération avec arborescence correcte
ÉTAPE 5 — Rapport : snippets réutilisés, créés, règles appliquées
```

### Option B — Prompts spécialisés

Un fichier par domaine : `prompt_frontend_v1.md`, `prompt_backend_v1.md`, `prompt_fullstack_v1.md`.
Le MCP sélectionne selon `request.target_domain`.

Plus maintenable à grande échelle mais plus coûteux à faire évoluer en parallèle.
**Basculer vers B uniquement si le prompt unique devient trop long ou contradictoire.**

---

## 6. Structure des projets générés

```
projet-final/
├── frontend/
│   ├── src/components/{atoms,molecules,organisms}/
│   ├── src/hooks/
│   └── src/pages/
├── backend/
│   ├── src/handlers/
│   ├── src/services/
│   ├── src/models/
│   └── src/queries/
├── shared/
│   ├── types/
│   └── constants/
└── devops/
    ├── docker/
    └── scripts/
```

---

## 7. Système de détection automatique — Plan zéro coût

### Objectif

Quand un utilisateur colle un snippet, pré-remplir automatiquement :
`language`, `domain`, `kind`, `category`, `framework`, `tags` — sans aucune clé API ni coût variable.

### Architecture

```
┌───────────────────────────────────────────────┐
│ L'utilisateur colle son code                  │
│              │                                 │
│              ▼                                 │
│ [1] Détection du language  (tree-sitter)      │
│              │                                 │
│              ▼                                 │
│ [2] Mapping language → domain  (table)        │
│              │                                 │
│              ▼                                 │
│ [3] Détection du kind  (regex patterns)       │
│              │                                 │
│              ▼                                 │
│ [4] Détection de la category  (regex/mots-clés)│
│              │                                 │
│              ▼                                 │
│ [5] Embedding vectoriel  (fastembed local)    │
│              │                                 │
│              ▼                                 │
│ Formulaire pré-rempli, éditable               │
│ L'utilisateur valide → sauvegarde             │
└───────────────────────────────────────────────┘
```

### Stack technique (100 % gratuit, 100 % local)

| Étape | Outil Rust | Coût | Précision |
|---|---|---|---|
| Détection language | `tree-sitter` + `hyperpolyglot` | 0 € | ~98 % |
| Mapping domain | Table statique | 0 € | ~95 % |
| Détection kind | Regex + AST tree-sitter | 0 € | ~75-85 % |
| Détection category | Regex / dictionnaire de mots-clés | 0 € | ~70 % |
| Embeddings vectoriels | `fastembed` (bge-small-en) | 0 € | ~85 % |

**Aucune clé API, aucun appel réseau, aucun coût variable.**

### Mapping language → domain (exemple)

```rust
fn default_domain(language: &str) -> SnippetDomain {
    match language {
        "tsx" | "jsx" | "vue" | "svelte" | "html" | "css" | "scss"
            => SnippetDomain::Frontend,
        "rust" | "python" | "go" | "java" | "kotlin" | "ruby" | "php"
            => SnippetDomain::Backend,
        "sql" | "prisma"
            => SnippetDomain::Data,
        "yaml" | "toml" | "dockerfile" | "bash" | "sh"
            => SnippetDomain::DevOps,
        "ts" | "js"
            => SnippetDomain::Shared,   // ambigu : à affiner par regex
        _ => SnippetDomain::Shared,
    }
}
```

### Exemples de patterns de détection du `kind`

| Pattern | → Kind |
|---|---|
| `export default function [A-Z]\w+\(.*\)\s*{[^}]*return\s*<` | `frontend/atom` ou `molecule` |
| `async fn \w+\(.*Request.*\).*Response` (Rust) | `backend/handler` |
| `app\.(get\|post\|put\|delete)\(` (Node) | `backend/handler` |
| `CREATE TABLE` / `ALTER TABLE` | `data/migration` |
| `SELECT .* FROM` | `data/query` ou `backend/query` |
| `FROM \w+:\w+` (Dockerfile) | `devops/dockerfile` |
| `#!/bin/\w*sh` | `devops/script` |
| `pub (struct\|enum) \w+` sans méthodes | `shared/type` ou `backend/model` |

### Validation humaine

La détection est **une suggestion**, pas une décision.
Le formulaire affiche les valeurs pré-remplies et l'utilisateur peut corriger avant de sauvegarder. Cela garantit :

- **Zéro dépendance à une IA faillible**
- **Précision finale = 100 %** (l'humain valide)
- **Apprentissage implicite** : les corrections utilisateur alimentent un futur jeu de données pour l'étape 8.

---

## 8. Plan d'évolution de la détection

Stratégie progressive, sans engagement financier tant que la valeur n'est pas prouvée.

### Phase 1 — MVP (0 € / mois)

- Heuristiques locales (tree-sitter + regex)
- Embeddings `fastembed` locaux
- Validation humaine systématique
- **Aucune clé API requise**

### Phase 2 — Amélioration opportuniste (< 1 € / mois)

Ajouter un **LLM cloud en fallback** uniquement quand les heuristiques échouent (confiance < seuil).

| Modèle | Coût input / 1M tokens | Coût 10 000 snippets |
|---|---|---|
| Gemini Flash | ~0,075 $ | ~0,15 $ |
| GPT-4o-mini | ~0,15 $ | ~0,30 $ |
| Claude Haiku 4.5 | ~1 $ | ~2 $ |

Clé API requise côté **serveur Rust uniquement** (jamais exposée au frontend).

### Phase 3 — Mise à l'échelle (coût fixe, pas variable)

Déployer un **LLM local** (Ollama + Llama 3.1 8B ou Qwen 2.5) sur le serveur :

- Coût variable : **0 €**
- Coût fixe : hébergement GPU (~30-80 € / mois selon volume)
- Aucune dépendance à un fournisseur externe
- Zéro fuite de données utilisateur vers des tiers

### Phase 4 — Apprentissage interne

Utiliser les **corrections utilisateur** accumulées (Phase 1 à 3) pour :

- Fine-tuner un petit modèle dédié à la classification
- Améliorer les regex patterns au fil du temps
- Personnaliser la détection par utilisateur (il corrige toujours `service` en `handler` → le système s'adapte)

### Synthèse du parcours

| Phase | Coût variable | Clé API | Précision visée |
|---|---|---|---|
| 1 — MVP | 0 € | ❌ | ~80 % |
| 2 — Fallback LLM | < 1 € / mois | ✅ (serveur) | ~92 % |
| 3 — LLM local | 0 € (coût fixe) | ❌ | ~95 % |
| 4 — Modèle fine-tuné | 0 € (coût fixe) | ❌ | ~97 % |

**Règle d'or :** ne jamais payer avant d'avoir des utilisateurs.

---

## 9. Tableau récapitulatif des impacts

| Aspect | Impact |
|---|---|
| **Schéma DB** | `domain` + `kind` remplacent `atomic_level` |
| **Nomenclature** | Préfixe `domain` obligatoire |
| **Prompt MCP** | Branchements par domaine |
| **Frontend UI** | Sidebar qui bascule entre vues (arbre Atomic Design vs arbre par couches backend) |
| **Search** | Filtres `domain` + `language` en plus du texte |
| **Détection auto** | Pipeline 100 % local au MVP, évolution progressive sans lock-in |
| **Coût initial** | 0 € (aucune clé API requise) |

---

## Annexe — Décisions d'architecture validées

- ✅ Frontend : **React + Tailwind**
- ✅ Backend : **Rust** (serveur MCP, parser, storage)
- ✅ Nomenclature : `{domain}-{kind}-{category}-{name}-{variant?}` avec UUID stable
- ✅ Web3 : **non implémenté**, architecture pensée pour permettre l'ajout sans réécriture (auth abstraite, ownership comme donnée, subscriptions provider-agnostic)
- ✅ Détection auto : **zéro coût** au MVP, évolution progressive
- ✅ Prompt MCP : **Option A** (prompt unique avec branchement) au démarrage
