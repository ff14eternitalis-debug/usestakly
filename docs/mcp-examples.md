# MCP usage examples

UseStakly MCP is meant to help coding agents choose dependencies with visible provenance instead of guessing from stars alone.

## Recommend a dependency

Ask:

```text
Find a reliable React table library with UseStakly. Explain the score, caveats, and provenance, then log_usage after the test.
```

Expected flow:

1. `recommend_github_repos` returns a shortlist with reasons and caveats.
2. `get_repo_quality_context` inspects the best candidate.
3. The agent explains freshness, reliability, abandonment, and provenance.
4. After trying the dependency, the agent calls `log_usage`.

## Compare options

Ask:

```text
I need a TypeScript ORM. Recommend GitHub repos with UseStakly and compare reliability, freshness, and abandonment risk.
```

Expected flow:

1. `recommend_github_repos` searches the registry.
2. The agent compares top candidates by score dimensions, not only popularity.
3. The agent calls `get_repo_quality_context` for finalists.

## Monitor a dependency

Ask:

```text
Before adding this dependency, use UseStakly to inspect the repo detail and add it to my watchlist if it looks healthy.
```

Expected flow:

1. `get_repo_quality_context` checks the repo.
2. The agent explains the risk.
3. `watch_repo` adds it to the user's UseStakly watchlist.

## Record usage feedback

Ask:

```text
I tested this package and the build passed. Log that outcome in UseStakly.
```

Expected flow:

1. `log_usage` records `build_success`.
2. UseStakly recomputes score provenance.
3. Future recommendations gain stronger reliability data.

## Stick to established choices

Ask:

```text
Find a reliable testing library for TypeScript with UseStakly. I want established choices first, not emerging picks.
```

Expected flow:

1. `recommend_github_repos` is called with the testing intent.
2. The agent filters or prioritises results whose `radar.maturity_band = "established"`.
3. Emerging or experimental candidates are mentioned only as caveats, not first picks.

## Scout emerging alternatives

Ask:

```text
Find emerging alternatives to Prisma in the TypeScript ORM space. I accept higher risk for newer projects with strong vitality.
```

Expected flow:

1. `recommend_github_repos` returns the established baseline (Prisma) plus emerging contenders.
2. The agent surfaces candidates whose `radar.maturity_band = "emerging"` and explains the vitality signals (contributors 90d, commits 30d, releases, CI).
3. The response makes the tradeoff explicit: emerging means promising, not production-proven.

## Watch a need over time

Ask:

```text
Watch new OSS tools for observability. Add the established options to my watchlist and tell me when an emerging contender shows up.
```

Expected flow:

1. `recommend_github_repos` returns observability candidates split by maturity.
2. `watch_repo` is called for the established picks the user wants to track.
3. The agent reminds the user that a use-case watch (web `/watchlist` Besoins section) will surface future emerging contenders even when the agent is offline.
