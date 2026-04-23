-- Moderation workflow for active repo signals.

ALTER TABLE quality_signals
  ADD COLUMN IF NOT EXISTS review_status TEXT NOT NULL DEFAULT 'accepted',
  ADD COLUMN IF NOT EXISTS reviewed_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS reviewed_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS review_note TEXT,
  ADD COLUMN IF NOT EXISTS disputed_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS disputed_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS dispute_reason TEXT;

CREATE INDEX IF NOT EXISTS idx_quality_signals_review_status
  ON quality_signals(review_status, created_at DESC);
