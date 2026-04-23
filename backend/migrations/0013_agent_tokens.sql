-- R5a: agent tokens for MCP Bearer auth.
-- Un token = un user. Stocké en hash SHA-256 hex, plaintext jamais persisté.
-- Le label aide l'user à distinguer ses tokens ("claude-desktop", "cursor", etc.).

CREATE TABLE IF NOT EXISTS agent_tokens (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  label TEXT NOT NULL,
  token_hash TEXT NOT NULL UNIQUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_used_at TIMESTAMPTZ,
  revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_agent_tokens_user
  ON agent_tokens(user_id, created_at DESC);

-- Lookup rapide pour l'auth MCP (hot path).
CREATE INDEX idx_agent_tokens_active
  ON agent_tokens(token_hash)
  WHERE revoked_at IS NULL;
