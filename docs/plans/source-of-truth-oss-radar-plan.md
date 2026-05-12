# UseStakly Source Of Truth + OSS Radar Action Plan

> **Status (2026-05-03)** — Phases 1, 2, 3, 5 livrées sur `main`. Phase 4 (MCP) partielle. Phase 6 (positionnement public) partielle. Reste consolidé : voir `docs/plans/remaining-work-2026-05-03.md`.
>
> **Livré** :
> - Phase 1 : migrations 0021 (`repo_categories`) + 0022 (`repo_radar_snapshots`), `domain::repo::RepoRadarSnapshot`, `services::radar` (commits `21eed4d`, `5fc9164`).
> - Phase 2 : mode "Emerging radar" dans `frontend/src/routes/discover.tsx` (toggle Reliable/Radar, `maturity_bands=emerging,experimental`, `sort=trend`), explication maturité sur `RepoHeader.tsx` + `RepoCard.tsx` (commits `b5e3a29`, `6a8c617`, `5c6ffdf`). `RepoSort::Trend` câblé dans `services/repos.rs`.
> - Phase 3 : sections séparées `Besoins` / `Repos` dans `frontend/src/routes/watchlist.tsx`, persistance via `services/use_case_watches.rs` + migrations 0020/0025. Notifications use case branchées via scheduler le 2026-05-12.
> - Phase 5 : classification catégories à l'ingestion via `services/repo_categories.rs` (signaux README + métadonnées GitHub), seed `scripts/seed-public-corpus.ps1` organisé par familles (UI kits, ORM, Auth, Testing, etc.).
>
> **Reste** :
> - Phase 4 MCP : livré le 2026-05-06 (`recommend_github_repos` consomme le service web, expose `stable_picks` / `emerging_picks` / `fallback_candidates`, et `watch_use_case` crée une veille d'intention depuis le MCP).
> - Phase 6 copy : exemples "emerging alternatives" ajoutés dans `docs/mcp-examples.md`; vérifier `/mcp-guide` côté frontend.

> **For agentic workers:** implement this plan task-by-task. Keep commits small, verify each phase, and do not dilute the existing score/trust model.

**Goal:** Make UseStakly both a source of truth for OSS repo quality and a radar that helps developers filter the constant noise of new dev tools.

**Architecture:** Keep the existing repo score as the product base. Add a second axis for maturity/trend, then expose it through Explorer, repo detail, watch needs, notifications, and MCP recommendations. The radar must explain why a repo is useful without pretending a young repo is production-proven.

**Tech Stack:** Rust Axum backend, SQLx/Postgres, React/Vite frontend, TanStack Router, MCP tools, Coolify deployment.

---

## Product North Star

UseStakly should answer two different questions:

1. **Can I trust this repo?**
   - Current score, freshness, reliability, adoption, abandonment, flags, provenance.
   - This is the durable source of truth.

2. **What should I watch or discover for my need?**
   - Established tools, emerging tools, noisy/low-signal tools, and risky tools.
   - This is the OSS radar.

The product should not rank all repos in one flat list. A mature dependency and a promising young repo are useful for different decisions.

---

## Phase 1: Vocabulary And Data Model

**Goal:** Add language that separates quality from maturity.

### New concepts

- `quality_score`: existing score, unchanged.
- `maturity_band`:
  - `established`: mature, active, enough public signals.
  - `emerging`: young or less adopted, but active and relevant.
  - `experimental`: interesting but too early or too thin.
  - `stale`: weak freshness or high abandonment.
  - `noisy`: lots of surface signals but low evidence of usefulness.
- `radar_relevance`: how well the repo matches a use need/category.
- `trend_signal`: recent activity/growth indicator, separate from adoption.

### Backend tasks

- Add migration for a repo radar snapshot table:
  - `external_artifact_id`
  - `maturity_band`
  - `radar_relevance`
  - `trend_signal`
  - `explanation JSONB`
  - `computed_at`
- Add `domain::repo::RepoRadarSnapshot`.
- Add `services::radar` with a pure function:
  - input: repo metadata, quality score, categories, created/updated activity when available.
  - output: maturity band + explanation.
- Keep scoring formula untouched.

### Tests

- Fresh active repo with few stars becomes `emerging`, not `established`.
- Old inactive repo becomes `stale`.
- Known active repo with strong score becomes `established`.
- Repo with vague category and weak signals becomes `experimental` or `noisy`.

---

## Phase 2: Explorer Modes

**Goal:** Make Explorer useful for both beginners and experienced devs.

### UI modes

- `Reliable choices`
  - Current default.
  - Prioritizes quality, low abandonment, low flags.

- `Emerging radar`
  - Shows young/promising repos.
  - Requires clear category/relevance and recent activity.
  - Must display a clear warning: promising does not mean production-proven.

- `Needs / use cases`
  - Existing recommendation panel.
  - User enters an intent like `auth B2B`, `testing`, `charts`, `agent framework`, `email`, `observability`.

### Frontend tasks

- Add a segmented control or tabs in `/discover`:
  - Reliable
  - Emerging
  - Needs
- Add chips:
  - `established`
  - `emerging`
  - `experimental`
  - `stale`
- Add short explanation text on repo cards:
  - `Emerging because: recent commits, clear category, active releases.`
  - Do not show README content.

### Backend tasks

- Extend repo search endpoint with:
  - `radarMode=reliable|emerging|all`
  - `maturityBand`
  - `sort=score|stars|recency|abandonment|trend`
- Return radar snapshot in `RepoSearchResult` and `RepoProfile`.

---

## Phase 3: Use Need Watchlist

**Goal:** Let users watch a need, not only repos.

### Product behavior

The user can create watches like:

- `testing tools for TypeScript`
- `auth for B2B SaaS`
- `React data grid`
- `AI agent framework`
- `observability for Rust`
- `email sending`
- `billing/subscription`

UseStakly should notify when:

- a new repo in the corpus matches the need;
- a repo changes category and becomes relevant;
- a repo becomes `emerging`;
- an established repo degrades;
- a watched repo gets a severe flag.

### Backend tasks

- Extend existing use-case watch table/service if present; otherwise add:
  - `use_case_watches`
  - `user_id`
  - `query`
  - `parsed_intent JSONB`
  - `risk_tolerance`
  - `radar_mode`
  - `created_at`
- Add service:
  - `services::use_case_watches::evaluate_watch`
  - compares current matching repos against last notification state.
- Store dedup state so users are not notified repeatedly for the same repo/match.

### Frontend tasks

- Add `Watch this need` in the recommendation panel.
- Add a section in `/watchlist` or a dedicated tab:
  - Repo watchlist
  - Need watchlist
- Add notification cards:
  - `New emerging repo for testing tools`
  - `Repo in your auth watch became risky`

---

## Phase 4: MCP Radar Tools

**Goal:** Let agents use the same radar logic.

### Existing tools to improve

- `recommend_github_repos` ✅ livré 2026-05-06
  - Must map broad needs to categories strongly:
    - testing -> `testing`
    - auth -> `auth`
    - orm/database persistence -> `orm`
    - UI kit/components -> `ui-kit`
    - table/datagrid -> `data-grid`
  - Category match should become a hard or near-hard filter for clear intents.
  - Return separate sections:
    - `stable_picks`
    - `emerging_picks`
    - `fallback_candidates`

### New tool added

- `watch_use_case` ✅ livré 2026-05-06
  - Input: need, risk tolerance, mode.
  - Creates a need watch for the authenticated token/user.

### Tests

- `recommend_github_repos("outil de test JavaScript")` should return Vitest, Playwright, Jest before unrelated repos.
- `recommend_github_repos("kit UI React")` should return UI kits before FastAPI or generic API repos.
- `recommend_github_repos("alternatives recentes a Prisma")` should include emerging ORM candidates if present.

---

## Phase 5: Corpus Growth Without Noise

**Goal:** Let the corpus grow while avoiding spam and low-value AI-generated repos.

### Ingestion policy

Each ingested repo receives:

- category classification from metadata + README signals;
- quality score;
- maturity band;
- radar explanation;
- confidence.

### Anti-noise gates

- Do not promote a repo as `established` if it lacks enough evidence.
- Young repos can be `emerging`, but only with:
  - clear category;
  - recent activity;
  - non-empty README;
  - at least minimal structural signals;
  - no severe flags.
- Repos with vague README, generic categories, or weak metadata stay `experimental` or `noisy`.

### Future optional enrichment

- GitHub creation date.
- Star velocity.
- Release cadence.
- Issue response time.
- Maintainer reputation.
- Package registry presence.
- README embeddings or AI summarization, only after deterministic rules are solid.

---

## Phase 6: Public Positioning

**Goal:** Make the value obvious to users.

### Homepage / marketing language

UseStakly should say:

> UseStakly is a quality source of truth and OSS radar for developer tools. It helps you choose proven repos, spot promising projects early, and ignore noisy AI-generated churn.

### Product pages

- `Lire UseStakly`
  - Explain score vs maturity.
  - Explain established vs emerging.
  - Explain why stars are not enough.

- `/discover`
  - Reliable choices first.
  - Emerging radar as a clear alternate mode.

- `/mcp-guide`
  - Add examples:
    - `Find a reliable testing library.`
    - `Find emerging alternatives for auth in TypeScript.`
    - `Watch new OSS tools for observability.`

---

## Delivery Order

1. **Fix MCP intent/category matching**
   - Immediate product quality win.
   - Prevents incoherent recommendations like non-testing repos for testing needs.

2. **Add maturity/radar snapshot backend**
   - Enables the product distinction between stable and emerging.

3. **Expose radar modes in Explorer**
   - Makes the new positioning visible.

4. **Add need watchlist**
   - Turns one-shot discovery into recurring value.

5. **Add notifications for radar events**
   - Makes UseStakly useful even when the user is not actively searching.

6. **Update docs and public copy**
   - Aligns product story with actual behavior.

---

## Success Criteria

- A beginner can find a reliable dependency quickly.
- An experienced dev can ask for emerging tools and get plausible, clearly caveated results.
- A user can follow a need and receive useful notifications.
- The MCP gives coherent recommendations without requiring perfect user wording.
- Young repos are visible without being falsely presented as production-proven.
- README-derived data improves categories but README content is never displayed on repo cards.
