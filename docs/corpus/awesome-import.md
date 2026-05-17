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
  -InputPath "docs/corpus/awesome-candidates-approved.json" `
  -Limit 500 `
  -DryRun
```

4. **Import** (local/staging first, then prod quiet window)

```powershell
.\scripts\import-awesome-corpus.ps1 `
  -Api "https://mcp.usestakly.com" `
  -InputPath "docs/corpus/awesome-candidates-approved.json" `
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

### Production import — 2026-05-17

| Field | Value |
|-------|--------|
| API | `https://mcp.usestakly.com` |
| Input | `docs/corpus/awesome-candidates-approved.json` (500 candidates, copied from collector output) |
| Limit / delay | 500 / 750 ms |
| **added** | **490** |
| **alreadyIndexed** | **4** (`ant-design/ant-design`, `ag-grid/ag-grid`, `chakra-ui/chakra-ui`, `facebook/react`) |
| **failed** | **6** (GitHub/API 404 — repo absent or renamed) |
| Rate-limit stops | None (no 403/429 burst; import completed full queue) |
| Duration | ~34 min wall time |
| Results file | `docs/corpus/awesome-import-results.json` (gitignored) |

**Failed slugs (retry only if repos exist on GitHub):** `adembudak/vim-doctor`, `0x4d31/sqhunter`, `addyosmani/9f10c555e32a8d06ddb0`, `apps/guardrails`, `crowdstrike/falcon-orchestrator`, `skydb/sky`.

**Follow-up:** wait 1–2 scheduler cycles (or trigger admin recompute) so scores/radar refresh on new artifacts; spot-check `/discover` corpus size.

**Collector note (same day):** `node scripts/collect-awesome-corpus.mjs` → 9092 unique links, cap 500 after round-robin; see `awesome-candidates-summary.md`.
