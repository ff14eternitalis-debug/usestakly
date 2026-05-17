# Public Launch Hardening Plan

> **For agentic workers:** this is an execution plan for the last hardening pass before a wider public launch. Track steps with checkboxes and keep `docs/source-of-truth.md` + `docs/plans/remaining-work-2026-05-03.md` aligned after implementation.

**Goal:** Move UseStakly from "public beta exposée / soft launch" to "wider public beta launch ready".

**Current status:** Core product, MCP, OAuth, legal pages, public status, local DB backups, restore test, monitoring, and the live post-deploy gate are already in place.

**Already validated:** the final live gate after redeploy is considered done: health, public status, OAuth réel, discover, repo detail, MCP smoke, and watchlist were validated. Do not re-open this as a blocker unless a new deploy changes the runtime.

**Gate policy:** do not re-run or re-open the live post-deploy gate during this plan unless one of these tasks ships a new backend/frontend deployment. If a deploy happens, run the existing `docs/validation/live-release-checklist.md` as a release verification step, not as a new blocker category.

---

## Launch Decision

UseStakly can stay public in limited beta today. A **wider** public announcement should wait for the remaining **blocking** items below (not deferred ops).

| Area | Launch status | Why |
|---|---|---|
| Offsite backup | Deferred (budget) | Local Coolify backup + restore test are in place. S3/offsite copy is postponed until budget allows; full VPS/disk loss risk remains accepted for limited beta. |
| `/api/repos/{id}/refresh` hardening | Done (code) | Session + DB limits + memory cooldown; deploy + env on Coolify still required. |
| GitHub quota visibility | Done (code) | L1 logs + `GET /api/admin/github/quota` + public `ingestion` status (deploy to verify). |
| Public UX/mobile smoke | Done | Validated manually on production desktop and mobile (2026-05-17); no visible console errors or blocking layout issues reported. |
| Real outbound email notification | Blocking | Channel test is not enough; one real watch/digest delivery must be proven. |
| Live post-deploy gate | Done | Health/status/OAuth/discover/repo/MCP/watchlist already validated. |

Non-blocking for limited beta launch: offsite S3 (deferred budget), Sybil GitHub graph, custom alert rules, richer account page, full score history/timeline UI, large refactors, broader search calibration, GitHub quota admin endpoint (Task 3 L2).

---

## Task 1 — Offsite Backup / S3

**Status (2026-05-17):** **Deferred — budget.** Not required to keep the current limited public beta running. Re-open before a wider launch or when a low-cost target exists (S3-compatible provider, free tier, or documented manual copy of Coolify dumps off the VPS).

**Mitigation in place today:**
- Scheduled local Coolify backup on `usestakly-postgres` (daily, 7-day retention).
- Local restore test validated 2026-05-07 (see `docs/ops-mcp-coolify-hardening.md`).

**Residual risk while deferred:** loss of the entire VPS or disk still means loss of production data unless a copy exists elsewhere.

**Goal:** Protect production data against VPS or disk loss, not only app-level mistakes.

**Files/docs to update:**
- `docs/ops-mcp-coolify-hardening.md`
- `docs/security-secrets-playbook.md` if new storage credentials are introduced
- `docs/plans/remaining-work-2026-05-03.md`

- [ ] ~~Pick the offsite target~~ — deferred; pick when budget allows: S3-compatible storage, provider backup, or another managed backup location.
- [ ] Configure Coolify backup storage for `usestakly-postgres`.
- [ ] Trigger one manual backup to the offsite target.
- [ ] Verify the backup artifact exists outside the VPS.
- [ ] Run one restore test from the offsite artifact into a temporary database/container.
- [ ] Document provider, schedule, retention, restore command, and last tested date.
- [ ] Mark offsite backup as launch-ready in `remaining-work`.

**Acceptance criteria:**
- A database backup exists outside the VPS.
- Restore from that offsite backup has been tested once.
- Secrets or access keys are stored outside git.

---

## Task 2 — Harden `POST /api/repos/{id}/refresh`

**Goal:** Keep repo profile refresh useful without letting anonymous traffic burn GitHub quota or backend compute.

**Runtime truth today:**
- Route: `backend/src/app/mod.rs`
- Handler: `backend/src/handlers/repos_refresh.rs`
- Current behavior: requires `GITHUB_TOKEN`, then `ingest_repo` → `recompute_external_artifact` → radar refresh, with in-memory cooldown per repo.
- Frontend behavior: `frontend/src/routes/repo-detail.tsx` auto-calls `refreshRepoProfile(id)` once when `ingestionStatus.structuralStale` or `!ingestionStatus.structuralComplete`; today this can happen for anonymous visitors because repo profile reading is public.

**Preferred implementation:**
- Default policy for launch: keep profile reads public, but reserve automatic refresh for authenticated sessions and add an app-level endpoint rate-limit by IP/user in addition to the existing per-repo cooldown.
- Keep anonymous users able to read repo profiles.
- Return the existing cached profile state when refresh is throttled.
- Do not rely only on `OnceLock<Mutex<HashMap>>` for launch hardening: it resets on restart and is not shared across multiple backend processes/replicas.

**Implementation recommendation:**
- Use Postgres for the launch-grade refresh guard, not Redis. The stack already depends on Postgres and the MCP write guard pattern already uses persisted event data.
- Add a migration such as `repo_refresh_events` / `artifact_refresh_attempts` with `user_id`, `artifact_id`, optional `ip_hash`, `created_at`, and `status`/`reason`.
- Enforce session auth for `POST /api/repos/{id}/refresh`.
- Enforce a DB-backed user limit, for example `10 refresh/hour/user`, and a DB-backed repo limit, for example `1 refresh/15 min/repo`.
- Keep the current memory cooldown as a best-effort fast path only.
- Treat IP limits as phase 2 or best-effort unless trusted proxy handling is confirmed. `app/mod.rs` already has `source_ip()` logic for MCP; reuse that pattern only when `X-Forwarded-For` can be trusted behind Coolify.

**Files likely touched:**
- `backend/src/app/mod.rs`
- `backend/src/handlers/repos_refresh.rs`
- possibly `backend/src/app/*` if reusing/adding a limiter
- `frontend/src/routes/repo-detail.tsx` if auth behavior changes visible UX
- `.env.example` if new limits are configurable
- `docs/architecture-backend-current.md`
- `docs/source-of-truth.md`

- [x] Implement the launch policy: anonymous read stays public; auto-refresh is session-gated; endpoint has DB-backed user/repo rate limiting; IP limit is optional/phase 2; per-repo memory cooldown remains as a secondary guard.
- [x] Add a DB-backed refresh attempt table and indexed window queries for user/repo limits (`0029_repo_refresh_events.sql`).
- [x] Add backend guard/rate-limit (`services/repos/refresh_limits.rs`, `handlers/repos_refresh.rs`).
- [x] Add tests for allowed refresh, throttled refresh, and missing `GITHUB_TOKEN` (unit tests on limit thresholds; handler returns 403 without `GITHUB_TOKEN`).
- [x] Update repo-detail behavior so anonymous visitors do not trigger background refresh, but still see the cached profile and refresh/incomplete state without noisy UI failure.
- [x] Document the final behavior and env vars (`.env.example`, `architecture-backend-current.md`, `source-of-truth.md`).

**Acceptance criteria:**
- Repeated refresh attempts cannot repeatedly call GitHub for the same repo/user/IP window.
- The primary launch guard survives backend restarts and works correctly if more than one backend instance is running.
- Existing profile read remains public.
- The docs no longer imply the endpoint is safe only because of a memory cooldown.

---

## Task 3 — GitHub Quota / Corpus Monitoring

**Goal:** Make ingestion operations observable enough before wider traffic.

**Runtime context:**
- Scheduler refreshes watchlist + stale corpus.
- Manual/profile refresh can call GitHub.
- Ingestion has ETags/backoff, but remaining quota visibility is still light.
- Some rate-limit timestamps already exist on `external_artifacts` (`github_rate_limit_reset_at`, `github_last_rate_limit_at`), so this task should aggregate existing runtime signals before adding new storage.

**Files likely touched:**
- `backend/src/services/ingestion/github.rs`
- `backend/src/services/scheduler.rs`
- `backend/src/handlers/health.rs` or admin/status handler if surfacing metrics
- `docs/ops-mcp-coolify-hardening.md`
- `docs/architecture-backend-current.md`

- [x] Log GitHub rate-limit headers on ingestion responses where available.
- [x] Add warning logs for low remaining quota and secondary rate-limit events.
- [x] Launch level: structured logs first (`x-ratelimit-remaining`, `x-ratelimit-reset`, secondary limit, `retry-after`) with warnings when low.
- [x] Admin endpoint `GET /api/admin/github/quota` (`services/ingestion/github_quota.rs`).
- [x] Public status `ingestion` block on `/api/status/public` (generic message only).
- [x] Operational runbook in `docs/ops-mcp-coolify-hardening.md` (GitHub API quota section).
- [x] Scheduler defaults documented in same runbook (`APP_INGEST_MAX_REPOS_PER_CYCLE`, stale secs).

**Acceptance criteria:**
- An operator can answer: “Are we close to GitHub quota exhaustion?”
- Secondary rate-limit events are visible without digging through raw failures.
- There is a documented response path.

---

## Task 4 — Public UX / Mobile Smoke And Polish

**Goal:** Ensure public pages are launchable on mobile and desktop without console errors or obvious density/friction issues.

This task does **not** replace R7 / deeper validation in `docs/plans/remaining-work-2026-05-03.md`. Task 4 is a manual public UX smoke before announcement; R7 remains the deeper functional track for OAuth live, full MCP agent flow, `test:e2e:real`, and ongoing release gates.

**Pages:**
- `/`
- `/discover`
- `/repos/$id`
- `/how-to-read`
- `/mcp-guide`
- `/privacy`
- `/legal`
- `/status`

- [x] Run a desktop smoke on the production frontend.
- [x] Run a mobile viewport smoke on the production frontend.
- [x] Check browser console for errors.
- [x] Verify no horizontal overflow, overlapping text, broken buttons, or missing legal/contact links.
- [x] Reduce density only where it blocks comprehension; avoid redesigning the whole app in this launch pass.
- [x] Capture before/after screenshots for any visual fixes.

**Validation:** manually checked on production desktop and mobile on 2026-05-17. No visible console errors or blocking layout issues were reported.

**Acceptance criteria:**
- The listed public pages are usable on desktop and mobile.
- No visible console errors on the smoke path.
- Any fixes are small and launch-focused.

---

## Task 5 — Real Outbound Email Notification

**Goal:** Prove the notification pipeline sends a real product email, not only a channel test.

**Runtime context:**
- Brevo SMTP is configured.
- Account channel test is validated.
- Need one real watchlist alert or digest emitted by product logic.

**Runbook:** `docs/ops-email-notification-runbook.md` (paths A/B, checklist, troubleshooting).

**Files/docs likely touched if issues appear:**
- `backend/src/services/notifications.rs`
- `backend/src/services/notification_channels.rs`
- `backend/src/services/notification_digest.rs`
- `backend/src/services/email_templates.rs`
- `docs/dev-workflow.md`
- `docs/plans/remaining-work-2026-05-03.md`

- [ ] Create/use a test account with email channel enabled.
- [ ] Watch a known repo.
- [ ] Trigger a controlled notification path in staging or production-safe conditions.
- [ ] Confirm email delivery in the inbox and/or Brevo logs.
- [ ] Confirm no duplicate/noisy delivery.
- [ ] Document the exact scenario and result.

**Acceptance criteria:**
- At least one real watchlist alert or digest email is received.
- The test does not pollute production user data beyond a controlled test account.
- The result is documented with date and environment.

---

## Task 6 — Final Launch Readiness Update

**Goal:** Keep docs honest after the hardening pass.

**Files to update:**
- `docs/plans/remaining-work-2026-05-03.md`
- `docs/ops-mcp-coolify-hardening.md`
- `docs/source-of-truth.md`
- `TODO.md` only if it still contradicts the source of truth

- [x] Mark completed blockers as done (quota L2, refresh hardening, UX smoke; email + offsite still open).
- [x] Keep live post-deploy gate marked done, not reopened.
- [ ] After deploy of quota/status changes: run `docs/validation/live-release-checklist.md` once (release verification, not a new blocker category).
- [x] `docs/source-of-truth.md` aligned (refresh, quota ops, email runbook link).
- [x] Non-blocking items remain in roadmap (`remaining-work` P3+).
- [x] Run `.\scripts\audit-doc-source-truth.ps1`.
- [x] Run `git diff --check`.

**Acceptance criteria:**
- A new agent can tell whether the project is ready for wider public beta from `source-of-truth` + `remaining-work`.
- No launch blocker is hidden in historical TODO docs.
