-- Persist GitHub conditional-request and maintainer-inactivity metadata.
-- Existing external_artifacts.etag remains reserved for repo metadata ETag.

ALTER TABLE external_artifacts
  ADD COLUMN github_releases_etag TEXT,
  ADD COLUMN github_readme_etag TEXT,
  ADD COLUMN github_events_etag TEXT,
  ADD COLUMN github_rate_limit_reset_at TIMESTAMPTZ,
  ADD COLUMN github_last_rate_limit_at TIMESTAMPTZ,
  ADD COLUMN owner_last_activity_at TIMESTAMPTZ,
  ADD COLUMN owner_inactive_days INT;

CREATE INDEX idx_external_artifacts_owner_inactive_days
  ON external_artifacts(owner_inactive_days DESC NULLS LAST)
  WHERE owner_inactive_days IS NOT NULL;
