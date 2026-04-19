# UseStakly — Documentation

> Bibliothèque universelle de snippets de code multi-langages, pilotée par un serveur MCP qui force une IA à réutiliser les briques existantes de l'utilisateur au lieu d'inventer du code.

> `UseStakly` est le nom produit retenu. `Project-K` reste le nom de travail historique encore présent dans certains documents et chemins techniques.

## 📚 Index

### Fondations
- [**vision.md**](./vision.md) — Vision, proposition de valeur, personas
- [**product-vision-and-safety.md**](./product-vision-and-safety.md) — Vision universelle, bibliothèques adressables et couche Trust / Safety
- [**business/competitive-analysis.md**](./business/competitive-analysis.md) — Analyse concurrentielle
- [**business/market-analysis-2026.md**](./business/market-analysis-2026.md) — Analyse du marché (Contexte 2026)
- [**business/financial-study.md**](./business/financial-study.md) — Étude de rentabilité financière
- [**user-journey.md**](./user-journey.md) — Parcours utilisateur (8 flows clés)
- [**tech-stack.md**](./tech-stack.md) — Choix techniques (Rust + React + Tailwind)
- [**architecture.md**](./architecture.md) — Classification multi-domaines & schéma DB
- [**nomenclature.md**](./nomenclature.md) — Convention de nommage des snippets
- [**data-model.md**](./data-model.md) — Schéma PostgreSQL complet

### Systèmes
- [**mcp-protocol.md**](./mcp-protocol.md) — Protocole MCP (outils, handlers, flux)
- [**rules-system.md**](./rules-system.md) — Les 5 RULES & format JSON
- [**detection-system.md**](./detection-system.md) — Détection automatique (plan zéro coût)
- [**deployment-coolify.md**](./deployment-coolify.md) — Stratégie d'hébergement cible sur Coolify
- [**security-secrets-playbook.md**](./security-secrets-playbook.md) — Rotation des secrets et reprise de contrôle des variables sensibles

### Plans d'action (exécution)
- [**plans/00-overview.md**](./plans/00-overview.md) — Vue d'ensemble des phases
- [**plans/mvp-action-plan.md**](./plans/mvp-action-plan.md) — Plan d'action complet du MVP autonome
- [**plans/mvp-one-shot-blueprint.md**](./plans/mvp-one-shot-blueprint.md) — Spécification maître pour construire le MVP en une seule passe
- [**plans/mvp-file-by-file-checklist.md**](./plans/mvp-file-by-file-checklist.md) — Checklist d'implémentation détaillée fichier par fichier
- [**plans/rename-to-usestakly.md**](./plans/rename-to-usestakly.md) — Plan de transition de `Project-K` vers `UseStakly`
- [**plans/01-foundation.md**](./plans/01-foundation.md) — Bootstrap du monorepo
- [**plans/02-backend-mcp.md**](./plans/02-backend-mcp.md) — Serveur MCP en Rust
- [**plans/03-frontend-studio.md**](./plans/03-frontend-studio.md) — Studio React + Tailwind
- [**plans/04-detection-engine.md**](./plans/04-detection-engine.md) — Moteur de détection
- [**plans/05-rules-engine.md**](./plans/05-rules-engine.md) — Moteur de règles
- [**plans/06-community.md**](./plans/06-community.md) — Publication & communauté
- [**plans/07-monetization.md**](./plans/07-monetization.md) — Free / Premium

### Suivi
- [**../TODO.md**](../TODO.md) — Checklist globale d'exécution

## 🧭 Comment lire cette doc

1. Si tu découvres le projet : **vision.md** → **architecture.md** → **plans/00-overview.md**
2. Si tu veux coder : pars directement d'un plan dans `plans/`
3. Si tu cherches une décision : consulte la section correspondante dans `docs/`

## 📐 Conventions de la doc

- Chaque fichier a un **en-tête avec sa version** et la date de mise à jour
- Les plans d'action contiennent une section **"Definition of Done"**
- Les décisions architecturales sont **justifiées** (pas juste énoncées)
- Ce qui est **hors périmètre** est explicitement marqué
