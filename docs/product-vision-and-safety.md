# UseStakly — Vision Produit Universelle & Trust / Safety

> Version : 1.0 — 2026-04-18
> Statut : document de cadrage produit et sécurité

## 🎯 Ambition

> Note de transition : `UseStakly` est le nom produit retenu. `Project-K` reste l'ancien nom de travail encore visible dans certaines structures techniques et documents plus anciens.

`UseStakly` vise à devenir une **infrastructure universelle de bibliothèques de code** :

- multi-langages
- multi-domaines
- privées ou publiques
- interrogeables par MCP
- exploitables par des IA pour **assembler** des applications au lieu de les générer depuis zéro

L'ambition n'est pas de créer un simple gestionnaire de snippets.
L'ambition est de créer une couche de réutilisation adressable qui transforme les IA en **builders d'applications** plutôt qu'en générateurs probabilistes de code.

---

## 🧭 Vision produit

## Le problème à résoudre

Aujourd'hui, une IA :
- réécrit souvent ce qui existe déjà
- hallucine des APIs, structures et conventions
- consomme beaucoup de tokens pour reformuler du boilerplate
- ne sait pas naturellement réutiliser le savoir accumulé par les développeurs

En parallèle, les développeurs ont déjà énormément de matière utile :
- composants frontend
- handlers backend
- scripts DevOps
- requêtes SQL
- schémas de base de données
- fichiers de configuration
- helpers partagés

Mais cette matière est dispersée, mal indexée et rarement exploitable par un agent.

## La réponse de `UseStakly`

`UseStakly` transforme cette matière en bibliothèques structurées, adressables et résolubles.

Un utilisateur peut :
- créer sa propre bibliothèque
- la garder privée
- publier tout ou partie de sa bibliothèque
- référencer explicitement une bibliothèque publique
- référencer explicitement un snippet précis
- laisser une IA assembler une app entière à partir de bibliothèques compatibles

L'IA ne part plus d'une feuille blanche.
Elle suit une hiérarchie :

1. résoudre une référence exacte
2. chercher dans des bibliothèques ciblées
3. chercher dans des bibliothèques publiques compatibles
4. assembler
5. n'inventer qu'en dernier recours

---

## 🌍 Univers du produit

## Couverture fonctionnelle visée

Le produit doit être conçu pour accueillir **tous les langages** et **tous les domaines du code**.

Exemples de domaines :
- frontend
- backend
- data
- devops
- shared
- docs techniques
- infra
- automation

Exemples de langages et formats :
- TypeScript
- JavaScript
- TSX / JSX
- Rust
- Go
- Python
- Java
- Kotlin
- C#
- PHP
- Ruby
- SQL
- Prisma
- Bash / Shell
- PowerShell
- YAML
- TOML
- JSON
- Dockerfile
- Terraform
- Markdown

## Principe important

La **vision** est universelle.
Le **MVP** ne doit pas chercher à supporter parfaitement chaque langage.

Le bon principe d'exécution est :
- modèle de données compatible avec tout
- détection et expérience excellentes sur un premier sous-ensemble
- extension progressive par familles de langages

---

## 🧱 Primitive centrale : la bibliothèque

## Bibliothèque comme unité d'adressage

Le produit n'est pas seulement une collection de snippets.
Il est composé de **bibliothèques** qui regroupent des snippets partageant un auteur, une intention, une stack ou une ligne éditoriale.

Chaque bibliothèque doit être :
- identifiable
- résoluble
- versionnable dans son catalogue
- visible selon des permissions claires

### Attributs minimaux

- `library_id` : UUID stable
- `owner_id`
- `slug` public lisible
- `name`
- `description`
- `visibility` : `private | public | unlisted`
- `default_stack`
- `allowed_domains`
- `trust_level`

### Exemple de slug

```text
@alice/react-ui-kit
@bob/rust-api-primitives
@team-delta/postgres-patterns
```

## Snippet comme unité d'assemblage

Chaque snippet doit aussi être adressable explicitement.

### Attributs minimaux

- `snippet_id` : UUID stable
- `library_id`
- `slug`
- `name`
- `domain`
- `kind`
- `language`
- `framework`
- `visibility`
- `current_version_id`
- `content_hash`

### Contrainte

Le couple `(library_id, slug)` doit être unique.

## Référence canonique

Le produit doit supporter une syntaxe humaine simple :

```text
@alice/react-ui-kit:frontend-atom-action-button-primary
@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0
```

Et une forme machine interne :

```text
lib_3f6b7d5e:snippet_9a2c1f4b@1.2.0
```

La forme humaine est utilisée dans les prompts et dans l'UI.
La forme machine est utilisée par le backend et le MCP.

---

## 🤖 Modes d'usage MCP

Le MCP doit supporter plusieurs degrés de liberté.

## 1. Résolution explicite

L'utilisateur connaît :
- la bibliothèque
- le snippet
- éventuellement la version

Exemple :

> Récupère `frontend-atom-action-button-primary` dans `@alice/react-ui-kit`.

Dans ce mode :
- aucune recherche sémantique n'est nécessaire
- le MCP agit comme un résolveur exact
- l'hallucination est minimale

## 2. Recherche guidée

L'utilisateur connaît :
- une ou plusieurs bibliothèques
- une stack
- un type de besoin

Exemple :

> Cherche tous les composants auth React Tailwind dans `@alice/react-ui-kit`.

Dans ce mode :
- la recherche est limitée à un scope précis
- le système peut retourner un classement de snippets compatibles

## 3. Assemblage automatique

L'utilisateur décrit un besoin plus large :

> Construis-moi une app X.
> Pour le frontend : React + Tailwind.
> Pour le backend : Rust + Axum + PostgreSQL.
> Va chercher automatiquement les briques nécessaires.

Dans ce mode :
- le MCP cherche dans les bibliothèques autorisées
- sélectionne les snippets compatibles
- résout les dépendances
- produit un plan d'assemblage
- ne génère du code inédit qu'en dernier recours

## Politique de priorité

L'ordre recommandé est :

1. référence explicite exacte
2. slug exact dans la bibliothèque demandée
3. recherche approximative dans les bibliothèques demandées
4. recherche dans les bibliothèques publiques compatibles
5. création de nouveau code en fallback

---

## 🔓 Liberté utilisateur

Le produit doit être **strict dans sa résolution** mais **libre dans ses modes d'usage**.

Un utilisateur peut :
- n'utiliser que sa bibliothèque privée
- publier certains snippets
- publier toute une bibliothèque
- cibler une bibliothèque externe spécifique
- laisser l'IA explorer librement les bibliothèques publiques compatibles

Le produit ne doit pas imposer un seul mode.

## Scopes de recherche recommandés

- `private_only`
- `own_plus_public`
- `public_only`
- `selected_libraries_only`

## Modes d'assemblage recommandés

- `strict`
  - utilise uniquement les bibliothèques fournies
  - s'arrête si une brique manque
- `guided`
  - privilégie les bibliothèques fournies, puis complète si autorisé
- `auto`
  - explore les bibliothèques publiques compatibles
  - maximise la réutilisation

---

## 🧠 Modèle de compatibilité

Pour permettre un vrai assemblage multi-bibliothèques, chaque snippet doit être accompagné de métadonnées de compatibilité.

## Champs minimaux

- `domain`
- `language`
- `framework`
- `framework_version`
- `runtime`
- `dependencies`
- `tags`
- `exports`
- `imports`
- `compatibility`

## Exemples de compatibilité

```json
{
  "frontend": {
    "framework": "react",
    "framework_version": ">=19",
    "styling": "tailwind",
    "language": "tsx"
  }
}
```

```json
{
  "backend": {
    "language": "rust",
    "framework": "axum",
    "database_driver": "sqlx",
    "database": "postgres"
  }
}
```

Cette couche est essentielle pour éviter un simple moteur de snippets textuels sans cohérence technique.

---

## 🛡️ Trust & Safety

Le produit ne peut pas être une bibliothèque publique ouverte sans une couche forte de sécurité.

L'objectif n'est pas seulement d'empêcher le code malveillant.
L'objectif est aussi d'empêcher :
- l'injection de prompt
- l'empoisonnement de bibliothèque
- les snippets toxiques pour agents
- l'exfiltration de secrets
- l'assemblage aveugle de commandes dangereuses

## Principe fondamental

Un snippet doit toujours être traité comme **une donnée**.
Jamais comme une autorité de pilotage.

Le contenu d'une bibliothèque publique ne doit jamais pouvoir surclasser les règles système du MCP.

## Menaces principales

### 1. Code malveillant

Exemples :
- reverse shell
- scripts destructifs
- accès filesystem sensible
- exfiltration réseau
- téléchargement silencieux de payloads

### 2. Prompt injection

Exemples :
- commentaires du type "ignore previous instructions"
- README ou description qui tentent de détourner l'agent
- instructions cachées dans des champs texte

### 3. Poisoning sémantique

Exemples :
- faux tags
- faux noms de snippets
- snippets trompeurs publiés comme "safe"
- snippets incompatibles déclarés compatibles

### 4. Dépendances à risque

Exemples :
- imports vers packages malveillants
- scripts d'installation douteux
- commandes shell non sûres

## Défense en couches

### Couche 1 — Sanitize du contenu libre

Scanner et normaliser :
- descriptions
- README
- tags libres
- notes
- commentaires importés

Détecter au minimum :
- `ignore previous instructions`
- `reveal secrets`
- `send env`
- `execute this command`
- `system override`

### Couche 2 — Analyse statique

Avant publication ou indexation, analyser :
- appels shell dangereux
- opérations réseau suspectes
- accès fichiers sensibles
- patterns d'obfuscation
- code encodé de manière inhabituelle
- dépendances connues à risque

### Couche 3 — Policy engine MCP

Le MCP doit appliquer des règles d'exécution strictes :
- pas d'exécution automatique d'un snippet
- pas d'accès implicite aux secrets
- pas de commande système sans validation explicite
- pas de migration destructrice sans confirmation
- pas de téléchargement externe silencieux

### Couche 4 — Provenance obligatoire

Tout snippet utilisé par le MCP doit conserver :
- bibliothèque source
- auteur
- identifiant du snippet
- version
- hash de contenu

### Couche 5 — Réputation et modération

Pour les snippets publics, prévoir :
- signalement
- flag
- quarantaine
- retrait
- revue humaine ou semi-automatique

## Niveaux de confiance suggérés

- `private`
- `public_unverified`
- `verified_author`
- `community_trusted`
- `flagged`
- `quarantined`

Un snippet `flagged` ou `quarantined` ne doit pas être consommable en mode auto.

## Règle produit clé

Plus le mode est automatique, plus le filtre de confiance doit être élevé.

Exemple :
- mode `resolve` explicite : l'utilisateur peut prendre plus de responsabilité
- mode `auto` : le système doit exclure tout contenu douteux

---

## 🔐 Sécurité spécifique au multi-langage

Tous les langages n'ont pas le même niveau de risque.

## Familles de risque

### Risque faible à modéré

- composants UI
- helpers purs
- types
- constantes
- requêtes déclaratives

### Risque élevé

- shell scripts
- PowerShell
- Dockerfiles
- Terraform
- migrations
- handlers réseau
- snippets d'auth
- snippets manipulant secrets ou credentials

## Décision recommandée

Le produit doit classer les snippets aussi par **niveau de risque opérationnel**.

Exemple :
- `safe`
- `review_required`
- `restricted`

Cette classification est orthogonale au `domain`.

---

## 🌐 Web3 / Intuition

La stratégie Web3 doit rester complémentaire au cœur du produit.

## Ce qui reste offchain

- code source privé
- versions privées
- recherche privée
- embeddings
- métadonnées internes
- règles d'assemblage privées

## Ce qui peut devenir onchain plus tard

- preuves de provenance
- certification publique d'un snippet
- relation entre snippets et bibliothèques
- réputation d'un auteur
- attestations de compatibilité ou de qualité
- hash de contenu

## Position recommandée

`UseStakly` doit rester :
- **private-first**
- **network-enabled**
- **web3-compatible**

La couche Intuition est utile pour :
- snippets publics
- réputation
- certification
- découverte
- confiance

Elle ne doit pas remplacer le cœur MVP de la bibliothèque.

---

## 🚀 Conséquences produit pour le MVP

Le MVP doit déjà poser les bases de cette vision, même si toute l'universalité n'est pas activée.

## À faire dès le MVP

- bibliothèques adressables
- snippets adressables
- visibilité `private/public`
- métadonnées de compatibilité
- provenance obligatoire
- modèle de confiance minimal
- filtrage de sécurité de base
- politique MCP "résoudre avant chercher, chercher avant générer"

## À reporter après le MVP

- support excellent de dizaines de langages
- modération avancée
- score de réputation sophistiqué
- publication onchain
- attestations Intuition
- graphe social / économique

---

## 📌 Décisions de référence

1. `UseStakly` est une infrastructure de bibliothèques de code, pas un simple gestionnaire de snippets.
2. Le produit doit être pensé pour **tous les langages**, même si le MVP n'en optimise qu'un sous-ensemble.
3. La bibliothèque est l'unité d'adressage principale.
4. Le snippet est l'unité d'assemblage principale.
5. Le MCP doit d'abord **résoudre**, ensuite **chercher**, ensuite **assembler**, et seulement enfin **générer**.
6. La sécurité et l'anti-prompt-injection sont des fondations produit, pas des améliorations tardives.
7. La publication publique doit être compatible avec la provenance, la réputation et une future couche Web3.

---

## Formule produit courte

> `UseStakly` est une infrastructure universelle de bibliothèques de code, privées ou publiques, que les IA peuvent résoudre et assembler via MCP pour construire des applications avec plus de fiabilité, moins d'hallucinations et moins de génération brute.
