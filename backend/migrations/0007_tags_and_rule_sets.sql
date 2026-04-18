CREATE TABLE IF NOT EXISTS tags (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS snippet_tags (
  snippet_id UUID NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
  tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (snippet_id, tag_id)
);

CREATE TABLE IF NOT EXISTS rule_sets (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_id UUID REFERENCES users(id) ON DELETE CASCADE,
  library_id UUID REFERENCES libraries(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  rules JSONB NOT NULL DEFAULT '{}',
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trg_rule_sets_updated_at
  BEFORE UPDATE ON rule_sets
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

DO $$ BEGIN
  ALTER TABLE snippets
    ADD CONSTRAINT fk_rule_set
    FOREIGN KEY (rule_set_id) REFERENCES rule_sets(id);
EXCEPTION
  WHEN duplicate_object THEN NULL;
END $$;
