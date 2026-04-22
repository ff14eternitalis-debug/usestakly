# Stratégie — Pivot 2026-04-21 : scope public OSS, business model ouvert, Intuition exploré

> Version : 1.0 — 2026-04-21
> Statut : **décisions de scope actées** (business model et Intuition = exploration)
> Base : complète et resserre `docs/strategy-quality-scored-registry.md` (2026-04-20)

## TL;DR

Trois décisions issues de la session du 2026-04-21 :

1. **Scope resserré** : UseStakly cible **uniquement les repos GitHub open source publics**. Le privé / entreprise sort du roadmap — « qu'ils se débrouillent ».
2. **Business model** : **non tranché**. Panorama ci-dessous, recommandation = pas de décision maintenant, laisser le comportement utilisateur trancher à 6 mois.
3. **Intuition (web3)** : exploré, **cohérence conceptuelle excellente** mais garde-fou — jamais en backbone. Couche 2 optionnelle au mieux.

La stratégie v1 (`strategy-quality-scored-registry.md`) reste **valide sur le fond** (signaux d'usage > stars, Rotten Tomatoes du code, moat par data lock-in). Ce doc ne la remplace pas, il la resserre.

## 1. Scope resserré aux repos GitHub publics

### Le problème attaqué

Un dev qui fait de la veille GitHub est **mal-guidé par défaut** :

- Les stars capturent une **intention** (« ce repo a l'air cool »), pas un **verdict** (« ce repo a tenu 6 mois en prod »).
- Les forks et clones additionnent le bruit sans fournir de feedback actionnable.
- Un repo populaire peut se faire dépasser silencieusement par un concurrent plus frais sans que les stats reflètent ce décalage.
- Une fois qu'un dev star, il ne revient jamais dire si ça a vraiment marché.

GitHub ne peut pas corriger ce biais — son modèle économique s'y oppose, et il n'a pas de télémétrie post-clone. UseStakly comble ce trou.

### Ce que ça implique concrètement

- **L'app n'héberge pas le code.** Elle annote des URLs / SHA canoniques de repos publics.
- **La table `external_artifacts`** (migration `0010`) devient le cœur du modèle de données, pas un satellite.
- **Pas de CRUD snippets privés** dans l'UX principale. Le stockage maison (libraries/snippets internes) reste dans le schéma mais perd son rôle central — potentiellement un corpus de démo + seed, pas un produit.
- **Pas de Team tier / registry privé** dans la roadmap. La mécanique de stratégie v1 sur « équipes qui paient pour leur registry maison » sort.

### Ce qu'on gagne

- **Message limpide** : « on note les repos GitHub open source, point. »
- **Zéro cold start corpus** : tout GitHub public est le terrain de jeu dès le jour 1.
- **Zéro friction légal privé** : rien à gérer côté RGPD entreprise ou IP.
- **Adoption virale facile** : gratuit, utile dès l'install MCP, aucun compte requis pour consommer.
- **Démo killer immédiate** : « voici `request@2.88` avec 26k stars, score UseStakly : `abandonment 0.92`. Ton agent ne l'utilisera pas en mode `auto`. »

### Ce qu'on perd

- **Le tier Team, identifié comme vrai cash dans la stratégie v1.** La monétisation est à réinventer (cf. §2).
- **La couverture npm / crates.io / shadcn** — reportée. Scope v1 = GitHub uniquement. Les autres écosystèmes viendront quand le produit est validé.

## 2. Business model — non tranché

Panorama détaillé des modèles possibles (maintainer claim, agent licensing, freemium API, data licensing, sponsoring) + recommandation temporelle : voir le doc **privé** `docs/business/business-model-exploration.md`.

Ce fichier est volontairement exclu de git (`.gitignore`) pour garder la réflexion business off-public tant qu'elle n'est pas tranchée.

Résumé en une ligne : **ne pas trancher maintenant**, les briques techniques à construire sont neutres par rapport au modèle final.

## 3. Intuition — exploration, pas engagement

### Ce que c'est

Protocole on-chain (Layer 3 sur Base, Arbitrum Orbit) avec 3 primitives :

- **Atom** : identifiant canonique unique d'une entité (personne, repo, concept).
- **Triple** : claim structurée subject-predicate-object.
- **Signal** : stake économique sur une triple — skin in the game.

Trust = historique de stakes corrects. Reputation = émergence des bons stakers.

### Le mapping 1:1 avec UseStakly

| UseStakly | Intuition |
|---|---|
| Un repo GitHub public | Un **atom** (URL canonique) |
| Un signal `broken-on-ts5` avec evidence | Une **triple** `repo_X` — `broken_on` — `ts5` |
| Réputation owner (prévu formula_v2) | Émergence naturelle via historique de stakes |
| Provenance signée `slug@v + score@t` | Gratuit — on-chain par définition |

### Ce qu'Intuition résout bien

1. **Anti-gaming built-in** — le stake **est** l'evidence, pas besoin de coder une formula_v2.
2. **Provenance cryptographique** — signatures, historique tamper-proof.
3. **Graphe portable** — d'autres apps peuvent lire les scores UseStakly, joue en faveur de l'adoption.

### Les 3 risques durs

#### Risque #1 — Friction UX

Un dev qui veut flaguer `broken-on-ts5` avec 5 lignes de repro **n'ouvrira pas un wallet**. Si l'action nécessite une signature on-chain, on perd 95 % des contributeurs. La cible (« dev qui fait de la veille GitHub ») n'est **pas** web3 native.

#### Risque #2 — Narratif crypto qui divise

« Registry qualité pour agents IA » = message universel.
« Registry qualité avec stakes on-chain sur Base L3 » = fraction du marché boycotte automatiquement. Anti-crypto = vocal en 2026.

#### Risque #3 — Dépendance sur un protocole beta

Intuition est en beta. Construire le core dessus = risque existentiel si le protocole pivote / sunset / meurt.

### Position retenue

**Jamais Intuition en backbone.** UseStakly en couche 1 (DB Postgres maison, evidence obligatoire, scoring transparent). Intuition en couche 2 optionnelle pour des actions premium :

- **Maintainer claim on-chain** : un maintainer stake pour prouver qu'il est le vrai owner → badge vérifié.
- **Hard vouch** : un dev confiant stake sur « ce repo tiendra 12 mois ». Signal très fort, surpondéré vs signaux gratuits.
- **Export du graphe** : les scores mirrorés on-chain pour portabilité.

Avantages :

- **UX mainstream** préservée (gratuit, pas de wallet par défaut).
- **Revenu possible** via frais sur les stakes premium.
- **Narratif web3** disponible pour qui le veut, invisible pour les autres.
- **Fallback** : si Intuition meurt, on coupe la couche 2, le produit survit.

## 4. Implications concrètes

### Pour le TODO / roadmap

- **Phase 6 (Quality signals)** : ajustement — la table `external_artifacts` devient centrale, pas satellite. Pipeline d'ingestion GitHub à prioriser.
- **Phase 7 (Search / Resolve)** : inchangée sur les filtres, mais la source devient majoritairement `external_artifacts`.
- **Phase 8 (MCP)** : `resolve_reference` doit gérer un repo GitHub par URL / SHA, pas seulement un slug interne.
- **Phase 10 (Bootstrap corpus)** : **change de nature**. Plus « curer 200 snippets seed » mais « ingérer les top N repos GitHub populaires et leurs priors externes ». C'est la phase la plus impactée.
- **Team tier / registry privé** : retiré du roadmap. La stratégie v1 `docs/strategy-quality-scored-registry.md` ligne 167 est **obsolète sur ce point**.

### Pour le code

Rien à refaire dans l'immédiat. Les briques construites (domain quality, scoring service, endpoint signals, search filtré, migration `0010`) sont **agnostiques au scope**. La table `external_artifacts` existe déjà.

Ce qui manque : le **pipeline d'ingestion GitHub** (GitHub API, rate-limit handling, refresh des priors, mapping repo → artifact). C'est du neuf, pas du refactor.

## 5. Décisions encore ouvertes

- [ ] **Quand lance-t-on l'ingestion GitHub ?** Avant ou après Phase 8 (MCP) ?
- [ ] **Quel seuil de popularité** pour être scoré ? Tout GitHub ? Top 10k stars ? Top N par langage ?
- [ ] **Refresh cadence** des priors GitHub (stars, last_commit, issue ratio) — temps réel, daily, weekly ?
- [ ] **Maintainer claim** : quel mécanisme de preuve d'ownership (OAuth GitHub suffit-il) ?
- [ ] **Intuition** : POC timing — maintenant, MVP+6, ou jamais ?

---

> Ce doc sera fusionné ou archivé une fois les décisions business model et Intuition tranchées. En attendant, il fait foi sur le scope (§1).
