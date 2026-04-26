# UseStakly MCP endpoint security

Date: 2026-04-26
Status: active guidance

## Summary

The UseStakly MCP endpoint must be treated as a public service URL, not as a secret.

The safe model is:

- no production endpoint hard-coded in the npm package;
- a stable domain-owned endpoint shown inside UseStakly;
- `Authorization: Bearer <token>` required on every `/mcp` request;
- one revocable MCP token per machine or agent;
- rate limits and monitoring on `/mcp`;
- narrow scopes as soon as the MCP surface grows.

The endpoint can be public. The token cannot.

## Why the npm package must not contain the endpoint

`usestakly-mcp@0.1.2` contained the temporary Coolify `sslip.io` MCP endpoint in `bin/usestakly-mcp.mjs`.

That URL did not expose a secret, but it exposed infrastructure metadata:

- the temporary Coolify hostname;
- the VPS public IP through `sslip.io`;
- the active `/mcp` route.

Because npm packages are public and indexed by third-party services, old package contents should be considered permanently public once published. Future versions must avoid embedding temporary infrastructure URLs.

`usestakly-mcp@0.1.3` removes the hard-coded endpoint. Users must pass the endpoint explicitly.

## Recommended public endpoint model

Use a product-owned domain:

```text
https://mcp.usestakly.example/mcp
```

or, if the app and API share one domain:

```text
https://api.usestakly.example/mcp
```

Avoid publishing temporary deployment hostnames such as:

```text
https://<coolify-uuid>.<ip>.sslip.io/mcp
```

The domain should be shown in the UseStakly UI, ideally on the MCP guide or account token page, with a copy button.

## User installation flow

Interactive:

```bash
npx usestakly-mcp install
```

The CLI asks for:

```text
Endpoint (https://.../mcp):
UseStakly MCP token (usk_...):
```

Non-interactive:

```bash
export USESTAKLY_MCP_ENDPOINT="https://mcp.usestakly.example/mcp"
export USESTAKLY_MCP_TOKEN="usk_xxxxx"
npx usestakly-mcp install --client codex --token-env USESTAKLY_MCP_TOKEN
```

PowerShell:

```powershell
$env:USESTAKLY_MCP_ENDPOINT = "https://mcp.usestakly.example/mcp"
$env:USESTAKLY_MCP_TOKEN = "usk_xxxxx"
npx usestakly-mcp install --client codex --token-env USESTAKLY_MCP_TOKEN
```

Direct endpoint option:

```bash
npx usestakly-mcp install \
  --client codex \
  --endpoint https://mcp.usestakly.example/mcp \
  --token-env USESTAKLY_MCP_TOKEN
```

Prefer interactive input or `--token-env` for tokens. Avoid passing `--token usk_...` in shared shells because it can remain in shell history.

## Server-side security requirements

UseStakly should enforce these rules before treating `/mcp` as public beta ready.

### 1. Require Authorization everywhere

Every `/mcp` request must require:

```http
Authorization: Bearer usk_xxxxx
```

This includes:

- `initialize`;
- `tools/list`;
- `tools/call`;
- any future resource or prompt endpoint.

Sessions must not be treated as authentication. A session ID can help route MCP streams, but every inbound request must still be authorized.

### 2. Token storage and revocation

MCP tokens should remain:

- randomly generated;
- displayed only once at creation;
- stored hashed in the database;
- individually revocable;
- scoped to one machine, agent, or environment when possible.

If a user loses a token, they should revoke it and create a new one.

### 3. Rate limit `/mcp`

Add application rate limits by:

- token hash;
- source IP;
- route or method class.

Suggested first pass:

- low-cost reads: moderate limit;
- write-like tools such as `watch_repo` and `log_usage`: stricter limit;
- failed auth attempts: strict limit and alerting.

### 4. Scope minimization

The first public token can be coarse-grained, but the target model should be scopes such as:

- `read:repos`;
- `write:watchlist`;
- `write:usage`;
- `admin:tokens` only for account UI, not MCP agents.

Do not publish a wildcard scope such as `*`, `all`, or `full-access`.

### 5. No token passthrough

UseStakly MCP must only accept tokens issued by UseStakly for the MCP server.

Do not accept GitHub tokens, OAuth provider tokens, or arbitrary third-party access tokens from MCP clients and pass them downstream.

### 6. Monitoring

Monitor at least:

- `/health`;
- `/api/status/public`;
- `/mcp` with a controlled valid token in a private monitor;
- 401/403 spikes;
- 429 spikes;
- 500 errors on MCP calls.

Coolify gives deployment and container visibility, but an external uptime monitor should also verify the public path.

## MCP Inspector usage

The MCP Inspector is a development and QA tool. It does not make the endpoint private.

Use it to verify:

- connection setup;
- `initialize`;
- tool listing;
- auth failures with missing or invalid tokens;
- protected tool calls with a valid token;
- error responses for invalid arguments.

Run:

```bash
npx @modelcontextprotocol/inspector
```

Then configure a remote Streamable HTTP connection:

```text
URL: https://mcp.usestakly.example/mcp
Header: Authorization: Bearer usk_xxxxx
```

Do not paste production tokens into screenshots, shared terminals, videos, or issue reports.

## Stronger alternatives and tradeoffs

Extra protections are possible:

- Cloudflare Access;
- mTLS;
- VPN-only MCP;
- IP allowlists;
- private network-only MCP.

These are stronger for internal deployments, but they reduce compatibility with public MCP clients and external coding agents. For UseStakly public beta, the recommended balance is:

```text
public domain + mandatory Bearer auth + revocable tokens + rate limits + monitoring
```

## npm remediation checklist

After removing a hard-coded endpoint from the package:

1. Publish a new version.

```bash
cd cli
npm publish --otp=<code>
```

2. Deprecate older versions.

```bash
npm deprecate "usestakly-mcp@<0.1.3" "Temporary Coolify endpoint was hard-coded. Upgrade to 0.1.3+."
```

3. Verify the public tarball.

```bash
npm view usestakly-mcp version dist.tarball
npm pack usestakly-mcp@latest --dry-run
```

4. Treat the old endpoint as public forever.

Do not rely on URL secrecy. Rely on authorization, revocation, rate limits, and monitoring.

## References

- MCP security best practices: https://modelcontextprotocol.io/docs/tutorials/security/security_best_practices
- MCP Inspector: https://modelcontextprotocol.io/docs/tools/inspector
