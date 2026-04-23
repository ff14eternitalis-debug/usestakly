# User Journey Audit — 2026-04-23

## Scope

Audit du parcours utilisateur sur l'application locale `http://localhost:5173`, avec backend local `http://localhost:4000`.

Parcours couverts :

- landing
- discover
- search
- add repo
- repo detail
- login
- accès non authentifié à `watchlist`, `notifications`, `account`

Méthode :

- navigation réelle dans un navigateur headless via Playwright
- vérification des réponses API locales
- lecture ciblée des routes frontend impliquées

Limite :

- les flows connectés complets n'ont pas été validés jusqu'au bout dans le navigateur, car le parcours OAuth réel n'a pas été exécuté dans cette session. Les écrans et gardes non-auth ont en revanche été observés.

## Verdict

Le parcours public principal est maintenant crédible :

- la landing explique bien le produit
- `discover` charge vite et la search est désormais cohérente
- `add repo` redirige bien vers le détail repo
- `repo detail` expose correctement le score et le contexte qualité
- `login` est clair et propre

La friction principale n'est plus la search. Elle est maintenant dans la gestion des zones protégées et dans quelques micro-décisions UX qui rendent le produit plus technique qu'il ne devrait l'être.

## Findings

### 1. Protected routes do not fail gracefully for anonymous users

Impact :

- un visiteur non connecté peut ouvrir `/watchlist`, `/notifications` et `/account`
- les pages affichent leur vrai shell puis restent dans un état de chargement trompeur
- le produit donne l'impression d'être lent ou cassé, alors que le vrai problème est l'auth

Constat :

- `/watchlist` affiche `PULLING THE FILE…`
- `/notifications` affiche `SORTING THE MAIL…`
- `/account` affiche `TUNING THE INSTRUMENTS…`

Cause probable :

- les routes sont protégées par `beforeLoad`, mais `requireAuth()` ne redirige que si le store est déjà à `anonymous`
- au premier chargement, le store démarre à `loading`, donc la route passe
- ensuite `useHydrateAuth()` bascule à `anonymous`, mais trop tard pour empêcher le rendu initial

Références :

- [frontend/src/app/router.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/app/router.tsx)
- [frontend/src/features/auth/hooks.ts](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/features/auth/hooks.ts)
- [frontend/src/routes/watchlist.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/routes/watchlist.tsx)
- [frontend/src/routes/notifications.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/routes/notifications.tsx)
- [frontend/src/routes/account.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/routes/account.tsx)

Recommandation :

- ajouter une garde auth qui attend explicitement la fin d'hydratation avant de décider
- ou rediriger côté route si `status !== "authenticated"` après hydratation
- et prévoir un vrai state non-auth avec CTA clair vers `/login` au lieu d'un faux loader

### 2. The discover add-repo CTA is too technical

Impact :

- le flow marche, mais le wording n'aide pas assez un utilisateur normal
- `Ingest repo` sonne backend / pipeline, pas produit

Constat :

- le bloc est bien nommé `ADD GITHUB REPO`
- mais le bouton principal porte le libellé `Ingest repo`

Référence :

- [frontend/src/routes/discover.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/routes/discover.tsx)

Recommandation :

- renommer vers `Add repo`, `Index repo`, ou `Add to observatory`
- garder `ingest` pour le backend ou l'admin, pas pour le CTA utilisateur principal

### 3. Repo detail has a weak anonymous next step

Impact :

- sur la page repo, un utilisateur non connecté comprend le score
- mais l'action suivante côté produit est peu mise en valeur

Constat :

- le repo detail affiche `Sign in to watch`
- le CTA principal n'est pas aussi évident qu'il pourrait l'être face à la richesse du reste de l'écran

Références :

- [frontend/src/routes/repo-detail.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/routes/repo-detail.tsx)
- [frontend/src/features/repos/components/RepoHeader.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/features/repos/components/RepoHeader.tsx)

Recommandation :

- transformer ce point d'entrée en CTA plus explicite
- exemple : `Sign in to watch this repo`
- idéalement avec une phrase de bénéfice visible : `Get alerts when score drops or severe flags land`

### 4. Landing duplicates discover entry points

Impact :

- friction légère seulement
- mais le signal de navigation est un peu redondant

Constat :

- `Discover` existe dans le header
- `Open the observatory` existe dans le hero
- `Try discover` existe dans le bloc pillar
- `Discover` réapparaît aussi dans le footer

Références :

- [frontend/src/features/layout/AppHeader.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/features/layout/AppHeader.tsx)
- [frontend/src/routes/index.tsx](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/routes/index.tsx)

Recommandation :

- garder cette redondance structurelle, mais clarifier la hiérarchie
- un seul CTA hero dominant, les autres plus secondaires visuellement

## What worked well

- la landing communique clairement la proposition de valeur
- la search répond bien aux requêtes naturelles
- `discover` donne un bon sentiment de catalogue exploitable
- le redirect après `add repo` vers le détail repo est bon
- la page login est rassurante et compréhensible
- la page repo explique correctement les dimensions du score

## Priority plan

### P1

- corriger la garde auth et les états non-auth sur `watchlist`, `notifications`, `account`

### P2

- améliorer le CTA anonyme sur `repo detail`
- renommer `Ingest repo`

### P3

- resserrer la hiérarchie de CTA sur la landing
- faire un second audit avec vraie session connectée

## Suggested next audit

Une fois la garde auth corrigée, refaire un passage Playwright sur :

- login réel
- add to watchlist
- notifications
- account
- création / révocation token MCP

Ce second passage donnera le vrai audit “post-login”, qui manque encore pour valider le produit de bout en bout.
