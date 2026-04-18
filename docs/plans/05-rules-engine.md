# Phase 5 — Moteur de RULES

> Version : 1.0 — 2026-04-15
> Durée estimée : 1 semaine
> Dépendances : Phase 2

## 🎯 Objectif

Implémenter le **système de règles** qui contraint l'IA : définir le JSON Schema, les rule_sets par défaut, la validation, l'injection dans le prompt MCP.

## 📋 Livrables

1. JSON Schema pour la structure des rules (`shared/rules/schema.json`)
2. 3 rule_sets par défaut livrés en seed : `default-react-tailwind`, `default-rust-axum`, `default-fullstack-starter`
3. Module Rust de validation d'un `rule_set`
4. Injection des règles dans le prompt système MCP via `list_rules`
5. UI frontend pour cloner et éditer un rule_set

## 🔨 Tâches détaillées

### 5.1 JSON Schema
- [ ] Écrire `shared/rules/schema.json` conforme à Draft 2020-12
- [ ] Couvrir les 5 catégories : assembly, structure, stack, context_awareness, quality
- [ ] Publier le schema à `/schemas/rules/v1.json` pour que l'UI l'utilise

### 5.2 Rule_sets par défaut
- [ ] Créer `shared/rules/defaults/default-react-tailwind.json`
- [ ] Créer `shared/rules/defaults/default-rust-axum.json`
- [ ] Créer `shared/rules/defaults/default-fullstack-starter.json`
- [ ] Seed automatique au démarrage : ces rule_sets sont créés avec `is_default = true` et `owner_id = null` (ou user système)

### 5.3 Validation Rust
- [ ] Crate `jsonschema` pour valider à la sauvegarde
- [ ] Validation sémantique supplémentaire :
  - [ ] Les `component_paths` existent et sont des chemins relatifs valides
  - [ ] Les frameworks déclarés existent dans notre whitelist
  - [ ] `max_file_lines` > 0 et raisonnable (< 1000)
- [ ] Erreurs précises pour chaque violation

### 5.4 Endpoints API
- [ ] `GET /rule-sets` — liste les rule_sets accessibles (défaut + perso)
- [ ] `GET /rule-sets/:id`
- [ ] `POST /rule-sets` — créer
- [ ] `POST /rule-sets/:id/clone` — cloner un rule_set (notamment les défauts)
- [ ] `PATCH /rule-sets/:id`
- [ ] `DELETE /rule-sets/:id` (interdit sur les défauts)

### 5.5 Intégration MCP
- [ ] Outil `list_rules(rule_set_id)` retourne le JSON parsé
- [ ] Au démarrage d'une session MCP, injecter dans le prompt :
  - La stack attendue
  - Les 5 catégories de règles actives
  - Les interdictions explicites

### 5.6 UI frontend
- [ ] Page `Rule Sets` : liste des rule_sets, bouton "Cloner le défaut"
- [ ] Éditeur JSON avec validation en live (Monaco + JSON Schema)
- [ ] Vue "structurée" (formulaire par catégorie) en alternative à l'éditeur brut
- [ ] Badge "utilisé par X projets"

### 5.7 Tests
- [ ] Test : un rule_set valide passe la validation
- [ ] Test : un JSON mal formé retourne une erreur localisée
- [ ] Test : les rule_sets par défaut sont présents après migration
- [ ] Test : un user ne peut pas modifier un rule_set par défaut

## ✅ Definition of Done

- [ ] Les 3 rule_sets par défaut sont accessibles à tout utilisateur
- [ ] Un utilisateur peut cloner un défaut, l'éditer, le lier à un projet
- [ ] Une génération MCP reçoit les règles via `list_rules` et les respecte (vérification manuelle)
- [ ] Un JSON invalide est rejeté avec un message clair
- [ ] Le schema JSON est versionné (v1) → prêt pour une v2 plus tard

## ⚠️ Pièges à éviter

- ❌ Stocker les règles en format propriétaire — JSON + JSON Schema standards
- ❌ Laisser l'IA ignorer une règle silencieusement → doit lever un warning dans le rapport
- ❌ Surcharger les règles au MVP — rester sur les 5 catégories, ne pas ajouter de 6e
- ❌ Hard-coder la whitelist des frameworks → mettre dans un fichier de config

## 📚 Références
- [rules-system.md](../rules-system.md)
- [mcp-protocol.md](../mcp-protocol.md)
