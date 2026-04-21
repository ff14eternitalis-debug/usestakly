# Projet K — Komorebi: GEMINI.md Contextual Instructions

> **Bibliothèque universelle de snippets de code, pilotée par une IA contrainte de réutiliser tes propres briques.**

Ce fichier définit les règles, conventions et le contexte du projet pour les interactions avec Gemini CLI.

---

## 🎯 Vue d'ensemble du projet

Le **Projet K (Komorebi)** est un "GitHub personnel intelligent". Il permet de stocker des snippets de code multi-langages et multi-domaines. Un serveur MCP expose ces snippets à l'IA, et un système de **RULES** force l'IA à assembler ces briques existantes au lieu d'en inventer de nouvelles.

### Architecture & Domaines
Le projet utilise une classification à deux axes (voir `docs/architecture.md`) :
1.  **Axe 1 — Domaine :** `frontend`, `backend`, `devops`, `data`, `shared`.
2.  **Axe 2 — Type (Kind) :** Adaptatif selon le domaine (ex: `atom` pour frontend, `handler` pour backend).

### Stack Technique
-   **Backend :** Rust (Axum, sqlx, PostgreSQL + pgvector).
-   **Frontend :** React 19 (Tailwind v4, TypeScript, Vite, TanStack Router).
-   **Détection :** tree-sitter + fastembed (100% local).
-   **Infrastructure :** Docker + Coolify sur VPS auto-hébergé (frontend + backend + Postgres sur la même plateforme, aucun SaaS externe).

---

## 📁 Structure du Projet (Monorepo prévu)

Le projet est actuellement en phase de démarrage (Phase 1). L'arborescence cible est la suivante :
-   `backend-core/` : Serveur API Rust & MCP.
-   `frontend-studio/` : Interface React.
-   `shared/` : Types et schémas partagés.
-   `docs/` : Documentation complète (Source de vérité pour l'architecture).
-   `deploy/` : Scripts et configurations de déploiement.

---

## 🛠️ Commandes & Développement

*Note : Ces commandes sont prévues pour la Phase 1.*

-   **Backend :** `cd backend && cargo run`
-   **Frontend :** `cd frontend && npm run dev`
-   **Base de données :** `docker compose up -d` (PostgreSQL + pgvector)

---

## 📜 Conventions & Règles de Codage

### Nomenclature des Snippets
Tout snippet doit suivre le format : `{domain}-{kind}-{category}-{name}-{variant?}`.
*Exemple : `backend-handler-auth-login-jwt`*

### Principes d'Architecture
-   **Interdiction d'inventer :** L'IA doit toujours chercher des snippets existants via `search_library` avant de proposer du code.
-   **Provenance :** Chaque composant généré doit inclure un commentaire de provenance (ex: `// Assemblé depuis: domain-kind-name@version`).
-   **Séparation des responsabilités (Backend) :** `handler` (I/O) → `service` (logique) → `query` (DB).
-   **Atomic Design (Frontend) :** Strictement appliqué (`atom` → `molecule` → `organism`).

### Qualité & Sécurité
-   **Zéro Coût Variable :** Privilégier les solutions locales (tree-sitter, fastembed) pour la détection.
-   **Validation Humaine :** Les suggestions de l'IA sont des propositions que l'utilisateur doit valider.

---

## 🚦 État Actuel & Priorités (TODO)

Se référer au fichier `TODO.md` pour le suivi des tâches.
**Priorité actuelle : Phase 1 — Fondations**
-   Initialisation du monorepo.
-   Mise en place du serveur Rust minimal.
-   Mise en place du frontend React + Tailwind v4.
-   Configuration de la base de données vectorielle.

---

## 📖 Documentation de Référence

-   `docs/architecture.md` : Modèle de données et nomenclature (CRITIQUE).
-   `docs/vision.md` : Objectifs et proposition de valeur.
-   `docs/tech-stack.md` : Détails des technologies utilisées.
-   `docs/plans/` : Plans d'exécution détaillés par phase.
