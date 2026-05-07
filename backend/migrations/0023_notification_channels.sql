CREATE TABLE IF NOT EXISTS notification_channels (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  channel_type TEXT NOT NULL CHECK (channel_type IN ('email', 'discord_webhook')),
  label TEXT NOT NULL,
  destination TEXT NOT NULL,
  secret_ciphertext TEXT,
  enabled BOOLEAN NOT NULL DEFAULT TRUE,
  critical_alerts_enabled BOOLEAN NOT NULL DEFAULT TRUE,
  daily_digest_enabled BOOLEAN NOT NULL DEFAULT FALSE,
  last_tested_at TIMESTAMPTZ,
  last_error TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (user_id, channel_type)
);

CREATE INDEX IF NOT EXISTS idx_notification_channels_user
  ON notification_channels(user_id, created_at DESC);
