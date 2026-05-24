# UseStakly Account Data Deletion Flow Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give users a clear way to request or perform deletion of their UseStakly account data without corrupting public repo scores.

**Architecture:** Start with a safe manual request flow in UI and docs, then add backend self-service deletion once the data retention/anonymization rules are explicit. Use soft/anonymized handling for scoring-related public signals where deleting rows would damage score provenance.

**Tech Stack:** Rust/Axum/SQLx/Postgres backend, React account page, existing session auth, existing `users`, `agent_tokens`, `watched_artifacts`, `notifications`, `notification_channels`, `quality_signals`, `agent_token_events`.

**Implementation note (2026-05-24):** shipped directly as a self-service flow rather than stopping at the manual request panel. The UI requires typing `DELETE`; backend deletion uses a tombstone user row with a unique `deleted-user-<uuid>` username to avoid the `users.username` uniqueness conflict that a shared `deleted-user` value would create.

---

## Context

UseStakly currently has:

- OAuth login;
- account page;
- MCP token creation/revocation;
- notification channel management;
- watchlist;
- in-app notifications;
- MCP usage signals tied to token owner/user.

But there is no self-service “delete my account/data” flow. `/privacy` should therefore say deletion is handled by request until this plan ships.

---

## Data Deletion Policy

When a user requests deletion:

Delete or revoke:

- active MCP tokens;
- watchlist rows;
- notification rows;
- notification channels, including encrypted Discord webhook secrets;
- digest delivery records tied to notification channels if present;
- use-case watches and matches;
- account preferences that are not needed after deletion.

Anonymize or detach:

- `quality_signals` and `quality_signal_events` that contribute to public scoring, if removing them would rewrite public history unexpectedly;
- `agent_token_events` used for aggregate abuse/rate-limit/audit metrics.

Delete/anonymize user identity:

- email;
- username;
- display name;
- avatar URL;
- OAuth identity linkage if present.

Keep:

- public GitHub repository metadata;
- aggregate score rows not directly personal;
- migrations and operational schema.

Important: decide whether deleted users become a tombstone row like:

```text
deleted-user-<uuid-short>@deleted.usestakly.local
username = deleted-user
display_name = Deleted user
avatar_url = null
deleted_at = now()
```

or whether user rows are physically deleted after dependent rows are handled. Tombstone is safer for audit/provenance.

---

## Non-Goals

- Do not implement full GDPR export portability in the first pass unless easy.
- Do not delete public GitHub corpus data.
- Do not silently rewrite historical score provenance without an explicit recompute decision.
- Do not expose admin-only data in the user export.
- Do not build a complex preference center.

---

## Task 1: Add Manual Request Flow First

**Goal:** Give users a clear immediate path before automated deletion exists.

**Files:**
- Modify: `frontend/src/i18n/en.ts`
- Modify: `frontend/src/routes/account.tsx` or account component area
- Modify: `frontend/src/routes/privacy.tsx` only if needed after privacy plan

- [x] **Step 1: Add account copy**

Add an account section:

```text
Data deletion
To delete your UseStakly account and associated personal data, contact contact@usestakly.com from the email linked to your account. Until self-service deletion is available, requests are handled manually.
```

- [x] **Step 2: Add UI section on `/account`**

Add a small non-destructive panel. No button that deletes data yet.

Expected UI:

```text
Data deletion
Request deletion by emailing contact@usestakly.com from your account email. MCP tokens can be revoked immediately above.
```

- [x] **Step 3: Build frontend**

```powershell
cd frontend
npm run build
```

- [ ] **Step 4: Commit**

```powershell
git add frontend/src/i18n/en.ts frontend/src/routes/account.tsx frontend/src/features/account
git commit -m "docs: add account deletion request guidance"
```

---

## Task 2: Design Backend Deletion Service

**Files:**
- Create: `backend/src/services/account_deletion.rs`
- Modify: `backend/src/services/mod.rs`
- Test: `backend/src/services/account_deletion.rs`

- [x] **Step 1: Add service input/output types**

Create:

```rust
pub struct DeleteAccountPlan {
    pub user_id: Uuid,
    pub revoke_tokens: bool,
    pub delete_watchlist: bool,
    pub delete_notifications: bool,
    pub delete_channels: bool,
    pub anonymize_identity: bool,
}

pub struct DeleteAccountOutcome {
    pub revoked_tokens: u64,
    pub deleted_watchlist_rows: u64,
    pub deleted_notifications: u64,
    pub deleted_channels: u64,
    pub anonymized_user: bool,
}
```

- [x] **Step 2: Add DB transaction function**

Implement:

```rust
pub async fn delete_account_data(
    db: &PgPool,
    user_id: Uuid,
) -> Result<DeleteAccountOutcome, ApiError>
```

Inside one transaction:

1. revoke/delete `agent_tokens` for user;
2. delete `watched_artifacts`;
3. delete `notifications`;
4. delete `notification_channels`;
5. delete `use_case_watch_matches` for watches owned by user;
6. delete `use_case_watches`;
7. anonymize `users` row.

Use tombstone identity:

```sql
UPDATE users
SET email = CONCAT('deleted+', id::text, '@deleted.usestakly.local'),
    username = 'deleted-user',
    display_name = 'Deleted user',
    avatar_url = NULL,
    updated_at = NOW()
WHERE id = $1
```

If a `deleted_at` column does not exist, do not add it in this task unless a migration is explicitly added.

- [x] **Step 3: Add tests around SQL shape where feasible**

If DB integration tests are not available, add unit tests for tombstone email builder:

```rust
fn deleted_email_for(user_id: Uuid) -> String
```

Expected:

```text
deleted+<uuid>@deleted.usestakly.local
```

- [x] **Step 4: Run backend checks**

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

---

## Task 3: Add Authenticated Delete Endpoint

**Files:**
- Modify: `backend/src/handlers/account.rs`
- Modify: `backend/src/app/mod.rs`
- Modify: `frontend/src/lib/api/account.ts`
- Test: handler/service tests if patterns exist

- [x] **Step 1: Add route**

Add:

```text
DELETE /api/account
```

Requirements:

- session required;
- deletes/anonymizes current user only;
- clears session cookie on success;
- returns outcome summary without sensitive data.

- [x] **Step 2: Handler behavior**

Pseudo-flow:

```rust
let user = resolve_current_user(...).await?;
let outcome = account_deletion::delete_account_data(&state.db, user.id).await?;
let clear_cookie = clear_session_cookie(&state.config)?;
return Json(outcome) with Set-Cookie clear header;
```

- [x] **Step 3: Add frontend API function**

In `frontend/src/lib/api/account.ts`:

```ts
export function deleteAccount() {
  return apiDelete<AccountDeletionOutcome>("/api/account");
}
```

Add type in `frontend/src/lib/types.ts`:

```ts
export type AccountDeletionOutcome = {
  revokedTokens: number;
  deletedWatchlistRows: number;
  deletedNotifications: number;
  deletedChannels: number;
  anonymizedUser: boolean;
};
```

- [x] **Step 4: Run checks**

```powershell
cd backend
cargo test
cargo clippy --all-targets -- -D warnings

cd ../frontend
npm run build
```

---

## Task 4: Add Confirmed Self-Service UI

**Files:**
- Modify: `frontend/src/routes/account.tsx`
- Modify: `frontend/src/i18n/en.ts`
- Modify: `frontend/src/state/auth-store.ts` if logout state must clear manually

- [x] **Step 1: Add danger section**

UI requirements:

- visible but not prominent;
- requires typing `DELETE`;
- explains account data affected;
- warns public GitHub corpus is not deleted;
- revokes tokens and clears session.

- [x] **Step 2: Add mutation**

Use React Query mutation:

```ts
const deleteAccount = useMutation({
  mutationFn: deleteAccountApi,
  onSuccess: () => {
    clearAuthState();
    navigate({ to: "/" });
  }
});
```

Use existing auth store patterns.

- [x] **Step 3: Build frontend**

```powershell
cd frontend
npm run build
```

---

## Task 5: Update Privacy And Docs

**Files:**
- Modify: `frontend/src/i18n/en.ts`
- Modify: `docs/source-of-truth.md`
- Modify: `docs/architecture-backend-current.md`
- Modify: `docs/plans/remaining-work-2026-05-03.md`

- [x] **Step 1: Update privacy text**

Once self-service exists, replace:

```text
Until self-service deletion exists, account/data deletion is handled manually by request.
```

with:

```text
You can request deletion by email or delete your account from the account page. Deletion revokes MCP tokens, removes watchlists, notifications, and notification channels, and anonymizes account identity while preserving public GitHub corpus data and aggregate scoring provenance.
```

- [x] **Step 2: Update docs**

Document:

- endpoint `DELETE /api/account`;
- deletion/anonymization behavior;
- public corpus retained;
- token revocation included.

- [ ] **Step 3: Commit**

```powershell
git add backend/src frontend/src docs
git commit -m "feat: add account data deletion flow"
```

---

## Acceptance Criteria

- Users have at least a documented manual deletion path.
- Self-service deletion, when shipped, is session-authenticated and current-user-only.
- MCP tokens are revoked/deleted.
- Watchlist, notifications, channels, and use-case watches are deleted.
- User identity is anonymized or deleted according to the documented policy.
- Public GitHub corpus and aggregate score provenance are not accidentally destroyed.
- Privacy page accurately reflects whether deletion is manual or self-service.
