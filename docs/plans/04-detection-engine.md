# Phase 4 — Moteur de Détection Automatique

> Version : 1.0 — 2026-04-15
> Durée estimée : 1-2 semaines
> Dépendances : Phase 2

## 🎯 Objectif

Implémenter le pipeline complet de détection décrit dans [detection-system.md](../detection-system.md), **100 % local, 0 € de coût variable**.

## 📋 Livrables

1. Crate Rust `detection/` avec pipeline en 8 étapes
2. Endpoint API `POST /detect` qui prend du code brut et retourne des suggestions
3. Intégration `fastembed` + téléchargement du modèle au premier lancement
4. Intégration `tree-sitter` pour au moins 8 langages
5. Table `detection_feedback` pour mesurer la précision

## 🔨 Tâches détaillées

### 4.1 Détection du language
- [ ] Crate `hyperpolyglot` (ou équivalent)
- [ ] Fallback par extension si fournie
- [ ] Fonction `detect_language(code, filename?) -> Language`

### 4.2 Mapping language → domain
- [ ] Table statique + heuristique pour TS/JS ambigus
- [ ] Tests unitaires couvrant les 15+ langages supportés au MVP

### 4.3 Détection du kind (regex + AST)
- [ ] Moteur de règles : `Vec<KindRule { lang, pattern, kind }>`
- [ ] Tree-sitter pour les cas complexes (atomic vs molecule via comptage sous-composants)
- [ ] Fonction `detect_kind(code, language, domain) -> Kind`

### 4.4 Détection de la category
- [ ] Dictionnaire de mots-clés `Map<Category, Vec<String>>`
- [ ] Score par category, retourner celle avec le meilleur score
- [ ] Fallback `general` si aucun match

### 4.5 Détection du framework
- [ ] Extraction des imports (tree-sitter query)
- [ ] Mapping import → framework
- [ ] Détection de la version si possible (comments, pragmas)

### 4.6 Extraction des variables `{{...}}`
- [ ] Regex `\{\{\s*(\w+)\s*\}\}` sur le code
- [ ] Inférence de type par contexte
- [ ] Déduplication

### 4.7 Embeddings via fastembed
- [ ] Setup `fastembed` crate + modèle `bge-small-en-v1.5`
- [ ] Téléchargement lazy au premier appel (ou au démarrage serveur)
- [ ] Service `Embedder` partagé (Arc) injecté dans les handlers
- [ ] Tests : embedding de 2 textes proches → cosine > 0.8

### 4.8 Suggestion de tags
- [ ] Requête SQL de similarité + agrégation des tags voisins
- [ ] Top 5 tags retournés en suggestion

### 4.9 Endpoint API
- [ ] `POST /detect` : body = `{"code": "...", "filename": "..."}` → réponse = toutes les suggestions
- [ ] Intégration dans le formulaire de création frontend
- [ ] Mesure du temps de détection (cible : < 300 ms)

### 4.10 Feedback loop
- [ ] Table `detection_feedback` (cf [detection-system.md](../detection-system.md))
- [ ] Sauvegarde automatique au moment de la validation du formulaire
- [ ] Dashboard interne : % d'acceptation des suggestions par champ

## ✅ Definition of Done

- [ ] Le pipeline détecte correctement language + domain + kind pour les 15+ langages cibles
- [ ] Précision ≥ 80 % mesurée sur un jeu de 50 snippets de test
- [ ] Temps de détection moyen < 300 ms (mesure P95)
- [ ] Aucun appel réseau externe pendant la détection
- [ ] Le modèle fastembed est téléchargé une seule fois et caché localement
- [ ] Les corrections utilisateur sont enregistrées en base

## ⚠️ Pièges à éviter

- ❌ Charger tree-sitter à chaque requête → charger une fois au démarrage (Arc)
- ❌ Embedder à chaque caractère tapé → debounce côté frontend
- ❌ Bloquer le serveur au démarrage pendant le téléchargement du modèle → en tâche async
- ❌ Traiter les snippets trop longs (> 100 Ko) — ajouter une limite

## 🚀 Évolution prévue (hors MVP)

- **Phase 2 détection** (v1.1) : fallback LLM cloud (Gemini Flash) quand la confiance est faible
- **Phase 3 détection** (v1.2) : LLM local via Ollama côté serveur
- **Phase 4 détection** (v1.3) : modèle fine-tuné sur le dataset `detection_feedback` accumulé

## 📚 Références
- [detection-system.md](../detection-system.md)
- [data-model.md](../data-model.md) (table `snippet_versions.embedding`, `detection_feedback`)
