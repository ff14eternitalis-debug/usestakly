import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const bin = path.resolve("bin", "usestakly-mcp.mjs");
const token = "usk_0000000000000000000000000000000000000000000000000000000000000000";

function run(args, env = {}) {
  return execFileSync(process.execPath, [bin, ...args], {
    cwd: path.resolve("."),
    encoding: "utf8",
    env: { ...process.env, ...env },
    stdio: ["ignore", "pipe", "pipe"]
  });
}

test("generic install prints MCP JSON", () => {
  const output = run(["install", "--client", "generic", "--token", token]);
  const parsed = JSON.parse(output);
  assert.equal(parsed.mcpServers.usestakly.type, "streamable-http");
  assert.equal(
    parsed.mcpServers.usestakly.url,
    "https://xl4xtxfxbxm0lvqjywsl98il.137.74.112.197.sslip.io/mcp"
  );
  assert.equal(parsed.mcpServers.usestakly.headers.Authorization, `Bearer ${token}`);
});

test("codex install writes config and removes previous usestakly section", () => {
  const home = fs.mkdtempSync(path.join(os.tmpdir(), "usestakly-cli-"));
  const configPath = path.join(home, "config.toml");
  fs.writeFileSync(
    configPath,
    [
      'model = "gpt-5"',
      "",
      "[mcp_servers.usestakly]",
      'type = "old"',
      "",
      "[mcp_servers.usestakly.headers]",
      'Authorization = "Bearer old"',
      "",
      "[other]",
      'keep = "yes"',
      ""
    ].join("\n")
  );

  run(
    [
      "install",
      "--client",
      "codex",
      "--endpoint",
      "https://example.com/mcp",
      "--token",
      token
    ],
    { CODEX_HOME: home }
  );

  const written = fs.readFileSync(configPath, "utf8");
  assert.match(written, /model = "gpt-5"/);
  assert.match(written, /\[other\]/);
  assert.match(written, /\[mcp_servers\.usestakly\]/);
  assert.match(written, /\[mcp_servers\.usestakly\.http_headers\]/);
  assert.match(written, /url = "https:\/\/example\.com\/mcp"/);
  assert.doesNotMatch(written, /type = "old"/);
  assert.equal(fs.readdirSync(home).some((file) => file.includes(".bak-")), true);
});

test("dry run does not write codex config", () => {
  const home = fs.mkdtempSync(path.join(os.tmpdir(), "usestakly-cli-"));
  const output = run(
    ["install", "--client", "codex", "--token", token, "--dry-run"],
    { CODEX_HOME: home }
  );

  assert.match(output, /Dry run: no file written/);
  assert.equal(fs.existsSync(path.join(home, "config.toml")), false);
});

test("invalid token exits with a clear error", () => {
  assert.throws(
    () => run(["install", "--client", "generic", "--token", "usk_bad"]),
    /Expected a UseStakly token formatted as usk_<64 hex>/
  );
});
