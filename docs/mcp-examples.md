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
