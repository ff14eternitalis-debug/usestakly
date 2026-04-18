DO $$ BEGIN
  CREATE TYPE snippet_domain AS ENUM ('frontend', 'backend', 'devops', 'data', 'shared');
EXCEPTION
  WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS snippet_kinds (
  domain snippet_domain NOT NULL,
  kind TEXT NOT NULL,
  description TEXT,
  PRIMARY KEY (domain, kind)
);

INSERT INTO snippet_kinds (domain, kind, description) VALUES
  ('frontend', 'atom', 'Small reusable UI primitive'),
  ('frontend', 'molecule', 'Composite UI component'),
  ('frontend', 'organism', 'Feature-sized UI composition'),
  ('frontend', 'template', 'Layout or template block'),
  ('frontend', 'util', 'Frontend utility helper'),
  ('backend', 'function', 'Reusable backend function'),
  ('backend', 'handler', 'HTTP or RPC handler'),
  ('backend', 'middleware', 'Middleware component'),
  ('backend', 'model', 'Domain or persistence model'),
  ('backend', 'service', 'Business logic service'),
  ('backend', 'query', 'Database-oriented query helper'),
  ('devops', 'config', 'Configuration asset'),
  ('devops', 'script', 'Automation or shell script'),
  ('devops', 'dockerfile', 'Container build recipe'),
  ('data', 'query', 'Reusable data query'),
  ('data', 'migration', 'Database migration block'),
  ('shared', 'type', 'Shared type or interface'),
  ('shared', 'constant', 'Shared constant or enum'),
  ('shared', 'util', 'Shared utility function')
ON CONFLICT (domain, kind) DO NOTHING;
