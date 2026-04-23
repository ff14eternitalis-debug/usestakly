# Projet K — Modèle de Données

> Version : 1.0 — 2026-04-15 *(pré-pivot 2026-04-20)*
> Fichier de référence : migrations `sqlx` dans `backend/migrations/`

> ### ⚠ Bandeau de reconciliation — pivot 2026-04-20
>
> Le schéma décrit ici (migrations 0001–0009) est **implémenté et valide**. Post-pivot, il est **étendu** par une nouvelle migration `0010_quality_signals.sql` (voir TODO.md Phase 6). Ce document ne couvre pas encore cette extension.
>
> **À ajouter dans une V2 de ce document** :
> - Table `quality_signals(snippet_id, signal_type, value, evidence_url, reporter_id, weight, created_at)` pour signaux actifs.
> - Colonnes sur `snippets` : `resolve_count`, `build_success_rate`, `regret_rate`, `freshness_score`, `abandonment_score`, `flags TEXT[]`, `stack_match_cache JSONB`, `quality_score_current NUMERIC`, `quality_score_updated_at`.
> - Table `resolve_events(id, snippet_id, agent_context, client_hash, stack_signature, outcome, outcome_at, created_at)` pour télémétrie passive (signal `resolve_count`, `build_success_rate`, `regret_rate`).
> - Table `owner_reputation(owner_id, score, evidence_count, last_recomputed_at)` pour pondération anti-gaming.

## 🎯 Principes

1. **Append-only** pour les snippets (versions immuables → reproductibilité)
2. **UUID partout** (pas de bigint auto-increment) → prêt pour la distribution
3. **JSONB pour l'évolutif** (règles, metadata, stack) → pas de migration à chaque tweak
4. **pgvector** pour la recherche sémantique dans la même DB
5. **Bibliothèques adressables** comme primitive produit principale
6. **Provider-agnostic** pour auth et subscriptions (prêt pour OAuth, wallets, etc.)
7. **Trust & Safety** intégré au modèle pour les bibliothèques publiques
8. *(post-pivot)* **Signaux d'usage capturés dès le jour 1** — irréversible, la donnée historique ne se reconstruit pas.

## 📦 Extensions requises

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";  -- recherche texte floue
```

## 👥 Identité & comptes

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    bio TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE auth_identities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,              -- 'email', 'google', 'github', futur: 'wallet'
    provider_user_id TEXT NOT NULL,
    credentials JSONB,                   -- hash Argon2, tokens, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

CREATE TABLE api_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,     -- SHA256 du token réel
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 📚 Bibliothèques

Les snippets appartiennent à des **bibliothèques** qui constituent l'unité d'adressage principale du produit.

```sql
CREATE TYPE visibility AS ENUM ('private', 'public', 'unlisted');
CREATE TYPE trust_level AS ENUM (
    'private',
    'public_unverified',
    'verified_author',
    'community_trusted',
    'flagged',
    'quarantined'
);

CREATE TABLE libraries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    slug TEXT NOT NULL UNIQUE,          -- ex: '@alice/react-ui-kit'
    name TEXT NOT NULL,
    description TEXT,

    visibility visibility NOT NULL DEFAULT 'private',
    trust_level trust_level NOT NULL DEFAULT 'private',
    is_default BOOLEAN NOT NULL DEFAULT FALSE,

    default_stack JSONB NOT NULL DEFAULT '{}',
    allowed_domains JSONB NOT NULL DEFAULT '[]',
    metadata JSONB NOT NULL DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Règles minimales :
- une bibliothèque peut être privée, publique ou `unlisted`
- le `slug` est l'identifiant humain utilisé dans l'UI et les prompts
- une bibliothèque `flagged` ou `quarantined` ne doit pas être consommée en mode auto

## 📚 Snippets (cœur)

```sql
CREATE TYPE snippet_domain AS ENUM ('frontend', 'backend', 'devops', 'data', 'shared');

CREATE TABLE snippet_kinds (
    domain snippet_domain NOT NULL,
    kind TEXT NOT NULL,
    description TEXT,
    PRIMARY KEY (domain, kind)
);

CREATE TABLE snippets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    library_id UUID NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    slug TEXT NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    domain snippet_domain NOT NULL,
    kind TEXT NOT NULL,
    category TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,

    language TEXT NOT NULL,
    runtime TEXT,
    framework TEXT,
    framework_version TEXT,

    visibility visibility NOT NULL DEFAULT 'private',
    trust_level trust_level NOT NULL DEFAULT 'private',
    license TEXT NOT NULL DEFAULT 'MIT',
    current_version_id UUID,
    rule_set_id UUID,                    -- FK ajoutée plus bas

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(library_id, slug),
    FOREIGN KEY (domain, kind) REFERENCES snippet_kinds(domain, kind)
);

CREATE TABLE snippet_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    version TEXT NOT NULL,               -- semver '1.2.0'
    code TEXT NOT NULL,
    variables JSONB NOT NULL DEFAULT '[]',
    css_classes TEXT[],                  -- pour frontend uniquement
    dependencies JSONB DEFAULT '[]',     -- [{"snippet_id": "...", "version": "^1.0"}]
    exports JSONB NOT NULL DEFAULT '[]',
    imports JSONB NOT NULL DEFAULT '[]',
    compatibility JSONB NOT NULL DEFAULT '{}',
    metadata JSONB NOT NULL DEFAULT '{}',
    content_hash TEXT NOT NULL,          -- SHA256(code)
    embedding vector(384),               -- fastembed bge-small = 384 dims
    risk_level TEXT NOT NULL DEFAULT 'safe',  -- safe | review_required | restricted
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(snippet_id, version)
);

ALTER TABLE snippets
    ADD CONSTRAINT fk_current_version
    FOREIGN KEY (current_version_id) REFERENCES snippet_versions(id);
```

### Références canoniques

Le système doit permettre la résolution explicite :

```text
@alice/react-ui-kit:frontend-atom-action-button-primary
@alice/react-ui-kit:frontend-atom-action-button-primary@1.2.0
```

Le backend résout ensuite vers `libraries.id`, `snippets.id` et `snippet_versions.id`.

## 🏷️ Tags

```sql
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE snippet_tags (
    snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (snippet_id, tag_id)
);
```

## 📜 RULES (contraintes)

```sql
CREATE TABLE rule_sets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID REFERENCES users(id) ON DELETE CASCADE,
    library_id UUID REFERENCES libraries(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    rules JSONB NOT NULL,                -- cf docs/rules-system.md
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE snippets
    ADD CONSTRAINT fk_rule_set
    FOREIGN KEY (rule_set_id) REFERENCES rule_sets(id);
```

## 🏗️ Projets

```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    stack JSONB NOT NULL,                -- cf exemples docs/architecture.md
    rule_set_id UUID REFERENCES rule_sets(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE project_snippets (
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    snippet_version_id UUID NOT NULL REFERENCES snippet_versions(id),
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (project_id, snippet_version_id)
);
```

## 🧭 Résolution, permissions et modération

```sql
CREATE TABLE library_permissions (
    library_id UUID NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    can_read BOOLEAN NOT NULL DEFAULT TRUE,
    can_resolve BOOLEAN NOT NULL DEFAULT TRUE,
    can_search BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (library_id, user_id)
);

CREATE TABLE snippet_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    reported_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    details JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Ces tables servent à :
- filtrer les bibliothèques visibles par scope
- préparer la modération publique
- exclure certains contenus des modes d'assemblage automatiques

## 🤖 Générations IA

```sql
CREATE TABLE generations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    project_id UUID REFERENCES projects(id) ON DELETE SET NULL,
    target_domain snippet_domain,
    prompt TEXT NOT NULL,
    used_snippets UUID[] NOT NULL,
    output_code TEXT NOT NULL,
    plan JSONB,                          -- plan d'assemblage
    llm_model TEXT NOT NULL,
    tokens_input INT,
    tokens_output INT,
    duration_ms INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 💳 Abonnements

```sql
CREATE TYPE subscription_tier AS ENUM ('free', 'premium');
CREATE TYPE subscription_status AS ENUM ('active', 'canceled', 'past_due');

CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tier subscription_tier NOT NULL DEFAULT 'free',
    status subscription_status NOT NULL DEFAULT 'active',
    provider TEXT NOT NULL,              -- 'stripe', 'paypal', futur: 'crypto'
    provider_subscription_id TEXT,
    current_period_end TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 🌟 Communauté

```sql
CREATE TABLE snippet_stars (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, snippet_id)
);

CREATE TABLE snippet_forks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    original_version_id UUID NOT NULL REFERENCES snippet_versions(id),
    fork_snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    forked_by UUID NOT NULL REFERENCES users(id),
    forked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 📇 Index

```sql
CREATE INDEX idx_snippets_owner ON snippets(owner_id);
CREATE INDEX idx_snippets_library ON snippets(library_id);
CREATE INDEX idx_snippets_domain_kind ON snippets(domain, kind);
CREATE INDEX idx_snippets_public ON snippets(visibility) WHERE visibility = 'public';
CREATE INDEX idx_snippets_slug_trgm ON snippets USING gin (slug gin_trgm_ops);

CREATE INDEX idx_libraries_owner ON libraries(owner_id);
CREATE INDEX idx_libraries_visibility ON libraries(visibility);
CREATE INDEX idx_libraries_trust ON libraries(trust_level);
CREATE INDEX idx_libraries_slug_trgm ON libraries USING gin (slug gin_trgm_ops);

CREATE INDEX idx_versions_snippet ON snippet_versions(snippet_id);
CREATE INDEX idx_versions_hash ON snippet_versions(content_hash);
CREATE INDEX idx_versions_embedding ON snippet_versions
    USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);

CREATE INDEX idx_generations_user ON generations(user_id, created_at DESC);
CREATE INDEX idx_stars_snippet ON snippet_stars(snippet_id);
```

## 🔄 Triggers utiles

```sql
CREATE OR REPLACE FUNCTION touch_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_snippets_updated_at BEFORE UPDATE ON snippets
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER trg_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER trg_libraries_updated_at BEFORE UPDATE ON libraries
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
-- idem sur projects, rule_sets, subscriptions
```

## 🔐 Sécurité applicative

- **Row-level security** activable en phase 2 (`ENABLE ROW LEVEL SECURITY`)
- Le backend filtre systématiquement par `owner_id`, `library_id`, `visibility` et `trust_level`
- Pas de SQL dynamique → `sqlx` force les requêtes paramétrées
- Le mode `auto` du MCP exclut les contenus `flagged` ou `quarantined`

## 📊 Vues utiles

```sql
-- Snippets avec leur version courante jointe
CREATE VIEW v_snippets_current AS
SELECT s.*, l.slug AS library_slug, l.visibility AS library_visibility,
       l.trust_level AS library_trust_level,
       sv.code, sv.version, sv.content_hash, sv.embedding
FROM snippets s
JOIN libraries l ON s.library_id = l.id
JOIN snippet_versions sv ON s.current_version_id = sv.id;

-- Statistiques par utilisateur
CREATE VIEW v_user_stats AS
SELECT
    u.id,
    u.username,
    COUNT(DISTINCT s.id) AS snippets_count,
    COUNT(DISTINCT g.id) AS generations_count,
    MAX(s.updated_at) AS last_snippet_at
FROM users u
LEFT JOIN snippets s ON s.owner_id = u.id
LEFT JOIN generations g ON g.user_id = u.id
GROUP BY u.id, u.username;
```
