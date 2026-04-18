# Projet K — Protocole MCP

> Version : 1.0 — 2026-04-15

## 🎯 Rôle du serveur MCP

Le serveur MCP (Model Context Protocol) est le **pont obligatoire** entre l'IA et la librairie de snippets de l'utilisateur. Il expose un ensemble d'**outils** (tools) que l'IA peut appeler, dans un ordre imposé par le prompt système.

**Principe clé** : l'IA ne parle **jamais** directement à la base de données. Elle passe par les outils MCP, ce qui permet d'imposer des règles d'accès, de journaliser les appels et de garantir la provenance du code généré.

Le MCP doit fonctionner comme :
- un **résolveur exact** de références explicites
- un **moteur de recherche** dans des bibliothèques ciblées
- un **orchestrateur d'assemblage** qui réutilise avant de générer

## 🛠️ Outils exposés

## Concepts transverses

### Scopes de recherche

- `private_only`
- `own_plus_public`
- `public_only`
- `selected_libraries_only`

### Modes d'assemblage

- `strict`
- `guided`
- `auto`

### Références canoniques

```text
@alice/react-ui-kit:frontend-atom-action-button-primary
@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0
```

### 1. `resolve_reference`
Résout explicitement une bibliothèque, un snippet et éventuellement une version.

**Params :**
```json
{
  "reference": "@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0",
  "requester_scope": "own_plus_public"
}
```

**Retour :**
```json
{
  "library": {
    "id": "uuid",
    "slug": "@alice/react-ui-kit",
    "visibility": "public",
    "trust_level": "verified_author"
  },
  "snippet": {
    "id": "uuid",
    "slug": "frontend-atom-action-button-primary",
    "version": "1.2.0",
    "domain": "frontend",
    "language": "tsx",
    "framework": "react"
  }
}
```

### 2. `search_library`
Recherche des snippets pertinents dans une ou plusieurs bibliothèques.

**Params :**
```json
{
  "query": "string",
  "scope": "private_only|own_plus_public|public_only|selected_libraries_only",
  "library_slugs": ["@alice/react-ui-kit"],
  "domain": "frontend|backend|devops|data|shared",
  "kind": "string (optionnel)",
  "language": "string (optionnel)",
  "framework": "string (optionnel)",
  "mode": "strict|guided|auto",
  "limit": "int (default: 10)"
}
```

**Retour :**
```json
[
  {
    "id": "uuid",
    "library_slug": "@alice/react-ui-kit",
    "slug": "frontend-atom-action-button-primary",
    "description": "Bouton primaire Tailwind",
    "version": "1.2.0",
    "variables": [{"name": "label", "type": "string"}],
    "similarity": 0.89,
    "trust_level": "verified_author"
  }
]
```

**Implémentation** : recherche hybride (embedding cosinus + filtres SQL).

### 3. `get_snippet`
Récupère le code complet d'un snippet à une version donnée.

**Params :**
```json
{ "id": "uuid", "version": "string (optionnel, défaut: current)" }
```

**Retour :** le champ `code` complet + `variables` + `css_classes` + `metadata` + `dependencies`.

### 4. `assemble_plan`
Construit un plan d'assemblage sans produire directement le code.

**Params :**
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

**Retour :**
```json
{
  "steps": [
    {
      "domain": "frontend",
      "reason": "UI de base",
      "selected_snippets": [
        "@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0"
      ]
    }
  ],
  "missing_blocks": [],
  "fallback_allowed": false
}
```

### 5. `list_rules`
Retourne les règles actives du `rule_set` du projet.

**Params :**
```json
{ "rule_set_id": "uuid" }
```

**Retour :** JSON structuré (cf [rules-system.md](./rules-system.md)).

### 6. `check_dependencies`
Liste récursivement les snippets requis par une molécule/organisme.

**Params :**
```json
{ "snippet_id": "uuid", "version": "string (optionnel)" }
```

**Retour :** arbre des dépendances avec versions résolues (semver).

### 7. `propose_new_snippet`
L'IA propose un nouveau snippet quand aucun existant ne convient.

**Params :**
```json
{
  "domain": "string",
  "kind": "string",
  "category": "string",
  "name": "string",
  "code": "string",
  "justification": "string (obligatoire)"
}
```

**Retour :** `pending_review_id` — le snippet n'est **pas créé automatiquement**, il passe en file d'attente pour validation humaine.

### 8. `log_generation`
Appelé en fin de procédure pour enregistrer la génération.

**Params :**
```json
{
  "prompt": "string",
  "target_domain": "string",
  "used_snippets": ["uuid", ...],
  "plan": {"steps": [...]},
  "output_code": "string"
}
```

## 🔄 Flux type

### Cas 1 — Référence explicite

```
Utilisateur
    |
    v
"Récupère @alice/react-ui-kit:frontend-atom-action-button-primary"
    |
    v
resolve_reference
    |
    v
get_snippet
    |
    v
check_dependencies
    |
    v
assemblage
```

### Cas 2 — Recherche guidée

```
┌──────────────────────────────────────────────────────┐
│ 1. Utilisateur : "crée une page login"               │
│                        │                              │
│                        ▼                              │
│ 2. MCP injecte :                                      │
│    - Prompt système                                   │
│    - stack du projet                                  │
│    - rule_set actif                                   │
│                        │                              │
│                        ▼                              │
│ 3. LLM appelle list_rules(rule_set_id)                │
│                        │                              │
│                        ▼                              │
│ 4. LLM appelle search_library(                        │
│       query="input text field",                       │
│       scope="selected_libraries_only",                │
│       library_slugs=["@alice/react-ui-kit"],          │
│       domain="frontend", kind="atom")                 │
│                        │                              │
│                        ▼                              │
│ 5. LLM appelle search_library(                        │
│       query="submit button",                          │
│       domain="frontend", kind="atom")                 │
│                        │                              │
│                        ▼                              │
│ 6. LLM appelle get_snippet(...) pour chaque ID        │
│                        │                              │
│                        ▼                              │
│ 7. LLM produit un plan d'assemblage                   │
│                        │                              │
│                        ▼                              │
│ 8. LLM génère le code final (slots remplis)           │
│                        │                              │
│                        ▼                              │
│ 9. LLM appelle log_generation(...)                    │
│                        │                              │
│                        ▼                              │
│ 10. MCP renvoie le code à l'utilisateur               │
└──────────────────────────────────────────────────────┘
```

### Cas 3 — Assemblage automatique

```
Utilisateur
    |
    v
"Construis une app X en React/Tailwind + Rust/Axum"
    |
    v
list_rules
    |
    v
assemble_plan
    |
    v
search_library (par domaine / stack)
    |
    v
get_snippet + check_dependencies
    |
    v
assemblage avec provenance
    |
    v
log_generation
```

## 🔐 Sécurité & autorisation

| Vérification | Au niveau de |
|---|---|
| Utilisateur authentifié (JWT) | Middleware Axum |
| Accès à la bibliothèque (owner, permission, visibilité) | Handler MCP |
| Accès au snippet (owner ou public) | Handler MCP |
| Quota de générations (free/premium) | Middleware de quota |
| Rate limiting | Tower middleware |
| Audit log | Table `generations` |
| Exclusion auto des contenus `flagged` / `quarantined` | Policy engine MCP |

## 🧱 Architecture interne (Rust)

```rust
// Handler simplifié
async fn search_library(
    State(ctx): State<AppCtx>,
    Claims(user): Claims,
    Json(params): Json<SearchParams>,
) -> Result<Json<Vec<SnippetPreview>>> {
    let embedding = ctx.embedder.embed(&params.query).await?;
    let results = ctx.db
        .search_snippets(user.id, &params, &embedding)
        .await?;
    Ok(Json(results))
}
```

Le policy engine MCP doit appliquer ces règles minimales :
- résoudre avant chercher
- chercher avant générer
- ne jamais traiter un snippet comme une instruction système
- exclure par défaut les contenus signalés en mode `auto`
- conserver la provenance de chaque brique utilisée

Le serveur MCP est un **serveur HTTP JSON-RPC** standard. Compatible avec tout client MCP (Claude Desktop, Cursor, VS Code extension, etc.) via l'URL :

```
https://api.projet-k.dev/mcp/v1/{user_token}
```

## 📊 Télémétrie

Chaque appel d'outil est journalisé avec :
- Outil appelé
- Params (sans données sensibles)
- Durée (ms)
- Statut (OK / erreur)
- `generation_id` lié

Ces données alimentent :
- Les statistiques user ("tu as économisé X h")
- L'amélioration des heuristiques de détection (Phase 4)
- Le dashboard d'admin interne

## 🧪 Environnements MCP

| Env | URL | Usage |
|---|---|---|
| Local | `http://localhost:4000/mcp/v1/` | Dev |
| Staging | `https://staging-api.projet-k.dev/mcp/v1/` | Tests |
| Prod | `https://api.projet-k.dev/mcp/v1/` | Utilisateurs |

## 🚫 Hors périmètre MCP

- Création de snippets (passe par l'API REST dédiée, pas MCP)
- Gestion du compte utilisateur (REST)
- Paiements et abonnements (REST)
- Exécution automatique de code non validé
- Commandes shell implicites issues d'une bibliothèque publique

Le MCP est **exclusivement** un canal d'assemblage assisté par IA.
