-- Watchlists & notifications for R3.
-- MVP simplification:
--   - one implicit watchlist per user (pas de watchlists nommées)
--   - triggers fixes en code (pas de règles custom per-user)
--   - in-app only (pas de canal email/webhook)

CREATE TABLE IF NOT EXISTS watched_artifacts (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  external_artifact_id UUID NOT NULL REFERENCES external_artifacts(id) ON DELETE CASCADE,
  muted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(user_id, external_artifact_id)
);

CREATE INDEX idx_watched_artifacts_user
  ON watched_artifacts(user_id, created_at DESC);

-- Pour le worker de diff : résoudre vite "qui watche ce repo et n'a pas muté ?"
CREATE INDEX idx_watched_artifacts_active_by_artifact
  ON watched_artifacts(external_artifact_id)
  WHERE muted = FALSE;

CREATE TYPE notification_kind AS ENUM (
  'score_drop',
  'abandonment_up',
  'flag_added',
  'flag_severe'
);

CREATE TABLE IF NOT EXISTS notifications (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  external_artifact_id UUID NOT NULL REFERENCES external_artifacts(id) ON DELETE CASCADE,
  kind notification_kind NOT NULL,
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  read_at TIMESTAMPTZ
);

CREATE INDEX idx_notifications_user_unread
  ON notifications(user_id, created_at DESC)
  WHERE read_at IS NULL;

CREATE INDEX idx_notifications_user_all
  ON notifications(user_id, created_at DESC);
