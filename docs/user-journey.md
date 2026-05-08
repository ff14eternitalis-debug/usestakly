# UseStakly — Parcours utilisateur

> Version : 2.1 — 2026-05-08
> Vue à jour des flows réels du produit. La version pré-pivot (snippets, packs, registry perso) est archivée sous `docs/archive/snippets/user-journey-prepivot.md`.

## Personas

| Persona | Point d'entrée | Premier moment de valeur |
|---|---|---|
| Dev qui fait de la veille GitHub | Landing publique → `/discover` | Voit un repo proposé avec score multi-dimensionnel, comprend pourquoi un repo populaire est rejeté en mode `auto` |
| Dev qui veut suivre des deps critiques | Landing → login OAuth → `/discover` → `/repos/$id` → Watch | Reçoit une notif quand un score chute ou qu'un flag toxique apparaît |
| Agent IA (Codex / Cursor / Claude Code) | `npx usestakly-mcp install` | Obtient `recommend_github_repos` filtré + provenance dans la réponse |
| Maintainer GitHub d'un repo annoté | Notification owner → `/repos/$id` | Peut disputer un signal négatif via OAuth GitHub matching |
| Admin UseStakly | `/account` (gate `ADMIN_API_TOKEN`) | Review pending signals, supervise la file modération, observe MCP metrics |

## Flow 1 — Découverte publique (anonyme)

1. Landing `/` — promesse produit + CTA "Discover repos"
2. `/discover` — recherche lexicale (+ sémantique si activée), filtres `auto`/`strict`/`explore`, filtres avancés (langage, license, stars, freshness)
3. Clic repo → `/repos/$id` — profil complet : dimensions, flags publics, signals timeline, score provenance (`formula_version`, `scored_at`, `source`)
4. Pages annexes accessibles depuis le footer : `/how-to-read` (guide lecture du score), `/privacy` (gestion données), `/status` (santé public beta), `/mcp-guide` (intégration agent)

**État d'audit** : phase 1 (anonyme) close au 2026-04-23. Frictions corrigées (CTA clarifiés, hiérarchie landing, garde auth routes privées).

## Flow 2 — Connexion + watch ton premier repo

1. Depuis `/repos/$id` ou un CTA landing, clic "Sign in"
2. `/login` — OAuth GitHub ou Discord. Le `returnTo` est porté dans le `state` OAuth signé
3. Callback OAuth — backend lit `return_to` sanitizé, set le cookie `usestakly_session`, redirige
4. User retombe sur la page d'origine (pas la landing)
5. Sur `/repos/$id` connecté → bouton "Watch" — ajout immédiat à la watchlist
6. `/watchlist` — liste des repos watchés, mute, remove avec confirmation explicite
7. `/account` — configure les canaux optionnels : email de destination, Discord webhook chiffré, test webhook, résumé quotidien et créneau local
8. Si le score d'un watché bouge significativement (abandonment +0.20, score overall ↓ ≥0.10, nouveau flag `security_issue`/`broken`) → notification in-app + alerte Discord si le canal est activé
9. Si le résumé quotidien est activé sur Discord, le scheduler envoie au maximum un digest court par jour et par canal, seulement s'il y a eu des changements importants
10. `/notifications` — centre de notifs, mark-read on click vers le repo, retry sur erreur 401/network

**État d'audit** : phase 2 (connecté) en cours, corrections principales livrées (return_to signé, confirm remove, error states queries, mark-read on click, canaux sortants configurables).

## Flow 3 — Agent IA via MCP

1. User génère un token MCP depuis `/account` (`POST /api/agent-tokens`) — plaintext affiché une seule fois, format `usk_<64 hex>`
2. User exécute `npx usestakly-mcp install` — le CLI demande le token + endpoint, écrit la config Codex/Cursor avec `Authorization: Bearer usk_...`
3. Agent appelle `tools/list` — middleware MCP vérifie le Bearer (rejette si absent/invalide depuis 2026-04-26)
4. Agent appelle `recommend_github_repos` ou `search_github_repos` avec filtres → réponse scorée + provenance dans chaque entrée
5. Agent peut creuser via `get_repo_quality_context` pour le profil complet
6. Après usage, agent peut envoyer `log_usage(repo, outcome)` :
   - garde-fous : quota par token, cooldown anti-doublon, fenêtre négatifs, réputation user min
   - retour : score recalculé pour feedback agent immédiat
   - refus loggés en `agent_token_events` (`kind='mcp_guard_rejection'`)
7. Agent peut suivre via `watch_repo` — ajoute à la watchlist du user propriétaire du token

## Flow 4 — Signal actif modéré

1. User connecté avec réputation > seuil ouvre `/repos/$id` → "Report a signal"
2. Choisit type (`deprecated`, `broken`, `doesnt_match_claim`, `security_issue`…) + evidence obligatoire
3. Backend (`POST /api/repos/:id/signals`) :
   - réputation reporter < seuil sur signal sévère → `pending` review admin
   - sinon contribue au consensus N users distincts
4. Pour `security_issue` : toujours `pending` jusqu'à review admin, pas d'exposition publique en attendant
5. Owner du repo (matching OAuth GitHub direct, membre public d'org, membre privé via PAT, collaborateur/maintainer via API) peut **disputer** depuis le repo
6. Timeline transparente persistée (`quality_signal_events`) : submitted, reviewed, accepted, disputed
7. Admin gère la file depuis `/account` (panel admin gate token) : accept / reject / annoter

## Flow 5 — Admin / observabilité

Routes admin gated par `ADMIN_API_TOKEN` :

- `POST /api/admin/scoring/recompute` — recompute manuel
- `GET /api/admin/scoring/explain/{repo_id}` — breakdown signal par signal (formula v1.1)
- `POST /api/admin/embeddings/backfill` — backfill embeddings repos existants (si `semantic-search`)
- `GET /api/admin/mcp/metrics?window=24h|7d|30d` — totaux MCP, distribution outcomes, breakdown refus, top repos, top users, daily volume
- `GET /api/admin/signals/queue` — file pending/disputed

UI : panel `AdminMcpObservabilityPanel` dans `/account` derrière le gate.

## Principes UX tenus

- **Pas de réseau social** : pas de profils publics, pas de followers, pas de karma vanity. Que des signaux d'usage et de modération.
- **Evidence obligatoire** : pas de thumbs-down nu, tout signal négatif demande une evidence parsable.
- **Provenance partout** : score, formule, timestamp visibles côté UI et MCP.
- **Honnêteté du beta** : page `/status` publique, page `/privacy` qui dit ce qu'on collecte, score affiché comme "indicateur transparent" pas "certification".
- **Garde auth claire** : routes privées redirigent vers `/login` avec `returnTo`, pas de mur silencieux.
- **i18n EN/FR** dès le frontend.

## Frictions encore non couvertes (post-audit phase 2)

- Mobile / responsive dédié pas encore validé
- États vides connectés sur watchlist / notifications / compte sans token à finaliser
- Onboarding connecté complet (login OAuth → discover → repo detail → watch → notif → account/tokens) pas encore testé en bout de chaîne
- Erreurs UI sur échec `POST /api/repos/add`, session expirée, refus auth — partiellement couvertes
- Page compte plus riche (historique contributions, règles d'alerte perso, settings) à faire
- UX d'explication scoring sur discovery (barres de dimensions, "pourquoi ce repo / pourquoi exclu") à faire
- Graph historique score + timeline signaux sur profil repo à faire

Voir `docs/audits/user-journey-audit-phase2-2026-04-24.md` pour le détail.
