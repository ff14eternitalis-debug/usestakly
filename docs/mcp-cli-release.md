# UseStakly MCP CLI release checklist

The CLI package lives in `cli/` and is meant to power:

```bash
npx usestakly-mcp install
```

## What is ready

- Interactive install for Codex, Cursor, Claude Desktop, and generic JSON output.
- Non-interactive install via `--token-env USESTAKLY_MCP_TOKEN`.
- `--dry-run` to preview config without writing files.
- Automatic backups before modifying an existing config file.
- `test` command that sends MCP `initialize` to `/mcp`.
- `doctor` command that checks whether known client config files contain UseStakly.
- Node built-in tests, no runtime dependencies.

## Local verification

```bash
cd cli
npm test
npm pack --dry-run
```

## Publish steps

1. Make sure the npm package name `usestakly-mcp` is available.
2. Log in:

```bash
npm login
```

3. Publish:

```bash
cd cli
npm publish
```

4. Verify the public install:

```bash
npx usestakly-mcp --help
npx usestakly-mcp install --client generic --token-env USESTAKLY_MCP_TOKEN
```

## Publication status

`usestakly-mcp@0.1.0` is published on npm and can be installed with:

```bash
npx usestakly-mcp install
```
