# Coolify and MCP hardening plan

This document tracks the immediate operational hardening work for the public beta.

## Current State

Verified on Coolify:

- backend: `running:healthy`
- frontend: `running:healthy`
- PostgreSQL: `running:healthy`
- database public exposure: `is_public: false`
- database backups: no scheduled backup configuration was listed by the Coolify CLI

Verified in the application:

- MCP endpoint is served over HTTPS at `/mcp`
- MCP tools require `Authorization: Bearer usk_...`
- MCP tokens are hashed in the database
- tokens can be revoked from the account page
- MCP write tools have quota and cooldown guards
- public status endpoint exists at `/api/status/public`

## Priority 1: Schedule Coolify Database Backups

Risk:

The main operational risk is data loss, not MCP request traffic. The PostgreSQL database stores users, watchlists, tokens, signals, scores, notifications, and repo corpus metadata.

Goal:

Configure scheduled backups for the Coolify PostgreSQL resource.

Recommended baseline:

- frequency: daily
- retention: at least 7 days
- storage: remote storage if available, otherwise Coolify-managed storage as an interim step
- manual restore test: at least once before a wider public launch

CLI discovery:

```bash
coolify database backup list z3xzjc0sy03kr6mpv8xvka7l --format json
coolify database backup create --help
coolify database backup trigger --help
```

Acceptance criteria:

- `coolify database backup list z3xzjc0sy03kr6mpv8xvka7l --format json` returns at least one scheduled backup configuration
- at least one manual backup execution succeeds
- restore procedure is documented

## Priority 2: Add Application Rate Limiting on `/mcp`

Risk:

MCP is a public HTTP endpoint. Even when tools require a token, anonymous or invalid requests can still consume backend work at the transport layer.

Goal:

Add application-level rate limits to all `/mcp` traffic, not only write tools.

Recommended baseline:

- per-IP limit for unauthenticated MCP requests
- per-token limit for authenticated read calls
- stricter per-token limit for write calls
- separate counters for:
  - `initialize`
  - `tools/list`
  - read tools
  - write tools

Current coverage:

- write tools have token quota and cooldown guards
- read tools and protocol calls do not yet have a global application rate limit

Acceptance criteria:

- repeated unauthenticated `/mcp` requests are throttled
- repeated authenticated read calls are throttled by token
- existing write guards remain in place
- rate-limit rejections are observable in logs or metrics

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

Risk:

Coolify health checks show local container health, but they do not notify users by default and do not fully simulate public traffic.

Goal:

Use an external monitor such as UptimeRobot, Better Stack, Grafana Cloud, or another hosted monitor.

Recommended checks:

```text
GET https://<backend>/health
GET https://<backend>/api/status/public
POST https://<backend>/mcp
```

For the MCP check, use a controlled token and a lightweight protected call. Do not use a personal agent token. Create a dedicated monitoring token and revoke it if the monitor is replaced.

Acceptance criteria:

- external alert fires when backend is unreachable
- external alert fires when `/api/status/public` is degraded
- MCP monitor validates that authenticated protected tool calls work
- alert destination is configured, for example email, Discord, Slack, or webhook

## Recommended Order

1. Configure DB backups.
2. Add `/mcp` global Authorization enforcement.
3. Add `/mcp` read/protocol rate limiting.
4. Add external uptime alerting.
5. Document restore and incident steps.

## Notes

MCP making requests to the VPS is expected: MCP is an HTTP API surface. The important controls are token authentication, rate limiting, private database networking, host validation, backup strategy, and external monitoring.
