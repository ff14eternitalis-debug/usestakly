# Projet K — Système de RULES

> Version : 1.0 — 2026-04-15

## 🎯 Objectif

Les RULES sont le **contrat** imposé à l'IA. Stockées en JSONB dans `rule_sets.rules`, elles définissent **comment** l'IA doit assembler les snippets, dans quelle arborescence, avec quelles contraintes.

**Règle d'or** : ce qui n'est pas dans les RULES n'est pas garanti. Tout comportement de l'IA doit être traçable à une règle.

## 🧱 Les 5 catégories

### 1. Règle d'Assemblage (Atomic Design)

Comment réutiliser les briques existantes.

```json
{
  "assembly": {
    "reuse_threshold": 0.8,
    "forbid_inline_creation": true,
    "prefer_extension_over_recreation": true,
    "max_new_snippets_per_generation": 2
  }
}
```

| Clé | Description |
|---|---|
| `reuse_threshold` | Si un snippet couvre ≥ X% du besoin, l'utiliser |
| `forbid_inline_creation` | Interdit de redéfinir un composant qui existe déjà |
| `prefer_extension_over_recreation` | Étendre via props plutôt que recréer |
| `max_new_snippets_per_generation` | Limite de snippets inédits par génération |

### 2. Règle de Structure (Squelette)

Arborescence imposée du projet final.

```json
{
  "structure": {
    "frontend_root": "/src",
    "component_paths": {
      "atom": "components/atoms",
      "molecule": "components/molecules",
      "organism": "components/organisms",
      "template": "components/templates"
    },
    "backend_paths": {
      "handler": "src/handlers",
      "service": "src/services",
      "model": "src/models",
      "query": "src/queries",
      "middleware": "src/middlewares"
    },
    "shared_path": "shared",
    "max_file_lines": 150,
    "split_on_exceed": true
  }
}
```

### 3. Règle de Stack Technique (Contrat)

Le contrat technique que l'IA ne peut pas rompre.

```json
{
  "stack": {
    "frontend": {
      "framework": "react",
      "framework_version": ">=19",
      "styling": "tailwind",
      "styling_version": ">=4",
      "language": "tsx",
      "state_management": "zustand",
      "allowed_imports": ["react", "zustand", "@tanstack/*", "lucide-react"],
      "forbidden_imports": ["jquery", "lodash"]
    },
    "backend": {
      "language": "rust",
      "framework": "axum",
      "database_driver": "sqlx",
      "allowed_crates": ["tokio", "serde", "sqlx", "axum", "uuid", "tracing"]
    },
    "naming_conventions": {
      "components": "PascalCase",
      "functions": "camelCase",
      "files": "kebab-case",
      "css": "utility-first"
    }
  }
}
```

### 4. Règle de Context Awareness (Lien MCP)

Ce qui rend le Projet K **transparent** pour l'utilisateur.

```json
{
  "context_awareness": {
    "require_search_before_generation": true,
    "require_plan_before_code": true,
    "require_provenance_comment": true,
    "provenance_format": "// Assemblé depuis: {slug}@{version}",
    "explain_snippet_choice": true
  }
}
```

L'IA doit **justifier** chaque utilisation d'atome et **lister** les snippets trouvés avant de générer.

### 5. Règle de Documentation & Qualité

Qualité minimale imposée.

```json
{
  "quality": {
    "require_jsdoc": true,
    "jsdoc_must_include": ["role", "provenance", "params"],
    "require_types_export": true,
    "forbid_any_type": true,
    "max_function_complexity": 10,
    "require_tests_for": ["service", "function", "handler"],
    "accessibility": {
      "require_aria_labels": true,
      "require_semantic_html": true
    }
  }
}
```

## 📦 Exemple complet d'un `rule_set`

```json
{
  "version": "1.0",
  "assembly": {
    "reuse_threshold": 0.8,
    "forbid_inline_creation": true,
    "prefer_extension_over_recreation": true,
    "max_new_snippets_per_generation": 2
  },
  "structure": {
    "frontend_root": "/src",
    "component_paths": {
      "atom": "components/atoms",
      "molecule": "components/molecules",
      "organism": "components/organisms"
    },
    "backend_paths": {
      "handler": "src/handlers",
      "service": "src/services"
    },
    "max_file_lines": 150
  },
  "stack": {
    "frontend": {
      "framework": "react",
      "styling": "tailwind",
      "language": "tsx"
    },
    "backend": {
      "language": "rust",
      "framework": "axum"
    }
  },
  "context_awareness": {
    "require_search_before_generation": true,
    "require_plan_before_code": true,
    "require_provenance_comment": true
  },
  "quality": {
    "require_jsdoc": true,
    "forbid_any_type": true
  }
}
```

## 🎁 Rule sets par défaut livrés

| Nom | Description |
|---|---|
| `default-react-tailwind` | React 19 + Tailwind v4 + TS strict |
| `default-rust-axum` | Rust + Axum + sqlx + PostgreSQL |
| `default-fullstack-starter` | Combo des deux, parfait pour un SaaS |

L'utilisateur peut **cloner** ces défauts et les modifier → table `rule_sets.is_default = true` pour les templates officiels.

## 🔒 Validation d'un rule_set

À la sauvegarde :
1. JSON Schema (à définir dans `shared/rules/schema.json`)
2. Validation sémantique (chemins valides, frameworks connus)
3. Compatibilité avec `snippet_kinds` existants

## 🔄 Évolution

- Les rule_sets sont **versionnés** (`version: "1.0"` dans le JSON)
- Un upgrade de version majeure lance un **assistant de migration** côté frontend
- Les anciennes générations référencent la version utilisée (dans `generations.plan`)
