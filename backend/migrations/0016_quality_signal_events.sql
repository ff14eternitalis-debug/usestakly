-- Audit trail for active signal lifecycle.

CREATE TABLE IF NOT EXISTS quality_signal_events (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  quality_signal_id UUID NOT NULL REFERENCES quality_signals(id) ON DELETE CASCADE,
  event_kind TEXT NOT NULL,
  actor_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_quality_signal_events_signal_created
  ON quality_signal_events(quality_signal_id, created_at DESC);
