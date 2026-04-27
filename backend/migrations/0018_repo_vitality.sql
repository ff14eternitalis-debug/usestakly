-- Structural vitality signals captured at ingestion time.
-- Anti-slop layer that does NOT depend on declarative MCP signals (resolves, regrets).
-- Used by formula v2 to discriminate fresh-but-solo-vibe-coded repos from collective live ones.
--
-- Sources :
--   - distinct_contributors_90d / commits_30d : GET /repos/{o}/{r}/commits?since=
--   - has_ci                                  : GET /repos/{o}/{r}/contents/.github/workflows (404 = false)
--   - releases_count / last_release_at        : GET /repos/{o}/{r}/releases?per_page=100
--
-- All columns are nullable : a fetch failure (rate-limit, network) must not block ingestion ;
-- the formula treats NULL as "unknown / neutral" rather than "zero".

ALTER TABLE external_artifacts
  ADD COLUMN distinct_contributors_90d INT,
  ADD COLUMN commits_30d INT,
  ADD COLUMN has_ci BOOLEAN,
  ADD COLUMN releases_count INT,
  ADD COLUMN last_release_at TIMESTAMPTZ,
  ADD COLUMN structural_signals_at TIMESTAMPTZ;

-- Discovery filters / formula inputs benefit from cheap lookups on these columns.
-- Partial index on has_ci to keep it small (one bool, sparse usage).
CREATE INDEX idx_external_artifacts_has_ci
  ON external_artifacts(has_ci)
  WHERE has_ci IS NOT NULL;

-- last_release_at is used by both freshness-derived heuristics and the abandonment dimension.
CREATE INDEX idx_external_artifacts_last_release_at
  ON external_artifacts(last_release_at DESC NULLS LAST)
  WHERE last_release_at IS NOT NULL;
