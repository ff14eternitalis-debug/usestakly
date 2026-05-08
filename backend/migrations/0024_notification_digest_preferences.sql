ALTER TABLE users
  ADD COLUMN IF NOT EXISTS digest_time_local TEXT NOT NULL DEFAULT '08:00',
  ADD COLUMN IF NOT EXISTS timezone TEXT NOT NULL DEFAULT 'UTC';

CREATE TABLE IF NOT EXISTS notification_digest_deliveries (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  notification_channel_id UUID NOT NULL REFERENCES notification_channels(id) ON DELETE CASCADE,
  digest_date DATE NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('pending', 'delivered', 'skipped_empty', 'failed')),
  error TEXT,
  delivered_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (notification_channel_id, digest_date)
);

CREATE INDEX IF NOT EXISTS idx_notification_digest_deliveries_user
  ON notification_digest_deliveries(user_id, digest_date DESC);
