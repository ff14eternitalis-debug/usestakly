# UseStakly Privacy Page RGPD Update Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `/privacy` accurately explain UseStakly data processing before broader public sharing and before adding analytics.

**Architecture:** Keep this as a documentation/product-copy change unless a missing UI structure blocks clear presentation. The privacy page remains a public frontend route backed by `frontend/src/i18n/en.ts`; no backend data model change is required in this plan.

**Tech Stack:** React 19, TanStack Router, TypeScript, Vite, frontend i18n in `frontend/src/i18n/en.ts`.

---

## Context

Current `/privacy` is honest but too short for a public beta:

- it lists OAuth identity, watchlist/notifications, MCP tokens, and usage signals;
- it does not clearly name the controller/contact;
- it does not describe purposes, legal bases, retention, user rights, processors, cookies/session, analytics posture, or deletion requests;
- it does not mention Brevo/email delivery, GitHub/Discord OAuth providers, Coolify/VPS hosting, or future analytics.

This plan is not legal advice. It creates a clear good-faith privacy notice suitable for a small public beta and leaves lawyer review as optional future hardening.

---

## Non-Goals

- Do not implement account deletion or export here; that is covered by `docs/plans/account-data-deletion-flow-2026-05-24.md`.
- Do not add Umami here; that is covered by `docs/plans/umami-analytics-integration-2026-05-24.md`.
- Do not add a cookie consent banner in this task.
- Do not promise features that do not exist yet, such as self-service deletion/export.
- Do not claim legal certification or full GDPR compliance.

---

## Target Content

The updated page should cover:

1. **Controller/contact**
   - UseStakly operates the service.
   - Contact: `contact@usestakly.com`.

2. **Data collected**
   - OAuth identity: provider user id, username, avatar, email when returned.
   - Session cookie: `usestakly_session`, HTTP-only, used for login.
   - Watchlist and notifications.
   - MCP tokens: plaintext once, hash stored.
   - MCP usage signals: repo owner/name, outcome, timestamp, token owner, optional notes.
   - Notification channels: email destination, encrypted Discord webhook destination.
   - Public GitHub metadata about repositories.

3. **Purposes**
   - provide login/account;
   - run repo scoring/discovery;
   - maintain watchlist and notifications;
   - secure MCP access;
   - improve scoring from usage signals;
   - operate/debug the public beta.

4. **Legal bases**
   - service contract / requested service for account, session, watchlist, MCP token;
   - legitimate interest for security, abuse prevention, aggregate product operation;
   - consent or user action for optional notification channels and future analytics if configured that way.

5. **Processors/third parties**
   - GitHub and Discord for OAuth / public repo data;
   - Brevo or SMTP provider for email notifications if configured;
   - infrastructure host/VPS/Coolify database environment;
   - Umami only after the analytics plan is shipped.

6. **Retention**
   - account data kept while account exists;
   - tokens kept until revoked;
   - watchlist/notification data kept while needed for the service;
   - operational logs kept only as needed for security/debugging;
   - public GitHub repo metadata kept as part of the public corpus.

7. **User rights**
   - access, correction, deletion, objection where applicable;
   - contact email;
   - deletion currently handled by request until self-service flow exists.

8. **Analytics status**
   - currently no invasive third-party analytics;
   - if Umami is added, it will be documented before/when enabled;
   - no marketing mailing list.

---

## Task 1: Update i18n Privacy Copy

**Files:**
- Modify: `frontend/src/i18n/en.ts`

- [x] **Step 1: Replace the current short `privacy` object**

Replace `privacy.intro`, `privacy.sections`, and `privacy.closing` with expanded sections.

Use copy with this structure:

```ts
privacy: {
  eyebrow: "Privacy",
  h1: "How UseStakly handles data",
  intro:
    "UseStakly is a public beta for GitHub repository discovery, watchlists, notifications, and MCP access. This page explains what data is used to run the service.",
  sections: [
    {
      title: "Controller and contact",
      body:
        "UseStakly operates this service. For privacy, legal, security, or product questions, contact contact@usestakly.com."
    },
    {
      title: "Account and session",
      body:
        "GitHub or Discord OAuth is used for login. UseStakly stores your provider user id, username, avatar, and email when the provider returns one. The usestakly_session cookie is HTTP-only and is used only to keep you signed in."
    },
    {
      title: "Watchlist and notifications",
      body:
        "Repos you watch, notification read state, and optional notification channels are stored so UseStakly can alert you when scores drift. Email destinations are stored for email alerts; Discord webhook URLs are encrypted at rest."
    },
    {
      title: "MCP tokens",
      body:
        "Agent tokens use the usk_ format. Plaintext is shown once, then only a SHA-256 hash is stored server-side. Tokens can be revoked from the account page."
    },
    {
      title: "Usage signals",
      body:
        "MCP log_usage and user reports can store repo owner/name, outcome, timestamp, token owner, and optional notes so scores can improve with real usage. Do not include secrets or private source code in notes."
    },
    {
      title: "Public GitHub metadata",
      body:
        "UseStakly stores public GitHub repository metadata such as owner, name, topics, language, release activity, CI signals, and freshness inputs. Private repositories and private source code are not ingested."
    },
    {
      title: "Service providers",
      body:
        "UseStakly relies on GitHub and Discord for OAuth, GitHub for public repository metadata, infrastructure hosting for the app and database, and an email provider such as Brevo when outbound notifications are enabled."
    },
    {
      title: "Legal bases and retention",
      body:
        "Account, session, watchlist, MCP, and notification data are processed to provide the service you request. Security and abuse-prevention data are processed for legitimate operational interest. Data is kept only as long as needed for the public beta, security, and scoring integrity."
    },
    {
      title: "Your rights",
      body:
        "You can ask for access, correction, deletion, or objection where applicable by contacting contact@usestakly.com. Until self-service deletion exists, account/data deletion is handled manually by request."
    },
    {
      title: "Analytics",
      body:
        "UseStakly does not use invasive marketing analytics or a marketing mailing list. If privacy-friendly analytics such as Umami are enabled, this page will describe what is collected and how to opt out if needed."
    }
  ],
  closing:
    "UseStakly is a beta service, not a legal or security certification system. Scores are transparent decision aids and should not replace your own dependency review."
}
```

- [x] **Step 2: Keep `/privacy` layout readable**

If the expanded text feels cramped, adjust `frontend/src/routes/privacy.tsx` only for layout:

```tsx
<article className="shell-narrow grid gap-12 py-12 md:py-16">
```

and consider a wider title:

```tsx
<h1 className="display-lg max-w-[20ch]">{t.privacy.h1}</h1>
```

Do not add decorative cards or nested cards.

- [x] **Step 3: Build frontend**

Run:

```powershell
cd frontend
npm run build
```

Expected: build passes.

- [x] **Step 4: Manual page check**

Run the frontend locally or use existing dev server:

```powershell
cd frontend
npm run dev
```

Open `/privacy` and verify:

- all sections render;
- no text overlaps on desktop/mobile;
- contact email is visible;
- page does not imply self-service deletion exists.

- [x] **Step 5: Commit**

```powershell
git add frontend/src/i18n/en.ts frontend/src/routes/privacy.tsx
git commit -m "docs: expand privacy notice"
```

---

## Acceptance Criteria

- `/privacy` explains controller/contact, data categories, purposes, providers, retention, rights, and analytics posture.
- It does not claim full legal certification.
- It does not promise unimplemented self-service deletion/export.
- Frontend build passes.
