# UseStakly — Documentation

> **Veille GitHub OSS avec scoring qualité**, consommable par des devs et des agents IA via MCP.

> **Pivot du 2026-04-21** : le scope a été resserré aux **repos GitHub publics OSS**. Le précédent produit de bibliothèque de snippets est abandonné. Plusieurs docs plus anciennes restent présentes comme archives de conception et ne doivent plus être lues comme source de vérité opérationnelle.

> `UseStakly` est le nom produit retenu. `Project-K` reste le nom de travail historique encore présent dans certains documents et chemins techniques.

## 📚 Index

### Fondations
- [**strategy-pivot-2026-04-21.md**](./strategy-pivot-2026-04-21.md) — **Scope produit actuel** : repos GitHub publics OSS uniquement
- [**strategy-quality-scored-registry.md**](./strategy-quality-scored-registry.md) — Moat et principes de scoring qualité
- [**architecture-backend-current.md**](./architecture-backend-current.md) — Vue backend actuelle post-refacto
- [**trust-model-v1.md**](./trust-model-v1.md) — Règles de réputation, consensus, review et dispute
- [**business/competitive-analysis.md**](./business/competitive-analysis.md) — Analyse concurrentielle
- [**business/market-analysis-2026.md**](./business/market-analysis-2026.md) — Analyse du marché (Contexte 2026)
- [**business/financial-study.md**](./business/financial-study.md) — Étude de rentabilité financière
- [**user-journey.md**](./user-journey.md) — Parcours utilisateur *(à relire avec le pivot en tête)*
- [**tech-stack.md**](./tech-stack.md) — Choix techniques

### Systèmes
- [**mcp-protocol.md**](./mcp-protocol.md) — Protocole MCP GitHub post-pivot
- [**deployment-coolify.md**](./deployment-coolify.md) — Stratégie d'hébergement cible sur Coolify
- [**dev-workflow.md**](./dev-workflow.md) — Démarrage local, commandes courantes, principes d'automatisation
- [**security-secrets-playbook.md**](./security-secrets-playbook.md) — Rotation des secrets et reprise de contrôle des variables sensibles

### Plans d'action (exécution)

> **Les plans `mvp-*.md` et plusieurs docs architecture/data pré-datent le pivot snippets → GitHub OSS.** Ils servent surtout d'archives techniques pour les fondations déjà livrées. La référence d'exécution actuelle est `../TODO.md`.

- [**plans/rename-to-usestakly.md**](./plans/rename-to-usestakly.md) — Plan de transition de `Project-K` vers `UseStakly`

### Archives snippets

Les documents centrés sur l'ancien produit snippets ont été déplacés dans [**archive/snippets/**](./archive/snippets/README.md) pour éviter toute confusion avec le produit GitHub actuel.

### Suivi
- [**../TODO.md**](../TODO.md) — Checklist globale d'exécution **(source de vérité exécution post-pivot)**
- [**coherence-audit-2026-04-23.md**](./coherence-audit-2026-04-23.md) — Audit final de cohérence après sprints 1 à 4 de refacto

## 🧭 Comment lire cette doc

1. Si tu découvres le projet : **strategy-pivot-2026-04-21.md** → **strategy-quality-scored-registry.md** → **../TODO.md**
2. Si tu veux coder : **../TODO.md** d'abord, puis **mcp-protocol.md** ou les docs techniques pertinentes
3. Si tu tombes sur une doc snippets/bibliothèques : la traiter comme **archive**, sauf mention contraire explicite dans `TODO.md`

## 📐 Conventions de la doc

- Chaque fichier a un **en-tête avec sa version** et la date de mise à jour
- Les plans d'action contiennent une section **"Definition of Done"**
- Les décisions architecturales sont **justifiées** (pas juste énoncées)
- Ce qui est **hors périmètre** est explicitement marqué
