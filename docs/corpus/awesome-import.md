# Awesome corpus import

Bounded import of GitHub OSS repos from curated [Awesome lists](https://github.com/sindresorhus/awesome). **Not a product surface** — corpus expansion only.

## Prerequisites

- Task 4 duplicate short-circuit deployed (`POST /api/repos/add` must not call GitHub when `alreadyIndexed`).
- `GITHUB_TOKEN` on the target API.
- Execution plan: `docs/plans/awesome-corpus-import-2026-05-17.md`.

## Workflow

1. **Collect (dry-run only)**

```powershell
node scripts/collect-awesome-corpus.mjs `
  --allowlist docs/corpus/awesome-lists-allowlist.json `
  --max 500 `
  --out docs/corpus/awesome-candidates.json `
  --summary docs/corpus/awesome-candidates-summary.md
```

2. **Review** `awesome-candidates-summary.md` and trim/edit JSON → save as `docs/corpus/awesome-candidates-approved.json` (same schema: `{ "candidates": [ ... ] }`).

3. **Import dry-run**

```powershell
.\scripts\import-awesome-corpus.ps1 `
  -Api "http://127.0.0.1:4000" `
  -Input "docs/corpus/awesome-candidates-approved.json" `
  -Limit 500 `
  -DryRun
```

4. **Import** (local/staging first, then prod quiet window)

```powershell
.\scripts\import-awesome-corpus.ps1 `
  -Api "https://mcp.usestakly.com" `
  -Input "docs/corpus/awesome-candidates-approved.json" `
  -Limit 500 `
  -DelayMs 750
```

5. **Post-import** — monitor GitHub rate-limit logs; wait 1–2 scheduler cycles for scores/radar; spot-check `/discover`.

## Limits

- First import cap: **500** repos after ranking (see allowlist + per-source cap in collector).
- Default per-source cap: 45 repos per Awesome list (round-robin across lists).
- Idempotent: existing repos return `alreadyIndexed` without GitHub calls (after Task 4).

## Artifacts

| File | Role |
|------|------|
| `awesome-lists-allowlist.json` | Versioned depth-1 list sources |
| `awesome-candidates.json` | Collector output |
| `awesome-candidates-approved.json` | Human-approved import input |
| `awesome-import-results.json` | Per-run import log (generated) |

## Audit trail

Record in this file or a PR comment: date, environment, counts (`added` / `alreadyIndexed` / `failed`), rate-limit events, scheduler follow-up.
