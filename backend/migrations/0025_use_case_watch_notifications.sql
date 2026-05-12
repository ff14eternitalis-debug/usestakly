ALTER TYPE notification_kind ADD VALUE IF NOT EXISTS 'use_case_new_candidate';
ALTER TYPE notification_kind ADD VALUE IF NOT EXISTS 'use_case_best_candidate_changed';
ALTER TYPE notification_kind ADD VALUE IF NOT EXISTS 'use_case_quality_drop';
ALTER TYPE notification_kind ADD VALUE IF NOT EXISTS 'use_case_flag_added';

ALTER TABLE use_case_watch_matches
  ADD COLUMN IF NOT EXISTS flags TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[];

CREATE INDEX IF NOT EXISTS idx_use_case_watches_enabled
  ON use_case_watches(last_checked_at ASC NULLS FIRST)
  WHERE enabled = TRUE;
