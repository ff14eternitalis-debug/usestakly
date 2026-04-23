ALTER TABLE external_artifacts
  ADD COLUMN embedding vector(384),
  ADD COLUMN embedding_updated_at TIMESTAMPTZ;

CREATE INDEX idx_external_artifacts_embedding_hnsw
  ON external_artifacts
  USING hnsw (embedding vector_cosine_ops)
  WHERE embedding IS NOT NULL;
