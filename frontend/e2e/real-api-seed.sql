INSERT INTO users (
  id,
  email,
  username,
  display_name,
  avatar_url
) VALUES (
  '00000000-0000-0000-0000-000000000001',
  'dev@usestakly.local',
  'usestakly-dev',
  'UseStakly Dev',
  NULL
) ON CONFLICT (id) DO UPDATE SET
  email = EXCLUDED.email,
  username = EXCLUDED.username,
  display_name = EXCLUDED.display_name,
  avatar_url = EXCLUDED.avatar_url,
  updated_at = NOW();

DELETE FROM notifications
WHERE user_id = '00000000-0000-0000-0000-000000000001'
  AND external_artifact_id IN (
    '11111111-1111-4111-8111-111111111111',
    '22222222-2222-4222-8222-222222222222'
  );

DELETE FROM watched_artifacts
WHERE user_id = '00000000-0000-0000-0000-000000000001'
  AND external_artifact_id IN (
    '11111111-1111-4111-8111-111111111111',
    '22222222-2222-4222-8222-222222222222'
  );

DELETE FROM use_case_watch_matches
WHERE use_case_watch_id IN (
  SELECT id
  FROM use_case_watches
  WHERE user_id = '00000000-0000-0000-0000-000000000001'
    AND use_case_query_id IN (
      SELECT id
      FROM use_case_queries
      WHERE query_text = 'date picker react timezone'
    )
);

DELETE FROM use_case_watches
WHERE user_id = '00000000-0000-0000-0000-000000000001'
  AND use_case_query_id IN (
    SELECT id
    FROM use_case_queries
    WHERE query_text = 'date picker react timezone'
  );

DELETE FROM agent_tokens
WHERE user_id = '00000000-0000-0000-0000-000000000001'
  AND label = 'real api audit';

INSERT INTO external_artifacts (
  id,
  source,
  canonical_slug,
  package_name,
  github_id,
  github_owner,
  github_repo,
  default_branch,
  html_url,
  description,
  language,
  license_spdx,
  topics,
  archived,
  stars_count,
  forks_count,
  open_issues_count,
  subscribers_count,
  last_commit_at,
  priors_fetched_at,
  distinct_contributors_90d,
  commits_30d,
  has_ci,
  releases_count,
  last_release_at,
  structural_signals_at
) VALUES
(
  '11111111-1111-4111-8111-111111111111',
  'github',
  'github:react-dates/timezone-picker',
  'react-dates/timezone-picker',
  900001,
  'react-dates',
  'timezone-picker',
  'main',
  'https://github.com/react-dates/timezone-picker',
  'Accessible React date picker with timezone-aware parsing.',
  'TypeScript',
  'MIT',
  ARRAY['react','datepicker','timezone','typescript'],
  FALSE,
  1840,
  120,
  8,
  94,
  NOW() - INTERVAL '5 days',
  NOW(),
  8,
  18,
  TRUE,
  12,
  NOW() - INTERVAL '20 days',
  NOW()
),
(
  '22222222-2222-4222-8222-222222222222',
  'github',
  'github:legacy-ui/old-datepicker',
  'legacy-ui/old-datepicker',
  900002,
  'legacy-ui',
  'old-datepicker',
  'main',
  'https://github.com/legacy-ui/old-datepicker',
  'Old React date picker without active maintenance.',
  'JavaScript',
  'MIT',
  ARRAY['react','datepicker'],
  FALSE,
  520,
  80,
  42,
  30,
  NOW() - INTERVAL '420 days',
  NOW(),
  1,
  0,
  FALSE,
  2,
  NOW() - INTERVAL '500 days',
  NOW()
)
ON CONFLICT (source, canonical_slug) DO UPDATE SET
  package_name = EXCLUDED.package_name,
  github_id = EXCLUDED.github_id,
  github_owner = EXCLUDED.github_owner,
  github_repo = EXCLUDED.github_repo,
  default_branch = EXCLUDED.default_branch,
  html_url = EXCLUDED.html_url,
  description = EXCLUDED.description,
  language = EXCLUDED.language,
  license_spdx = EXCLUDED.license_spdx,
  topics = EXCLUDED.topics,
  archived = EXCLUDED.archived,
  stars_count = EXCLUDED.stars_count,
  forks_count = EXCLUDED.forks_count,
  open_issues_count = EXCLUDED.open_issues_count,
  subscribers_count = EXCLUDED.subscribers_count,
  last_commit_at = EXCLUDED.last_commit_at,
  priors_fetched_at = EXCLUDED.priors_fetched_at,
  distinct_contributors_90d = EXCLUDED.distinct_contributors_90d,
  commits_30d = EXCLUDED.commits_30d,
  has_ci = EXCLUDED.has_ci,
  releases_count = EXCLUDED.releases_count,
  last_release_at = EXCLUDED.last_release_at,
  structural_signals_at = EXCLUDED.structural_signals_at;

INSERT INTO artifact_scores (
  artifact_kind,
  external_artifact_id,
  formula_version,
  freshness,
  adoption,
  reliability,
  abandonment,
  vitality,
  overall,
  resolve_count,
  build_success_count,
  build_failure_count,
  regret_count,
  flags,
  computed_at
) VALUES
(
  'external',
  '11111111-1111-4111-8111-111111111111',
  'v2.0',
  0.910,
  0.740,
  0.820,
  0.110,
  0.690,
  0.840,
  7,
  4,
  0,
  0,
  ARRAY[]::text[],
  NOW()
),
(
  'external',
  '22222222-2222-4222-8222-222222222222',
  'v2.0',
  0.120,
  0.200,
  0.300,
  0.720,
  0.100,
  0.210,
  1,
  0,
  3,
  2,
  ARRAY['deprecated']::text[],
  NOW()
)
ON CONFLICT (external_artifact_id, formula_version)
WHERE external_artifact_id IS NOT NULL
DO UPDATE SET
  freshness = EXCLUDED.freshness,
  adoption = EXCLUDED.adoption,
  reliability = EXCLUDED.reliability,
  abandonment = EXCLUDED.abandonment,
  vitality = EXCLUDED.vitality,
  overall = EXCLUDED.overall,
  resolve_count = EXCLUDED.resolve_count,
  build_success_count = EXCLUDED.build_success_count,
  build_failure_count = EXCLUDED.build_failure_count,
  regret_count = EXCLUDED.regret_count,
  flags = EXCLUDED.flags,
  computed_at = NOW();

INSERT INTO repo_categories (
  external_artifact_id,
  category,
  confidence,
  source,
  evidence
) VALUES
(
  '11111111-1111-4111-8111-111111111111',
  'date-picker',
  0.86,
  'local_real_e2e_seed',
  '{"topics":["datepicker","timezone"]}'::jsonb
),
(
  '22222222-2222-4222-8222-222222222222',
  'date-picker',
  0.70,
  'local_real_e2e_seed',
  '{"topics":["datepicker"]}'::jsonb
)
ON CONFLICT (external_artifact_id, category) DO UPDATE SET
  confidence = EXCLUDED.confidence,
  source = EXCLUDED.source,
  evidence = EXCLUDED.evidence,
  updated_at = NOW();

INSERT INTO repo_radar_snapshots (
  external_artifact_id,
  maturity_band,
  radar_relevance,
  trend_signal,
  explanation
) VALUES
(
  '11111111-1111-4111-8111-111111111111',
  'established',
  0.64,
  0.28,
  '{"matched":["recent_commit","healthy_score"]}'::jsonb
),
(
  '22222222-2222-4222-8222-222222222222',
  'stale',
  0.20,
  0.05,
  '{"matched":["stale_commit","high_abandonment"]}'::jsonb
)
ON CONFLICT (external_artifact_id) DO UPDATE SET
  maturity_band = EXCLUDED.maturity_band,
  radar_relevance = EXCLUDED.radar_relevance,
  trend_signal = EXCLUDED.trend_signal,
  explanation = EXCLUDED.explanation,
  computed_at = NOW();

INSERT INTO quality_signals (
  id,
  artifact_kind,
  external_artifact_id,
  signal,
  is_passive,
  evidence_description,
  review_status,
  created_at
) VALUES (
  '44444444-4444-4444-8444-444444444444',
  'external',
  '11111111-1111-4111-8111-111111111111',
  'build_success',
  TRUE,
  'Local real E2E seed: installed and smoke-tested.',
  'accepted',
  NOW() - INTERVAL '1 hour'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO notifications (
  id,
  user_id,
  external_artifact_id,
  kind,
  payload
) VALUES (
  '33333333-3333-4333-8333-333333333333',
  '00000000-0000-0000-0000-000000000001',
  '11111111-1111-4111-8111-111111111111',
  'score_drop',
  '{"prev_overall":0.94,"new_overall":0.84}'::jsonb
);
