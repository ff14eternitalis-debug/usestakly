# Plan de Renommage — `Project-K` vers `UseStakly`

> Version : 1.0 — 2026-04-18
> Statut : plan de transition validable avant renommage effectif
> But : faire passer le projet du nom technique `Project-K` au nom produit `UseStakly` sans casser le repo, la doc, l'infra ou les futurs déploiements

## 1. Décision de naming

### Décision retenue

- **Nom produit / marque affichée** : `UseStakly`
- **Nom technique actuel** : `Project-K`

### Lecture recommandée

À partir de maintenant, il faut penser le projet comme suit :

- `UseStakly` = nom public, produit, branding, UI, domaine, GitHub org
- `Project-K` = ancien nom de travail interne, toléré temporairement dans le repo et certaines docs

### Objectif de transition

Le but n'est pas de tout renommer brutalement en une seule fois.
Le but est de faire une transition en **3 couches** :

1. **branding public**
2. **documentation**
3. **noms techniques / dépôt / variables / déploiement**

---

## 2. Pourquoi faire une transition et non un rename brutal

Un renommage complet immédiat présente plusieurs risques :

- casser des chemins locaux ou scripts
- introduire des incohérences entre repo, domaine et UI
- forcer des changements inutiles alors que le MVP n'est pas encore totalement câblé
- brouiller l'historique alors que `Project-K` est encore le nom utilisé dans plusieurs documents

La bonne approche est donc :

- **UseStakly devient la marque maintenant**
- **Project-K reste un alias technique transitoire**
- le renommage profond se fait ensuite par étapes maîtrisées

---

## 3. Cible finale

Quand la transition sera terminée, l'état cible devra être :

- repo GitHub : `usestakly`
- éventuelle org GitHub : `usestakly`
- nom affiché dans l'app : `UseStakly`
- domaine principal : idéalement `usestakly.com`
- docs principales : renommées vers `UseStakly`
- variables, containers et services : alignés sur `usestakly`
- `Project-K` conservé uniquement dans l'historique Git et les anciennes notes si nécessaire

---

## 4. Ce qu'il faut renommer tout de suite

Ces éléments peuvent être renommés dès maintenant sans risque majeur.

### Branding produit

- le titre du `README`
- le titre de `docs/README.md`
- les en-têtes des docs maîtresses
- le nom affiché futur dans le frontend

### Références produit

- “Projet K” → “UseStakly”
- “Project-K” → “UseStakly” quand le texte parle du produit
- conserver “Project-K” uniquement quand le texte parle du **repo actuel**, du **dossier local**, ou d'un **ancien nom**

### Messaging recommandé

Formulation cible :

> `UseStakly` est une infrastructure de bibliothèques de code privées ou publiques que les IA peuvent résoudre, chercher et assembler via MCP.

---

## 5. Ce qu'il faut garder temporairement

Ces éléments peuvent rester tels quels pendant encore un moment.

### Repo local

- dossier local : `Project-K/`
- certains chemins absolus déjà cités dans la documentation

### Noms techniques temporaires

- nom du crate Rust si on veut éviter un rename prématuré
- nom des containers si le déploiement n'est pas encore stabilisé
- certains labels internes dans les docs de build

### Règle

Tant que le produit n'est pas encore déployé publiquement avec son nom final :

- la **marque** peut être `UseStakly`
- les **identifiants techniques internes** peuvent encore rester `project-k`

---

## 6. Ordre de transition recommandé

## Phase 1 — Branding doc

Objectif :
- afficher `UseStakly` comme nom du produit dans toute la doc stratégique

À faire :
- mettre à jour `README.md`
- mettre à jour `docs/README.md`
- mettre à jour les docs MVP principales
- ajouter une note explicite :
  - `UseStakly` est le nom produit
  - `Project-K` est l'ancien nom de travail

Résultat attendu :
- toute personne qui ouvre le repo comprend immédiatement que le produit s'appelle `UseStakly`

## Phase 2 — Branding app

Objectif :
- rendre l'interface et l'environnement de dev cohérents avec la marque

À faire :
- renommer les titres HTML
- renommer les placeholders UI
- renommer le label d'application côté frontend
- préparer favicon, logo texte, éventuellement slug d'app

Résultat attendu :
- le produit “semble réel” dès l'ouverture de l'app

## Phase 3 — Dépôt et infra

Objectif :
- aligner les surfaces techniques publiques

À faire :
- créer org GitHub `usestakly` si souhaité
- créer repo `usestakly`
- pousser le code dedans
- réserver domaine principal
- configurer Coolify avec ce nom pour le frontend, le backend et la base PostgreSQL

Résultat attendu :
- repo, domaine et interface racontent la même histoire

## Phase 4 — Nettoyage profond

Objectif :
- retirer les dernières traces inutiles de `Project-K`

À faire :
- renommer crate, images Docker, services, labels, scripts
- mettre à jour les docs plus anciennes ou secondaires
- archiver les références historiques quand elles ne servent plus

Résultat attendu :
- `Project-K` ne reste qu'en note historique

---

## 7. Politique de renommage dans la documentation

Pour éviter les incohérences, il faut appliquer cette règle simple :

### Utiliser `UseStakly` quand

- on parle du produit
- on parle de la proposition de valeur
- on parle du futur site ou de la production
- on parle de l'expérience utilisateur
- on parle du branding

### Utiliser `Project-K` quand

- on parle du nom de code initial
- on parle du dossier local actuel
- on parle d'un ancien état de conception
- on cite un chemin ou un contexte historique

### Formule de transition recommandée

> `UseStakly` est le nom produit retenu pour le MVP. `Project-K` reste le nom de travail historique utilisé dans certaines structures techniques et documents plus anciens.

---

## 8. Politique de renommage dans le code

Le code n'a pas besoin d'être renommé partout immédiatement.

Priorité :

1. surfaces visibles par l'utilisateur
2. variables de déploiement publiques
3. repo public
4. labels techniques internes

À ne pas renommer trop tôt :

- noms de modules stables si cela n'apporte rien
- chemins locaux qui casseraient ton environnement
- identifiants internes tant que le flow de prod n'est pas fixé

---

## 9. Impact sur le déploiement

Si tu mets l'app en prod rapidement, le plus important est :

- que l'UI affiche `UseStakly`
- que le domaine public utilise `usestakly.*`
- que le repo public visible soit cohérent si possible

Ce qui est moins grave au début :

- avoir encore `project-k` dans certains noms internes de services
- avoir encore un dossier local `Project-K`

Donc pour la prod early-stage :

- **nom public cohérent > pureté du renommage interne**

---

## 10. Décisions pratiques recommandées maintenant

### Recommandation immédiate

- garder le dossier local tel quel pour le moment
- garder le repo technique local tel quel tant qu'on n'a pas créé le repo GitHub final
- commencer à utiliser `UseStakly` dans les docs, le README et l'interface

### Recommandation GitHub

- créer l'org ou le compte cible `usestakly`
- créer le repo public final sous ce nom
- pousser le projet dedans quand la base MVP est assez propre

### Recommandation domaine

- réserver au minimum :
  - `usestakly.com`
  - ou fallback `usestakly.dev`

### Recommandation Docker / packages

Comme le namespace Docker Hub `usestakly` semble déjà pris, prévoir un namespace du type :

- `usestaklyhq/backend`
- `usestaklyapp/backend`
- ou utiliser un registre GitHub Container Registry

---

## 11. Checklist de transition

### Checklist immédiate

- [ ] Valider définitivement `UseStakly` comme nom produit
- [ ] Réserver le domaine principal
- [ ] Vérifier INPI manuellement
- [ ] Créer la présence GitHub cible
- [ ] Renommer `README.md`
- [ ] Renommer `docs/README.md`
- [ ] Ajouter une note “anciennement Project-K”

### Checklist prochain sprint

- [ ] Renommer les docs MVP prioritaires
- [ ] Renommer le nom affiché frontend
- [ ] Préparer le favicon / logo texte
- [ ] Aligner les variables publiques de déploiement

### Checklist plus tard

- [ ] Renommer le crate/backend si nécessaire
- [ ] Renommer les services Docker
- [ ] Renommer les labels d'observabilité
- [ ] Nettoyer les références historiques restantes

---

## 12. Recommandation finale

La meilleure stratégie n'est pas :

- tout renommer maintenant

La meilleure stratégie est :

- **adopter `UseStakly` immédiatement comme nom produit**
- **garder `Project-K` comme alias technique transitoire**
- **effectuer le renommage profond en plusieurs étapes sans casser l'exécution**

En une phrase :

> Le produit s'appelle maintenant `UseStakly`. `Project-K` devient un ancien nom de travail toléré temporairement dans l'interne.
