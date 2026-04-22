# Audit de sécurité — pipeline quality signals

> Date : 2026-04-21
> Portée : commit `4e16c0a` — « feat: add quality scoring pipeline with resolve and filtered search »
> Branche : `main`
> Auditeur : Claude (Opus 4.7), méthodologie `/security-review`

## Résumé

**Verdict : aucune vulnérabilité HIGH ou MEDIUM de confiance ≥ 0.8.**

Le pipeline qualité-scoré (Phase 6) et les endpoints `resolve`/`search` (Phase 7) sont mergeable en l'état. Les contrôles d'autorisation, les requêtes paramétrées et la gestion du token admin respectent les pratiques attendues.

## Portée de l'audit

### Fichiers analysés

**Migration DB**
- `backend/migrations/0010_quality_signals.sql`

**Domaine / types**
- `backend/src/domain/quality.rs`
- `backend/src/domain/reference.rs`

**Services**
- `backend/src/services/quality/capture.rs`
- `backend/src/services/quality/scoring.rs`
- `backend/src/services/resolution.rs`
- `backend/src/services/search.rs`

**Handlers**
- `backend/src/handlers/admin.rs`
- `backend/src/handlers/signals.rs`
- `backend/src/handlers/resolve.rs`
- `backend/src/handlers/search.rs`

**Wiring**
- `backend/src/app/mod.rs` (nouvelles routes)
- `backend/src/config/mod.rs` (`ADMIN_API_TOKEN`)
- `backend/src/app/error.rs` (`ApiError::internal`)

### Hors scope (exclusions standard)

- DOS / rate limiting / épuisement de ressources
- Secrets au repos (gérés par le playbook dédié)
- Memory safety (Rust, impossible)
- Log spoofing
- SSRF qui ne contrôle que le path
- Regex injection / regex DOS
- Code pré-existant non modifié par le commit

## Contrôles effectués

### SQL injection — OK

Toutes les requêtes SQLx utilisent des bindings `$N` paramétrés. Aucun `format!`, `concat!` ou interpolation dans les strings SQL.

**Cas notables** :
- `services/search.rs` : le pattern ILIKE est construit avec `'%' || $3 || '%'` côté SQL, param lié — safe.
- `services/quality/capture.rs` : casts d'enum via `CAST($1 AS signal_kind)` forcent le typage côté Postgres.
- `services/quality/scoring.rs` : agrégations avec `COUNT(*) FILTER (WHERE qs.signal = 'resolve')` — littéraux SQL, pas de contamination possible.

### Autorisation resolve / search — OK

Les endpoints publics appliquent un prédicat d'autorisation correct :

```sql
WHERE s.visibility = 'public' OR s.owner_id = $user_id
```

Quand l'utilisateur n'est pas authentifié, `$user_id` est `NULL` — la seconde branche du `OR` échoue systématiquement (NULL != NULL), donc seules les lignes `visibility='public'` remontent. Pas de fuite de snippets privés via utilisateur anonyme.

### Admin token — OK

`backend/src/handlers/admin.rs` :
- Comparaison constant-time via XOR masqué par la longueur (pas de timing leak).
- Token vérifié **avant** tout accès DB (pas d'oracle par temps de réponse).
- Token absent et token incorrect renvoient tous deux `403` avec le même corps — pas de différenciation.
- `ADMIN_API_TOKEN` chargé depuis l'env, jamais loggé.

### Endpoint signals — OK

`backend/src/handlers/signals.rs` :
- `actor_user_id` fixé côté serveur depuis la session — non contrôlable par le client.
- Signaux passifs (`resolve`, `build_success`, `build_failure`, `regret`, `re_resolve`) rejetés en REST : ne peuvent venir que du MCP (Phase 8). Empêche un user de truquer sa reliability en spammant `build_success`.
- Seuls les snippets `visibility='public'` acceptent des signaux — pas de leak de snippet privé via erreur 404/403 distincte.
- `evidence_url` validé par `validator::url` avant insertion.

### JSONB agent_context — OK

Stocké comme `serde_json::Value`. Postgres parse en JSONB : pas de vecteur d'injection SQL via le contenu JSON. Contenu arbitraire accepté par design (agent peut y mettre son stack, sa version, son prompt).

### Formula TOML — OK

`backend/scoring/formula_v1.toml` chargé via `include_str!` à la compilation. Pas user-controllable au runtime. La stratégie « formule publique versionnée » est un choix produit assumé (`docs/strategy-quality-scored-registry.md`).

### IDOR via UUID — OK

Les path params `snippet_id` sont des UUIDs v4, donc inguessables. Les contrôles de visibilité restent en place de toute façon.

### Secrets hardcodés — Aucun

Zéro secret en dur introduit par le commit. `ADMIN_API_TOKEN` est optionnel (absent → endpoint 403 systématique).

## Observations non-bloquantes

### Trust model public — by design

N'importe quel utilisateur authentifié peut signaler un snippet public avec un flag toxique (`broken`, `security-issue`, `deprecated`). C'est **intentionnel** — c'est la mécanique de réputation du registry. L'anti-gaming est prévu en formula_v2 (pondération réputation owner, consensus N users, appel auteur). Cette phase est listée en tâche ouverte dans `TODO.md` — Phase 6, dernière ligne.

Surveillance recommandée :
- Logs des flags toxiques créés (volume, auteurs).
- Alerte si un auteur unique émet plus de N flags toxiques sur une fenêtre de 24 h.

### Dev user fallback

Quand `APP_SESSION_SECRET` ou les credentials OAuth sont absents, `resolve_current_user` retombe sur le dev user injecté via env. **Comportement pré-existant**, pas introduit par ce commit. À ne pas activer en production — vérification possible au boot via une feature flag dédiée.

## Recommandations de suivi

1. **Phase 6 (reste)** : implémenter la politique de flags toxiques (evidence + consensus + appel auteur) avant ouverture publique du registry.
2. **Phase 8 (MCP)** : les signaux passifs arrivent par MCP — s'assurer que le MCP authentifie l'agent (token dédié, distinct de la session web) pour éviter qu'un client web détourné puisse spammer `build_success`.
3. **Déploiement Coolify** : vérifier que `ADMIN_API_TOKEN` est défini avec ≥ 32 octets aléatoires et rotationné selon le playbook `docs/security-secrets-playbook.md`.
4. **Monitoring** : loguer les appels `/api/admin/scoring/recompute` (auteur IP, durée) — l'endpoint est long et coûteux.

## Checklist pré-production

- [ ] `ADMIN_API_TOKEN` rotationné et stocké dans le secret manager Coolify
- [ ] `APP_SESSION_SECRET` défini (désactive le fallback dev user)
- [ ] `DEV_USER_*` non définis en prod
- [ ] CORS `FRONTEND_BASE_URL` pointe sur le domaine prod uniquement
- [ ] Politique flags toxiques implémentée avant ouverture du registry public
- [ ] Logs d'audit sur les endpoints admin activés

## Références

- Commit audité : `4e16c0a`
- Stratégie produit : `docs/strategy-quality-scored-registry.md`
- Playbook secrets : `docs/security-secrets-playbook.md`
- Plan MVP : `TODO.md` (Phase 6, Phase 9)
