-- GitHub-specific columns on external_artifacts.
-- Core priors and metadata are promoted to top-level columns so they are
-- indexable and queryable (filter by language, full-text search on description,
-- freshness via last_commit_at). Source-specific extras stay in `priors` JSONB.

ALTER TABLE external_artifacts
  ADD COLUMN github_id BIGINT,
  ADD COLUMN github_owner TEXT,
  ADD COLUMN github_repo TEXT,
  ADD COLUMN default_branch TEXT,
  ADD COLUMN html_url TEXT,
  ADD COLUMN description TEXT,
  ADD COLUMN language TEXT,
  ADD COLUMN license_spdx TEXT,
  ADD COLUMN topics TEXT[] NOT NULL DEFAULT '{}',
  ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE,
  ADD COLUMN stars_count INT NOT NULL DEFAULT 0,
  ADD COLUMN forks_count INT NOT NULL DEFAULT 0,
  ADD COLUMN open_issues_count INT NOT NULL DEFAULT 0,
  ADD COLUMN subscribers_count INT NOT NULL DEFAULT 0,
  ADD COLUMN last_commit_at TIMESTAMPTZ,
  ADD COLUMN etag TEXT;

-- GitHub id is stable across repo renames; use it as the de-dup key for source='github'.
CREATE UNIQUE INDEX idx_external_artifacts_github_id
  ON external_artifacts(github_id)
  WHERE github_id IS NOT NULL;

-- Filters on discovery.
CREATE INDEX idx_external_artifacts_language
  ON external_artifacts(language)
  WHERE language IS NOT NULL;

CREATE INDEX idx_external_artifacts_archived
  ON external_artifacts(archived);

-- Lexical search on description (trigram for fast ILIKE %q%).
CREATE INDEX idx_external_artifacts_description_trgm
  ON external_artifacts USING GIN (description gin_trgm_ops);

-- Topic containment (search by tag, e.g. topics @> ARRAY['react']).
CREATE INDEX idx_external_artifacts_topics
  ON external_artifacts USING GIN (topics);

-- Source-specific integrity: a github artifact must carry github_id once fetched.
-- Soft check (not enforced at row insert time) because we may insert a placeholder
-- row before the first fetch completes. Enforcement is at the service level.
