# Audit de cohérence — 2026-04-23

> Version : 1.0
> Portée : cohérence code + docs après sprints 1 à 4 du plan de refacto

## Verdict

Le projet est globalement cohérent.

Le runtime actif, le README, le TODO et l'index docs racontent maintenant la même histoire produit :

- veille GitHub OSS
- scoring qualité
- watchlist + notifications
- MCP
- signaux modérés

Les plus gros écarts observés relevaient surtout de la documentation, pas du code runtime.

## Vérifications passées

- `cargo check` : OK
- `cargo test` : OK
- `npm run build` : OK

## Écarts corrigés dans ce sprint

### 1. Index docs

`docs/README.md` listait encore `data-model.md` comme fondation active alors que cette doc est en partie legacy.

Décision :

- retrait de `data-model.md` des fondations vivantes
- ajout d'une architecture backend actuelle
- ajout d'une doc trust model v1

### 2. Source de vérité frontend

Le frontend utilise bien :

- TanStack Router
- TanStack Query
- routes actives discovery / repo / watchlist / notifications / account / login

Certaines docs d'agent avaient dérivé sur ce point et ont été réalignées.

### 3. Refacto sprint 3

La décomposition frontend est désormais cohérente avec le plan de refacto :

- routes plus fines
- composants de page extraits
- clients API métier séparés

## Écarts restants acceptés

### 1. Data model historique

`docs/data-model.md` reste utile comme archive technique partielle, mais n'est plus une doc de vérité du produit actif.

### 2. Legacy SQL snippets

Les tables historiques snippets/libraries peuvent encore exister en base.
Elles sont considérées comme dette passive, pas comme surface produit active.

### 3. Trust model v2

Le trust model v1 est documenté et en place, mais la réputation v2 et le support org GitHub privé restent des chantiers ouverts.

## Recommandation

Le projet est dans un état cohérent pour continuer le produit sans refacto structurelle urgente supplémentaire.

Les prochains chantiers à forte valeur ne sont plus des nettoyages de structure, mais des chantiers produit :

- réputation / trust v2
- recherche sémantique R2b
- E2E
- amélioration du parcours utilisateur
