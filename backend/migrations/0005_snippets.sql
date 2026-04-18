CREATE OR REPLACE FUNCTION touch_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS snippets (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  library_id UUID NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
  owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  slug TEXT NOT NULL,
  domain snippet_domain NOT NULL,
  kind TEXT NOT NULL,
  category TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  language TEXT NOT NULL,
  runtime TEXT,
  framework TEXT,
  framework_version TEXT,
  visibility visibility NOT NULL DEFAULT 'private',
  trust_level trust_level NOT NULL DEFAULT 'private',
  license TEXT NOT NULL DEFAULT 'MIT',
  current_version_id UUID,
  rule_set_id UUID,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(library_id, slug),
  FOREIGN KEY (domain, kind) REFERENCES snippet_kinds(domain, kind)
);

CREATE TRIGGER trg_snippets_updated_at
  BEFORE UPDATE ON snippets
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

CREATE TRIGGER trg_users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

CREATE TRIGGER trg_libraries_updated_at
  BEFORE UPDATE ON libraries
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
