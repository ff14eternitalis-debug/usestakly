# Plan d'action refacto — 2026-04-23

> Portee : produit vivant **UseStakly** apres pivot GitHub OSS.  
> Objectif : reduire la dette de structure avant qu'elle ne ralentisse les prochaines features (recherche semantique, E2E, reputation v2, moderation plus riche).  
> Statut : plan en cours d'execution. Sprint 1 termine le 2026-04-23.

## Verdict rapide

Le projet n'a pas besoin d'une refonte generale immediate. Le socle tourne, le pivot est bien materialise, et les flows critiques existent.

En revanche, **oui, il y a de la refacto a faire**. Le risque principal n'est pas un bug actuel, c'est la dette de **concentration** :

- trop de logique dans certains handlers backend
- trop de responsabilites dans certaines pages frontend
- regles produit / moderation / scoring encore disperses
- docs encore legerement en retard sur la structure finale

Si on ne traite pas ca bientot, chaque nouvelle feature va devenir plus lente a ajouter et plus risquee a tester.

## Constats principaux

### 1. Le handler `repos` est devenu un point de concentration

Fichier concerne : `backend/src/handlers/repos.rs`

Il gere aujourd'hui :

- search
- get repo
- add repo
- create active signal
- viewer-state
- dispute owner

Ce fichier concentre de l'I/O HTTP, de la validation, des decisions produit et du wiring de moderation. Il fonctionne, mais il commence a devenir un mini-orchestrateur metier.

### 2. Le scoring est encore trop proche des regles produit de moderation

Fichier concerne : `backend/src/services/quality/scoring.rs`

Le service fait a la fois :

- chargement de formule
- calcul numerique
- resolution des flags publics
- consensus
- dependance a la reputation user
- emission de notifications

Le coeur de calcul est encore lisible, mais le fichier agrège des responsabilites de plus en plus heterogenes.

### 3. La logique "trust" est repartie sur trop de services

Fichiers concernes :

- `backend/src/services/reputation.rs`
- `backend/src/services/repo_owners.rs`
- `backend/src/services/signal_reviews.rs`
- `backend/src/services/signal_events.rs`
- `backend/src/services/agent_token_events.rs`

Individuellement, ces fichiers sont corrects. Collectivement, ils racontent une meme sous-architecture "trust & moderation" qui n'est pas encore exprimee comme un sous-domaine clair.

### 4. Le frontend a des pages qui melangent trop de concerns

Fichiers concernes :

- `frontend/src/routes/repo-detail.tsx`
- `frontend/src/routes/account.tsx`

`repo-detail.tsx` gere a la fois :

- affichage du repo
- watchlist
- signaux actifs
- timeline de review
- dispute owner

`account.tsx` gere a la fois :

- infos compte
- reputation
- tokens MCP
- moderation admin

Ca reste tenable a court terme, mais ce n'est deja plus agreable a faire evoluer.

### 5. La couche API frontend commence a exposer des usages "speciaux"

Fichier concerne : `frontend/src/lib/api-client.ts`

L'ajout de variantes `apiGetWithInit` / `apiPostWithInit` est utile, mais signale un glissement :

- les appels "normaux" et les appels "admin token" ne devraient peut-etre pas passer par la meme abstraction simple

Ce n'est pas une erreur, mais c'est souvent un precurseur de couplage diffus.

### 6. La doc n'est pas completement homogene

Point notable :

- `docs/README.md` reference encore `data-model.md` dans les fondations alors que les docs snippets ont ete archivees

Ce n'est pas bloquant, mais ca confirme que la narration du repo n'est pas encore totalement stabilisee.

## Priorites de refacto

## Priorite 1 — Decoupage backend par sous-domaines

### Objectif

Faire emerger des sous-domaines clairs au lieu de continuer a grossir les handlers/services generalistes.

### Action

Creer une structure explicite autour de 4 blocs :

- `repos`
- `trust`
- `mcp`
- `notifications`

### Refacto cible

#### Backend HTTP

Scinder `backend/src/handlers/repos.rs` en plusieurs fichiers :

- `handlers/repos_query.rs`
- `handlers/repos_ingestion.rs`
- `handlers/repo_signals.rs`
- `handlers/repo_viewer.rs`

#### Backend services

Introduire un namespace ou dossier `services/trust/` pour regrouper :

- reputation
- owner verification
- signal review
- signal events
- MCP token write guardrails

### Benefices

- handlers plus lisibles
- tests plus simples a cibler
- evolutions moderation/reputation moins risquees

## Priorite 2 — Isoler le coeur du scoring

### Objectif

Separer le calcul pur de la formule du reste du pipeline.

### Action

Dans `backend/src/services/quality/`, separer :

- `formula.rs` : chargement TOML + types
- `compute.rs` : fonctions pures de score
- `flags.rs` : consensus et normalisation flags
- `pipeline.rs` : orchestration DB + notifications + upsert

### Benefices

- meilleure testabilite
- moins de couplage entre formule et moderation
- plus facile pour une `formula_v2`

## Priorite 3 — Simplifier les pages frontend trop chargees

### Objectif

Sortir la logique d'ecran des gros composants de route.

### Action

Pour `repo-detail.tsx`, extraire :

- `features/repos/components/RepoHeader.tsx`
- `RepoMetricsPanel.tsx`
- `RepoSignalsList.tsx`
- `ReportSignalForm.tsx`
- `OwnerDisputePanel.tsx`

Pour `account.tsx`, extraire :

- `AccountIdentityCard.tsx`
- `ReputationCard.tsx`
- `AgentTokensPanel.tsx`
- `AdminModerationPanel.tsx`

### Benefices

- meilleure lisibilite
- moins de regressions UI
- futurs tests React plus faciles

## Priorite 4 — Clarifier la front API layer

### Objectif

Eviter que l'API client generaliste devienne un fourre-tout.

### Action

Garder `api-client.ts` minimal, puis ajouter des clients metier fins :

- `frontend/src/lib/api/account.ts`
- `frontend/src/lib/api/repos.ts`
- `frontend/src/lib/api/watchlist.ts`
- `frontend/src/lib/api/admin.ts`

`admin.ts` peut porter explicitement la logique `x-admin-token`, au lieu de dissoudre ca dans l'abstraction commune.

### Benefices

- appels plus explicites
- moins d'options generiques
- meilleure separation utilisateur/admin

## Priorite 5 — Nettoyage doc et source de verite

### Objectif

Finir le realignement documentaire post-pivot.

### Action

- corriger `docs/README.md` pour ne plus lister de docs snippets archivees comme fondations vivantes
- ajouter une doc courte "architecture backend actuelle"
- ajouter une doc courte "trust model v1"

### Benefices

- onboarding plus rapide
- moins d'ambiguite entre archive et produit vivant

## Chantiers a ne pas faire maintenant

Ces refactos seraient prematurees aujourd'hui :

- migration vers une architecture DDD lourde
- abstraction generique de tous les handlers CRUD
- refonte totale du routing frontend
- changement de stack data-fetching
- reecriture complete du scoring

Le projet a encore plus besoin de clarifier son decoupage actuel que d'introduire une nouvelle sophistication.

## Plan d'execution recommande

### Sprint 1

Statut : termine.

- [x] scinder `handlers/repos.rs`
- [x] creer `services/trust/`
- [x] deplacer les helpers trust dedans

### Sprint 2

- extraire le scoring pur vs pipeline
- ajouter tests unitaires sur consensus / moderation / flags

### Sprint 3

- decouper `repo-detail.tsx`
- decouper `account.tsx`
- introduire des clients API metier

### Sprint 4

- cleanup doc
- petit audit de coherence final

## Definition of Done

Le refacto sera considere comme utilement termine quand :

- aucun handler backend majeur ne depasse un niveau de responsabilite "single use-case family"
- `scoring.rs` n'est plus un fichier-orchestrateur fourre-tout
- `repo-detail.tsx` et `account.tsx` deviennent des compositions de sous-composants
- la logique admin et la logique user sont distinctes cote frontend
- `docs/README.md` n'entretient plus de confusion avec le legacy snippets

## Recommandation finale

Le bon timing pour lancer cette refacto est **maintenant**, avant :

- la recherche semantique
- l'E2E
- la reputation v2
- une ouverture plus large du produit

Le projet est dans une bonne fenetre : assez stable pour refactorer sans panique, mais pas encore trop gros pour que ce soit douloureux.
