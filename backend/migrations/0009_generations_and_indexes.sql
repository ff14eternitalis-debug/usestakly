CREATE TABLE IF NOT EXISTS generations (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  target_domain snippet_domain,
  prompt TEXT NOT NULL,
  used_snippets UUID[] NOT NULL DEFAULT '{}',
  output_code TEXT NOT NULL,
  plan JSONB,
  llm_model TEXT NOT NULL,
  tokens_input INT,
  tokens_output INT,
  duration_ms INT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_snippets_owner ON snippets(owner_id);
CREATE INDEX IF NOT EXISTS idx_snippets_library ON snippets(library_id);
CREATE INDEX IF NOT EXISTS idx_snippets_domain_kind ON snippets(domain, kind);
CREATE INDEX IF NOT EXISTS idx_snippets_public ON snippets(visibility) WHERE visibility = 'public';
CREATE INDEX IF NOT EXISTS idx_snippets_slug_trgm ON snippets USING gin (slug gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_libraries_owner ON libraries(owner_id);
CREATE INDEX IF NOT EXISTS idx_libraries_visibility ON libraries(visibility);
CREATE INDEX IF NOT EXISTS idx_libraries_trust ON libraries(trust_level);
CREATE INDEX IF NOT EXISTS idx_libraries_slug_trgm ON libraries USING gin (slug gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_versions_snippet ON snippet_versions(snippet_id);
CREATE INDEX IF NOT EXISTS idx_versions_hash ON snippet_versions(content_hash);

CREATE INDEX IF NOT EXISTS idx_generations_user ON generations(user_id, created_at DESC);
