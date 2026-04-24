# User Journey Audit — Phase 2 (connecté) — 2026-04-24

## Scope

Second audit du parcours utilisateur, cette fois **connecté**, sur l'app déployée (Coolify). Complète l'audit phase 1 (`user-journey-audit-2026-04-23.md`) qui couvrait le parcours public / anonyme.

Objectif rappel : valider que le produit est **solide, présentable et fonctionnel** avant d'inviter quiconque. Pas de recherche d'utilisateurs tant que cet audit n'a pas été fait et corrigé.

Parcours à couvrir :

- flow post-login réel : retour sur page d'origine ou destination par défaut
- flow "watch your first repo" : landing → discover → repo detail → watch → watchlist
- flow notification → action : centre de notifs → repo détail → prise de décision
- états vides connectés : watchlist vide, notifications vides, compte sans token
- erreurs réelles côté UI : échec `POST /api/repos/add`, session expirée, refus auth
- parcours onboarding complet : login OAuth → discover → repo detail → watchlist → notifications → account/tokens
- passage mobile / responsive dédié

Méthode :

- session OAuth réelle (GitHub puis Discord)
- navigation manuelle sur l'app déployée
- DevTools ouvert (Network + Console) pour capturer erreurs silencieuses
- tester à la fois en desktop et mobile (viewport iPhone)
- captures d'écran des points de friction

## Zones de pré-friction identifiées par lecture du code

Ces points sont visibles avant même l'audit manuel — à valider en premier dans la session réelle.

### P-1. OAuth callback redirige vers la landing, pas la page d'origine

**Constat code** : `backend/src/handlers/auth.rs:36` et `:58` — `github_callback` et `discord_callback` font `Redirect::to(&state.config.frontend_base_url)` en dur. Aucun paramètre `state`/`return_to` pour mémoriser la page d'origine.

**Scénario friction** :

- user anonyme sur `/repos/react/timezone-picker` clique "Sign in to watch" = actuellement on peut voir tout les repos sans être logger. se qui n'est pas un mal en sois. 
- redirigé vers `/login`, clique GitHub = c'est ok.
- après auth → atterrit sur la landing `/`, pas sur le repo qu'il voulait watcher = actuellement retourne sur le landing peut importe où l'utilisateur se connecte. 
- friction forte : il faut qu'il re-navigue, re-search, re-clique = c'est le cas actuellement. 

**À valider dans l'audit** : quel est le ressenti réel après auth ? Est-ce que la landing connectée est assez claire pour ne pas perdre l'utilisateur ? = on retombe toujours sur la page d'accueil après connexion.

**Statut correctif 2026-04-24** : corrigé côté backend/frontend. Les liens vers `/login` portent désormais un `returnTo`, et les callbacks OAuth GitHub/Discord lisent un `return_to` signé dans le `state` OAuth. Le retour est sanitizé pour éviter les open redirects. Tests ajoutés côté backend + E2E mis à jour.

### P-2. Watchlist — pas de confirmation sur remove destructif

**Constat code** : `frontend/src/routes/watchlist.tsx:173` — `onClick={() => remove.mutate(w.artifactId)}` direct sans confirm dialog. Un clic = suppression immédiate.

**Scénario friction** : clic accidentel sur "Remove" supprime le repo de la watchlist sans filet. Pas d'undo visible.

**À valider** : risque réel d'erreur utilisateur ? Si oui, ajouter confirm modal ou undo toast. = oui. 

**Statut correctif 2026-04-24** : corrigé côté watchlist. Le retrait demande maintenant une seconde action explicite (`confirmer le retrait`) et propose `annuler`. Les boutons sont désactivés pendant mutation.

### P-3. Watchlist / notifications — erreurs query non gérées

**Constat code** : `watchlist.tsx:67` et `notifications.tsx:116` — seul `query.isLoading` est géré. `query.isError` tombe dans le `items.length === 0` → **faux empty state** qui prétend "tout va bien" quand le backend est down / session expirée / CORS cassé.

**Scénario friction** : session expirée pendant que l'user regarde sa watchlist → refresh → "no repo watched yet", alors qu'il en a 10. Confusion totale, bug silencieux.

**À valider** : forcer une erreur 401 (supprimer cookie en devtools) et observer ce que l'UI affiche. = je n'ai pu faire le test pour sa.

**Statut correctif 2026-04-24** : corrigé côté UI. Watchlist et notifications affichent maintenant un état d'erreur dédié avec bouton retry au lieu de tomber dans l'empty state.

### P-4. Notifications — cliquer le lien repo ne mark pas read

**Constat code** : `notifications.tsx:160-168` — le `<Link to="/repos/$id">` n'appelle pas `markRead.mutate`. Seul le bouton explicite "mark read" fait la mutation.

**Scénario friction** : user clique la notif, va sur le repo, revient → la notif est encore "unread". L'user croit avoir raté quelque chose.

**À valider** : est-ce que le pattern "unread jusqu'à action explicite" est volontaire ou un oubli ? Sur Gmail-like UX, cliquer = read implicite. = il faudrais crée des test pour sa afin de simuler cette action. 

**Statut correctif 2026-04-24** : corrigé côté notifications. Cliquer le lien repo d'une notification unread déclenche maintenant `markRead` avant navigation. Le flow reste couvert par l'E2E MVP mocké.

### P-5. Watchlist / notifications — mutations sans feedback visuel immédiat

**Constat code** : `watchlist.tsx:164` `toggleMute` et `:173` `remove`, `notifications.tsx:189` `markRead` — aucun `disabled` pendant `isPending`, pas de spinner, pas de toast succès.

**Scénario friction** : user clique "mute", rien ne se passe visuellement pendant 300-800 ms (network déployé). Il reclique. Double-action.

**À valider** : latence réseau réelle sur Coolify ? Si > 150 ms, ajouter `disabled={mutation.isPending}` minimum. = lorssque je clique le satut muted arrive vite au visuel. maintenant vue que je ne peut pas simuler une vraie notification, je ne peut confirmer.

**Statut correctif 2026-04-24** : partiellement corrigé. Watchlist (`mute/unmute`, `remove`) et notification `mark read` désactivent les contrôles pendant mutation. Pas encore de toast global.

### P-6. Account — champ admin token saisi à chaque visite

**Constat code** : `account.tsx:28` `useState("")` pour `adminToken`. Pas de persistance (ni localStorage, ni cookie). Chaque rechargement de la page force la resaisie.

**Scénario friction** : user admin (= toi) recharge `/account` → panel admin vide → doit recoller le token.

**À valider** : friction acceptable pour un usage admin rare ? Ou stocker en session (pas en localStorage, car token sensible) ? = si on stock le token et qu'un hacker trouve une faille de sécurité cela peut être problématique. 

**Statut correctif 2026-04-24** : décision provisoire : ne pas persister le token admin. La friction est acceptable pour un usage admin rare et évite de stocker un secret sensible côté navigateur.

### P-7. Repo detail — Sign in to watch redirige sans mémoriser le repo

Conséquence directe de P-1. Le CTA `signInToWatch` envoie vers `/login` mais le flow post-auth ne ramène pas sur ce repo.

**À valider en même temps que P-1**. = je ne trouve pas le `signInToWatch`. si c'est : " ouvrir l'observatoire " ou " Essayer l'exploration " ou " Voir toutes les entrées " ou View profile " ou " parcourir les dépôts ", l'utilisateur n'a pas besoin de se connecter. sinon tout le reste pour la veille l'utilisateur a besoin d'une connection. par contre, si c'est : " se connecter pour suivre ce dépôt" alor oui le flow enmene a la page d'Auth. 

**Statut correctif 2026-04-24** : corrigé avec P-1. Le CTA "Se connecter pour suivre ce dépôt" renvoie vers `/login?returnTo=/repos/<id>`, puis OAuth revient sur le profil repo.

## Scénarios à dérouler dans l'audit manuel

Cocher au fur et à mesure du passage en session réelle. Noter tout écart dans "Findings" ci-dessous.

### S1. Login → destination

- [X] anonyme sur `/` → clic "Sign in" header → OAuth GitHub → ? = yes
- [X] anonyme sur `/discover` → clic "Sign in" header → OAuth GitHub → ? = yes
- [X] anonyme sur `/repos/<id>` → clic "Sign in to watch" → OAuth GitHub → ? = yes
- [X] anonyme sur `/watchlist` → redirect `/login` → OAuth → ? = ici si c'est pour la page "explorer" alors non pas de redirection Oauth. par contre, 
- [X] anonyme sur `/notifications` → redirect `/login` → OAuth → ? = on ne vois pas notification tant qu'on est pas connecter
- [X] anonyme sur `/account` → redirect `/login` → OAuth → ? = on ne vois pas account tant qu'on est pas connecter
- [X] pareil avec Discord = yes
- [X] déconnexion (header sign out) → quel écran final ? = page d'accueil

### S2. Premier repo watché

- [X] user fresh, watchlist vide → `/watchlist` affiche l'empty state = oui
- [X] clic "Browse the observatory" → arrive sur `/discover`= oui
- [X] search "zod" → résultats triables = oui
- [X] clic repo → repo detail = oui
- [X] clic "Add to watchlist" → feedback ? = oui, bouton passe en état watched/unwatch. Pas de toast global.
- [X] retour `/watchlist` → repo présent ? = oui
- [X] mute / unmute → feedback ? = oui, état visuel rapide. Pas de toast.
- [X] remove → confirm ? annulable ? = oui, confirmation inline + annulation validées sur Coolify.

### S3. Notifications → action

- [x] simuler une notif côté backend (insert manuel DB ou déclencher scheduler) = couvert en E2E mocké, reste à valider sur DB Coolify réelle.
- [ ] `/notifications` affiche la notif
- [x] clic le lien repo → repo detail ouvre = couvert en E2E dédié.
- [x] retour `/notifications` → notif encore unread ? = corrigé et couvert : clic repo marque maintenant read.
- [x] clic "mark read" explicite → passe en read ? = couvert en E2E.
- [x] filtre "unread only" → masque la notif lue ? = couvert en E2E dédié.
- [ ] "mark all read" → feedback ?

### S4. États vides connectés

- [X] watchlist vide → empty state clair, CTA actif = oui
- [X] notifications vides → empty state clair, lien vers watchlist fonctionnel = corrigé : ajout d'un CTA explicite "Ouvrir la veille".
- [X] account sans token MCP → section tokens montre un empty state ? = oui et je ne peut pas crée de token.
- [X] account sans admin token saisi → panels admin affichent quoi ? = je ne vois que : " colle x-admin-token " dans l'input.

### S5. Erreurs réelles

- [x] supprimer cookie session via DevTools → refresh `/watchlist` → comportement ? = comportement attendu corrigé côté UI : erreur dédiée + retry au lieu d'un faux empty state. À refaire manuellement en phase 3.
- [ ] `POST /api/repos/add` avec URL invalide → message d'erreur côté UI ?
- [ ] `POST /api/repos/add` repo inexistant sur GitHub → message d'erreur ?
- [ ] `POST /api/repos/add` déjà indexé → message (informatif, pas d'erreur agressive) ?
- [ ] créer agent token avec label vide → bloque ? message ?
- [ ] révoquer un token → confirm ? feedback ?

### S6. Onboarding complet

- [ ] navigation privée (nouveau user) : landing → sign in GitHub → discover → add repo → repo detail → watch → watchlist → /account → créer MCP token
- [ ] temps total, points où on hésite, où on perd le fil

### S7. Mobile / responsive

- [ ] viewport iPhone 14 (390×844)
- [ ] landing : hiérarchie lisible ? CTAs accessibles ?
- [ ] discover : search fonctionnelle pouce-friendly ?
- [ ] repo detail : score readable, boutons pas collés ?
- [ ] watchlist : liste scrollable, bouton remove pas piégé ?
- [ ] notifications : cards lisibles ?
- [ ] account : sections pas cassées ?
- [ ] header : nav burger ou horizontal ?
- [ ] footer : pas superposé au contenu ?

## Findings

Format par finding : **Impact** / **Constat** / **Référence code** / **Recommandation**. Numéroter à partir de 1.

### 1. Retour post-login perdait l'intention utilisateur

**Impact** : élevé. Un utilisateur qui se connectait depuis un repo ou une route protégée revenait sur la landing et devait reconstruire son chemin.

**Constat** : les callbacks OAuth redirigeaient toujours vers `FRONTEND_BASE_URL`. Les liens `/login` ne portaient pas de destination d'origine.

**Référence code** : `backend/src/handlers/auth.rs`, `backend/src/auth/mod.rs`, `frontend/src/app/router.tsx`, `frontend/src/routes/login.tsx`, `frontend/src/features/repos/components/RepoHeader.tsx`.

**Recommandation / statut** : corrigé. `returnTo` est transmis au login, converti en `return_to` signé dans le state OAuth, puis sanitizé au callback.

### 2. Watchlist pouvait mentir en cas d'erreur

**Impact** : élevé. Une session expirée, une panne backend ou un problème CORS pouvait afficher "rien en veille" au lieu d'une erreur.

**Constat** : `query.isError` n'était pas traité, donc `query.data ?? []` tombait dans l'empty state.

**Référence code** : `frontend/src/routes/watchlist.tsx`.

**Recommandation / statut** : corrigé. État d'erreur dédié + bouton retry.

### 3. Notifications pouvaient mentir en cas d'erreur

**Impact** : élevé. Même risque que la watchlist : une erreur pouvait devenir un faux "tout est calme".

**Constat** : `query.isError` n'était pas traité sur `/notifications`.

**Référence code** : `frontend/src/routes/notifications.tsx`.

**Recommandation / statut** : corrigé. État d'erreur dédié + bouton retry.

### 4. Retrait watchlist destructif en un clic

**Impact** : moyen à élevé. Un clic accidentel supprimait un repo suivi sans confirmation.

**Constat** : bouton `remove` appelait directement la mutation DELETE.

**Référence code** : `frontend/src/routes/watchlist.tsx`.

**Recommandation / statut** : corrigé par confirmation inline en deux étapes (`retirer` → `confirmer le retrait`) avec annulation.

### 5. Notification ouverte mais encore unread

**Impact** : moyen. L'utilisateur pouvait ouvrir le repo depuis une notif, revenir, et voir la notification encore non lue.

**Constat** : seul le bouton explicite "mark read" déclenchait la mutation.

**Référence code** : `frontend/src/routes/notifications.tsx`.

**Recommandation / statut** : corrigé. Le clic sur le lien repo marque la notification comme lue.

### 6. Feedback mutation incomplet

**Impact** : moyen. Sur réseau réel, certaines actions pouvaient sembler ne rien faire pendant la mutation.

**Constat** : `mute/unmute`, `remove`, `mark read` ne désactivaient pas toujours les contrôles pendant `isPending`.

**Référence code** : `frontend/src/routes/watchlist.tsx`, `frontend/src/routes/notifications.tsx`.

**Recommandation / statut** : partiellement corrigé. Contrôles désactivés pendant mutation ; pas encore de toast succès/undo global.

### 7. Empty state notifications manquait de CTA clair

**Impact** : faible à moyen. Le lien "veille" était présent mais trop discret pour orienter un utilisateur fresh.

**Constat** : le lien vers la watchlist était seulement inline dans le texte.

**Référence code** : `frontend/src/routes/notifications.tsx`, `frontend/src/i18n/en.ts`, `frontend/src/i18n/fr.ts`.

**Recommandation / statut** : corrigé. Ajout d'un bouton explicite vers la watchlist.

### 8. Token admin non persisté

**Impact** : faible. Friction admin seulement, pas utilisateur final.

**Constat** : `adminToken` reste en state React et disparaît au reload.

**Référence code** : `frontend/src/routes/account.tsx`.

**Recommandation / statut** : pas de correctif volontaire. Ne pas persister ce secret côté navigateur pour l'instant.

## What worked well

- Le parcours réel connecté discovery → repo detail → OAuth retour → add watchlist → watchlist fonctionne sur Coolify.
- Les états vides connectés existent et sont globalement compréhensibles.
- Le login OAuth GitHub et Discord fonctionne.
- La lecture publique du registre sans compte reste possible, ce qui est cohérent avec le produit.
- Le compte affiche les sections utiles : identité, réputation, tokens MCP, admin léger.
- Les mutations watchlist sont rapides sur Coolify dans le test manuel.
- Les E2E MVP couvrent déjà une bonne partie du parcours discovery, repo detail, watchlist, notifications.
- Le clic notification → repo detail → mark read implicite est maintenant couvert par un E2E dédié.

## Priority plan

### P1 — bloquant avant ouverture publique

- [x] Retour post-login vers la destination d'origine.
- [x] États d'erreur explicites sur watchlist et notifications.
- [x] Suppression watchlist protégée par confirmation.
- [x] Vérification sécurité anti open-redirect sur `return_to`.

### P2 — à faire avant audit phase 3

- [x] Clic notification → marque la notification comme lue.
- [x] Feedback minimum des mutations watchlist / notifications (`disabled` + libellé pending).
- [x] CTA explicite depuis notifications vides vers watchlist.
- [ ] Tester une notification réelle sur DB Coolify, pas seulement en E2E mocké.
- [ ] Tester `POST /api/repos/add` erreurs réelles : URL invalide, repo inexistant, déjà indexé.
- [ ] Tester création/révocation de token MCP avec un compte réel.

### P3 — nice-to-have

- Ajouter toast global succès/erreur pour mutations importantes.
- Ajouter undo toast pour retrait watchlist si on veut une UX plus douce que confirmation inline.
- Ajouter une micro-explication dans `/account` sur pourquoi le token admin n'est pas mémorisé.
- Persister le token admin seulement en `sessionStorage` si le besoin admin devient récurrent, avec bouton clear visible.

## Followups tech

- `backend/src/auth/mod.rs` : `OAuthStateClaims` contient maintenant `return_to`; garder les tests anti open-redirect si le flow évolue.
- `frontend/src/lib/return-to.ts` : helper central pour garder les liens login cohérents.
- `frontend/src/routes/watchlist.tsx` : confirmation inline suffisante pour MVP ; si l'app gagne un système de toast/modal partagé, migrer vers un pattern commun.
- `frontend/src/routes/notifications.tsx` : vérifier en production réelle la course éventuelle entre `markRead.mutate` et la navigation si latence forte ; l'E2E dédié couvre le contrat MVP.
- `frontend/e2e/mvp.spec.ts` : test dédié ajouté pour "notification link marks read".

## Suggested next audit

Phase 3 recommandée avant ouverture externe large :

- session réelle neuve en navigation privée ;
- parcours complet OAuth → discover → repo detail → watchlist → notifications → account/tokens ;
- simulation d'une notification réelle en DB Coolify ou via endpoint/admin/scheduler ;
- erreurs réelles `repos/add` ;
- mobile iPhone 14 / 390×844 ;
- console + network ouverts pour capturer les erreurs silencieuses.

Si Phase 3 passe sans finding P1, l'app est présentable pour une petite ouverture contrôlée.
