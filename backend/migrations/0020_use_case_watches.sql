-- Saved intent-based watches.
-- MVP scope: persist the user's need and current matching repos.
-- Notifications are handled in a later lot.

CREATE TABLE IF NOT EXISTS use_case_queries (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  query_text TEXT NOT NULL,
  normalized_intent TEXT NOT NULL,
  categories TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
  topics TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
  languages TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
  risk_tolerance TEXT NOT NULL DEFAULT 'medium',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_use_case_queries_user
  ON use_case_queries(user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS use_case_watches (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  use_case_query_id UUID NOT NULL REFERENCES use_case_queries(id) ON DELETE CASCADE,
  label TEXT NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  last_checked_at TIMESTAMPTZ,
  last_notified_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_use_case_watches_user
  ON use_case_watches(user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS use_case_watch_matches (
  use_case_watch_id UUID NOT NULL REFERENCES use_case_watches(id) ON DELETE CASCADE,
  external_artifact_id UUID NOT NULL REFERENCES external_artifacts(id) ON DELETE CASCADE,
  match_score NUMERIC(4,3) NOT NULL,
  quality_score NUMERIC(4,3),
  last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (use_case_watch_id, external_artifact_id)
);

CREATE INDEX IF NOT EXISTS idx_use_case_watch_matches_artifact
  ON use_case_watch_matches(external_artifact_id);
