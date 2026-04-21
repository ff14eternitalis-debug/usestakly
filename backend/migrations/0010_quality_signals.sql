-- Quality signals foundation.
-- Snippets (internal) and external artifacts (npm/github/crates/...) share the
-- same scoring pipeline via an artifact_kind discriminator.

CREATE TYPE external_source AS ENUM ('npm', 'github', 'crates', 'pypi', 'shadcn');

CREATE TABLE IF NOT EXISTS external_artifacts (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  source external_source NOT NULL,
  canonical_slug TEXT NOT NULL,
  package_name TEXT NOT NULL,
  version TEXT,
  priors JSONB NOT NULL DEFAULT '{}'::jsonb,
  priors_fetched_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(source, canonical_slug)
);

CREATE TRIGGER trg_external_artifacts_updated_at
  BEFORE UPDATE ON external_artifacts
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

CREATE TYPE artifact_kind AS ENUM ('snippet', 'external');

CREATE TYPE signal_kind AS ENUM (
  'resolve',
  'build_success',
  'build_failure',
  'regret',
  're_resolve',
  'works_in_prod',
  'broken',
  'security_issue',
  'deprecated',
  'doesnt_match_claim'
);

CREATE TABLE IF NOT EXISTS quality_signals (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  artifact_kind artifact_kind NOT NULL,
  snippet_id UUID REFERENCES snippets(id) ON DELETE CASCADE,
  external_artifact_id UUID REFERENCES external_artifacts(id) ON DELETE CASCADE,
  signal signal_kind NOT NULL,
  is_passive BOOLEAN NOT NULL,
  actor_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  agent_context JSONB,
  evidence_url TEXT,
  evidence_description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CHECK (
    (artifact_kind = 'snippet'  AND snippet_id IS NOT NULL AND external_artifact_id IS NULL)
    OR
    (artifact_kind = 'external' AND external_artifact_id IS NOT NULL AND snippet_id IS NULL)
  )
);

CREATE INDEX idx_quality_signals_snippet
  ON quality_signals(snippet_id, created_at DESC)
  WHERE snippet_id IS NOT NULL;

CREATE INDEX idx_quality_signals_external
  ON quality_signals(external_artifact_id, created_at DESC)
  WHERE external_artifact_id IS NOT NULL;

CREATE INDEX idx_quality_signals_kind
  ON quality_signals(signal, created_at DESC);

-- Aggregated scores, recomputed by the daily batch job.
-- formula_version is the discriminator so that recomputing with a new formula
-- does not erase audit trail of past MCP responses.
CREATE TABLE IF NOT EXISTS artifact_scores (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  artifact_kind artifact_kind NOT NULL,
  snippet_id UUID REFERENCES snippets(id) ON DELETE CASCADE,
  external_artifact_id UUID REFERENCES external_artifacts(id) ON DELETE CASCADE,
  formula_version TEXT NOT NULL,

  freshness NUMERIC(4,3),
  adoption NUMERIC(4,3),
  reliability NUMERIC(4,3),
  abandonment NUMERIC(4,3),
  overall NUMERIC(4,3),

  resolve_count INT NOT NULL DEFAULT 0,
  build_success_count INT NOT NULL DEFAULT 0,
  build_failure_count INT NOT NULL DEFAULT 0,
  regret_count INT NOT NULL DEFAULT 0,

  flags TEXT[] NOT NULL DEFAULT '{}',

  computed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CHECK (
    (artifact_kind = 'snippet'  AND snippet_id IS NOT NULL AND external_artifact_id IS NULL)
    OR
    (artifact_kind = 'external' AND external_artifact_id IS NOT NULL AND snippet_id IS NULL)
  )
);

CREATE UNIQUE INDEX idx_artifact_scores_snippet_formula
  ON artifact_scores(snippet_id, formula_version)
  WHERE snippet_id IS NOT NULL;

CREATE UNIQUE INDEX idx_artifact_scores_external_formula
  ON artifact_scores(external_artifact_id, formula_version)
  WHERE external_artifact_id IS NOT NULL;

CREATE INDEX idx_external_artifacts_slug
  ON external_artifacts(canonical_slug);
