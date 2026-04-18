# Analyse de Marché (Contexte 2026) — Pourquoi maintenant ?

> Analyse rétrospective des barrières technologiques et psychologiques ayant empêché la création de cet outil avant 2026.

## 1. Le Biais de la "Génération Pure" (2023-2025)

L'industrie de l'IA s'est focalisée sur la puissance de génération brute (plus de paramètres, plus de données open-source). L'idée prédominante était que l'IA finirait par écrire le code parfait sans aide. 
**Erreur constatée :** Les entreprises n'ont pas besoin de "code parfait théorique", elles ont besoin de code qui respecte **leur** architecture et **leurs** composants. Le besoin de "Gouvernance de l'IA" a été ignoré jusqu'en 2026.

## 2. L'Illusion de la Fenêtre de Contexte Géante

Les outils comme Cursor (2024) ont parié sur des fenêtres de contexte massives (1M+ tokens) pour "lire" tout le repo.
**Le problème :** Le phénomène de *Lost in the Middle* (l'IA se perd dans la masse) et le coût prohibitif des tokens ont montré les limites de la force brute. Projet K propose une alternative chirurgicale : injecter uniquement les snippets pertinents et des règles de fer.

## 3. Le Pivot technologique : Model Context Protocol (MCP)

Avant l'introduction du standard **MCP** par Anthropic fin 2024, connecter de manière fluide une base de snippets locale (PostgreSQL) aux serveurs des LLM était une tâche complexe et propriétaire.
**Opportunité 2026 :** Le Projet K est l'un des premiers outils "MCP-native", profitant d'un écosystème enfin standardisé où l'IA peut "appeler" une bibliothèque de briques externes.

## 4. L'Angle Mort Multi-Domaine

La plupart des startups se sont concentrées sur le Frontend (le plus visuel et facile à vendre). 
**Le vide :** Le Backend, le DevOps et la Data ont été délaissés à cause de la complexité de leur taxonomie. Projet K comble ce vide en appliquant une logique d'assemblage universelle, traitant un script Bash avec la même rigueur qu'un composant React.

## 5. Le Rejet du "Cloud-Only"

Le marché 2026 voit une fatigue des abonnements SaaS IA coûteux et des inquiétudes sur la confidentialité des données.
**Réponse de Projet K :** Un moteur de détection et de recherche 100% local (Rust + Tree-sitter + Fastembed). C'est un positionnement "Developer-First" qui privilégie la vie privée et la performance locale.
