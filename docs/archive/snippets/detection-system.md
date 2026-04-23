# Projet K — Système de Détection Automatique

> Version : 1.0 — 2026-04-15

## 🎯 Objectif

Quand un utilisateur colle un snippet, le système pré-remplit automatiquement :
`language`, `domain`, `kind`, `category`, `framework`, `tags`, `variables`.

**Contrainte clé** : zéro coût variable, zéro clé API au MVP.

## 🏗️ Pipeline de détection

```
┌─────────────────────────────────────┐
│ 1. Détection du language            │  tree-sitter + extension
│ 2. Mapping language → domain        │  table statique
│ 3. Détection du kind                │  regex + AST
│ 4. Détection de la category         │  mots-clés
│ 5. Détection du framework           │  imports / signatures
│ 6. Extraction des variables {{...}} │  regex
│ 7. Génération de l'embedding        │  fastembed local
│ 8. Suggestion de tags               │  top-K similarité
└─────────────────────────────────────┘
                │
                ▼
   Formulaire pré-rempli, éditable
   → L'utilisateur valide
```

## 🧩 Étape 1 — Détection du language

**Outils :**
- `hyperpolyglot` (Rust) — reconnaissance par fingerprint (basé sur linguist de GitHub)
- Fallback : extension de fichier si fournie

**Cas particuliers :**
- `.ts` ambigu entre frontend et backend → étape 2 + heuristiques imports
- `.js` idem
- Code inline sans nom de fichier → analyse tree-sitter multi-langue

## 🧩 Étape 2 — Mapping language → domain

```rust
fn default_domain(language: &str, code: &str) -> SnippetDomain {
    match language {
        "tsx" | "jsx" | "vue" | "svelte" | "html" | "css" | "scss"
            => Frontend,
        "rust" | "python" | "go" | "java" | "ruby" | "php" | "kotlin"
            => Backend,
        "sql" | "prisma"
            => Data,
        "yaml" | "toml" | "dockerfile" | "bash" | "sh"
            => DevOps,
        "ts" | "js" => disambiguate_ts_js(code),   // voir ci-dessous
        _ => Shared,
    }
}

fn disambiguate_ts_js(code: &str) -> SnippetDomain {
    if code.contains("from 'react'") || code.contains("<")
        { Frontend }
    else if code.contains("express") || code.contains("fastify") || code.contains("Prisma")
        { Backend }
    else
        { Shared }
}
```

## 🧩 Étape 3 — Détection du `kind`

Table de patterns par `(domain, language)` :

| Pattern regex | Domain | Kind |
|---|---|---|
| `export (default )?function [A-Z]\w+.*return\s*<` | frontend | `atom` ou `molecule` (cf étape 4) |
| `async fn \w+\(.*Request.*\)` | backend (rust) | `handler` |
| `app\.(get\|post\|put\|delete\|patch)\(` | backend (node) | `handler` |
| `@(Controller\|RestController)` | backend (java) | `handler` |
| `async fn.*middleware`/`Next\(req\)` | backend | `middleware` |
| `pub (struct\|enum) \w+\s*\{[^}]*\}` sans `impl` | shared ou backend | `type` ou `model` |
| `CREATE TABLE\|ALTER TABLE` | data | `migration` |
| `SELECT .* FROM` | data ou backend | `query` |
| `FROM \w+:\w+` | devops | `dockerfile` |
| `#!/bin/\w*sh` | devops | `script` |

**Distinction atom/molecule/organism** (frontend uniquement) :
- Compte le nombre de sous-composants custom (balises en PascalCase)
  - 0 → `atom`
  - 1-3 → `molecule`
  - 4+ → `organism`

## 🧩 Étape 4 — Détection de la `category`

Dictionnaire de mots-clés dans le code et commentaires :

| Mots-clés | Category |
|---|---|
| `login`, `signin`, `jwt`, `auth`, `bearer` | `auth` |
| `button`, `onClick`, `type='submit'` | `action` |
| `input`, `form`, `textarea`, `select` | `input` |
| `navbar`, `header`, `menu`, `breadcrumb` | `nav` |
| `log`, `info!`, `error!`, `tracing` | `logging` |
| `validate`, `validator`, `regex`, `schema` | `validation` |
| `CRUD`, `find`, `create`, `update`, `delete` | `crud` |

Si aucun mot-clé : category = `general` par défaut.

## 🧩 Étape 5 — Détection du `framework`

Via les imports :

| Import détecté | Framework |
|---|---|
| `from 'react'` | `react` |
| `from 'vue'` | `vue` |
| `use axum::` | `axum` |
| `use actix_web::` | `actix` |
| `from fastapi` | `fastapi` |
| `require('express')` | `express` |
| `@nestjs/` | `nestjs` |

## 🧩 Étape 6 — Extraction des variables

Regex globale sur le code : `\{\{\s*(\w+)\s*\}\}`

Puis inférence basique du type :
- Si utilisé entre balises texte → `string`
- Si utilisé comme `{{count}}+1` → `number`
- Si `if {{condition}}` → `boolean`

Les types sont **suggestions** ; l'utilisateur valide.

## 🧩 Étape 7 — Embedding vectoriel

**Outil** : `fastembed` (Rust) avec le modèle `bge-small-en-v1.5` (384 dims, ~90 Mo).

```rust
let embedding = embedder.embed(&[snippet.description_and_code()], None)?;
// stocké dans snippet_versions.embedding
```

**Performance** : ~50 ms par snippet sur CPU standard.
**Coût** : 0 €. Modèle téléchargé une fois au démarrage.

## 🧩 Étape 8 — Suggestion de tags

Avec l'embedding, trouve les N snippets les plus proches dans la librairie publique/perso et agrège leurs tags les plus fréquents :

```sql
SELECT t.name, COUNT(*) AS score
FROM snippet_versions sv
JOIN snippet_tags st ON st.snippet_id = sv.snippet_id
JOIN tags t ON t.id = st.tag_id
ORDER BY sv.embedding <=> $1  -- cosine distance
LIMIT 20;
```

## ✅ Validation humaine

La détection n'est **jamais** décisive. Le formulaire de création affiche :
- Les valeurs détectées en **suggestion pré-remplie**
- Un badge "suggéré" à côté de chaque champ détecté
- L'utilisateur peut éditer et valider

**Bénéfices :**
- Précision finale 100 % (l'humain valide)
- Les corrections alimentent un futur dataset d'amélioration (Phase 4)

## 📈 Plan d'évolution en 4 phases

| Phase | Stratégie | Coût variable | Précision cible |
|---|---|---|---|
| **1 — MVP** | Heuristiques locales + fastembed | **0 €** | ~80 % |
| **2 — Fallback LLM** | Gemini Flash / GPT-4o-mini en cas d'échec heuristique | < 1 €/mois pour 10k snippets | ~92 % |
| **3 — LLM local** | Ollama + Llama 3.1 8B côté serveur | 0 € variable (GPU fixe) | ~95 % |
| **4 — Fine-tuning** | Modèle spécialisé entraîné sur les corrections utilisateurs | 0 € variable | ~97 % |

**Règle** : on ne passe à la phase suivante que si la précédente montre ses limites en conditions réelles.

## 🔬 Mesure de la précision

À chaque création de snippet, on compare les valeurs **détectées** vs **validées par l'utilisateur** :

```sql
CREATE TABLE detection_feedback (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    suggested JSONB NOT NULL,       -- ce qu'on a détecté
    accepted JSONB NOT NULL,         -- ce que l'utilisateur a validé
    corrections JSONB,               -- diff
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

KPI clé : **% de suggestions acceptées sans modification**.

## 🚫 Ce qu'on ne fait **pas** au MVP

- ❌ Appels réseau vers OpenAI/Anthropic/Google pour la détection
- ❌ Stockage des snippets chez un tiers pour analyse
- ❌ Détection multi-fichiers (un snippet = un fichier isolé)
- ❌ Analyse sémantique profonde (data-flow, type inference poussée)
