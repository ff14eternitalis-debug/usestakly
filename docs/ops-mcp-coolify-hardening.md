# Coolify and MCP hardening plan

This document tracks the immediate operational hardening work for the public beta.

## Current State

Verified on Coolify:

- backend: `running:healthy`
- frontend: `running:healthy`
- PostgreSQL: `running:healthy`
- database public exposure: `is_public: false`
- database backups: local scheduled backup enabled for `usestakly-postgres`

Verified in the application:

- MCP endpoint is served over HTTPS at `/mcp`
- MCP tools require `Authorization: Bearer usk_...`
- MCP tokens are hashed in the database
- tokens can be revoked from the account page
- MCP write tools have quota and cooldown guards
- `/mcp` has in-process rate limits for invalid auth by IP and valid traffic by token
- public status endpoint exists at `/api/status/public`
- external Uptime Kuma monitors are configured for website, health, public status, and authenticated MCP initialize

## Priority 1: Schedule Coolify Database Backups

Status: delivered 2026-05-06 for the local backup baseline. Local restore test validated on 2026-05-07.

Risk:

The main operational risk is data loss, not MCP request traffic. The PostgreSQL database stores users, watchlists, tokens, signals, scores, notifications, and repo corpus metadata.

Goal:

Configure scheduled backups for the Coolify PostgreSQL resource.

Recommended baseline:

- frequency: daily
- retention: at least 7 days
- storage: remote storage if available, otherwise Coolify-managed storage as an interim step
- manual restore test: at least once before a wider public launch

Current production configuration:

- database resource: `usestakly-postgres` (`z3xzjc0sy03kr6mpv8xvka7l`)
- backup schedule: `n12jqb2qn56mcmiqrwnjbh1z`
- frequency: `0 2 * * *`
- databases: `usestakly`
- local retention: 7 days / 7 backups
- S3/offsite backup: not configured yet
- manual trigger on 2026-05-06: succeeded
- local restore test on 2026-05-07: succeeded from `pg-dump-usestakly-1778119206.dmp`

Restore validation:

- restore target: temporary Docker container `usestakly-restore-test`
- image: `pgvector/pgvector:pg16`
- database: `usestakly_restore_test`
- restore command: `pg_restore --verbose --clean --if-exists --no-acl --no-owner`
- cleanup: temporary container removed after validation

Validated restored data:

| Table | Rows |
| --- | ---: |
| `users` | 3 |
| `external_artifacts` | 55 |
| `artifact_scores` | 97 |
| `agent_tokens` | 9 |
| `watched_artifacts` | 3 |
| `notifications` | 0 |
| `repo_categories` | 76 |
| `repo_radar_snapshots` | 55 |

Additional checks:

- `_sqlx_migrations` restored through version `22`
- active MCP tokens present: 2
- GitHub corpus repos present: 55
- sample scored repos restored with `formula_version = v2.0`

CLI discovery:

```bash
coolify database backup list z3xzjc0sy03kr6mpv8xvka7l --format json
coolify database backup create --help
coolify database backup trigger --help
```

Note: on Coolify v4.0.0 with CLI 1.6.2, `coolify database backup list` and `coolify database backup executions` can fail on JSON type mismatches. The authoritative verification used the Coolify API:

```bash
GET /api/v1/databases/z3xzjc0sy03kr6mpv8xvka7l/backups
```

Acceptance criteria:

- scheduled backup configuration exists and is enabled. Done.
- at least one manual backup execution succeeds. Done.
- restore procedure is documented. Done.
- at least one restore is tested before wider public launch. Done.

Remaining risk:

- Offsite/S3 backup storage is still not configured. Local Coolify retention protects against application-level mistakes, but not against full VPS or disk loss.

## Priority 2: Application Rate Limiting on `/mcp`

Status: delivered 2026-05-06.

Risk:

MCP is a public HTTP endpoint. Even when tools require a token, anonymous or invalid requests can still consume backend work at the transport layer.

Goal:

Add application-level rate limits to all `/mcp` traffic, not only write tools.

Recommended baseline:

- per-IP limit for unauthenticated or invalid MCP requests
- per-token limit for authenticated protocol/read calls
- stricter per-token limit for write calls via existing `agent_token_events` guards
- separate counters for:
  - `initialize`
  - `tools/list`
  - read tools
  - write tools

Current coverage (delivered 2026-05-06):

- write tools have token quota and cooldown guards via `agent_token_events`
- unauthenticated or invalid `/mcp` requests are throttled per IP (`APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`)
- authenticated protocol/read calls are throttled per token (`APP_MCP_READ_LIMIT_PER_MINUTE`)
- middleware rejects `/mcp` without Bearer before transport (Priority 3)

Acceptance criteria:

- repeated unauthenticated `/mcp` requests are throttled with `429` and `Retry-After`
- repeated authenticated protocol/read calls are throttled by token
- existing write guards remain in place
- configurable defaults exist in `.env.example`:
  - `APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE=30`
  - `APP_MCP_READ_LIMIT_PER_MINUTE=120`

## Priority 3: Require Authorization for All `/mcp` Requests

Risk:

The MCP protocol allows `initialize` and `tools/list` before tool calls. Today, protected tools validate the Bearer token, but the server can still reveal tool availability before auth.

Goal:

Require `Authorization: Bearer usk_...` for every `/mcp` request, including:

- `initialize`
- `tools/list`
- `tools/call`

Implementation options:

1. Add an Axum middleware only around `/mcp` that validates the Authorization header format before the `rmcp` service.
2. Keep tool-level DB validation inside each tool.
3. For `initialize` and `tools/list`, validate the token exists before forwarding to the MCP transport.

Acceptance criteria:

- `/mcp` without Authorization returns 401/403 before MCP protocol handling
- `/mcp` with invalid token returns 401/403 before tool discovery
- valid token still supports the full MCP flow
- `npx usestakly-mcp test` passes

## Priority 4: Add External Uptime Alerting

Status: delivered 2026-05-07 with Uptime Kuma.

Risk:

Coolify health checks show local container health, but they do not notify users by default and do not fully simulate public traffic.

Goal:

Use an external monitor such as UptimeRobot, Better Stack, Grafana Cloud, or another hosted monitor.

Recommended checks:

```text
GET https://www.usestakly.com
GET https://mcp.usestakly.com/health
GET https://mcp.usestakly.com/api/status/public
POST https://mcp.usestakly.com/mcp
```

For the MCP check, use a controlled token and a lightweight protected call. Do not use a personal agent token. Create a dedicated monitoring token and revoke it if the monitor is replaced.

Current Uptime Kuma monitors:

- `UseStakly Website`
- `UseStakly API Health`
- `UseStakly Public Status`
- `UseStakly MCP`

The MCP monitor sends an authenticated `initialize` JSON-RPC request with:

```json
{
  "Accept": "application/json, text/event-stream",
  "Content-Type": "application/json",
  "MCP-Protocol-Version": "2025-06-18",
  "Authorization": "Bearer <dedicated-monitoring-token>"
}
```

Acceptance criteria:

- external alert fires when backend is unreachable. Done.
- external alert fires when `/api/status/public` is degraded. Done.
- MCP monitor validates that authenticated protected MCP initialize works. Done.
- alert destination is configured, for example email, Discord, Slack, or webhook. Done via existing Uptime Kuma notification channel.

## Recommended Order

1. Configure DB backups.
   - Local baseline done.
2. Add `/mcp` global Authorization enforcement. Done.
3. Add `/mcp` read/protocol rate limiting. Done.
4. Add external uptime alerting. Done.
5. Document and test restore and incident steps.

## Notes

MCP making requests to the VPS is expected: MCP is an HTTP API surface. The important controls are token authentication, rate limiting, private database networking, host validation, backup strategy, and external monitoring.
