CREATE TABLE repo_categories (
  external_artifact_id UUID NOT NULL REFERENCES external_artifacts(id) ON DELETE CASCADE,
  category TEXT NOT NULL,
  confidence DOUBLE PRECISION NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
  source TEXT NOT NULL DEFAULT 'github_metadata',
  evidence JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (external_artifact_id, category)
);

CREATE INDEX idx_repo_categories_category
  ON repo_categories(category);

CREATE INDEX idx_repo_categories_artifact_confidence
  ON repo_categories(external_artifact_id, confidence DESC);
