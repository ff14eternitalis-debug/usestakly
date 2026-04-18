DO $$ BEGIN
  CREATE TYPE visibility AS ENUM ('private', 'public');
EXCEPTION
  WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
  CREATE TYPE trust_level AS ENUM (
    'private',
    'public_unverified',
    'verified_author',
    'community_trusted',
    'flagged',
    'quarantined'
  );
EXCEPTION
  WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS libraries (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  slug TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT,
  visibility visibility NOT NULL DEFAULT 'private',
  trust_level trust_level NOT NULL DEFAULT 'private',
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  default_stack JSONB NOT NULL DEFAULT '{}',
  allowed_domains JSONB NOT NULL DEFAULT '[]',
  metadata JSONB NOT NULL DEFAULT '{}',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

