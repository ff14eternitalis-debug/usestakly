# Projet K — Vision & Proposition de Valeur

> Version : 1.0 — 2026-04-15 *(pré-pivot 2026-04-20)*

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Ce document **précède le pivot** vers l'angle *registry qualité-scored pour agents IA*. Source de vérité produit actuelle : [`strategy-quality-scored-registry.md`](./strategy-quality-scored-registry.md).
>
> **Ce qui change** :
> - Le pitch « GitHub personnel intelligent » est remplacé par « Rotten Tomatoes pour le code, lu par des agents ». Le produit ne vend plus *le stockage de tes snippets* mais *le jugement qualité* consommé par des agents IA.
> - La persona principale MVP n'est plus uniquement le dev solo qui curate sa bibliothèque — c'est le dev solo dont l'agent IA veut savoir *ce qu'il doit éviter* dans l'écosystème open source.
> - Le Nord Star « snippets créés par utilisateur > 20 » devient secondaire. Les vraies métriques post-pivot : `resolve_count`, `build_success_rate`, `regret_rate`, couverture du scoring sur les deps du `package.json` utilisateur.
>
> **Ce qui reste valide** :
> - Principes directeurs 1–5 (propriété utilisateur, zéro verrou, qualité > quantité, IA exécutante, progressif).
> - Personas secondaires (tech lead, créateur de contenu) — inchangés.
> - Hors périmètre (Web3, collab temps réel, marketplace, mobile, IDE natif).

## 🎯 Pitch en une phrase

> ~~**Projet K est un "GitHub personnel intelligent" : une bibliothèque vivante de tes snippets de code, pilotée par une IA qui ne peut coder qu'en assemblant tes propres briques.**~~
>
> **Pitch retenu post-pivot** : Un registry de code avec scoring qualité dérivé de l'usage réel, que les agents IA utilisent pour filtrer automatiquement les briques obsolètes, cassées ou dangereuses avant même de proposer du code.

## 🔥 Le problème

Un développeur passe **30 % de son temps** à :
- Chercher un bout de code déjà écrit dans un ancien projet
- Copier-coller depuis StackOverflow sans contrôle qualité
- Recréer le même composant UI ou la même fonction utilitaire
- Batailler avec des IA génératives qui inventent des classes, des noms et des dépendances

Les Design Systems et bibliothèques de composants existent, mais personne ne les maintient vraiment : trop coûteux, pas assez tangible.

## 💡 La solution

**Projet K** transforme tout snippet personnel en un bloc réutilisable, indexé, et surtout **contraignant pour l'IA** :

1. L'utilisateur stocke ses snippets (frontend, backend, devops…) dans la librairie
2. Un serveur MCP expose ces snippets à l'IA
3. Un **prompt système strict** oblige l'IA à réutiliser les briques existantes avant d'en inventer
4. Des **RULES** (JSON) définissent les contraintes d'assemblage (stack, nommage, structure)
5. L'IA livre un projet cohérent, signé avec la provenance de chaque ligne

## 📊 Proposition de valeur

| Avant Projet K | Avec Projet K |
|---|---|
| Copier-coller depuis StackOverflow ou vieux dossiers | L'IA assemble des briques que **tu possèdes déjà** |
| Design System ignoré par flemme | L'Atomic Design est la **seule voie possible** via les RULES |
| ChatGPT invente des noms de classes farfelus | IA **bridée par le MCP** : utilise tes noms et ta logique |
| Onboarding : des semaines pour comprendre les conventions | L'outil **impose** les conventions automatiquement |
| Maintenance manuelle de la librairie | L'IA **suggère** les mises à jour des snippets dépendants |

**Promesse finale** : passer de **20 % / 80 %** (architecture/boilerplate) à **80 % / 20 %** (architecture/assemblage).

## 👤 Personas

### Le dev solo (persona principal au MVP)
- Freelance ou indé, 3-10 ans d'expérience
- A déjà des Gists, des repos "utils", des fichiers `.md` de notes
- Veut capitaliser sur son travail passé
- Incitation : **gain de temps immédiat** + fierté de sa librairie

### Le tech lead (persona secondaire)
- 7+ ans, supervise 3-10 devs
- Souffre de la dérive des conventions dans son équipe
- Veut imposer un Design System sans friction
- Incitation : **cohérence automatique** + onboarding accéléré

### Le créateur de contenu / dev educator (persona communauté)
- YouTube, Twitter, blog
- Publie des packs "certifiés" d'atomes/molécules
- Incitation : **reconnaissance** + monétisation

## 🎁 Ce qui fait qu'on l'utilise (les "hooks")

1. **Zéro friction** — intégration IDE (VS Code, Cursor) : l'utilisateur ne change pas son workflow
2. **Curation de savoir** — "donne-moi tes snippets, je les rends actifs"
3. **Communauté** — publier ses packs, être reconnu, éventuellement monétiser

## 🚫 Ce que Projet K **n'est pas**

- ❌ Un générateur de code généraliste (type Copilot)
- ❌ Un gestionnaire de Gists amélioré
- ❌ Un IDE
- ❌ Un registre npm ou un package manager
- ❌ Une plateforme no-code

## 🧭 Principes directeurs

1. **La propriété du code reste à l'utilisateur** — ses snippets lui appartiennent, exportables à tout moment
2. **Zéro verrou technique** — standards ouverts (MCP, SQL, JSON, semver)
3. **Qualité > quantité** — 50 atomes bien faits valent mieux que 500 moyens
4. **L'IA est un exécutant, pas un créateur** — elle assemble, elle n'invente pas
5. **Progressif** — chaque phase doit fonctionner seule, sans dépendre de la suivante

## 📈 Indicateurs de succès (Nord Star)

| Étape | Métrique | Cible MVP |
|---|---|---|
| Adoption | Snippets créés par utilisateur | > 20 en 1 semaine |
| Activation | Générations IA par utilisateur | > 5 en 1 semaine |
| Rétention | Utilisateurs actifs à J+30 | > 40 % |
| Valeur perçue | Temps gagné auto-déclaré | > 2 h / semaine |

## 🗺️ Hors périmètre (explicite)

- Web3 / crypto → abstrait dans le modèle, non implémenté
- Collaboration temps réel sur un même snippet → v2+
- Marketplace payante → v2+
- Mobile app → v2+
- Intégration IDE native → après MVP (API publique d'abord)
