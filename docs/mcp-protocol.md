# UseStakly — Protocole MCP

> Version : 2.4 — 2026-05-17 (repo context display truth layer)
> Implémentation : `backend/src/mcp/` (handlers dans `server.rs`, DTOs/mappers dans `tools/*`, `get_repo_quality_context` enrichi avec `proof_tier`, `dimension_states`, `ingestion_status`).
> Transport : Streamable HTTP via `rmcp` 1.5.
> L'ancienne v1 orientée snippets est retirée du produit vivant.

---

## Rôle

UseStakly expose un serveur MCP pour interroger une registry scorée de repos GitHub publics OSS.

Les agents peuvent :

1. chercher des repos scorés (`search_github_repos`)
2. recevoir une recommandation haut niveau filtrée + provenance (`recommend_github_repos`)
3. récupérer un contexte qualité détaillé pour un repo précis (`get_repo_quality_context`)
4. enregistrer un signal passif d'usage réel (`log_usage`, retourne le score recalculé)
5. ajouter un repo à la watchlist du user propriétaire du token (`watch_repo`)
6. créer une veille d'intention/radar sur un besoin (`watch_use_case`)

Le but est de remplacer une sélection basée sur les stars par une sélection basée sur :

- freshness
- adoption
- reliability
- abandonment
- vitality
- flags actifs

`quality_overall` reste le score formula v2. La couche display expose aussi la vérité par dimension : corpus GitHub observable (`freshness`, `vitality`, CI/releases/cadence) versus communauté UseStakly décisionnelle (`adoption`, `reliability` via `log_usage` et signaux pondérés).

---

## Transport

- Transport : **Streamable HTTP**
- Endpoint : `/mcp`
- Auth : `Authorization: Bearer usk_<token>` — **obligatoire dès `initialize` et `tools/list`** depuis 2026-04-26 (middleware pré-transport, voir `docs/mcp-endpoint-security.md`). Une requête sans Bearer reçoit `401` avant que `rmcp` ne traite la session.
- Sessions : `LocalSessionManager`
- Installation côté agent : `npx usestakly-mcp install` (voir `docs/mcp-cli-release.md`)

---

## Authentification

Les tokens MCP sont stockés dans `agent_tokens` (migration `0013_agent_tokens.sql`).

Format :

- `usk_<64 hex>`
- hash SHA-256 en base
- plaintext affiché une seule fois à la création

Endpoints REST de gestion :

| Méthode | Route | Usage |
|---|---|---|
| `POST` | `/api/agent-tokens` | créer un token |
| `GET` | `/api/agent-tokens` | lister les tokens actifs |
| `DELETE` | `/api/agent-tokens/{id}` | révoquer un token |

---

## Tools exposés

### `search_github_repos`

Cherche des repos GitHub scorés.

Entrée :

```json
{
  "query": "date picker react",
  "filter": "auto",
  "language": "TypeScript",
  "stars_min": 100,
  "limit": 20
}
```

Retour :

- candidats classés par `overall`, puis stars, puis récence
- provenance `usestakly://registry/github`
- résumé radar si disponible, y compris mention `corpus_backed` quand l'explication radar contient cette raison
- limite connue : les résultats search n'exposent pas `dimension_states` / `proof_tier` / `ingestion_status`; appeler `get_repo_quality_context` pour le profil complet

### `recommend_github_repos`

Tool haut niveau pour les agents : combine search + filter + provenance dans une seule réponse pensée pour la consommation directe par un agent.

Entrée :

```json
{
  "need": "react state manager",
  "ecosystem": "TypeScript",
  "risk_tolerance": "medium",
  "limit": 5
}
```

Retour :

- `recommendations` : liste courte compatible avec les anciennes intégrations
- `stable_picks` : candidats établis ou non émergents
- `emerging_picks` : candidats `emerging` / `experimental`, à traiter comme radar
- `fallback_candidates` : repos candidats à ajouter si le corpus ne contient pas encore assez de résultats
- pour chaque candidat : score multidimensionnel, radar maturity, raison de la recommandation, caveats, next actions, provenance
- pensé pour qu'un agent puisse en citer 1–3 directement sans appel supplémentaire

Si aucun candidat ne passe les filtres, le tool retourne une liste vide avec une explication structurée.

### `get_repo_quality_context`

Retourne le profil qualité complet pour un repo.

Entrée :

```json
{
  "owner": "JedWatson",
  "name": "react-datepicker"
}
```

Retour :

- dimensions de score
- `proof_tier` : `corpus_only`, `usage_limited` ou `community_backed` (label UI/MCP, pas un nouveau score)
- `dimension_states` : tableau JSON des cinq dimensions avec `key`, `value`, `displayState`, `source`, `confidence`, `asOf`, `summary`
- `ingestion_status` : JSON avec `priorsFetchedAt`, `structuralSignalsAt`, `structuralStale`, `structuralComplete`, `partialFields`
- `vitality_inputs` : signaux structurels GitHub bruts utilisés par la dimension vitality
- counts d'usage (`quality_resolve_count`, `quality_build_success_count`, `quality_build_failure_count`, `quality_regret_count`)
- flags
- signaux récents
- provenance `usestakly://registry/github/<owner>/<name>`

Si le repo n'est pas ingéré, le tool renvoie une erreur.

`ingestion_status` ne contient pas `lastIngestError` aujourd'hui. Les agents ne doivent pas l'inventer.

### `log_usage`

Enregistre un signal passif après usage réel du repo par l'agent.

Entrée :

```json
{
  "owner": "JedWatson",
  "name": "react-datepicker",
  "outcome": "build_success",
  "notes": "Installed cleanly and passed smoke test"
}
```

Outcomes autorisés :

- `resolve`
- `build_success`
- `build_failure`
- `regret`
- `re_resolve`

Ce tool :

- attache le signal au user du token MCP
- stocke un `agent_context`
- déclenche un recompute global du scoring juste après l'enregistrement
- retourne le score et les counts recalculés pour confirmer l'effet du signal

Si le repo n'est pas encore ingéré, le serveur tente une ingestion à la volée si `GITHUB_TOKEN` est configuré.

Retour utile après écriture :

```json
{
  "signal": "build_success",
  "quality_overall": 0.72,
  "quality_adoption": 0.18,
  "quality_reliability": 0.8,
  "quality_abandonment": 0.11,
  "quality_resolve_count": 2,
  "quality_build_success_count": 4,
  "quality_build_failure_count": 1,
  "quality_regret_count": 0,
  "provenance": {
    "source": "usestakly://registry/github/vitejs/vite",
    "formula_version": "v2.0",
    "scored_at": "2026-04-25T14:30:00Z"
  }
}
```

### `watch_repo`

Ajoute un repo à la watchlist du user propriétaire du token MCP.

Entrée :

```json
{
  "owner": "JedWatson",
  "name": "react-datepicker"
}
```

Retour :

- `artifact_id`
- `watching: true`
- provenance du repo

Si le repo n'est pas encore ingéré, le serveur tente une ingestion à la volée si `GITHUB_TOKEN` est configuré.

### `watch_use_case`

Crée une veille sur un besoin naturel, pas seulement sur un repo.

Entrée :

```json
{
  "need": "testing tools for TypeScript",
  "label": "Veille Testing TypeScript",
  "risk_tolerance": "medium"
}
```

Retour :

- `watch_id`
- intention normalisée, catégories, topics, languages
- `initial_matches` + `top_matches`
- provenance `usestakly://watch/use-case`

Ce tool est utile quand l'agent doit surveiller un espace de besoin dans le temps : observability, auth, testing, UI kits, ORM, agent frameworks, etc.

---

## Provenance

Chaque tool retourne une provenance structurée :

```json
{
  "source": "usestakly://registry/github[/owner/name]",
  "formula_version": "v2.0",
  "scored_at": "2026-04-23T10:00:00Z"
}
```

Convention recommandée côté agent :

```ts
// Evalué via UseStakly: github.com/JedWatson/react-datepicker, formula_v2
```

---

## Flux type

```text
recommend_github_repos              # recommandation directe (cas usage agent)
  -> log_usage                       # feedback après usage réel
  -> watch_repo (si suivi souhaité)

search_github_repos                 # exploration plus large
  -> get_repo_quality_context        # zoom sur un candidat
  -> génération avec provenance
  -> log_usage
  -> watch_repo (si suivi souhaité)
```

`log_usage` retourne le score recalculé immédiatement pour permettre à l'agent d'observer l'effet de son signal sans appel supplémentaire.

---

## Sécurité et garde-fous

Déjà en place :

- token Bearer obligatoire
- vérification hashée contre `agent_tokens`
- `last_used_at` bumpé automatiquement
- `log_usage` limité à des outcomes passifs autorisés

Hardening déjà en place :

- quota write par token (`APP_MCP_WRITE_LIMIT_PER_HOUR`)
- limite read/transport par token valide (`APP_MCP_READ_LIMIT_PER_MINUTE`)
- limite auth failures par IP (`APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`)
- anti-doublon sur `log_usage` pour un même repo/outcome/token (`APP_MCP_LOG_USAGE_COOLDOWN_SECS`)
- fenêtre de refroidissement sur outcomes négatifs répétés (`APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS`)
- réputation v2 et consensus multi-users avant exposition des signaux publics toxiques
- page compte `/account` pour créer, lister et révoquer les tokens MCP

---

## Test local

```bash
# 1. Créer un token MCP via session web
curl -X POST http://localhost:4000/api/agent-tokens \
  -H "Content-Type: application/json" \
  --cookie "usestakly_session=<...>" \
  -d '{"label":"dev-test"}'

# 2. Initialiser le client MCP
curl -X POST http://localhost:4000/mcp \
  -H "Authorization: Bearer usk_<token>" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"curl","version":"0"}}}'
```
