# UseStakly Umami Analytics Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add privacy-friendly product analytics so UseStakly can understand public usage without invasive tracking.

**Architecture:** Use Umami as a small optional frontend integration. Keep analytics disabled unless explicit environment variables are configured. Track aggregate page views first, then a very small set of non-sensitive product events.

**Tech Stack:** React 19, Vite env vars, TanStack Router, optional Umami script, Coolify env config.

---

## Context

UseStakly is currently mostly blind:

- no page view analytics;
- no referral visibility after Discord/GitHub sharing;
- no funnel view for `/discover`, `/repos/$id`, `/mcp-guide`, `/account`;
- no aggregate view of CTA usage.

Umami is a good fit because it is lightweight, privacy-focused, self-hostable, and cookie-free by default. For RGPD posture, UseStakly should avoid sending personal data or user ids to analytics events.

---

## Non-Goals

- Do not track emails, OAuth ids, MCP tokens, admin tokens, Discord webhook URLs, IP addresses manually, or free-form notes.
- Do not add session replay, heatmaps, ad tracking, cross-site profiling, or marketing pixels.
- Do not require analytics for app functionality.
- Do not add a cookie banner unless the selected deployment/configuration requires consent.
- Do not send raw search queries if they may become sensitive; use coarse events first.

---

## Recommended Deployment

Preferred:

- self-host Umami on Coolify if affordable and easy;
- use a dedicated Umami database;
- expose it on a subdomain such as `analytics.usestakly.com`;
- keep the Umami dashboard admin-only.

Acceptable temporary option:

- Umami Cloud, if budget allows and the privacy page names Umami as a processor.

---

## Event Policy

Allowed events:

```text
discover_open
discover_search_submit
repo_detail_open
mcp_guide_open
mcp_token_create_click
watch_repo_click
add_repo_submit
```

Allowed event properties:

```text
route
repo_source: "discover" | "direct" | "watchlist"
radar_mode: "reliable" | "emerging"
auth_state: "anonymous" | "signed_in"
```

Avoid:

```text
email
user id
token
admin token
OAuth provider id
Discord webhook
free-form notes
raw MCP token labels
raw search queries until reviewed
```

---

## Task 1: Add Optional Umami Script Config

**Files:**
- Modify: `.env.example`
- Modify: `deploy/coolify/frontend.env.example`
- Modify: `frontend/src/main.tsx` or create `frontend/src/lib/analytics.ts`

- [ ] **Step 1: Add frontend env vars**

Add to `.env.example` and `deploy/coolify/frontend.env.example`:

```env
VITE_UMAMI_SCRIPT_URL=
VITE_UMAMI_WEBSITE_ID=
```

Do not put real IDs in committed files.

- [ ] **Step 2: Create analytics helper**

Create `frontend/src/lib/analytics.ts`:

```ts
declare global {
  interface Window {
    umami?: {
      track: (eventName: string, eventData?: Record<string, string>) => void;
    };
  }
}

const enabled =
  Boolean(import.meta.env.VITE_UMAMI_SCRIPT_URL) &&
  Boolean(import.meta.env.VITE_UMAMI_WEBSITE_ID);

export function analyticsEnabled() {
  return enabled;
}

export function trackEvent(
  eventName: string,
  eventData?: Record<string, string>
) {
  if (!enabled || typeof window === "undefined") return;
  window.umami?.track(eventName, eventData);
}
```

- [ ] **Step 3: Inject Umami script only when configured**

In `frontend/src/main.tsx`, before rendering or in a tiny helper, add:

```ts
function installUmamiScript() {
  const scriptUrl = import.meta.env.VITE_UMAMI_SCRIPT_URL;
  const websiteId = import.meta.env.VITE_UMAMI_WEBSITE_ID;
  if (!scriptUrl || !websiteId || document.querySelector("script[data-website-id]")) {
    return;
  }

  const script = document.createElement("script");
  script.defer = true;
  script.src = scriptUrl;
  script.setAttribute("data-website-id", websiteId);
  document.head.appendChild(script);
}

installUmamiScript();
```

Expected: no script is injected locally unless env vars are set.

- [ ] **Step 4: Build frontend**

```powershell
cd frontend
npm run build
```

Expected: build passes.

---

## Task 2: Add Minimal Product Events

**Files:**
- Modify: `frontend/src/routes/discover.tsx`
- Modify: `frontend/src/routes/repo-detail.tsx`
- Modify: `frontend/src/routes/mcp-guide.tsx`
- Modify: `frontend/src/features/repos/components/DiscoverFilters.tsx` only if submit/click lives there
- Modify: `frontend/src/features/repos/components/RepoHeader.tsx` or watch button owner only if needed

- [ ] **Step 1: Track page-level openings**

Add route-level `useEffect` events:

```ts
useEffect(() => {
  trackEvent("discover_open", { route: "/discover" });
}, []);
```

```ts
useEffect(() => {
  trackEvent("mcp_guide_open", { route: "/mcp-guide" });
}, []);
```

```ts
useEffect(() => {
  trackEvent("repo_detail_open", { route: "/repos/:id" });
}, []);
```

Do not include the raw repo id in the event unless reviewed.

- [ ] **Step 2: Track coarse actions**

Track only button/action events:

```ts
trackEvent("discover_search_submit", { route: "/discover" });
trackEvent("add_repo_submit", { route: "/discover" });
trackEvent("mcp_token_create_click", { route: "/mcp-guide" });
trackEvent("watch_repo_click", { route: "/repos/:id" });
```

Do not send raw query text, repo names, token labels, or error messages.

- [ ] **Step 3: Build frontend**

```powershell
cd frontend
npm run build
```

Expected: build passes.

---

## Task 3: Update Privacy Copy

**Files:**
- Modify: `frontend/src/i18n/en.ts`
- Depends on: `docs/plans/privacy-page-rgpd-update-2026-05-24.md`

- [ ] **Step 1: Update analytics section only after Umami is configured**

If Umami is enabled, update `/privacy` analytics section to:

```text
UseStakly uses privacy-friendly analytics to understand aggregate product usage: page views, referrers, device/browser category, and a small set of non-sensitive product events. No email, OAuth id, MCP token, admin token, private source code, or free-form note is sent to analytics.
```

If self-hosted, mention:

```text
Analytics is hosted by UseStakly.
```

If Umami Cloud is used, mention:

```text
Umami is used as an analytics processor.
```

- [ ] **Step 2: Build frontend**

```powershell
cd frontend
npm run build
```

---

## Task 4: Deployment And Verification

**Files:**
- No code change required after env config.

- [ ] **Step 1: Configure env vars**

In Coolify frontend env:

```env
VITE_UMAMI_SCRIPT_URL=https://analytics.usestakly.com/script.js
VITE_UMAMI_WEBSITE_ID=<umami website id>
```

Use the actual script URL from your Umami instance.

- [ ] **Step 2: Redeploy frontend**

Redeploy only the frontend.

- [ ] **Step 3: Browser verify**

Open production:

```text
/
/discover
/mcp-guide
```

Verify:

- no console errors;
- network shows Umami script loaded;
- Umami dashboard receives page views;
- no event payload contains personal data.

- [ ] **Step 4: Commit**

```powershell
git add .env.example deploy/coolify/frontend.env.example frontend/src
git commit -m "feat: add optional privacy-friendly analytics"
```

---

## Acceptance Criteria

- Analytics is disabled unless env vars are configured.
- No personal data is sent in event names or properties.
- `/privacy` documents analytics once enabled.
- Frontend build passes.
- Production dashboard receives aggregate page views and allowed events only.
