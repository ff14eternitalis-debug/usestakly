# UseStakly — Protocole MCP

> Version : 2.1 — 2026-04-23 (post-pivot veille GitHub)
> Implémentation : `backend/src/mcp/` (R5a + R5b partiel livrés), transport Streamable HTTP via `rmcp` 1.5.
> L'ancienne v1 orientée snippets est retirée du produit vivant.

---

## Rôle

UseStakly expose un serveur MCP pour interroger une registry scorée de repos GitHub publics OSS.

Les agents peuvent :

1. chercher des repos scorés
2. récupérer un contexte qualité détaillé
3. enregistrer un signal passif d'usage réel
4. ajouter un repo à la watchlist du user propriétaire du token

Le but est de remplacer une sélection basée sur les stars par une sélection basée sur :

- freshness
- adoption
- reliability
- abandonment
- flags actifs

---

## Transport

- Transport : **Streamable HTTP**
- Endpoint : `/mcp`
- Auth : `Authorization: Bearer usk_<token>`
- Sessions : `LocalSessionManager`

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
- flags
- signaux récents
- provenance `usestakly://registry/github/<owner>/<name>`

Si le repo n'est pas ingéré, le tool renvoie une erreur.

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

Si le repo n'est pas encore ingéré, le serveur tente une ingestion à la volée si `GITHUB_TOKEN` est configuré.

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

---

## Provenance

Chaque tool retourne une provenance structurée :

```json
{
  "source": "usestakly://registry/github[/owner/name]",
  "formula_version": "v1",
  "scored_at": "2026-04-23T10:00:00Z"
}
```

Convention recommandée côté agent :

```ts
// Evalué via UseStakly: github.com/JedWatson/react-datepicker, formula_v1
```

---

## Flux type

```text
search_github_repos
  -> get_repo_quality_context
  -> génération avec provenance
  -> log_usage
  -> watch_repo (si suivi souhaité)
```

---

## Sécurité et garde-fous

Déjà en place :

- token Bearer obligatoire
- vérification hashée contre `agent_tokens`
- `last_used_at` bumpé automatiquement
- `log_usage` limité à des outcomes passifs autorisés

Reste à faire :

- réputation utilisateur explicite / pondération des signaux
- consensus multi-users avant d'exposer des signaux plus toxiques

Hardening déjà en place :

- quota write par token (`APP_MCP_WRITE_LIMIT_PER_HOUR`)
- anti-doublon sur `log_usage` pour un même repo/outcome/token (`APP_MCP_LOG_USAGE_COOLDOWN_SECS`)
- fenêtre de refroidissement sur outcomes négatifs répétés (`APP_MCP_NEGATIVE_SIGNAL_WINDOW_HOURS`)
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
