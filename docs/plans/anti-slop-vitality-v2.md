# Plan — Anti-slop & dimension `vitality` (formula v2)

**Statut :** brouillon, en attente de validation point par point.
**Auteur :** discussion 2026-04-27.
**Cible code :** `backend/src/services/ingestion/github.rs`, `backend/src/services/quality/`, `backend/scoring/`, `backend/migrations/`, `frontend/src/features/repos/`, `backend/src/mcp/`.

---

## 1. Contexte & problème

À l'ère où n'importe quel repo OSS peut être *vibe-coded* en 48 h via IA, la promesse UseStakly — « trier les projets pertinents de ceux qui ne le sont pas » — exige que le score de qualité résiste au **slop frais**. Aujourd'hui, ce n'est pas le cas.

### Diagnostic du scoring actuel (formula v1.1)

| Dimension | Poids | État sur le corpus public seedé |
|---|---|---|
| `freshness` | 0.20 | Actif, dominant en pratique |
| `adoption` | 0.15 | **≈ 0** (pas de signaux MCP réels) |
| `reliability` | 0.40 | **Neutre 0.5** sous `min_sample = 5` |
| `abandonment` | 0.25 | Dérive de freshness + regret_rate (≈ 0) |

**55 % du poids du score est neutralisé** sur le corpus actuel. Le score effectif d'un repo sans signaux MCP est dominé par la fraîcheur de `last_commit_at` — exactement la dimension qu'un repo IA-slop tout neuf maximise sans effort.

### Failles structurelles complémentaires

- **Aucun signal anti-slop passif** capturé à l'ingestion : nombre de contributeurs distincts, cadence de commits, présence de CI, releases tagguées, etc.
- **Effet Matthew inversé pour la découverte** : seuls les repos déjà signalés via MCP sortent du plancher. Mauvais pour un produit qui s'appelle « Discover ».
- **Garde-fous flags toxiques exposés** : `formula_v2` (compte neuf = poids 0) et Sybil OAuth pas livrés (gotchas CLAUDE.md). 3 comptes neufs au-dessus de `min_reputation: 0.45` peuvent flagger un concurrent en `broken`.
- **`archived ≠ abandon`** (mémoire) : ne sera pas câblé comme kill-switch.

---

## 2. Objectif de cette itération

Ajouter une **dimension `vitality`** alimentée par signaux structurels GitHub passifs, capturés à l'ingestion, qui ne dépendent **pas** du corpus déclaratif MCP. Bump de la formule en `v2.0` avec préservation de l'audit historique v1.

Critère de succès : un repo solo sans CI, sans release, freshly-pushed, ne peut **pas** atteindre le seuil `auto` (score ≥ 0.45) uniquement sur sa fraîcheur.

---

## 3. Signaux candidats

### 3.1 Set complet (4 signaux)

| Signal | Endpoint GitHub | Coût rate-limit | Discriminant pour |
|---|---|---|---|
| `distinct_contributors_90d` | `GET /repos/{o}/{r}/commits?since=` paginé | Cher (jusqu'à N pages) | Collectif vs solo |
| `commits_30d` | Même endpoint, count | Partagé avec ci-dessus | Cadence réelle |
| `has_ci` | `GET /repos/{o}/{r}/contents/.github/workflows` (404 = false) | 1 req | Discipline minimum |
| `releases_count` + `last_release_at` | `GET /repos/{o}/{r}/releases?per_page=1` (header `Link`) | 1 req | Maturité de livraison |

### 3.2 Set minimaliste (alternative)

`has_ci` + `last_release_at` seuls. Quasi-gratuit en rate-limit, déjà très discriminant contre le slop typique. À retenir si le coût rate-limit du set complet pose un problème opérationnel.

**Décision attendue : `[Q1]` — set complet, ou minimaliste ?**

---

## 4. Stockage

### 4.1 Option A — colonnes sur `external_artifacts` (préférée)

Migration `0018_repo_vitality.sql` ajoute :

```sql
ALTER TABLE external_artifacts
  ADD COLUMN distinct_contributors_90d INT,
  ADD COLUMN commits_30d INT,
  ADD COLUMN has_ci BOOLEAN,
  ADD COLUMN releases_count INT,
  ADD COLUMN last_release_at TIMESTAMPTZ,
  ADD COLUMN structural_signals_at TIMESTAMPTZ;
```

Signaux directement attachés à l'artifact, recompute écrase. Pas d'historique versionné.

### 4.2 Option B — table dédiée `repo_structural_signals`

Une row par repo + horodatage, historisable. Utile si un jour on veut tracer la dérive temporelle des signaux.

**Tradeoff :** A est plus simple, suffit aux besoins actuels ; B est plus propre si on accumule. Inclination par défaut : **A**.

**Décision attendue : `[Q2]` — A (colonnes) ou B (table dédiée) ?**

---

## 5. Formula v2

### 5.1 Pondération proposée

| Dimension | v1.1 | v2.0 proposé |
|---|---|---|
| `freshness` | 0.20 | 0.15 |
| `adoption` | 0.15 | 0.05–0.10 (cf. Q3) |
| `reliability` | 0.40 | 0.30 |
| `abandonment` | 0.25 | 0.20 |
| `vitality` | — | 0.20 |

Total ≈ 0.90–0.95 ; le delta sert de marge ou complète une dimension. Ajustable après calibration sur corpus.

### 5.2 Calcul de `vitality`

À spécifier en détail dans `formula_v2.toml`. Première proposition :

```
vitality = w1 * collective_score
         + w2 * cadence_score
         + w3 * ci_score
         + w4 * release_score

où :
  collective_score = saturate(distinct_contributors_90d / 5)   # ∈ [0, 1]
  cadence_score    = saturate(commits_30d / 10)                # ∈ [0, 1]
  ci_score         = has_ci ? 1.0 : 0.0
  release_score    = exp_decay(last_release_at, half_life=180)
```

Pondérations internes à fixer. Le détail va dans la PR 2.

### 5.3 Gestion de la version

- `formula_v1.toml` **conservé tel quel** — audit historique des rows existantes.
- `formula_v2.toml` créé en parallèle.
- `compute.rs` étendu pour supporter v2 ; ancienne logique gardée derrière le `formula_version` lu de chaque row.
- Bascule lecture/écriture sur v2 dès la migration appliquée.

### 5.4 Cold-start résiduel assumé

Un repo légitime sans release ni CI (ex. tooling perso de qualité, scripts personnels publiés) sera pénalisé. **Acceptable** au regard du risque slop. À documenter sur `/how-to-read`.

**Décision attendue : `[Q3]` — poids `adoption` en v2 : 0.10 / 0.05 / 0 ?**

---

## 6. Recompute & scheduler

- Bumper `formula_version` invalide tous les `artifact_scores` v1.1 → batch recompute requis sur l'ensemble du corpus.
- **Scheduler reste OFF en dev** (gotcha CLAUDE.md, rate-limit GitHub).
- Recompute déclenché manuellement :
  - Vérifier la présence d'un endpoint admin `/api/admin/scoring/recompute` (déjà partiellement présent via `/api/admin/scoring/explain/{repo_id}`).
  - Sinon, ajouter un binaire one-shot `recompute_all` ou un endpoint admin dédié.
- Pas de double scoring concurrent : bascule franche.

---

## 7. Front & MCP

### 7.1 Frontend

- `/repos/$id` : nouvelle tile `Vitality` à côté des 4 dimensions existantes, avec breakdown (contributors 90j, has_ci, dernière release, commits 30j).
- `/how-to-read` : section dédiée expliquant ce que la v2 mesure et **ce qu'elle ne mesure pas** (transparence sur les limites).
- i18n EN/FR à mettre à jour.

### 7.2 MCP

- `formula_version` retourné par les 5 tools devient `"v2.0"`.
- Provenance préservée (`source: "usestakly://..."`, `formula_version`, `scored_at`) — non négociable.
- `recommend_github_repos` profite automatiquement du nouveau ranking, aucun changement d'interface.
- CLI `usestakly-mcp` non impacté.

---

## 8. Découpage en PR

| PR | Périmètre | Mergeable indépendamment ? |
|---|---|---|
| **PR 1** | Migration `0018` + extension ingestion GitHub + tests unitaires capture des signaux. **Aucun impact scoring.** | Oui |
| **PR 2** | `formula_v2.toml` + extension `compute.rs` + bascule `formula_version` + recompute corpus + tests scoring. | Oui (après PR 1) |
| **PR 3** | Affichage front `/repos/$id` + tile vitality + MCP provenance + i18n + doc `/how-to-read`. | Oui (après PR 2) |

**Décision attendue : `[Q4]` — découpage 3 PR confirmé ou bundle 1 PR ?**

---

## 9. Alternatives écartées (nommées)

- **A. Heuristique anti-slop sur le README** (détection patterns ChatGPT) — fragile, faux positifs sur des projets bien rédigés. Pas avant d'avoir un corpus de slop annoté.
- **B. Modèle ML sur le code** — pas de labels, trop tôt, contre la promesse de transparence/audit revendiquée par UseStakly.
- **C. Source externe (Tea, OpenSauced, OSSF Scorecard)** — dépendance externe, coût, ferme la promesse de souveraineté. Scorecard mérite peut-être un coup d'œil plus tard mais pas dans cette itération.
- **D. Re-pondérer v1 sans nouveaux signaux** — déplace le problème, ne le résout pas. Le fond reste `freshness` qui maximise sur le slop frais.
- **E. Repousser à formula_v3 quand corpus MCP réel arrive** — laisse la beta exposable avec une promesse non tenue. Inacceptable vu le statut `public beta exposable` (TODO v5.5).
- **F. Câbler `archived = true` comme kill-switch** — interdit par mémoire (`archived ≠ abandon`).

---

## 10. Tradeoffs assumés

| Tradeoff | Décision |
|---|---|
| Coût rate-limit recompute initial (~10k repos × 2–4 reqs) | Acceptable hors-heures ; **jamais en dev** |
| `archived ≠ abandon` non câblé en kill-switch | Respecte la mémoire ; un repo archivé peut garder `vitality > 0` si releases récentes |
| Bump `formula_version` = ré-écriture des `artifact_scores` | Préserve l'audit historique (rows v1.1 restent lisibles taggées) |
| Repos solo légitimes pénalisés | Documenté sur `/how-to-read` ; acceptable vs risque slop |
| Pas d'historique des structural signals (option A) | Revisitable plus tard via une table d'audit dédiée |

---

## 11. Hors-scope explicite (déféré)

Les items suivants sont **délibérément hors de cette itération** — pas un classement de priorité :

- **`formula_v2` réputation** (compte neuf = poids 0) et Sybil OAuth — chantier indépendant du scoring de qualité, lié aux flags toxiques. À traiter séparément.
- **Détection slop sur README/code** (ML ou heuristique).
- **Intégration OSSF Scorecard** ou source externe.
- **Re-calibration des seuils auto/strict** — à faire après mesure sur corpus v2.0.
- **Ré-architecture historisée des signaux structurels** (option B).
- **Quota MCP global multi-token / par IP** — autre item ops, déjà identifié dans `docs/ops-mcp-coolify-hardening.md`.

---

## 12. Critères de validation

Avant de considérer la v2 livrée :

- [ ] Migration `0018` appliquée, colonnes peuplées sur ≥ 95 % du corpus seedé.
- [ ] `cargo test` passe, incluant nouveaux tests unitaires `vitality`.
- [ ] `cargo clippy --all-targets -- -D warnings` clean.
- [ ] `cargo fmt --check` clean.
- [ ] Front : tile `vitality` visible et lisible sur `/repos/$id`, EN + FR.
- [ ] Recompute corpus terminé, `formula_version = "v2.0"` partout.
- [ ] MCP : `recommend_github_repos` retourne provenance v2.0 (test e2e).
- [ ] Test manuel : un repo solo récent sans CI/sans release ne passe **pas** le mode `auto`.
- [ ] Test manuel : un repo collectif établi (ex. `tokio-rs/axum`) passe le mode `strict`.
- [ ] `/how-to-read` mis à jour (EN + FR).

---

## 13. Questions ouvertes en attente de validation

| ID | Question | Inclination par défaut |
|---|---|---|
| **Q1** | Set complet (4 signaux) ou minimaliste (2) ? | Set complet |
| **Q2** | Colonnes sur `external_artifacts` ou table dédiée ? | Colonnes (option A) |
| **Q3** | Poids `adoption` en v2 : 0.10 / 0.05 / 0 ? | 0.10 |
| **Q4** | Découpage 3 PR ou bundle 1 PR ? | 3 PR |

À valider point par point avant de commencer la PR 1.
