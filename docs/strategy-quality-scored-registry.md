# Stratégie — Registry qualité-scored pour agents IA

> Version : 1.2 — 2026-04-26
> Statut : **moat valide, scope resserré au pivot 2026-04-21**
> Lien : ce doc pose les principes de fond. Le scope produit actuel est dans `strategy-pivot-2026-04-21.md`. L'exécution priorisée vit dans `docs/plans/remaining-work-2026-05-03.md`.

> **Principles-only (2026-05-16)** : moat data-lock-in, signaux d'usage > stars, télémétrie passive, evidence obligatoire, anti-gaming — toujours valides. Tableaux business multi-tier, GTM équipes, exemples snippets internes et MVP 6–8 semaines sont historiques.

> ### Bandeau de réconciliation post-pivot
>
> Ce document a été écrit avant le pivot du 2026-04-21. **Le fond (moat data-lock-in, signaux d'usage > stars, télémétrie passive, evidence obligatoire, anti-gaming) reste valide.** Sont **caducs** :
>
> - le tableau **business model multi-tier** (§💰) — pas de Pro / Team / Enterprise. Scope = repos GitHub publics OSS.
> - le **GTM en 3 temps** (§🗺) — phase B « démarchage des équipes » sortie du roadmap.
> - les exemples qui parlent de **snippets internes** (§📡, §📊, §🔁) — UseStakly annote des repos GitHub publics, pas des snippets privés.
> - le **MVP 6–8 semaines** (§🛠) — déjà livré sous une autre forme (R1 ingestion + R2 search + R3 watchlist + R4/R5 trust + MCP).
> - les **questions ouvertes** finales (§❓) — toutes tranchées au 2026-04-26.
>
> Source actuelle pour le scope, l'avancement et les décisions : `strategy-pivot-2026-04-21.md` + `TODO.md`.

## 🎯 Pitch en une phrase

> **Un registry de code avec des signaux de qualité dérivés de l'usage réel, que les agents IA utilisent pour filtrer automatiquement les briques obsolètes, cassées ou dangereuses avant même de te proposer du code.**

Ce n'est pas un GitHub avec dislike. C'est **Rotten Tomatoes pour le code**, lu par des agents, pas par des humains.

## 🧠 Le principe central

Les votes humains sont pourris :

- Les gens ne votent pas (99 % des utilisateurs ne votent jamais).
- Les votes sont politiques (review-bombing, cargo cult, stars de hype).
- Les votes ne sont pas actionnables par une IA.

**Les signaux doivent être dérivés du comportement, pas des opinions.**

Chaque fois qu'un agent résout un snippet via MCP, que le build passe ou pas, que le code reste ou disparaît 6 h plus tard → c'est du signal d'usage réel. C'est ça la data, pas des likes.

## 📡 Les 4 types de signaux à collecter

### 1. Signaux passifs (télémétrie auto)

Collectés sans effort utilisateur, par le MCP :

| Signal | Description |
|---|---|
| `resolve_count` | Combien de fois le snippet a été résolu par un agent |
| `build_success_rate` | % des cas où le build passait après l'inclusion |
| `regret_rate` | Suppression dans les 24 h après insertion |
| `re_resolve_rate` | Re-demande d'un autre snippet pour la même tâche (insatisfaction) |

**C'est de l'or.** Personne d'autre ne l'a. GitHub ne voit que ce que tu pushs, pas ce que tu utilises. npm ne voit que les installs, pas le succès post-install.

### 2. Signaux actifs structurés (pas de dislike nu)

Pas de thumbs-down. Des faits avec preuve :

| Signal | Preuve requise |
|---|---|
| `works-in-prod` | Lien vers PR / déploiement |
| `broken-on-ts5` | Repro minimal |
| `security-issue` | CVE ou repro |
| `doesn't-match-claim` | Description du mismatch |
| `deprecated-by-author` | Owner-only |

**Règle d'or : pas de preuve, pas de signal pris en compte.** Ça tue 90 % du spam et du review-bombing.

### 3. Signaux temporels (auto-calculés)

- `freshness` — date du dernier update
- `ecosystem-drift` — compatibilité avec les versions actuelles des deps
- `abandonment-score` — heuristique combinée (update + activité owner + PR ignorées)

### 4. Signaux contextuels (côté agent)

Calculés au moment de la query :

- `stack-match` — compatibilité avec le `package.json` de l'user (TS 5.4 vs 4.9, React 19 vs 17)
- `similar-context-success` — taux de réussite dans des projets similaires

## 📊 Le modèle de scoring

Pas une note 1–5. Un **profil multi-dimensionnel** :

```
@ui:data-table@2.1.0
├─ freshness:      0.9   (updated 2 months ago)
├─ adoption:       0.6   (moderate usage, 430 resolves)
├─ reliability:    0.95  (98% successful builds)
├─ stack-match:    0.8   (compatible with your deps)
├─ abandonment:    0.1   (active)
└─ flags:          []
```

L'agent query avec un **filtre** selon le niveau de tolérance :

| Mode | Filtre par défaut |
|---|---|
| `auto` (défaut) | `reliability > 0.9 AND abandonment < 0.3 AND flags excludes 'security'` |
| `strict` (prod critique) | `reliability > 0.95 AND stack-match > 0.8 AND verified = true` |
| `explore` | Aucun filtre |

**C'est le super-pouvoir.** L'agent ne « cherche » plus, il filtre un index qualité-scored. Les snippets morts deviennent invisibles par défaut.

## 🏰 Le moat (pourquoi c'est défensible)

1. **Data lock-in légitime.** Chaque résolution enrichit le dataset. Plus d'users → meilleurs signaux → meilleur filtrage → plus d'users. Flywheel classique, mais réel ici.
2. **GitHub ne peut pas copier.** GitHub est un hébergeur, pas un curateur. Ils ne peuvent pas dire publiquement « ce repo est pourri ». Leur modèle économique s'y oppose.
3. **npm ne peut pas copier.** Même raison. Et npm n'a pas de télémétrie d'usage post-install.
4. **Les modèles fondamentaux ne peuvent pas copier.** OpenAI / Anthropic peuvent améliorer leurs modèles, pas construire un index qualité externe en temps réel. **Complémentaire**, pas compétitif.

## 💡 Le gros coup tactique : ne pas héberger le code

**Tu n'as pas besoin d'héberger le code pour vendre du scoring.**

```
agent: "trouve-moi un date-picker React"
→ le MCP retourne:
  - shadcn/ui DatePicker    (score: 0.95, via npm)
  - react-datepicker v7      (score: 0.82, freshness: 0.3) ⚠
  - @my-lib/picker v1.2      (score: 1.0, your own)
```

Tu deviens le **Rotten Tomatoes** qui annote npm, GitHub, shadcn, crates.io. Tu ne produis pas le code. Tu produis le **jugement**.

**Avantages** :

- Zéro cold start côté corpus (tout npm / GitHub est disponible d'emblée).
- Zéro problème légal / IP (tu annotes, tu ne redistribues pas).
- Valeur immédiate même sans utilisateur qui upload quoi que ce soit.

Le code privé / maison reste sur le registry hébergé. Le code public est annoté. **Deux produits dans un.**

## ⚠ Les problèmes durs à résoudre

### A. Cold start sur les signaux

Jour 1, pas de data. Trois leviers :

- **Seed manuel curé** — 200 à 500 snippets high-quality à la main.
- **Bootstrap depuis sources externes** — stars GitHub, downloads npm, date du dernier commit, issues ouvertes vs fermées. Prior, pas vérité, mais mieux que rien.
- **Registries personnels d'abord** — signaux issus de l'usage propre de l'owner avant socialisation.

### B. Anti-gaming

Dès que les signaux comptent, les gens trichent. Défenses :

1. **Poids pondéré par la réputation.** Compte neuf = signal à 0. Compte avec 50 snippets validés en prod = signal à 100×.
2. **Preuve obligatoire** pour les signaux négatifs. Un `broken` sans repro est ignoré.
3. **Télémétrie > vote.** Un signal d'usage passif (« 1200 agents ont résolu ce snippet, 94 % build OK ») écrase 50 votes manuels. Tu ne peux pas faker 1200 builds réels.
4. **Graphe Sybil-resistant.** Via OAuth GitHub / Discord, graphe d'activité disponible. Compte créé hier sans historique = pas de poids.

### C. Transparence du scoring

Dilemme :

- Formule publique → gamable.
- Formule opaque → distrust.

**Solution médiane** : publier les inputs et leurs poids par catégorie, pas la formule exacte. « Freshness compte pour ~20 %, reliability pour ~40 %. » Assez pour faire confiance, pas assez pour gamer proprement.

### D. Les flags toxiques

Politique :

- `deprecated` / `unmaintained` → evidence ou consensus de N users distincts avec reputation > threshold.
- `security-issue` → process modéré (pas de publication avant validation).
- Appel possible par l'auteur, historique transparent.

**Ce n'est pas un réseau social. C'est un système de preuves.**

## 💰 Business model

| Tier | Prix | Valeur |
|---|---|---|
| Free | 0 € | Registry perso + accès lecture au scoring public |
| Pro solo | ~12 €/mo | Registry privé, filtres custom, mode strict pour agents |
| Team | ~40 €/user/mo | Registry équipe, reputation partagée, collecte auto via CI |
| Enterprise | contact | On-prem, compliance signals, SLA, rules custom |

**Le vrai cash est Team.** Dans une équipe de 15 devs, le graphe d'usage interne est **le signal parfait** pour cette équipe précisément. Le registry devient « qualité selon NOUS », pas « qualité selon internet ». Très haute valeur perçue.

## 🗺 Go-to-market en 3 temps

1. **Phase A (0–6 mois) — Solo devs + scoring public**
   Offre : « je connecte mon `package.json`, mon agent sait quoi éviter ». Gratuit, viral, prouve la valeur du scoring.

2. **Phase B (6–12 mois) — Teams**
   Une fois le scoring crédible, démarchage des équipes avec conventions maison + besoin de filtrer leurs deps. **C'est là qu'on charge.**

3. **Phase C (12+ mois) — Enterprise / compliance**
   Secteurs régulés (santé, finance, défense) qui doivent prouver que leur code généré par IA vient de sources auditées. Gros contrats, peu d'utilisateurs.

## 🛠 MVP ciblé sur cet angle (6–8 semaines)

Si on pivote maintenant :

1. **Schema** — ajouter une table `quality_signals` + colonnes de métriques sur `snippets`.
2. **Collection**
   - Passive : logger chaque `resolve_reference` avec un `outcome` posté par l'agent / user 1 h après.
   - Active : endpoint `POST /api/snippet/:id/signal` avec evidence link obligatoire.
3. **Scoring** — fonction simple documentée, recalculée en batch quotidien.
4. **Agent API** — `search_library` accepte un param `filter` (auto / strict / explore).
5. **Bootstrap corpus** — 200 snippets curés manuellement pour avoir un terrain de démo.
6. **Démo killer** — montrer un agent qui refuse de générer du code basé sur `request@2.88` parce que le signal `abandonment: 0.92` l'exclut en mode auto. Avant / après visible en 30 secondes.

## 🚨 Risques à ne pas minimiser

1. **Cold start brutal.** Sans 6 mois d'usage, les signaux sont faibles. Prévoir une phase de bootstrap manuel soutenu.
2. **Adoption MCP hors de contrôle.** Si Cursor / Claude Code / Codex ne callent pas le MCP, personne n'utilise le scoring. → prévoir **aussi** une extension IDE ou un plugin CLI pour contourner cette dépendance.
3. **Légal sur l'annotation.** Dire publiquement « ce package est cassé » peut entraîner du legal. Politique claire, tone factuel, possibilité d'appel.
4. **Dérive réseau social.** Garder le cap factuel. Pas de profils publics, pas de followers, pas de karma vanity. Rien que des signaux.

## 🔁 Conséquences sur le projet actuel

Si on y va :

- La table `snippets` devient « artefacts scorables ».
- **Ajouter dès maintenant** un schéma de collecte de signaux, même sans l'exploiter tout de suite. Data is gold — la capturer dès le jour 1.
- Le MCP n'est plus juste `get_snippet`, c'est `get_snippet_with_quality_context`.
- La provenance devient encore plus centrale : signer la sortie avec `slug@v + quality_score au moment de la résolution`. Audit trail naturel.
- **Phase 8 (safety) devient phase 2**, pas phase finale. C'est le cœur.

## 📝 Évaluation synthétique

| Critère | Note |
|---|---|
| Potentiel | Élevé — premier angle avec un vrai moat durable |
| Difficulté | Élevée — scoring + télémétrie + anti-gaming + légal, c'est un produit complet |
| Timing | Bon — les agents deviennent mainstream en 2026–2027, ~12–18 mois d'avance |
| Fit équipe actuelle | Moyen — plus lourd que « bibliothèque de snippets », engagement sérieux requis |

## ❓ Questions ouvertes à trancher

- [ ] On pivote totalement sur cet angle, ou on le garde en phase 2 après le MVP actuel ?
- [ ] Le code public est annoté (option 3) dès le MVP ou en V2 ?
- [ ] Formule de scoring v1 : publique avec poids, ou opaque avec inputs listés ?
- [ ] Extension IDE / CLI en parallèle du MCP, ou on mise tout sur MCP ?
- [ ] Seed corpus : qui cure les 200 premiers snippets, avec quels critères ?

---

> Pour l'état d'avancement actuel, voir `TODO.md` v5.5. Pour le scope tenu depuis le pivot, voir `strategy-pivot-2026-04-21.md`. Les anciens plans `mvp-one-shot-blueprint.md` et `product-vision-and-safety.md` ont été archivés sous `docs/archive/snippets/`.
