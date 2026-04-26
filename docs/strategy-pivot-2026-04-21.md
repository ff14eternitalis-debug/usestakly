# Stratégie — Pivot 2026-04-21 : scope public OSS, business model ouvert

> Version : 1.1 — 2026-04-26
> Statut : **scope assumé**, public beta exposable. Décisions business model et Intuition toujours ouvertes.
> Base : resserre `docs/strategy-quality-scored-registry.md` (v1.1, 2026-04-20). Le pivot n'a pas remplacé la stratégie de fond — il l'a focalisée.

## TL;DR

Trois décisions issues du 2026-04-21, tenues depuis :

1. **Scope resserré et tenu** : UseStakly cible **uniquement les repos GitHub open source publics**. Privé / entreprise / Team tier sont sortis du roadmap.
2. **Business model** : **non tranché**. Réflexion privée dans `docs/business/business-model-exploration.md` (gitignore). Pas d'urgence à décider — les briques tech sont neutres vs le modèle final.
3. **Intuition (web3)** : exploré, **jamais en backbone**. Couche 2 optionnelle au mieux, post-traction.

Le moat de la stratégie v1.1 (`strategy-quality-scored-registry.md`) reste valide sur le fond. Ce doc resserre le scope ; la mention « tier Team comme vrai cash » est **caduque** depuis ce pivot.

## 1. Scope resserré aux repos GitHub publics

### Le problème attaqué

Un dev qui fait de la veille GitHub est mal-guidé par défaut :

- Les stars capturent une **intention** (« ce repo a l'air cool »), pas un **verdict** (« ce repo a tenu 6 mois en prod »).
- Les forks et clones additionnent du bruit sans feedback actionnable.
- Un repo populaire peut se faire dépasser silencieusement par un concurrent plus frais sans que les stats reflètent ce décalage.
- Une fois qu'un dev star, il ne revient jamais dire si ça a vraiment marché.

GitHub ne peut pas corriger ce biais — son modèle économique s'y oppose et il n'a pas de télémétrie post-clone. UseStakly comble ce trou.

### Ce que ça implique concrètement

- L'app **n'héberge pas de code**. Elle annote des URLs / SHA canoniques de repos publics.
- La table `external_artifacts` (migration `0010` + colonnes GitHub-specific de `0011`) est le cœur du modèle de données.
- Pas de CRUD snippets privés. Le schéma legacy reste en base mais sans surface runtime active.
- Pas de Team tier / registry privé. La mécanique « équipes qui paient pour leur registry maison » de la stratégie v1 est sortie.

### Ce qu'on a gagné depuis le pivot

- **Message limpide** : « on note les repos GitHub OSS, point. »
- **Zéro cold start corpus** : tout GitHub public est le terrain de jeu dès le jour 1. Corpus initial seedé via `backend/seeds/top_repos.toml` + script `scripts/seed-public-corpus.ps1`.
- **Adoption agent immédiate** : `npx usestakly-mcp install` configure Codex/Cursor en une commande. Endpoint MCP `/mcp` opérationnel.
- **Démo killer prête** : un agent peut interroger `recommend_github_repos`, recevoir un score multidimensionnel + provenance, refuser un repo `auto`-incompatible. Visible immédiatement.

### Ce qu'on a perdu (assumé)

- Le tier Team identifié comme vrai cash dans la stratégie v1. La monétisation reste à réinventer (cf. §2).
- La couverture npm / crates.io / shadcn — reportée. Scope = GitHub uniquement.

## 2. Business model — non tranché

Panorama complet (maintainer claim, agent licensing, freemium API, data licensing, sponsoring) dans le doc privé `docs/business/business-model-exploration.md` (gitignore).

Position au 2026-04-26 : **ne pas trancher maintenant**. L'intention explicite est de déployer en public beta pour auditer le parcours utilisateur en conditions réelles, **pas pour chasser des users**. La monétisation viendra quand le produit est solide et que l'on observe ce qui colle ou pas.

Les briques techniques en place sont neutres vs le modèle final.

## 3. Intuition — exploration, pas engagement

### Position retenue, tenue

**Jamais Intuition en backbone.** UseStakly tourne sur Postgres maison, evidence obligatoire, scoring transparent. Intuition reste une couche 2 optionnelle pour des actions premium si un jour la traction le justifie :

- **Maintainer claim on-chain** — un maintainer stake pour prouver qu'il est l'owner légitime → badge vérifié.
- **Hard vouch** — un dev confiant stake « ce repo tient 12 mois ». Signal très fort.
- **Export du graphe** — scores mirrorés on-chain pour portabilité.

Avantages : UX mainstream préservée, narratif web3 disponible pour qui le veut, fallback si Intuition meurt.

POC pas planifié — post-traction.

## 4. Implications pour le code

### Ce qui était nouveau au pivot, livré depuis

- **Pipeline d'ingestion GitHub** (R1) — `services/ingestion/github.rs`, normalisation, priors snapshot. Endpoint `POST /api/repos/add`, binaire `seed_github`.
- **Discovery repos** (R2) — `/api/search` repointé sur `external_artifacts`, filtres `auto`/`strict`/`explore` + filtres avancés (langage, license, stars min, freshness).
- **Watchlist + notifications + scheduler opt-in** (R3) — migration 0012, `services/scheduler::spawn_recompute_loop`.
- **MCP read-only** (R5a) — 3 read tools, auth Bearer, migration 0013.
- **MCP write + recommend** (R5b) — `log_usage`, `watch_repo`, `recommend_github_repos`. Garde-fous quota/cooldown/fenêtre/réputation. Migration 0014. CLI npm `usestakly-mcp` publié.
- **Trust v1 + modération** (R4) — réputation runtime, owner detection multi-niveaux, review admin, dispute owner, timeline. Migrations 0015 et 0016.
- **Scoring v1.1** — pondération `outcome × reporter × dedup` (`services/quality/weighting.rs`). Endpoint admin explain.
- **Semantic search** (R2b) — fastembed + pgvector derrière feature `semantic-search` (OFF par défaut, OFF en prod). Migration 0017. Calibration ranking hybride à valider sur corpus plus large.
- **Public beta frontend** — `/status`, `/privacy`, `/how-to-read`, `/mcp-guide`. Audit phase 2 connecté livré + corrections (`return_to` signé, watchlist confirm remove, error states queries, mark-read on click).
- **Durcissement MCP** — middleware Authorization Bearer obligatoire dès `initialize`/`tools/list`.

### Ce qui reste neuf à construire

- Backup DB Coolify planifié (item ops bloquant ouverture)
- Rate-limit applicative globale `/mcp` (initialize + tools/list + reads)
- Alerte externe sur `/health`, `/api/status/public`, MCP test
- Page légale `/legal` ou `/terms`, domaine + email contact officiel
- formula_v2 (compte neuf = poids 0, usage prod surpondéré) + Graphe Sybil OAuth GitHub
- `owner_inactive_days` côté R1 → débloque la règle « maintainer silencieux 90 j » R3
- Cadence refresh ingestion automatique + ETags + backoff GitHub
- Critère corpus formel (top N par langage / sur demande / via watchlist)
- Page compte plus riche, UX explication scoring discovery, graph historique repo
- E2E flows connectés réels (login OAuth → watchlist → notif), aujourd'hui couverts par mocks

## 5. Décisions toujours ouvertes (au 2026-04-26)

- [ ] **Critère corpus initial** : top N par langage / sur demande / via watchlist uniquement ?
- [ ] **Canal notifications** : in-app suffit pour le MVP ou email obligatoire dès l'ouverture publique ?
- [ ] **Token agent** : Bearer généré par l'user (en place) ou OAuth device flow ?
- [ ] **Intuition** : POC quand ? Jamais, post-traction, post-MVP+6 mois ?
- [ ] **Tables snippets/libraries** dormantes : garder cachées (réactivables) ou drop net ?

Décisions tranchées depuis l'écriture initiale :

- ~~Pivoter ?~~ Oui, acté.
- ~~Quand ingestion GitHub ?~~ Avant MCP — fait.
- ~~Router frontend ?~~ TanStack Router — fait.
- ~~Scheduler externe ou maison ?~~ `tokio::spawn` opt-in — fait.

---

> Ce doc reste utile comme référence du **scope** (§1) et du cadre **Intuition** (§3). Le détail d'avancement vit dans `TODO.md` v5.5.
