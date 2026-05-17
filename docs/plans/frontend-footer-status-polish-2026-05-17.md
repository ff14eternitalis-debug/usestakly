# UseStakly Front Polish Plan - Header Badge, Dashes, Footer Spacing

> **For agentic workers:** This is a frontend polish plan. Keep the scope visual/content-only unless a small type/build fix is required. Do not change backend behavior, scoring, ingestion, MCP, or database code.

**Goal:** Clean up public-facing UI wording and spacing: move the beta/formula indicator to the footer, align visible formula copy with formula v2, remove awkward decorative dashes from high-visibility copy, and give the footer more breathing room.

**Runtime truth:** Backend scoring and MCP use formula v2. API values may appear as `v2.0` from `quality.formulaVersion`; marketing/status copy can say `formula_v2`. No visible frontend fallback should still say `formula_v1`.

**Encoding note:** Keep this plan ASCII-friendly. When implementing, the UI may still render a middle-dot separator or arrow if the existing design uses it intentionally.

---

## Files To Inspect First

- `frontend/src/routes/index.tsx` - homepage hero, live repo card, KPI/fallback formula copy.
- `frontend/src/features/layout/SiteFooter.tsx` - footer layout and bottom metadata row.
- `frontend/src/features/layout/AppHeader.tsx` - header only; do not move the badge here.
- `frontend/src/app/router.tsx` - confirms `SiteFooter` is mounted globally.
- `frontend/src/i18n/en.ts` - landing copy, footer copy, formula labels.
- `frontend/src/i18n/index.ts` - locale wiring; there is no `fr.ts` locale in this repo unless added later.
- `frontend/src/styles.css` or equivalent global CSS only if existing spacing utilities are not enough.

Initial search:

```powershell
rg -n "formula_v1|formula_v2|formula|Public beta|PUBLIC BETA|UseStakly|tuning|Public formula|transparent by design" frontend/src
```

Focused visible-copy search:

```powershell
rg -n "landing\.h1Part1|landing\.tickerTuning|pillar1Body|formulaEyebrow|tagFormula|Public formula|formula_v1" frontend/src/routes frontend/src/features frontend/src/i18n
```

Do not remove hyphens that are semantically required in technical strings such as:

- URLs
- CSS class names
- route paths
- package names
- SEO titles like `Page - UseStakly` if that convention is intentional
- `open-source` if the phrase would read worse without it
- `repo-detail`, `mcp-guide`, API paths, and config names

The target is visible decorative separators and awkward copy, especially on `/`, not every hyphen in code.

---

## Task 1 - Remove Hero Beta / Formula Badge

**Goal:** Remove the `Public beta` plus formula badge from the homepage hero area directly below the navbar.

**Known runtime location:** `frontend/src/routes/index.tsx`, near the hero top, renders a dot/pulse and `t.landing.eyebrow`.

**Steps:**

- [ ] Locate the current hero badge.

```powershell
rg -n "landing\\.eyebrow|dot-pulse|Public beta|formula_v1" frontend/src/routes/index.tsx frontend/src/i18n/en.ts
```

- [ ] Remove the badge block from `frontend/src/routes/index.tsx`.
- [ ] Keep the green dot visual concept available for the footer implementation in Task 2.
- [ ] Update `frontend/src/i18n/en.ts` so `landing.eyebrow` no longer contains `formula_v1`.
  - If the key is unused after removing the hero badge, either remove the key if TypeScript allows it cleanly or set it to a v2-safe value.
- [ ] Replace any frontend fallback `?? "formula_v1"` in `index.tsx` with `?? "v2.0"` or another v2-safe fallback.
- [ ] Replace `formula_v1.toml` copy with `formula_v2.toml` where it is visible product copy.

**Acceptance criteria:**

- The homepage no longer shows a beta/formula badge immediately below the navbar.
- `rg -n "formula_v1" frontend/src` returns no results.
- Runtime API-provided `quality.formulaVersion` remains respected where present.

---

## Task 2 - Add Footer Status Badge With Green Blinking Dot

**Goal:** Move the public beta/formula indicator into the global footer where it reads as operational metadata.

**Known runtime location:** `frontend/src/features/layout/SiteFooter.tsx`.

**Target footer metadata concept:**

```text
(c) 2026 UseStakly
[green pulse dot] Public beta [middle dot separator] formula_v2
transparent by design
```

**Steps:**

- [ ] In `SiteFooter.tsx`, add or reuse a compact status badge in the bottom metadata row.
- [ ] Use a green blinking/pulsing dot similar to the current hero dot.
- [ ] Merge the existing footer formula copy with the new status copy so the footer does not show duplicate formula labels.
- [ ] Update `frontend/src/i18n/en.ts` footer keys as needed:
  - `tagFormula` should no longer say `formula_v1`.
  - Preferred final visible string: `Public beta` + middle-dot separator + `formula_v2`.
  - Keep `transparent by design` as a separate phrase or adjacent footer phrase.
- [ ] Ensure the bottom row wraps cleanly on mobile.

**Suggested class direction:**

- dot: `h-2 w-2 rounded-full bg-accent shadow-[0_0_10px_rgba(...)] animate-pulse`
- label: small muted text, readable, not overly letter-spaced.

**Acceptance criteria:**

- Footer shows `Public beta` + middle-dot separator + `formula_v2` with a green pulsing dot.
- Footer still shows `transparent by design`.
- Header/hero no longer shows that badge.
- No duplicate formula metadata appears in the footer.

---

## Task 3 - Remove Decorative Dashes From High-Visibility Public Copy

**Goal:** Remove awkward separators like `UseStakly` followed by an em dash from visible hero and homepage copy.

**Known copy locations:**

- `frontend/src/i18n/en.ts`
  - `landing.h1Part1` currently contributes to `UseStakly` plus an em dash.
  - `landing.tickerTuning` may contain decorative long dash/tuning copy.
  - `landing.pillar1Body` may contain text like `Three modes` separated by em dashes.
  - `landing.formulaEyebrow` may reference `formula_v1.toml`.
- `frontend/src/routes/index.tsx`
  - may contain fallback strings such as `formula_v1`.
  - may render KPI value `v1` with label `Public formula`.

**Required changes:**

- [ ] Homepage hero title must not render `UseStakly` followed by an em dash.
  - Acceptable: `UseStakly` on its own line, followed by the existing value proposition.
  - Also acceptable: remove `UseStakly` from the hero H1 if the brand is already clear from the nav/logo.
- [ ] Rewrite decorative dash copy in the homepage ticker and key public blurbs.
  - Example: `Three modes` separated by em dashes -> `Three modes: auto, strict, and explore.`
  - Example: long dash `tuning` decoration -> `tuning` or a cleaner label.
- [ ] Update homepage KPI/formula references:
  - `v1` -> `v2` or `v2.0`, depending on nearby copy.
  - `formula_v1.toml` -> `formula_v2.toml`.
- [ ] Leave route/SEO title dashes alone unless they are visibly awkward in the page body.
- [ ] Leave arrows such as `Explore repositories ->` alone if they are intentional button affordances.

**Acceptance criteria:**

- No visible hero/title copy contains `UseStakly` followed by an em dash.
- `rg -n "formula_v1|formula_v1\\.toml" frontend/src` returns no results.
- Homepage decorative separators are rewritten into natural copy.
- Technical hyphens, route names, class names, and SEO conventions are preserved.

---

## Task 4 - Make Footer Breathe

**Goal:** Improve footer spacing and width usage so it does not feel compressed or stranded between empty side gutters.

**Known runtime location:** `frontend/src/features/layout/SiteFooter.tsx`.

**Current baseline to inspect:** likely `py-12`, `py-5` bottom bar, and a grid similar to `md:grid-cols-[1.4fr_1fr_1fr_1fr]`.

**Steps:**

- [ ] Increase footer vertical padding.
  - Suggested: mobile `py-10`, desktop `md:py-16`.
- [ ] Increase gap between the main footer grid and the bottom metadata row.
- [ ] Keep footer inner content aligned with the same shell/max-width system used by the rest of the site.
- [ ] Improve column distribution without turning the footer into a card.
  - Brand/description column should have enough width.
  - Product, signals, and about columns should have consistent spacing.
- [ ] Make bottom metadata row responsive.
  - Desktop: horizontal row with wrap.
  - Mobile: stacked or wrapped with comfortable gap.
- [ ] Preserve footer links and existing route targets.

**Acceptance criteria:**

- Footer no longer feels compressed vertically.
- Footer content aligns with the rest of the page container.
- Side gutters look intentional, not like missing content.
- Mobile footer has no overlap or cramped metadata.

---

## Task 5 - Visual Pass Across Main Public Pages

**Pages to check:**

- `/`
- `/discover`
- `/how-to-read`
- `/mcp-guide`
- `/status`
- `/privacy`
- `/legal`

**Checks:**

- [ ] No visible `formula_v1`.
- [ ] No awkward decorative dash in high-visibility headings or hero copy.
- [ ] Footer has enough spacing.
- [ ] Footer links do not look squeezed.
- [ ] Header spacing still looks normal.
- [ ] Mobile width does not cause footer overlap.
- [ ] Homepage live repo card still shows API-provided formula values correctly when data is present.

Use browser/dev server:

```powershell
cd frontend
npm run dev
```

Then inspect desktop and mobile viewport.

---

## Task 6 - Build Verification

Run:

```powershell
cd frontend
npm run build
```

Expected:

- TypeScript passes.
- Vite build passes.

Optional full repo sanity:

```powershell
git diff --check
```

---

## Commit Suggestion

```bash
git add frontend/src docs/plans/frontend-footer-status-polish-2026-05-17.md
git commit -m "polish frontend status badge and footer spacing"
```

---

## Done Criteria

- `Public beta` + middle-dot separator + `formula_v2` appears only in the footer with the green pulsing dot.
- `transparent by design` remains in the footer.
- `formula_v1` is gone from frontend visible copy and fallbacks.
- `formula_v1.toml` is replaced by `formula_v2.toml` in frontend copy.
- `UseStakly` followed by an em dash is removed from the homepage hero.
- Homepage decorative dash separators are cleaned up.
- Footer has more vertical spacing and better width usage.
- `npm run build` passes.
