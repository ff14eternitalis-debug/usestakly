# Documentation Source of Truth Cleanup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove or quarantine stale legacy documentation so agents use the current UseStakly runtime as the only source of truth.

**Architecture:** Keep a small active documentation spine and push all pre-pivot or snapshot material behind explicit archive warnings. Update agent entrypoints first, then reconcile status docs against code, then add automated checks that catch future legacy drift.

**Tech Stack:** Markdown docs, Rust/Axum backend, React/Vite frontend, `rg`-based documentation audit, existing backend/frontend/CLI verification commands.

---

## Findings From The 2026-05-16 Audit

These are the concrete mismatches found before writing this plan:

- `AGENTS.md` still says the project state reflects `2026-05-06`, references `TODO v5.5`, says wider launch is blocked by MCP backup/rate-limit/alerting, and says there is no global MCP rate-limit. The code now has `/mcp` auth failure limits by IP and authenticated limits by token in `backend/src/app/mod.rs` plus env config in `backend/src/config/mod.rs`.
- `AGENTS.md` and `docs/tech-stack.md` document CLI `usestakly-mcp` as `v0.1.3`, but `cli/package.json` is `0.1.4`.
- `docs/tech-stack.md` says there are 17 migrations; `backend/migrations/` now has 27 migrations through `0027_github_ingestion_reliability.sql`.
- `TODO.md` still lists R1 ETags/backoff and `owner_inactive_days` as open even though migration `0027` and `backend/src/services/ingestion/github.rs` now persist endpoint ETags, handle 304, apply bounded backoff, and store owner inactivity.
- `docs/ops-mcp-coolify-hardening.md` marks Priority 2 as delivered, but its "Current coverage" section still says read/protocol calls do not have a global application rate limit.
- `docs/architecture-backend-current.md` is mostly current, but it still says `formula_v2 (compte neuf = poids 0)` is future while `backend/scoring/formula_v2.toml` already contains `[trust].new_account_active_signal_weight = 0.0`.
- Active docs still point agents at strategy docs that contain snippet-era examples (`docs/strategy-quality-scored-registry.md`) without enough friction.
- Snapshot docs and archives contain many `Project-DK/Project-K` absolute links, `search_library`, `get_snippet`, `/api/snippets`, and Supabase-era notes. Those can remain only if every archive entry says "not source of truth" loudly and the active docs stop routing agents there for execution.

## Desired End State

- Agents start from `AGENTS.md`, `README.md`, `docs/README.md`, `docs/architecture-backend-current.md`, `docs/mcp-protocol.md`, `docs/trust-model-v1.md`, and `docs/plans/remaining-work-2026-05-03.md`.
- `TODO.md` is either renamed in practice as a historical roadmap snapshot or rewritten as a current status index. It must not be the primary execution source if it contains old phase narrative.
- Legacy snippets docs remain in `docs/archive/snippets/`, but active docs never ask agents to treat them as implementation guidance.
- Every active doc that mentions legacy snippets says the same thing: database tables may remain for compatibility; no product surface should be reintroduced without explicit user request.
- MCP rate-limit, formula v2 trust, migration count, CLI version, GitHub ingestion reliability, and validation status all match the code.
- A repeatable `rg` audit command exists so future agents can catch stale docs before making product decisions.

---

## File Map

Modify:

- `AGENTS.md` - primary agent instruction file; must be the sharpest source of truth.
- `README.md` - repo entrypoint for humans and agents.
- `TODO.md` - convert from old phase-heavy execution source into current beta status or clearly demote it.
- `docs/README.md` - documentation index and routing table.
- `docs/tech-stack.md` - versions, migrations, CLI version, hosting state.
- `docs/ops-mcp-coolify-hardening.md` - reconcile delivered MCP rate-limit coverage.
- `docs/architecture-backend-current.md` - formula v2 trust status, migration list completeness, test counts if kept.
- `docs/strategy-quality-scored-registry.md` - add stronger "principles only, not execution" warning or archive it.
- `docs/strategy-pivot-2026-04-21.md` - update open items that are now delivered or demote dated sections.
- `docs/security-audit-2026-04-21.md` - add snapshot warning and link to current security/ops docs.
- `docs/audits/user-journey-audit-2026-04-23.md` - fix or neutralize stale absolute paths.
- `docs/dev-workflow.md` - replace old `Project-DK/Project-K` local path examples with current `usestakly` paths.
- `docs/plans/remaining-work-2026-05-03.md` - keep as current prioritized backlog, with explicit "last reconciled" date.

Create:

- `docs/source-of-truth.md` - one-page canonical map of current runtime, source docs, and archived docs.
- `docs/archive/README.md` - archive boundary that applies to all archived material.
- `docs/plans/docs-source-of-truth-legacy-cleanup-2026-05-16.md` - this plan.
- Optional: `scripts/audit-doc-source-truth.ps1` - repeatable stale-doc scanner.

Do not modify:

- `backend/migrations/0001` through `0009` only to remove history. They are database history and must stay unless a separate DB deprecation plan exists.
- Archived pre-pivot content line-by-line unless adding archive banners. Preserving history is fine; routing agents away from it is the goal.

---

## Task 1: Add A Canonical Source-Of-Truth Map

**Files:**

- Create: `docs/source-of-truth.md`
- Modify: `docs/README.md`
- Modify: `README.md`

- [ ] **Step 1: Create `docs/source-of-truth.md`**

Create a concise map with these sections:

```markdown
# UseStakly Source Of Truth

> Last reconciled: 2026-05-16
> Scope: current runtime and documentation routing for agents.

## Current Product

UseStakly is a GitHub OSS discovery, scoring, watchlist, notification, and MCP product.
It is not a snippets library.

Active pillars:

- quality-scored GitHub repository discovery
- repo profiles with score provenance and signal history
- watchlist, notifications, notification channels, and use-case watches
- MCP tools for agent discovery, context, passive usage, and watches

## Canonical Docs

Read in this order:

1. `AGENTS.md`
2. `README.md`
3. `docs/architecture-backend-current.md`
4. `docs/mcp-protocol.md`
5. `docs/trust-model-v1.md`
6. `docs/plans/remaining-work-2026-05-03.md`

## Runtime Truth Beats Docs

If docs disagree with code, verify code first:

- backend routes: `backend/src/app/mod.rs`
- MCP tools: `backend/src/mcp/server.rs`
- config/env: `backend/src/config/mod.rs` and `.env.example`
- scoring formula: `backend/scoring/formula_v2.toml`
- migrations: `backend/migrations/`
- frontend routes: `frontend/src/app/router.tsx`
- CLI version: `cli/package.json`

## Legacy Boundary

The snippets product is abandoned.
Legacy tables and archived docs may remain for compatibility and history, but they are not product guidance.

Never add or revive snippets/libraries UI, API, MCP tools, or roadmap items unless the user explicitly asks for that legacy product.

## Archives

Everything under `docs/archive/` is historical.
Archived docs may mention snippets, Project-K, old local paths, old MCP tools, and old business assumptions.
Do not use archived docs as source of truth for implementation.
```

- [ ] **Step 2: Link it from `docs/README.md`**

In `docs/README.md`, change the "Tu codes" row so it starts with `source-of-truth.md`, then `architecture-backend-current.md`, then `mcp-protocol.md`.

Expected row:

```markdown
| Tu codes | `source-of-truth.md` → `architecture-backend-current.md` → `mcp-protocol.md` → `plans/remaining-work-2026-05-03.md` |
```

- [ ] **Step 3: Link it from root `README.md`**

Add a short "Source of truth" paragraph near the top:

```markdown
For current implementation work, start with [`docs/source-of-truth.md`](./docs/source-of-truth.md). Archived snippets-era documents are history only and must not drive current product work.
```

- [ ] **Step 4: Verify links**

Run:

```powershell
rg -n "source-of-truth.md|archive/snippets|not product guidance" README.md docs/README.md docs/source-of-truth.md
```

Expected:

- root README links `docs/source-of-truth.md`
- docs README routes coding agents through `source-of-truth.md`
- source-of-truth doc states archives are not product guidance

- [ ] **Step 5: Commit**

```powershell
git add README.md docs/README.md docs/source-of-truth.md
git commit -m "Document current source of truth"
```

---

## Task 2: Update Agent Entrypoints To Match Current Code

**Files:**

- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `docs/README.md`

- [ ] **Step 1: Update `AGENTS.md` header and product state**

Replace the stale state lines with:

```markdown
This file provides guidance to Codex (and other CLI agents) when working with code in this repository. Reflète l'état au 2026-05-16.
```

Replace the public beta bullet with:

```markdown
- État : **public beta exposée et redéployée**. Les blocs ops MCP critiques sont en place : backup local Coolify + restore testé, Authorization obligatoire sur `/mcp`, rate-limit `/mcp` par IP/token, alerte externe Uptime Kuma. Restent surtout : backup offsite/S3, règles notification avancées, validation continue et polish.
```

- [ ] **Step 2: Fix CLI version in `AGENTS.md`**

Replace:

```markdown
Package npm `usestakly-mcp` (v0.1.3 au 2026-04-26).
```

with:

```markdown
Package npm `usestakly-mcp` (v0.1.4 au 2026-05-16, voir `cli/package.json`).
```

- [ ] **Step 3: Fix MCP gotcha in `AGENTS.md`**

Replace the stale rate-limit gotcha:

```markdown
- **Rate-limit MCP par token** sur writes via `agent_token_events` (migration 0014). Pas encore de rate-limit globale multi-token / par IP — item ops #2 dans `docs/ops-mcp-coolify-hardening.md`.
```

with:

```markdown
- **Rate-limit MCP** : middleware `/mcp` in-process par IP pour auth absente/invalide (`APP_MCP_AUTH_FAILURE_LIMIT_PER_MINUTE`) et par token valide pour transport/read (`APP_MCP_READ_LIMIT_PER_MINUTE`). Les writes gardent en plus quota/cooldowns persistés via `agent_token_events` (migration 0014).
```

- [ ] **Step 4: Fix formula v2 trust gotcha in `AGENTS.md`**

Replace:

```markdown
- **Modération** : migrations 0015/0016 (review + events). Garde-fous v1 en place. Réputation v2 runtime livrée. Formula_v2 + Sybil OAuth à venir.
```

with:

```markdown
- **Modération** : migrations 0015/0016 (review + events). Réputation v2 runtime et trust `[formula_v2].trust` livrés (compte neuf poids actif 0 pour signaux sévères). Sybil OAuth GitHub reste à venir.
```

- [ ] **Step 5: Add `watch_use_case` to MCP scope in `AGENTS.md`**

Replace:

```markdown
- Produit vivant : **discovery repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications in-app**, **MCP read + write + recommend**.
```

with:

```markdown
- Produit vivant : **discovery repos GitHub publics scorés**, **profil repo**, **watchlist**, **notifications in-app/outbound**, **MCP read + write + recommend + watch_use_case**.
```

- [ ] **Step 6: Verify against code**

Run:

```powershell
rg -n "mcp_auth_failure_limit_per_minute|mcp_read_limit_per_minute|mcp_write_limit_per_hour" backend/src/app/mod.rs backend/src/config/mod.rs
rg -n '"version": "0.1.4"' cli/package.json
rg -n "new_account_active_signal_weight|owner_dispute_min_reputation" backend/scoring/formula_v2.toml
```

Expected:

- config and app rate-limit fields exist
- CLI version is `0.1.4`
- formula v2 trust keys exist

- [ ] **Step 7: Commit**

```powershell
git add AGENTS.md README.md docs/README.md
git commit -m "Refresh agent documentation entrypoints"
```

---

## Task 3: Reconcile Current Status Docs With Runtime

**Files:**

- Modify: `TODO.md`
- Modify: `docs/tech-stack.md`
- Modify: `docs/ops-mcp-coolify-hardening.md`
- Modify: `docs/architecture-backend-current.md`
- Modify: `docs/plans/remaining-work-2026-05-03.md`

- [ ] **Step 1: Update `docs/tech-stack.md`**

Change:

```markdown
> Version : 2.0 — 2026-04-26
```

to:

```markdown
> Version : 2.1 — 2026-05-16
```

Change:

```markdown
- Package npm public `usestakly-mcp` (v0.1.3 au 2026-04-26)
```

to:

```markdown
- Package npm public `usestakly-mcp` (v0.1.4 au 2026-05-16)
```

Change:

```markdown
- 17 migrations (1–9 legacy snippets dormantes, 10–17 actives produit GitHub)
```

to:

```markdown
- 27 migrations au 2026-05-16 : `0001`–`0009` legacy snippets/auth/generations dormantes, `0010`–`0027` actives ou optionnelles pour le produit GitHub courant.
```

- [ ] **Step 2: Fix `docs/ops-mcp-coolify-hardening.md` Priority 2 current coverage**

Replace:

```markdown
Current coverage:

- write tools have token quota and cooldown guards
- read tools and protocol calls do not yet have a global application rate limit
```

with:

```markdown
Current coverage:

- unauthenticated or invalid `/mcp` requests are limited in-process by source IP
- authenticated `/mcp` protocol/read traffic is limited in-process by token fingerprint
- write tools have stricter persisted token quota and cooldown guards via `agent_token_events`
- remaining limitation: the read/protocol limiter is per backend process, not distributed across multiple instances
```

- [ ] **Step 3: Fix `docs/architecture-backend-current.md` debt**

Replace:

```markdown
- réputation v2 runtime livrée ; formula_v2 (compte neuf = poids 0) + Graphe Sybil OAuth GitHub à venir
```

with:

```markdown
- réputation v2 runtime + trust formula_v2 livrés (`new_account_active_signal_weight = 0.0`) ; Graphe Sybil OAuth GitHub à venir
```

Also add missing migration rows if absent:

```markdown
| 0018 | `external_artifacts` vitality signals | actif |
| 0019 | `artifact_scores` vitality columns | actif |
| 0020 | `use_case_watches` | actif |
| 0021 | `repo_categories` | actif |
| 0022 | `repo_radar_snapshots` | actif |
| 0026 | email locale preference | actif |
```

- [ ] **Step 4: Reconcile R1 in `TODO.md`**

Replace the two stale R1 open items:

```markdown
- [ ] **Reste à faire** : rate-limit handling (ETags conditional requests, backoff, quota monitoring)
- [ ] **Reste à faire** : computation priors dérivés côté events API (`owner_inactive_days`)
```

with:

```markdown
- [x] ETags GitHub releases/README/events + 304 sans perte + backoff borné — livré 2026-05-16 via migration `0027` et `services::ingestion::github`.
- [ ] Monitoring quota GitHub exploitable en dashboard/alerting — les colonnes `github_rate_limit_reset_at` / `github_last_rate_limit_at` existent, mais l'usage ops reste à formaliser.
- [x] Computation priors dérivés côté events API (`owner_last_activity_at`, `owner_inactive_days`) — livré 2026-05-16 comme input read-only.
```

- [ ] **Step 5: Reconcile formula v2 trust in `TODO.md`**

Replace:

```markdown
- [ ] Pondération réputation owner / reporter plus riche dans les reviews elles-mêmes — formula_v2, compte neuf = poids 0, historique d'usage prod = surpondéré
```

with:

```markdown
- [x] Pondération réputation owner / reporter v2 dans les reviews sensibles — livré 2026-05-16 via `[trust]` dans `formula_v2.toml`.
- [ ] Graphe Sybil-resistant via OAuth GitHub (followers, contributions, âge compte)
```

- [ ] **Step 6: Ensure `remaining-work` stays canonical for open items**

In `docs/plans/remaining-work-2026-05-03.md`, add at top:

```markdown
> Last reconciled with code: 2026-05-16.
> This file is the current prioritized backlog. Older phase narratives in `TODO.md` are historical context when they disagree with this file.
```

- [ ] **Step 7: Verify no known stale status remains in active docs**

Run:

```powershell
rg -n "v0\\.1\\.3|17 migrations|read tools and protocol calls do not yet|pas encore de rate-limit globale|rate-limit handling \\(ETags|computation priors dérivés côté events API|formula_v2 \\(compte neuf = poids 0\\).*à venir" AGENTS.md README.md TODO.md docs -g "*.md"
```

Expected:

- No matches in active docs except archived files or this plan.

- [ ] **Step 8: Commit**

```powershell
git add TODO.md docs/tech-stack.md docs/ops-mcp-coolify-hardening.md docs/architecture-backend-current.md docs/plans/remaining-work-2026-05-03.md
git commit -m "Reconcile docs with current runtime status"
```

---

## Task 4: Quarantine Legacy Snippet And Pre-Pivot Docs

**Files:**

- Create: `docs/archive/README.md`
- Modify: `docs/archive/snippets/README.md`
- Modify: `docs/archive/business-prepivot/README.md`
- Modify: `docs/strategy-quality-scored-registry.md`
- Modify: `docs/strategy-pivot-2026-04-21.md`

- [ ] **Step 1: Create `docs/archive/README.md`**

Create:

```markdown
# UseStakly Documentation Archive

> Historical material only. Not source of truth for current implementation.

Documents under `docs/archive/` may describe:

- the abandoned snippets/library product
- old Project-K / Komorebi naming
- old MCP tool names such as `search_library` and `get_snippet`
- old local paths such as `Project-DK/Project-K`
- assumptions that were valid before the GitHub OSS radar pivot

For current work, use:

1. `docs/source-of-truth.md`
2. `AGENTS.md`
3. `docs/architecture-backend-current.md`
4. `docs/mcp-protocol.md`
5. `docs/plans/remaining-work-2026-05-03.md`
```

- [ ] **Step 2: Strengthen `docs/archive/snippets/README.md`**

Add this at the top:

```markdown
> Archive warning: the snippets/library product is abandoned. These docs are retained only for historical reasoning. Do not implement routes, UI, MCP tools, or roadmap items from this folder unless the user explicitly asks to revive the legacy product.
```

- [ ] **Step 3: Strengthen `docs/archive/business-prepivot/README.md`**

Add this at the top:

```markdown
> Archive warning: pre-pivot business assumptions are not current UseStakly strategy. Current product scope is GitHub OSS discovery, scoring, watchlists, notifications, and MCP.
```

- [ ] **Step 4: Demote `docs/strategy-quality-scored-registry.md` to principles-only**

Add this immediately below the title/front matter:

```markdown
> Status 2026-05-16: principles-only document.
> This file contains useful scoring philosophy, but examples that mention snippets, `search_library`, `get_snippet`, teams, or private registries are historical.
> Current implementation truth lives in `docs/source-of-truth.md`, `docs/architecture-backend-current.md`, and `docs/mcp-protocol.md`.
```

- [ ] **Step 5: Update `docs/strategy-pivot-2026-04-21.md` open status**

Where the file says `owner_inactive_days` is future, change it to:

```markdown
- `owner_inactive_days` is now captured by ingestion as of 2026-05-16; the notification rule "maintainer silencieux 90 j" remains open.
```

Where it says formula v2 account weight is future, change it to:

```markdown
- Formula v2 trust now sets new account active signal weight to `0.0`; Sybil-resistant GitHub graph remains open.
```

- [ ] **Step 6: Verify archives are clearly marked**

Run:

```powershell
rg -n "Archive warning|Historical material only|principles-only" docs/archive docs/strategy-quality-scored-registry.md
```

Expected:

- `docs/archive/README.md` warns all archive content is historical
- snippets archive README has abandoned-product warning
- business pre-pivot README has pre-pivot warning
- strategy-quality doc says principles-only

- [ ] **Step 7: Commit**

```powershell
git add docs/archive/README.md docs/archive/snippets/README.md docs/archive/business-prepivot/README.md docs/strategy-quality-scored-registry.md docs/strategy-pivot-2026-04-21.md
git commit -m "Quarantine legacy documentation"
```

---

## Task 5: Remove Misleading Legacy Paths From Active Snapshots

**Files:**

- Modify: `docs/audits/user-journey-audit-2026-04-23.md`
- Modify: `docs/security-audit-2026-04-21.md`
- Modify: `docs/dev-workflow.md`

- [ ] **Step 1: Add snapshot warnings to dated audits**

At the top of `docs/audits/user-journey-audit-2026-04-23.md`, add:

```markdown
> Snapshot warning: dated audit from 2026-04-23. File paths and UI structure may have changed. Use `docs/source-of-truth.md` and `frontend/src/app/router.tsx` for current routing truth.
```

At the top of `docs/security-audit-2026-04-21.md`, add:

```markdown
> Snapshot warning: dated security audit from 2026-04-21. Current MCP authorization/rate-limit and GitHub ingestion hardening have advanced since this file. Use `docs/ops-mcp-coolify-hardening.md`, `docs/mcp-endpoint-security.md`, and `docs/architecture-backend-current.md` for current status.
```

- [ ] **Step 2: Replace old local paths in active docs**

In `docs/dev-workflow.md`, replace:

```powershell
C:\Users\forgo\Documents\Code\Project-DK\Project-K\backend\target
```

with:

```powershell
C:\Users\forgo\Documents\Code\usestakly\backend\target
```

- [ ] **Step 3: Neutralize absolute links in `user-journey-audit`**

Replace each markdown link target like:

```markdown
(/C:/Users/forgo/Documents/Code/Project-DK/Project-K/frontend/src/app/router.tsx)
```

with a relative code path in backticks:

```markdown
`frontend/src/app/router.tsx`
```

Do this for every `Project-DK/Project-K` link in the file.

- [ ] **Step 4: Neutralize absolute link in `security-audit`**

Replace:

```markdown
[docs/security-secrets-playbook.md](/C:/Users/forgo/Documents/Code/Project-DK/Project-K/docs/security-secrets-playbook.md)
```

with:

```markdown
`docs/security-secrets-playbook.md`
```

- [ ] **Step 5: Verify no active docs contain old local paths**

Run:

```powershell
rg -n "Project-DK|Project-K" AGENTS.md README.md TODO.md docs -g "*.md"
```

Expected:

- Matches are allowed only in `docs/archive/`, `docs/plans/rename-to-usestakly.md`, or explicit historical-name notes in active docs.
- No active docs should contain `Project-DK/Project-K` absolute local links.

- [ ] **Step 6: Commit**

```powershell
git add docs/audits/user-journey-audit-2026-04-23.md docs/security-audit-2026-04-21.md docs/dev-workflow.md
git commit -m "Remove stale local paths from active docs"
```

---

## Task 6: Add A Repeatable Documentation Drift Audit

**Files:**

- Create: `scripts/audit-doc-source-truth.ps1`
- Modify: `docs/dev-workflow.md`
- Modify: `docs/source-of-truth.md`

- [ ] **Step 1: Create `scripts/audit-doc-source-truth.ps1`**

Create:

```powershell
$ErrorActionPreference = "Stop"

$patterns = @(
  'v0.1.3',
  '17 migrations',
  'read tools and protocol calls do not yet',
  'pas encore de rate-limit globale',
  'rate-limit handling (ETags',
  'computation priors dérivés côté events API',
  'Endpoint `POST /api/snippets',
  '/api/snippets',
  'search_library',
  'get_snippet',
  'Project-DK/Project-K'
)

$activeRoots = @(
  "AGENTS.md",
  "README.md",
  "TODO.md",
  "docs/README.md",
  "docs/architecture-backend-current.md",
  "docs/mcp-protocol.md",
  "docs/trust-model-v1.md",
  "docs/tech-stack.md",
  "docs/ops-mcp-coolify-hardening.md",
  "docs/dev-workflow.md",
  "docs/security-audit-2026-04-21.md",
  "docs/audits/user-journey-audit-2026-04-23.md",
  "docs/plans/remaining-work-2026-05-03.md"
)

$failed = $false

foreach ($pattern in $patterns) {
  $matches = & rg -n --fixed-strings $pattern $activeRoots 2>$null
  if ($LASTEXITCODE -eq 0) {
    $failed = $true
    Write-Host ""
    Write-Host "Potential stale documentation pattern: $pattern" -ForegroundColor Yellow
    $matches
  }
}

if ($failed) {
  Write-Host ""
  Write-Host "Documentation drift audit found potential stale active-doc references." -ForegroundColor Red
  exit 1
}

Write-Host "Documentation drift audit passed." -ForegroundColor Green
```

- [ ] **Step 2: Document the audit command in `docs/dev-workflow.md`**

Add:

````markdown
### Documentation source-of-truth audit

Run this before handing large docs changes to another agent:

```powershell
.\scripts\audit-doc-source-truth.ps1
```

The script scans active docs for stale snippet-era tools, old local paths, stale MCP rate-limit status, old CLI versions, and migration-count drift.
````

- [ ] **Step 3: Link audit from `docs/source-of-truth.md`**

Add:

```markdown
## Drift Audit

Run `.\scripts\audit-doc-source-truth.ps1` after documentation edits. A failure means an active doc may still contain a stale legacy reference.
```

- [ ] **Step 4: Run the audit**

Run:

```powershell
.\scripts\audit-doc-source-truth.ps1
```

Expected:

```text
Documentation drift audit passed.
```

- [ ] **Step 5: Commit**

```powershell
git add scripts/audit-doc-source-truth.ps1 docs/dev-workflow.md docs/source-of-truth.md
git commit -m "Add documentation drift audit"
```

---

## Task 7: Final Verification

**Files:**

- All documentation files touched above.

- [ ] **Step 1: Check current branch state**

Run:

```powershell
git status --short --branch
```

Expected:

- on the cleanup branch
- no untracked files except intentional files before final commit

- [ ] **Step 2: Run stale-doc scans**

Run:

```powershell
.\scripts\audit-doc-source-truth.ps1
rg -n "snippets|libraries|search_library|get_snippet|/api/snippets|Project-DK|Project-K|Supabase|v0.1.3|17 migrations|pas encore de rate-limit globale|read tools and protocol calls do not yet" AGENTS.md README.md TODO.md docs -g "*.md"
```

Expected:

- first command passes
- second command may show archive matches and allowed historical-name notes only
- no active source-of-truth doc contains stale implementation guidance

- [ ] **Step 3: Run docs-adjacent code verification**

Run:

```powershell
cd backend
cargo fmt --check
cargo test services::quality::formula::tests::formula_v2_loads app::mcp_rate_limit::tests
cd ..\cli
npm test
```

Expected:

- backend targeted tests pass
- CLI tests pass

- [ ] **Step 4: Review changed files**

Run:

```powershell
git diff --stat main...HEAD
git diff -- AGENTS.md docs/source-of-truth.md docs/README.md docs/tech-stack.md docs/ops-mcp-coolify-hardening.md TODO.md
```

Expected:

- diffs are docs/script-only
- no archive content was rewritten beyond archive warnings
- active docs route agents to current source-of-truth docs

- [ ] **Step 5: Final commit if anything remains**

If `git status --short` shows remaining intentional changes:

```powershell
git add <remaining-files>
git commit -m "Finalize documentation source of truth cleanup"
```

---

## Completion Criteria

- `AGENTS.md` no longer says the MCP global rate-limit is missing.
- Active docs no longer claim ETag/backoff or `owner_inactive_days` are unimplemented.
- Active docs no longer claim formula v2 trust account weight is future.
- CLI version and migration count match the repository.
- `docs/source-of-truth.md` exists and is linked from root/docs entrypoints.
- Archives are clearly quarantined and no active doc routes agents into archived snippets docs for implementation.
- The drift audit script passes.

## Recommended Execution Order

1. Source-of-truth map.
2. Agent entrypoints.
3. Runtime status reconciliation.
4. Archive quarantine.
5. Stale path cleanup.
6. Drift audit script.
7. Final verification.
