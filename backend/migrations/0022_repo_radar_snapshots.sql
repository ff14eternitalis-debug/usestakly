CREATE TABLE repo_radar_snapshots (
  external_artifact_id UUID PRIMARY KEY REFERENCES external_artifacts(id) ON DELETE CASCADE,
  maturity_band TEXT NOT NULL CHECK (
    maturity_band IN ('established', 'emerging', 'experimental', 'stale', 'noisy')
  ),
  radar_relevance DOUBLE PRECISION NOT NULL CHECK (radar_relevance >= 0.0 AND radar_relevance <= 1.0),
  trend_signal DOUBLE PRECISION NOT NULL CHECK (trend_signal >= 0.0 AND trend_signal <= 1.0),
  explanation JSONB NOT NULL DEFAULT '{}'::jsonb,
  computed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_repo_radar_snapshots_maturity_band
  ON repo_radar_snapshots(maturity_band);

CREATE INDEX idx_repo_radar_snapshots_trend_signal
  ON repo_radar_snapshots(trend_signal DESC);
