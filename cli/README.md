# UseStakly MCP CLI

Install UseStakly in an MCP-capable coding client without editing config files by hand.

```bash
npx usestakly-mcp install
```

The installer asks for a client, an endpoint, and a `usk_` token, then backs up and updates the matching local config file.

## Commands

```bash
npx usestakly-mcp install
npx usestakly-mcp test
npx usestakly-mcp doctor
```

### Install

```bash
npx usestakly-mcp install
```

Interactive flow. Asks for the client, endpoint, and token.

```bash
npx usestakly-mcp install --client codex --token-env USESTAKLY_MCP_TOKEN
```

Non-interactive flow. Safer for terminals and scripts because the token stays out of shell history.

```bash
npx usestakly-mcp install --client codex --token-env USESTAKLY_MCP_TOKEN --dry-run
```

Prints the config without writing a file.

### Test

```bash
npx usestakly-mcp test
```

Runs MCP `initialize`, then calls a protected UseStakly tool so the Bearer token is really checked.

## Supported clients

- `codex`: writes `~/.codex/config.toml`
- `cursor`: writes `~/.cursor/mcp.json`
- `claude`: writes Claude Desktop's `claude_desktop_config.json`
- `generic`: prints a JSON config without writing a file

## Non-interactive options

```bash
npx usestakly-mcp install --client codex --token-env USESTAKLY_MCP_TOKEN
npx usestakly-mcp test --token-env USESTAKLY_MCP_TOKEN
```

You can pass `--token`, but interactive input or `--token-env` is safer because tokens can otherwise remain in shell history.

## Publish checklist

Before publishing:

```bash
npm test
npm pack --dry-run
npm publish
```

The package name `usestakly-mcp` must be available on npm.
