CREATE TABLE IF NOT EXISTS snippet_versions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
  version TEXT NOT NULL,
  code TEXT NOT NULL,
  variables JSONB NOT NULL DEFAULT '[]',
  css_classes TEXT[],
  dependencies JSONB NOT NULL DEFAULT '[]',
  exports JSONB NOT NULL DEFAULT '[]',
  imports JSONB NOT NULL DEFAULT '[]',
  compatibility JSONB NOT NULL DEFAULT '{}',
  metadata JSONB NOT NULL DEFAULT '{}',
  content_hash TEXT NOT NULL,
  embedding vector(384),
  risk_level TEXT NOT NULL DEFAULT 'safe',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(snippet_id, version)
);

DO $$ BEGIN
  ALTER TABLE snippets
    ADD CONSTRAINT fk_current_version
    FOREIGN KEY (current_version_id) REFERENCES snippet_versions(id);
EXCEPTION
  WHEN duplicate_object THEN NULL;
END $$;
