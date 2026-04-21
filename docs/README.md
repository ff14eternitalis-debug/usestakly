# UseStakly — Documentation

> **Registry de code avec scoring qualité dérivé de l'usage réel, exposé aux agents IA via MCP pour filtrer automatiquement les briques obsolètes, cassées ou dangereuses avant même de proposer du code.** Rotten Tomatoes pour le code, lu par des agents.

> **Pivot du 2026-04-20** : l'angle « bibliothèque universelle de snippets » a été remplacé par l'angle registry qualité-scored. Plusieurs docs listés ci-dessous précèdent ce pivot et contiennent un bandeau de réconciliation en tête. Source de vérité produit : `strategy-quality-scored-registry.md`. Source de vérité exécution : `../TODO.md` v4.

> `UseStakly` est le nom produit retenu. `Project-K` reste le nom de travail historique encore présent dans certains documents et chemins techniques.

## 📚 Index

### Fondations
- [**strategy-quality-scored-registry.md**](./strategy-quality-scored-registry.md) — **Angle produit retenu** (décidé 2026-04-20) — à lire en premier
- [**vision.md**](./vision.md) — Vision, proposition de valeur, personas *(pré-pivot — bandeau de reconciliation)*
- [**product-vision-and-safety.md**](./product-vision-and-safety.md) — Vision universelle, bibliothèques adressables et couche Trust / Safety *(pré-pivot — bandeau de reconciliation)*
- [**business/competitive-analysis.md**](./business/competitive-analysis.md) — Analyse concurrentielle
- [**business/market-analysis-2026.md**](./business/market-analysis-2026.md) — Analyse du marché (Contexte 2026)
- [**business/financial-study.md**](./business/financial-study.md) — Étude de rentabilité financière
- [**user-journey.md**](./user-journey.md) — Parcours utilisateur (8 flows clés)
- [**tech-stack.md**](./tech-stack.md) — Choix techniques (Rust + React + Tailwind)
- [**architecture.md**](./architecture.md) — Classification multi-domaines & schéma DB
- [**nomenclature.md**](./nomenclature.md) — Convention de nommage des snippets
- [**data-model.md**](./data-model.md) — Schéma PostgreSQL complet *(à enrichir avec quality_signals)*

### Systèmes
- [**mcp-protocol.md**](./mcp-protocol.md) — Protocole MCP (outils, handlers, flux)
- [**rules-system.md**](./rules-system.md) — Les 5 RULES & format JSON
- [**detection-system.md**](./detection-system.md) — Détection automatique (plan zéro coût)
- [**deployment-coolify.md**](./deployment-coolify.md) — Stratégie d'hébergement cible sur Coolify
- [**dev-workflow.md**](./dev-workflow.md) — Démarrage local, commandes courantes, principes d'automatisation
- [**security-secrets-playbook.md**](./security-secrets-playbook.md) — Rotation des secrets et reprise de contrôle des variables sensibles

### Plans d'action (exécution)

> **Tous les plans `mvp-*.md` et `0X-*.md` ci-dessous précèdent le pivot du 2026-04-20.** Ils restent utiles pour les phases 0–5 (bootstrap, DB, frontend, auth, CRUD) mais leurs phases 6+ sont obsolètes. La référence d'exécution actuelle est `../TODO.md` v4.

- [**plans/00-overview.md**](./plans/00-overview.md) — Vue d'ensemble des phases *(pré-pivot)*
- [**plans/mvp-action-plan.md**](./plans/mvp-action-plan.md) — Plan d'action complet du MVP autonome *(pré-pivot)*
- [**plans/mvp-one-shot-blueprint.md**](./plans/mvp-one-shot-blueprint.md) — Spécification maître *(pré-pivot)*
- [**plans/mvp-file-by-file-checklist.md**](./plans/mvp-file-by-file-checklist.md) — Checklist d'implémentation *(pré-pivot)*
- [**plans/rename-to-usestakly.md**](./plans/rename-to-usestakly.md) — Plan de transition de `Project-K` vers `UseStakly`
- [**plans/01-foundation.md**](./plans/01-foundation.md) — Bootstrap du monorepo
- [**plans/02-backend-mcp.md**](./plans/02-backend-mcp.md) — Serveur MCP en Rust *(à enrichir : quality_context)*
- [**plans/03-frontend-studio.md**](./plans/03-frontend-studio.md) — Studio React + Tailwind
- [**plans/04-detection-engine.md**](./plans/04-detection-engine.md) — Moteur de détection *(secondaire post-pivot)*
- [**plans/05-rules-engine.md**](./plans/05-rules-engine.md) — Moteur de règles *(secondaire post-pivot)*
- [**plans/06-community.md**](./plans/06-community.md) — Publication & communauté *(à refondre : scoring public)*
- [**plans/07-monetization.md**](./plans/07-monetization.md) — Free / Premium *(à refondre : voir strategy §Business model)*

### Suivi
- [**../TODO.md**](../TODO.md) — Checklist globale d'exécution **(source de vérité exécution post-pivot)**

## 🧭 Comment lire cette doc

1. Si tu découvres le projet : **strategy-quality-scored-registry.md** → **vision.md** (avec le bandeau) → **../TODO.md**
2. Si tu veux coder : **../TODO.md** phases 6–11 puis reviens aux plans `plans/` pour les détails techniques
3. Si tu cherches une décision produit : d'abord **strategy-quality-scored-registry.md**, puis les docs spécifiques

## 📐 Conventions de la doc

- Chaque fichier a un **en-tête avec sa version** et la date de mise à jour
- Les plans d'action contiennent une section **"Definition of Done"**
- Les décisions architecturales sont **justifiées** (pas juste énoncées)
- Ce qui est **hors périmètre** est explicitement marqué
