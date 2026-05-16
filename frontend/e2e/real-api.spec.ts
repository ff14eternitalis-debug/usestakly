import { expect, test } from "@playwright/test";

const repoId = "11111111-1111-4111-8111-111111111111";

test.skip(
  process.env.REAL_E2E !== "1",
  "real API E2E requires frontend/scripts/run-real-e2e.mjs"
);

test("real API flow covers discovery, watchlist, notifications, account token, and MCP", async ({
  page,
  request
}) => {
  test.setTimeout(60_000);
  await page.goto("/");
  await expect(
    page.getByRole("heading", {
      name: "UseStakly — Choose GitHub OSS with a transparent quality score."
    })
  ).toBeVisible();

  await page.goto("/discover");
  await page.getByLabel("Tool or need").fill("date picker react timezone");
  await page.getByRole("button", { name: "Recommend", exact: true }).click();
  await expect(
    page.getByRole("link", { name: /react-dates\/timezone-picker/i })
  ).toBeVisible();
  await page.getByRole("button", { name: /create watch/i }).click();
  await expect(page.getByText(/need watch created/i)).toBeVisible();

  await page.getByRole("link", { name: /react-dates\/timezone-picker/i }).click();
  await expect(page).toHaveURL(new RegExp(`/repos/${repoId}$`));
  await expect(page.getByRole("heading", { name: "timezone-picker" })).toBeVisible();
  await expect(page.getByText("Recent signals")).toBeVisible();

  await page.getByRole("button", { name: /add to watchlist/i }).click();
  await expect(page.getByRole("button", { name: /unwatch/i })).toBeVisible();

  await page
    .getByRole("navigation")
    .getByRole("link", { name: "Watchlist" })
    .click();
  await expect(page).toHaveURL(/\/watchlist$/);
  await expect(page.getByRole("heading", { name: /timezone-picker/i })).toBeVisible();
  await expect(page.getByRole("heading", { name: "Veille OSS" })).toBeVisible();
  await expect(page.getByText("date picker react timezone")).toBeVisible();

  await page
    .getByRole("navigation")
    .getByRole("link", { name: /notifications/i })
    .click();
  await expect(page).toHaveURL(/\/notifications$/);
  await expect(page.getByText("score drop")).toBeVisible();
  await expect(page.getByText("overall 0.94")).toBeVisible();
  await page.getByRole("button", { name: /mark read/i }).click();
  await expect(page.getByRole("button", { name: /mark read/i })).toBeHidden();

  await page
    .getByRole("banner")
    .getByRole("button", { name: /@usestakly-dev/ })
    .click();
  await page.getByRole("menuitem", { name: /my profile/i }).click();
  await expect(page).toHaveURL(/\/account$/);
  await page.getByPlaceholder("e.g. claude-desktop, cursor, codex").fill("real api audit");
  await page.getByRole("button", { name: /create token/i }).click();
  await expect(page.getByText("Plaintext token")).toBeVisible();
  const token = (await page.locator("code").textContent())?.trim() ?? "";
  expect(token).toMatch(/^usk_[a-f0-9]{64}$/);
  await expect(page.getByRole("heading", { name: "real api audit" })).toBeVisible();

  const apiBase = process.env.VITE_API_BASE_URL ?? "http://127.0.0.1:4000";
  const init = await request.post(`${apiBase}/mcp`, {
    headers: mcpHeaders(token),
    data: {
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2025-06-18",
        capabilities: {},
        clientInfo: { name: "usestakly-real-e2e", version: "1" }
      }
    }
  });
  expect(init.ok()).toBeTruthy();
  const sessionId = init.headers()["mcp-session-id"];
  expect(sessionId).toBeTruthy();

  const search = await request.post(`${apiBase}/mcp`, {
    headers: {
      ...mcpHeaders(token),
      "mcp-session-id": sessionId
    },
    data: {
      jsonrpc: "2.0",
      id: 2,
      method: "tools/call",
      params: {
        name: "search_github_repos",
        arguments: {
          query: "date picker react timezone",
          filter: "explore",
          limit: 2
        }
      }
    }
  });
  expect(search.ok()).toBeTruthy();
  const payload = parseMcpPayload(await search.text());
  expect(payload.error).toBeUndefined();
  const content = JSON.parse(payload.result.content[0].text);
  expect(content.count).toBeGreaterThan(0);
  expect(content.results[0].full_name).toBe("react-dates/timezone-picker");
});

function mcpHeaders(token: string): Record<string, string> {
  return {
    Authorization: `Bearer ${token}`,
    Accept: "application/json, text/event-stream",
    "Content-Type": "application/json"
  };
}

function parseMcpPayload(body: string): any {
  const trimmed = body.trim();
  if (trimmed.startsWith("data:")) {
    const dataLine = trimmed
      .split(/\r?\n/)
      .find((line) => line.startsWith("data:") && line.slice(5).trim().startsWith("{"));
    if (!dataLine) {
      throw new Error(`Unable to parse MCP SSE response: ${body.slice(0, 200)}`);
    }
    return JSON.parse(dataLine.slice(5).trim());
  }
  return JSON.parse(trimmed);
}
