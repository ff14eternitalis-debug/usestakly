-- R5b hardening: track MCP write events per token to enforce quotas and cooldowns.

CREATE TABLE IF NOT EXISTS agent_token_events (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  token_id UUID NOT NULL REFERENCES agent_tokens(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  kind TEXT NOT NULL,
  repo_owner TEXT,
  repo_name TEXT,
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_agent_token_events_token_created
  ON agent_token_events(token_id, created_at DESC);

CREATE INDEX idx_agent_token_events_token_kind_repo_created
  ON agent_token_events(token_id, kind, repo_owner, repo_name, created_at DESC);

CREATE INDEX idx_agent_token_events_user_kind_repo_created
  ON agent_token_events(user_id, kind, repo_owner, repo_name, created_at DESC);
