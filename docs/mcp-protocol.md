# UseStakly — Protocole MCP

> Version : 2.0 — 2026-04-23 (post-pivot veille GitHub)
> Implémentation : `backend/src/mcp/` (R5a livré), transport Streamable HTTP via `rmcp` 1.5.
> L'ancienne v1 (snippet-oriented) est remplacée — les concepts transverses qui y figuraient (scopes privés, modes d'assemblage, références canoniques `@owner/lib:...`) ne s'appliquent plus.

---

## 🎯 Rôle

UseStakly expose un serveur MCP permettant aux agents IA d'interroger la **registry scorée de repos GitHub publics**. Les agents :

1. **cherchent** un repo pertinent pour une tâche (ex: "date picker React timezone-aware")
2. **récupèrent un contexte qualité complet** avant de recommander ce repo
3. (R5b à venir) **loguent l'usage** pour alimenter les signaux passifs, ajoutent un repo à la watchlist de l'utilisateur

Le but : remplacer la sélection basée sur les stars GitHub par une sélection basée sur le score multi-dimensionnel (freshness, adoption, reliability, abandonment) et les flags actifs.

---

## 🛜 Transport

- **Streamable HTTP** (spec MCP courante, SSE déprécié, stdio hors scope pour une app hébergée)
- Endpoint : `POST /mcp` (ou `GET` pour stream) sur l'app backend — même origine que le REST
- Sessions gérées par `LocalSessionManager` côté serveur (in-memory)
- SDK serveur : `rmcp = "1.5"` avec features `server, macros, transport-streamable-http-server`

Côté client MCP (Claude Desktop, Cursor, custom agent), configurer l'URL :

```
https://<host>/mcp
```

avec header `Authorization: Bearer usk_<token>`.

---

## 🔐 Authentification

**Bearer token** stocké dans la table `agent_tokens` (migration 0013).

Format du token : `usk_<64 hex>` (256 bits d'entropie via deux UUIDv4 concaténés), stocké en SHA-256 hex dans `token_hash`. Le plaintext n'est affiché qu'**une seule fois** à la création.

### Endpoints REST de gestion (session utilisateur requise)

| Méthode | Route | Corps | Retour |
|---|---|---|---|
| `POST` | `/api/agent-tokens` | `{ "label": "cursor-mac" }` | `201` + `{ id, label, token, created_at }` **(plaintext unique ici)** |
| `GET` | `/api/agent-tokens` | — | Liste des tokens actifs (sans plaintext) |
| `DELETE` | `/api/agent-tokens/{id}` | — | `204` (soft-delete via `revoked_at`) |

Lookup MCP : `token_hash` indexé avec `WHERE revoked_at IS NULL`. Bump `last_used_at` fire-and-forget (ne bloque pas l'auth).

### Flux typique

```bash
# 1. L'user génère un token via curl (UI compte-user à faire en R6)
curl -X POST http://localhost:4000/api/agent-tokens \
  -H "Content-Type: application/json" \
  --cookie "usestakly_session=..." \
  -d '{"label":"claude-desktop"}'
# → { "id":"...", "token":"usk_a1b2...", ... }

# 2. L'user colle le token dans la config de son client MCP
# 3. Le client appelle POST /mcp avec Authorization: Bearer usk_a1b2...
```

---

## 🛠️ Tools exposés (R5a)

### `search_github_repos`

Cherche dans la registry scorée. Ranking : `quality.overall` DESC, puis stars, puis last_commit.

**Paramètres :**

```json
{
  "query": "date picker react",
  "filter": "auto",           // auto (default) | strict | explore
  "language": "TypeScript",   // optionnel, ILIKE
  "stars_min": 100,           // optionnel
  "limit": 20                 // default 20, max 50
}
```

**Filtres qualité (définis dans `scoring/formula_v1.toml`) :**

| Filter | Seuils |
|---|---|
| `auto` (défaut) | `reliability ≥ 0.9` **ET** `abandonment ≤ 0.3` **ET** pas de flag `security-issue` ni `broken` |
| `strict` | `reliability ≥ 0.95` **ET** `abandonment ≤ 0.2` **ET** `overall ≥ 0.85` **ET** zéro flag |
| `explore` | aucun filtre qualité (utile pour debug / exploration) |

**Retour :**

```json
{
  "provenance": {
    "source": "usestakly://registry/github",
    "formula_version": "v1",
    "scored_at": "2026-04-23T10:00:00Z"
  },
  "filter_used": "auto",
  "count": 12,
  "results": [
    {
      "owner": "JedWatson",
      "name": "react-datepicker",
      "full_name": "JedWatson/react-datepicker",
      "html_url": "https://github.com/...",
      "description": "...",
      "language": "TypeScript",
      "license_spdx": "MIT",
      "topics": ["react", "datepicker"],
      "stars_count": 8000,
      "archived": false,
      "last_commit_at": "2026-04-10T...",
      "quality_overall": 0.89,
      "quality_reliability": 0.95,
      "quality_abandonment": 0.18,
      "flags": []
    }
  ]
}
```

### `get_repo_quality_context`

Profil qualité complet pour UN repo. À appeler après `search_github_repos` pour justifier le pick.

**Paramètres :**

```json
{ "owner": "JedWatson", "name": "react-datepicker" }
```

**Retour :**

```json
{
  "provenance": {
    "source": "usestakly://registry/github/JedWatson/react-datepicker",
    "formula_version": "v1",
    "scored_at": "..."
  },
  "owner": "...",
  "name": "...",
  "full_name": "JedWatson/react-datepicker",
  "html_url": "...",
  "description": "...",
  "language": "TypeScript",
  "topics": [...],
  "stars_count": 8000,
  "forks_count": 2000,
  "open_issues_count": 150,
  "subscribers_count": 80,
  "archived": false,
  "last_commit_at": "...",
  "default_branch": "main",
  "quality_overall": 0.89,
  "quality_freshness": 0.92,
  "quality_adoption": 0.85,
  "quality_reliability": 0.95,
  "quality_abandonment": 0.18,
  "flags": [],
  "recent_signals": [
    {
      "signal": "build_success",
      "is_passive": true,
      "evidence_url": null,
      "created_at": "..."
    }
  ]
}
```

Retourne `InvalidParams` si le repo n'est pas ingéré (jamais vu via `/api/admin/ingest/github` ou `/api/repos/add`).

---

## 🧬 Provenance (contrat)

Chaque réponse tool embarque un objet `provenance` :

```json
{
  "source": "usestakly://registry/github[/<owner>/<name>]",
  "formula_version": "v1",
  "scored_at": "2026-04-23T10:00:00Z"
}
```

**Convention agent** : dans le code généré, inclure un commentaire de provenance :

```ts
// Evalué via UseStakly: github.com/JedWatson/react-datepicker, score 0.89, formula_v1, 2026-04-23
```

L'agent est libre de la placer où il juge pertinent (tête de fichier, au-dessus d'un import). Le serveur n'impose pas le format exact côté code — il impose juste que la provenance soit disponible dans l'output.

---

## 🧭 Flux type

```
Agent IA
   │
   ▼
tool: search_github_repos(query="date picker react", filter="auto")
   │
   ▼  (< 20 candidats scorés)
tool: get_repo_quality_context(owner, name)  ← pour le pick retenu
   │
   ▼  (profil + signaux)
Agent génère le code avec commentaire de provenance
```

(R5b ajoutera `log_usage` à la fin de ce flux pour alimenter `build_success_rate` et `regret_rate` passifs.)

---

## 🛡️ Sécurité

| Contrôle | Lieu |
|---|---|
| Token valide, non révoqué | `mcp::auth::verify_bearer` au début de chaque tool |
| Hash SHA-256 comparaison | `agent_tokens::verify` (time constant via unique index) |
| Rate-limiting | **à faire R5b** quand il y aura plus d'usage |
| Poisoning-resistance sur `log_usage` | **à faire R5b** (seuil réputation, consensus) |

Aucune donnée privée n'est exposée : la registry est 100 % publique. Le token sert à l'audit (tracer qui a requêté quoi) et à la future rate-limit par user, pas à un quelconque scoping d'accès.

---

## 🚫 Hors périmètre R5a

- `log_usage` — signaux passifs (R5b)
- `watch_repo` — écriture sur la watchlist user (R5b)
- Rate-limit par token (R5b)
- UI de gestion des tokens (R6 page compte user — en attendant, gestion via curl)
- OAuth device flow (écarté : Bearer simple suffit pour MVP)

---

## 🧪 Test local

```bash
# 1. Session web : login OAuth GitHub via frontend, récupérer le cookie
# 2. Créer un token
curl -X POST http://localhost:4000/api/agent-tokens \
  -H "Content-Type: application/json" \
  --cookie "usestakly_session=<...>" \
  -d '{"label":"dev-test"}'

# 3. Tester le MCP directement en JSON-RPC (stateless init + tools/list)
curl -X POST http://localhost:4000/mcp \
  -H "Authorization: Bearer usk_<token>" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"curl","version":"0"}}}'

# 4. Ou brancher un vrai client MCP (Claude Desktop, MCP Inspector) sur
#    http://localhost:4000/mcp avec header Authorization: Bearer usk_...
```
