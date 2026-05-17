CREATE TABLE IF NOT EXISTS repo_refresh_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  artifact_id UUID NOT NULL REFERENCES external_artifacts(id) ON DELETE CASCADE,
  status TEXT NOT NULL,
  reason TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_repo_refresh_events_user_created
  ON repo_refresh_events(user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_repo_refresh_events_artifact_created
  ON repo_refresh_events(artifact_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_repo_refresh_events_artifact_completed
  ON repo_refresh_events(artifact_id, created_at DESC)
  WHERE status = 'completed';
