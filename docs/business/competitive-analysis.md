# Analyse Concurrentielle — Projet K (Komorebi)

> État des lieux du marché des assistants de code et positionnement stratégique. *(pré-pivot 2026-04-20)*

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Positionnement **mis à jour** post-pivot. Source actuelle : [`../strategy-quality-scored-registry.md`](../strategy-quality-scored-registry.md) §🏰 Moat.
>
> - UseStakly n'est plus positionné contre GitHub / Copilot / Cursor — il est **complémentaire** (ces outils sont les consommateurs via MCP).
> - Le vrai avantage compétitif n'est plus « bibliothèque personnelle » (perdant vs Copilot / shadcn) mais **scoring qualité dérivé de l'usage réel** — moat défendable : GitHub et npm ne peuvent pas y aller publiquement, les modèles fondamentaux ne peuvent pas construire d'index qualité externe en temps réel.
> - Les Gestionnaires de Snippets Traditionnels (Gists, SnippetsLab) ne sont plus des concurrents — ils sont trop bas niveau.
> - Nouveaux concurrents à surveiller : registries verticaux qui ajouteraient du scoring (hypothétique, aucun acteur identifié 2026-04).

## 1. Paysage Concurrentiel

Le marché est divisé en trois segments principaux :

### A. Les Gestionnaires de Snippets Traditionnels
*   **Outils :** GitHub Gists, SnippetsLab, Raycast Snippets.
*   **Limites :** Stockage passif. Aucune aide à l'assemblage ou à la réutilisation intelligente.

### B. Les Assistants IA Génératifs (Généralistes)
*   **Outils :** GitHub Copilot, Cursor, Windsurf (Codeium).
*   **Limites :** "Hallucinent" souvent du code générique. Ne respectent pas strictement les briques et patterns privés de l'utilisateur.

### C. Les Assistants IA Contextuels (Spécialisés)
*   **Outils :** Pieces.app, Sourcegraph Cody, v0.dev.
*   **Limites :** Souvent propriétaires (cloud), coûteux, et limités à un seul domaine (ex: frontend uniquement pour v0).

---

## 2. Analyse SWOT

| **Forces (Strengths)** | **Faiblesses (Weaknesses)** |
| :--- | :--- |
| **Système de RULES :** Force l'assemblage au lieu de la génération. | **Barrière à l'entrée :** Nécessite une bibliothèque initiale pour être puissant. |
| **MCP-Native :** Intégration directe dans tous les LLM via un standard ouvert. | **Maintenance :** Discipline requise pour garder les snippets à jour. |
| **Multi-Domaine :** Unifie Front, Back, DevOps et Data. | |
| **Local-First :** Coût variable nul (zéro API pour la recherche/détection). | |

| **Opportunités (Opportunities)** | **Menaces (Threats)** |
| :--- | :--- |
| **Marché B2B :** Gouvernance du code pour les équipes de dev. | **Évolution des LLM :** Si le "long context" devient parfait et sans hallucinations. |
| **Marketplace :** Vente de packs de snippets certifiés. | **Copilot "Rules" :** Si GitHub intègre nativement des contraintes strictes. |

---

## 3. Matrice de Positionnement

| Caractéristique | Traditionnel | Copilot/Cursor | **Projet K** |
| :--- | :---: | :---: | :---: |
| **Intelligence** | ❌ Non | ✅ Haute | ✅ Contextuelle |
| **Contrainte de réutilisation** | ❌ Non | ❌ Faible | ⭐ **Maximale (Rules)** |
| **Multi-domaine** | ✅ Oui | ✅ Oui | ✅ Oui |
| **Respect des standards privés** | ❌ Manuel | ⚠️ Aléatoire | ⭐ **Garanti** |
| **Coût d'usage** | Gratuit | 20$/mois | **0$ (Local)** |

---

## 4. Conclusion Stratégique : La "Blue Ocean"

Le Projet K ne cherche pas à être le meilleur "générateur" de code, mais le meilleur **gouverneur** de code. Sa proposition de valeur unique réside dans le passage de la magie générative (souvent instable) à la rigueur de l'assemblage (Komorebi).
