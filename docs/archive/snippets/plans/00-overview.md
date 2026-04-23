# Plan Global — Vue d'ensemble

> Version : 1.0 — 2026-04-15 *(pré-pivot 2026-04-20)*

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Ce plan **précède le pivot** vers le registry qualité-scored. Les 8 phases listées ci-dessous ne correspondent plus au découpage actuel.
>
> **Source de vérité d'exécution post-pivot** : [`../../TODO.md`](../../TODO.md) v4 (phases 0–11).
>
> Correspondance approximative :
> - Phases 1–3 historiques (fondations, backend, frontend) ≈ Phases 0–5 du TODO actuel — **acquis**.
> - Phase 2 (Backend MCP) ≈ Phases 6 + 8 du TODO actuel (quality signals + MCP) — **à refondre avec quality_context natif**.
> - Phase 4 (détection auto) ≈ secondaire post-pivot, peut alimenter les signaux mais n'est plus bloquant.
> - Phase 5 (RULES) ≈ secondaire — les RULES ne sont plus le filtre principal, le scoring prend le relais.
> - Phase 6 (communauté) ≈ à refondre — la publication publique devient annotation scoring.
> - Phase 7 (monétisation) ≈ à refondre — cf. strategy §💰 Business model (Free / Pro / Team / Enterprise).
>
> Le reste du document est conservé pour référence historique.

## 🎯 Objectif

Livrer un **MVP fonctionnel** du Projet K en **8 phases** séquentielles mais faiblement couplées.

Chaque phase a :
- Un **objectif unique et mesurable**
- Des **livrables concrets**
- Une **Definition of Done** claire
- Des **dépendances explicites**

## 🗺️ Les 8 phases

| # | Phase | Plan | Durée estimée | Bloquant pour |
|---|---|---|---|---|
| 1 | Fondations (monorepo, CI, DB) | [01-foundation.md](./01-foundation.md) | 1 semaine | Tout le reste |
| 2 | Serveur MCP (Rust, auth, API) | [02-backend-mcp.md](./02-backend-mcp.md) | 2-3 semaines | 3, 4, 5 |
| 3 | Studio frontend (React + Tailwind) | [03-frontend-studio.md](./03-frontend-studio.md) | 2-3 semaines | 6 |
| 4 | Moteur de détection automatique | [04-detection-engine.md](./04-detection-engine.md) | 1-2 semaines | - |
| 5 | Moteur de règles (RULES) | [05-rules-engine.md](./05-rules-engine.md) | 1 semaine | - |
| 6 | Communauté & publication | [06-community.md](./06-community.md) | 1-2 semaines | 7 |
| 7 | Monétisation (Free / Premium) | [07-monetization.md](./07-monetization.md) | 1 semaine | - |
| 8 | Déploiement production | Inclus dans 01 + par phase | Transverse | - |

**Total MVP** : ~10-12 semaines pour un développeur solo.

## 🔗 Dépendances

```
[01 Fondations]
      │
      ├──▶ [02 Backend MCP] ──┬──▶ [04 Détection]
      │                       │
      │                       ├──▶ [05 RULES]
      │                       │
      │                       └──▶ [03 Frontend] ──▶ [06 Communauté] ──▶ [07 Monétisation]
```

## 📅 Jalons

| Jalon | Critère | Phase |
|---|---|---|
| **M1 — Hello World** | Backend répond sur `/health`, DB connectée, frontend affiche la page d'accueil | Fin 01 |
| **M2 — Auth complète** | Création de compte, login, JWT fonctionnel | Mi 02 |
| **M3 — Snippets CRUD** | Création / lecture / update / delete via API | Fin 02 |
| **M4 — Détection fonctionnelle** | Un snippet collé est auto-classifié (domain, kind, language) | Fin 04 |
| **M5 — Studio utilisable** | Navigation, création, preview, editor Monaco | Fin 03 |
| **M6 — Premier assemblage MCP** | L'IA génère un composant depuis la librairie via le protocole MCP | Fin 02 (re-ouverture) |
| **M7 — RULES appliquées** | Une génération respecte les contraintes du rule_set | Fin 05 |
| **M8 — MVP public** | Premiers utilisateurs externes, free tier actif | Fin 07 |

## 🧪 Stratégie de tests

| Niveau | Outil | Couverture cible |
|---|---|---|
| Unit (Rust) | `cargo test` + `sqlx::test` | 70 % |
| Unit (TS) | `vitest` | 60 % |
| Integration API | Rust intégration tests + DB de test | Endpoints critiques |
| E2E | Playwright | Parcours principaux (signup, create snippet, generate) |

## 🚀 Stratégie de déploiement

- **Trunk-based** : 1 branche `main`, feature branches courtes
- **CI** : lint + test + build à chaque PR
- **CD** : auto-deploy staging sur merge, prod sur tag `v*`
- **Rollback** : via redéploiement d'un tag précédent
- **Feature flags** : `pgfeatureflags` simple table `feature_flags(name, enabled, user_ids[])`

## 📊 Suivi d'avancement

Voir `TODO.md` à la racine. Chaque item référence la phase correspondante :
```
- [ ] (01) Initialiser le monorepo
- [x] (02) Endpoint /health
```

## 🚫 Ce qui est explicitement repoussé après le MVP

- Intégration IDE native (VS Code extension) → v1.1
- Mobile app → v2
- Marketplace payante snippets → v1.2
- Web3 / crypto → v2+ (mais architecture prête)
- Collaboration temps réel sur un snippet → v2
- Fine-tuning de modèle propriétaire → Phase 4 détection (v1.x)
