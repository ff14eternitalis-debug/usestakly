# Smoke test — formula v1.1 (pondération signal-par-signal)

**Date** : 2026-04-24
**Formula version** : `v1.1`
**Scope** : valider end-to-end que `recompute_all_scores` et l'endpoint admin `/api/admin/scoring/explain/{repo_id}` appliquent bien la pondération `outcome_weight × review_weight(reporter) × 1/(1 + k · n_prev_same_user_same_repo)` définie dans `scoring/formula_v1.toml`.

## Setup

- Postgres local via `docker compose up -d` (image `pgvector/pgvector:pg17`, DB `project_k`, port 5432).
- Backend `cargo run` sur `127.0.0.1:4000`.
- `APP_SCHEDULER_ENABLED` non défini (scheduler off).
- Corpus existant au début du test : 10 `external_artifacts`, 0 `quality_signals`.

## Signaux injectés

Cible : repo `zod` (`4e693c16-ea9c-493c-8a18-cac71659e751`).

| # | Signal | User | Tier (post-seed) | `created_at` |
|---|---|---|---|---|
| 1 | `resolve` | `usestakly-dev` | emerging (score 0.51) | +0s |
| 2 | `resolve` | `usestakly-dev` | emerging | +60s |
| 3 | `resolve` | `usestakly-dev` | emerging | +120s |
| 4 | `resolve` | `noob-unproven` (user frais 1 jour, 0 historique) | unproven (score 0.41) | +120s |
| 5 | `resolve` | `noob-unproven` | unproven | +150s |
| 6 | `build_success` | `usestakly-dev` | emerging | +180s |

## Résultats observés

### Recompute

```
POST /api/admin/scoring/recompute
→ { "formulaVersion": "v1.1", "externalsProcessed": 10 }
```

### Explain sur `zod`

```
GET /api/admin/scoring/explain/4e693c16-...
```

#### Breakdown signal par signal

| Signal | Outcome weight | Reputation multiplier | Dedup multiplier | `n_prev` | Poids final |
|---|---|---|---|---|---|
| resolve #1 dev | 1.0 | 0.55 | 1.000 | 0 | **0.550** |
| resolve #2 dev | 1.0 | 0.55 | 0.800 | 1 | **0.440** |
| resolve #3 dev | 1.0 | 0.55 | 0.667 | 2 | **0.367** |
| resolve #1 noob | 1.0 | 0.30 | 1.000 | 0 | **0.300** |
| resolve #2 noob | 1.0 | 0.30 | 0.800 | 1 | **0.240** |
| build_success dev | 1.2 | 0.55 | 1.000 | 0 | **0.660** |

#### Weighted counts agrégés

| Bucket | Raw count | Weighted |
|---|---|---|
| `resolve` | 5 | **1.897** |
| `build_success` | 1 | **0.660** |
| `build_failure` | 0 | 0.000 |
| `regret` | 0 | 0.000 |

5 resolves bruts → 1.9 effectifs : **62 % du bruit est filtré** par la combinaison réputation + dedup.

#### Score final

| Dimension | Valeur |
|---|---|
| freshness | 0.770 |
| adoption | 0.154 |
| reliability | 0.500 (neutral — below `min_sample`) |
| abandonment | 0.230 |
| **overall** | **0.570** |

À comparer avec ce qu'on aurait obtenu sans pondération (`resolve_count = 5` brut avec coefficients uniformes) : overall > 0.75. La pondération v1.1 produit bien le score plus conservateur attendu quand la moitié des signaux vient d'un compte neuf et que trois autres sont de la répétition du même user.

## Validations couvertes

- ✅ Pondération réputation : même outcome du dev (0.55) vs du noob (0.30) = facteur 1.83×.
- ✅ Dedup douce : 3e resolve du dev pèse 0.367 au lieu de 0.55. Après 3 répétitions, poids ÷ 1.5.
- ✅ Outcome weight : `build_success` (1.2) pèse proportionnellement plus qu'un `resolve` (1.0) à réputation / dedup égaux.
- ✅ `formula_version = "v1.1"` persisté correctement.
- ✅ Counts bruts (`raw_resolve`, etc.) conservés en DB pour audit, à côté des counts pondérés utilisés par `compute_score`.

## Limites constatées

- Le dedup est **logarithmique**, pas un hard cap : 20 resolves du même user `trusted` pèsent ~6 (vs 20 sans pondération). Un compte motivé qui émet 200 signaux peut atteindre ~10. Si abus observés en ouverture externe : bumper `dedup_k` dans le TOML (ex. 1.0 → 20 spams ≈ 3), ou introduire un `max_per_user_per_repo` hard en v1.2.
- La réputation est calculée **à l'instant du recompute** et non figée au moment du signal. Un user qui monte en tier voit tout son historique re-pondéré. Comportement voulu pour le MVP, à reconsidérer pour un modèle à la `formula_v2`.

## Nettoyage post-test

- 6 signaux supprimés
- User `noob-unproven` supprimé
- 2 lignes `artifact_scores` sur zod supprimées (v1 + v1.1 stale)
- État DB final cohérent : 9 scores v1.1 sur les autres repos, zod sans signaux ni scores (sera régénéré au prochain recompute avec ses seuls priors GitHub)

## Tests automatisés

Couverture unitaire dans `backend/src/services/quality/weighting.rs` :

- `core_user_weighs_more_than_unproven`
- `dedup_caps_spam_from_same_user` (somme < 40 % du raw count)
- `regret_outweighs_resolve_per_signal`
- `re_resolve_buckets_into_resolve_with_higher_weight`
- `unknown_reporter_falls_back_to_low_multiplier` (0.30)
- `explain_mirrors_aggregate_total`

Ainsi que `services::quality::formula::tests::formula_v1_weighting_section_loads` qui vérifie que le TOML est lu correctement.

Total backend : **35/35 tests verts**, `cargo clippy --all-targets -- -D warnings` propre, `cargo fmt --check` propre.
