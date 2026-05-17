# Email notification runbook (launch hardening Task 5)

> Prove one **real product email** (watchlist alert or digest), not only the account channel test.
> Channel test = `POST /api/account/notification-channels/{id}/test` (template "test").
> Product email = emitted by scheduler/recompute logic (`notifications.rs`, `notification_digest.rs`).

## Prerequisites

- `APP_EMAIL_SMTP_HOST`, `APP_EMAIL_SMTP_PORT`, `APP_EMAIL_SMTP_USERNAME`, `APP_EMAIL_SMTP_PASSWORD` set on the backend (Brevo SMTP relay in prod).
- `APP_EMAIL_FROM_ADDRESS` / `APP_EMAIL_FROM_NAME` valid for the provider.
- `APP_NOTIFICATION_SECRET` set (encrypts email destinations in `notification_channels`).
- `GITHUB_TOKEN` + scheduler or manual recompute available if you trigger via score change.
- Use a **dedicated test account** (OAuth) with an inbox you control; do not spam real users.

## Path A — Watchlist alert (recommended)

**What it proves:** `services/notifications::detect_and_emit` after a score recompute.

1. Sign in on production (or staging with real SMTP) as the test user.
2. **Account** → notification channels → add **email** channel, verify address, run **Test** once (sanity only).
3. **Watchlist** → watch a repo that already has a score in DB (pick one you can refresh).
4. Trigger a recompute that changes `overall` or raises a flag:
   - **Option 4a (operator):** `POST /api/admin/scoring/recompute` with `x-admin-token` (full corpus; heavy).
   - **Option 4b (lighter):** `POST /api/repos/{id}/refresh` while signed in (refreshes one repo, then recompute for that artifact) if GitHub quota allows.
   - **Option 4c (staging):** temporarily lower a score in DB for the watched artifact, then run recompute (only on non-prod).
5. Check **in-app** `/notifications` for a new row (confirms product logic fired).
6. Check **inbox** (and Brevo transactional logs) for the email within a few minutes.
7. Confirm **no duplicate** burst (same event should not spam multiple emails).

**Expected notification kinds** (see `services/notifications.rs`): score drop, abandonment rise, severe flag, etc., depending on what changed.

## Path B — Daily digest email

**What it proves:** `services/notification_digest.rs` + `send_email` on digest channel.

1. Same test account with email channel enabled and `digest_time_local` / timezone set on the channel row.
2. Ensure the account has watchlist entries and recent in-app notifications (or run a cycle that creates them).
3. Wait for the digest scheduler (`APP_DIGEST_INTERVAL_SECS`, default 30 min loop) or trigger digest logic on staging if you have a manual hook.
4. Confirm one digest email per channel per local day (`notification_digest_deliveries` dedupes).

Path B is slower; prefer Path A for launch proof.

## Verification checklist

| Step | Pass |
|------|------|
| Channel test email received earlier | |
| Product email received (watch or digest) | |
| In-app notification matches email event | |
| Brevo (or provider) log shows delivered / not bounced | |
| No duplicate emails for same logical event | |
| Documented date, environment, scenario below | |

## Record the result (required for Task 5)

Add to `docs/plans/remaining-work-2026-05-03.md` or `docs/plans/public-launch-hardening-2026-05-17.md`:

```text
Email product proof — YYYY-MM-DD — prod|staging
Scenario: watchlist / <owner>/<repo> / <trigger>
In-app notification id or kind: ...
Brevo message id or subject: ...
Result: delivered | failed (reason)
```

## Troubleshooting

| Symptom | Check |
|---------|--------|
| Test works, product email never arrives | `detect_and_emit` only runs after recompute; confirm watchlist + score delta; logs `failed to emit notifications`. |
| SMTP auth error | `APP_EMAIL_SMTP_*`, Brevo IP allowlist, credentials in Infisical/Coolify (not git). |
| Email channel missing | `notification_channels` row, `enabled = true`, destination decrypts with `APP_NOTIFICATION_SECRET`. |
| Duplicate sends | `notification_digest_deliveries` for digest; in-app dedup in `notifications` insert logic. |

## Code map

- Channel CRUD + test: `handlers/notification_channels.rs`, `services/notification_channels.rs`
- Watchlist alerts: `services/notifications.rs` (`detect_and_emit`)
- Digest: `services/notification_digest.rs`
- Templates: `services/email_templates.rs`
- Scheduler: `services/scheduler.rs` (recompute + digest loops)
