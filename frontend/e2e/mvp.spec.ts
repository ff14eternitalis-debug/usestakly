import { expect, test, type Page, type Route } from "@playwright/test";

const repoId = "11111111-1111-4111-8111-111111111111";
const notificationId = "22222222-2222-4222-8222-222222222222";
const appOrigin = `http://127.0.0.1:${process.env.E2E_PORT ?? "5173"}`;

const user = {
  id: "33333333-3333-4333-8333-333333333333",
  email: "dev@example.com",
  username: "dev",
  displayName: "Dev User",
  avatarUrl: null
};

const quality = {
  freshness: 0.91,
  adoption: 0.74,
  reliability: 0.82,
  abandonment: 0.11,
  overall: 0.84,
  flags: [],
  formulaVersion: "v1.1",
  computedAt: "2026-04-24T08:00:00Z"
};

const repo = {
  artifactId: repoId,
  owner: "react-dates",
  name: "timezone-picker",
  fullName: "react-dates/timezone-picker",
  htmlUrl: "https://github.com/react-dates/timezone-picker",
  description: "Accessible React date picker with timezone-aware parsing.",
  language: "TypeScript",
  licenseSpdx: "MIT",
  topics: ["react", "datepicker", "timezone"],
  starsCount: 1840,
  forksCount: 120,
  openIssuesCount: 8,
  archived: false,
  lastCommitAt: "2026-04-20T12:00:00Z",
  quality
};

const repoProfile = {
  ...repo,
  subscribersCount: 94,
  defaultBranch: "main",
  priorsFetchedAt: "2026-04-24T08:00:00Z",
  recentSignals: [
    {
      id: "44444444-4444-4444-8444-444444444444",
      signal: "build_success",
      isPassive: true,
      evidenceUrl: null,
      evidenceDescription: "Installed in a Vite React app and passed smoke tests.",
      reviewStatus: "accepted",
      reviewNote: null,
      disputedAt: null,
      disputeReason: null,
      createdAt: "2026-04-24T07:30:00Z",
      events: []
    }
  ]
};

function json(data: unknown, status = 200) {
  return {
    status,
    headers: {
      "access-control-allow-credentials": "true",
      "access-control-allow-headers": "content-type, authorization",
      "access-control-allow-methods": "GET, POST, PATCH, DELETE, OPTIONS",
      "access-control-allow-origin": appOrigin,
      "content-type": "application/json"
    },
    body: JSON.stringify(data)
  };
}

function empty(status = 204) {
  return {
    status,
    headers: {
      "access-control-allow-credentials": "true",
      "access-control-allow-headers": "content-type, authorization",
      "access-control-allow-methods": "GET, POST, PATCH, DELETE, OPTIONS",
      "access-control-allow-origin": appOrigin
    },
    body: ""
  };
}

async function mockUseStaklyApi(page: Page, options: { authenticated: boolean }) {
  let watching = false;
  let notificationRead = false;

  async function handleApiRoute(route: Route) {
    const request = route.request();
    const url = new URL(request.url());
    const path = url.pathname;
    const method = request.method();

    if (method === "OPTIONS") {
      await route.fulfill(empty());
      return;
    }

    if (path === "/api/me") {
      if (!options.authenticated) {
        await route.fulfill(json({ error: "Unauthorized" }, 401));
        return;
      }
      await route.fulfill(json(user));
      return;
    }

    if (path === "/api/notifications/unread-count") {
      await route.fulfill(json({ unread: notificationRead ? 0 : 1 }));
      return;
    }

    if (path === "/api/repos/search") {
      await route.fulfill(
        json({
          filter: url.searchParams.get("filter") ?? "explore",
          items: [repo]
        })
      );
      return;
    }

    if (path === `/api/repos/${repoId}`) {
      await route.fulfill(json(repoProfile));
      return;
    }

    if (path === `/api/repos/${repoId}/viewer-state`) {
      await route.fulfill(json({ canDisputeSignals: false, visibleSignals: [] }));
      return;
    }

    if (path === "/api/watchlist" && method === "GET") {
      await route.fulfill(json(watching ? [watchedRepo()] : []));
      return;
    }

    if (path === "/api/watchlist" && method === "POST") {
      watching = true;
      await route.fulfill(empty());
      return;
    }

    if (path === `/api/watchlist/${repoId}` && method === "DELETE") {
      watching = false;
      await route.fulfill(empty());
      return;
    }

    if (path === "/api/notifications") {
      await route.fulfill(json([notification(notificationRead)]));
      return;
    }

    if (
      path === `/api/notifications/${notificationId}/read` &&
      method === "POST"
    ) {
      notificationRead = true;
      await route.fulfill(empty());
      return;
    }

    if (path === "/api/notifications/read-all" && method === "POST") {
      notificationRead = true;
      await route.fulfill(empty());
      return;
    }

    await route.fulfill(
      json({ error: `Unhandled mocked route: ${method} ${path}` }, 500)
    );
  }

  await page.route("http://localhost:4000/api/**", handleApiRoute);
  await page.route("http://127.0.0.1:4000/api/**", handleApiRoute);
}

function watchedRepo() {
  return {
    id: "55555555-5555-4555-8555-555555555555",
    artifactId: repoId,
    owner: repo.owner,
    name: repo.name,
    fullName: repo.fullName,
    htmlUrl: repo.htmlUrl,
    language: repo.language,
    starsCount: repo.starsCount,
    archived: repo.archived,
    lastCommitAt: repo.lastCommitAt,
    muted: false,
    watchedAt: "2026-04-24T09:00:00Z",
    overall: quality.overall,
    abandonment: quality.abandonment,
    flags: quality.flags
  };
}

function notification(read: boolean) {
  return {
    id: notificationId,
    artifactId: repoId,
    owner: repo.owner,
    name: repo.name,
    kind: "score_drop",
    payload: { prev_overall: 0.94, new_overall: 0.84 },
    createdAt: "2026-04-24T10:00:00Z",
    readAt: read ? "2026-04-24T10:05:00Z" : null
  };
}

test("anonymous users can browse discovery but protected routes redirect to login", async ({ page }) => {
  await mockUseStaklyApi(page, { authenticated: false });

  await page.goto("/watchlist");
  await expect(page).toHaveURL(/\/login$/);
  await expect(page.getByRole("heading", { name: /sign in/i })).toBeVisible();

  await page.goto("/discover");
  await page.getByRole("searchbox").fill("date picker react timezone");
  await expect(page.getByRole("heading", { name: /timezone-picker/i })).toBeVisible();

  await page.getByRole("heading", { name: /timezone-picker/i }).click();
  await expect(page).toHaveURL(new RegExp(`/repos/${repoId}$`));
  await expect(page.getByRole("heading", { name: "timezone-picker" })).toBeVisible();
  await expect(page.getByRole("link", { name: /sign in to watch this repo/i })).toBeVisible();
});

test("authenticated MVP flow covers discovery, repo profile, watchlist, and notifications", async ({ page }) => {
  await mockUseStaklyApi(page, { authenticated: true });

  await page.goto("/discover");
  await page.getByRole("searchbox").fill("date picker react timezone");
  await expect(page.getByText("Accessible React date picker")).toBeVisible();

  await page.getByRole("heading", { name: /timezone-picker/i }).click();
  await expect(page.getByRole("heading", { name: "timezone-picker" })).toBeVisible();
  await expect(page.getByText("Recent signals")).toBeVisible();

  await page.getByRole("button", { name: /add to watchlist/i }).click();
  await expect(page.getByRole("button", { name: /unwatch/i })).toBeVisible();

  await page
    .getByRole("navigation")
    .getByRole("link", { name: /^watchlist$/i })
    .click();
  await expect(page).toHaveURL(/\/watchlist$/);
  await expect(page.getByRole("heading", { name: /timezone-picker/i })).toBeVisible();
  await expect(page.getByText("TypeScript")).toBeVisible();

  await page
    .getByRole("navigation")
    .getByRole("link", { name: /notifications/i })
    .click();
  await expect(page).toHaveURL(/\/notifications$/);
  await expect(page.getByText("score drop")).toBeVisible();
  await expect(page.getByText("overall 0.94")).toBeVisible();

  await page.getByRole("button", { name: /mark read/i }).click();
  await expect(page.getByRole("button", { name: /mark read/i })).toBeHidden();

  await page.getByRole("link", { name: /react-dates\/timezone-picker/i }).click();
  await expect(page).toHaveURL(new RegExp(`/repos/${repoId}$`));
  await expect(page.getByRole("heading", { name: "timezone-picker" })).toBeVisible();
});
