# UseStakly Live Release Report - 2026-05-16

## Target

- Frontend: local mocked E2E, local real E2E, production-equivalent MCP through Codex MCP integration
- API: local backend through `frontend/scripts/run-real-e2e.mjs`
- MCP: installed UseStakly MCP connector in Codex
- Commit: `5d05445` plus working-tree validation fix for profile menu accessibility

## Automated Checks

| Check | Result | Notes |
| --- | --- | --- |
| backend fmt/clippy/test | PASS | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`; 134 tests passed |
| frontend build | PASS | `npm run build`; Vite still warns that `assets/index-*.js` is about 529 kB |
| frontend mocked E2E | PASS | `npm run test:e2e`; 4 passed, 1 real-api test skipped by default |
| cli tests | PASS | `npm test`; 6 tests passed |
| local real E2E | PASS | First run exposed an obsolete profile-link selector; after adding an accessible profile button label and updating the test, `npm run test:e2e:real` passed |
| MCP smoke read | PASS | Codex MCP `search_github_repos("vite")` and `get_repo_quality_context(vitejs/vite)` returned provenance `usestakly://registry/github`, formula `v2.0` |
| MCP smoke write | SKIPPED | Production write was not run because no explicit approval was given to persist a real `log_usage` signal |

## Manual Checks

| Flow | Result | Notes |
| --- | --- | --- |
| OAuth GitHub | SKIPPED | Requires an interactive live browser session against the deployed OAuth app |
| Discover -> repo detail | PASS | Covered by mocked E2E and local real E2E |
| Watchlist -> notifications | PASS | Covered by mocked E2E and local real E2E |
| Account token | PASS | Covered by local real E2E; token format matched `usk_<64 hex>` and MCP initialize/search worked |
| Public responsive smoke | SKIPPED | Dedicated desktop/mobile browser pass remains in Task 2 |

## Decision

Go/no-go: **GO for local/full-stack validation**, with two release caveats before public announcement:

- Run the live OAuth browser check on the deployed app.
- Decide whether to run a controlled MCP `log_usage` write on staging or production.

## Follow-ups

- Fix or document the Vite 500 kB chunk warning during Task 2.
- Keep the profile menu accessibility fix; it makes the avatar control testable and screen-reader friendly.
