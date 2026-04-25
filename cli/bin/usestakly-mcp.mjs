#!/usr/bin/env node
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import readline from "node:readline/promises";
import { stdin as input, stdout as output } from "node:process";

const DEFAULT_ENDPOINT =
  "https://xl4xtxfxbxm0lvqjywsl98il.137.74.112.197.sslip.io/mcp";
const CLIENTS = ["codex", "cursor", "claude", "generic"];

async function main() {
  const [command = "help", ...rest] = process.argv.slice(2);
  const options = parseArgs(rest);

  if (options.help || options.h) {
    printHelp(command);
    return;
  }

  if (command === "install") {
    await install(options);
    return;
  }
  if (command === "test") {
    await test(options);
    return;
  }
  if (command === "doctor") {
    await doctor(options);
    return;
  }
  printHelp("help");
}

function parseArgs(args) {
  const options = {};
  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i];
    if (!arg.startsWith("--")) continue;
    const key = arg.slice(2);
    const next = args[i + 1];
    if (!next || next.startsWith("--")) {
      options[key] = true;
      continue;
    }
    options[key] = next;
    i += 1;
  }
  return options;
}

async function install(options) {
  const rl = createPrompt();
  try {
    const client = await resolveClient(rl, options.client);
    const endpoint = await resolveEndpoint(rl, options.endpoint);
    const token = await resolveToken(rl, options);
    const config = buildMcpConfig(endpoint, token);

    if (client === "generic" || options["dry-run"]) {
      output.write(`${JSON.stringify(config, null, 2)}\n`);
      if (options["dry-run"] && client !== "generic") {
        output.write(`Dry run: no file written. Target would be: ${configPathFor(client)}\n`);
      }
      return;
    }

    const target = configPathFor(client);
    ensureParentDir(target);
    backupIfExists(target);

    if (client === "codex") {
      writeCodexConfig(target, endpoint, token);
    } else {
      writeJsonConfig(target, config);
    }

    output.write(`UseStakly MCP installed for ${client}.\n`);
    output.write(`Updated: ${target}\n`);
    output.write("Restart your MCP client, then run: npx usestakly-mcp test\n");
  } finally {
    rl.close();
  }
}

async function test(options) {
  const rl = createPrompt();
  try {
    const endpoint = await resolveEndpoint(rl, options.endpoint);
    const token = await resolveToken(rl, options);
    await initializeMcp(endpoint, token);
    output.write("UseStakly MCP is reachable and the token is valid.\n");
  } finally {
    rl.close();
  }
}

async function doctor(options) {
  const endpoint = options.endpoint || DEFAULT_ENDPOINT;
  output.write(`Endpoint: ${endpoint}\n`);
  for (const client of CLIENTS.filter((item) => item !== "generic")) {
    const target = configPathFor(client);
    const exists = fs.existsSync(target);
    const content = exists ? fs.readFileSync(target, "utf8") : "";
    const hasUseStakly = content.includes("usestakly");
    const hasEndpoint = content.includes(endpoint);
    output.write(
      `${client}: ${exists ? target : "not found"}${
        exists ? ` (${hasUseStakly ? "configured" : "no usestakly server"}, ${hasEndpoint ? "endpoint ok" : "endpoint missing"})` : ""
      }\n`
    );
  }
}

function createPrompt() {
  return readline.createInterface({ input, output });
}

async function resolveClient(rl, rawClient) {
  const client = String(rawClient || "").toLowerCase();
  if (CLIENTS.includes(client)) return client;
  if (client) {
    throw new Error(`Unsupported client "${client}". Use: ${CLIENTS.join(", ")}.`);
  }
  if (!input.isTTY) return "codex";

  output.write("Choose an MCP client:\n");
  CLIENTS.forEach((item, index) => output.write(`  ${index + 1}. ${item}\n`));
  const answer = await rl.question("Client [1]: ");
  const index = Number(answer || "1") - 1;
  return CLIENTS[index] || "codex";
}

async function resolveEndpoint(rl, rawEndpoint) {
  if (rawEndpoint) return validateEndpoint(String(rawEndpoint));
  if (!input.isTTY) return DEFAULT_ENDPOINT;
  const answer = await rl.question(`Endpoint [${DEFAULT_ENDPOINT}]: `);
  return validateEndpoint(answer.trim() || DEFAULT_ENDPOINT);
}

async function resolveToken(rl, options) {
  if (options["token-env"]) {
    const token = process.env[String(options["token-env"])];
    if (!token) {
      throw new Error(`Environment variable ${options["token-env"]} is empty.`);
    }
    return validateToken(token.trim());
  }
  if (options.token) return validateToken(String(options.token).trim());
  if (!input.isTTY) {
    throw new Error("Missing token. Use --token-env USESTAKLY_MCP_TOKEN or run interactively.");
  }

  const token = await rl.question("UseStakly MCP token (usk_...): ");
  return validateToken(token.trim());
}

function validateEndpoint(endpoint) {
  let parsed;
  try {
    parsed = new URL(endpoint);
  } catch {
    throw new Error("Endpoint must be a valid http(s) URL.");
  }
  if (!["http:", "https:"].includes(parsed.protocol)) {
    throw new Error("Endpoint must use http or https.");
  }
  if (!parsed.pathname.endsWith("/mcp")) {
    throw new Error("Endpoint should end with /mcp.");
  }
  return parsed.toString();
}

function validateToken(token) {
  if (!/^usk_[a-fA-F0-9]{64}$/.test(token)) {
    throw new Error("Expected a UseStakly token formatted as usk_<64 hex>.");
  }
  return token;
}

function buildMcpConfig(endpoint, token) {
  return {
    mcpServers: {
      usestakly: {
        type: "streamable-http",
        url: endpoint,
        headers: {
          Authorization: `Bearer ${token}`
        }
      }
    }
  };
}

function configPathFor(client) {
  const home = os.homedir();
  if (client === "codex") {
    return path.join(process.env.CODEX_HOME || path.join(home, ".codex"), "config.toml");
  }
  if (client === "cursor") {
    return path.join(home, ".cursor", "mcp.json");
  }
  if (client === "claude") {
    if (process.platform === "win32") {
      return path.join(
        process.env.APPDATA || path.join(home, "AppData", "Roaming"),
        "Claude",
        "claude_desktop_config.json"
      );
    }
    if (process.platform === "darwin") {
      return path.join(
        home,
        "Library",
        "Application Support",
        "Claude",
        "claude_desktop_config.json"
      );
    }
    return path.join(home, ".config", "Claude", "claude_desktop_config.json");
  }
  throw new Error(`Unsupported client: ${client}`);
}

function ensureParentDir(filePath) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
}

function backupIfExists(filePath) {
  if (!fs.existsSync(filePath)) return;
  const stamp = new Date().toISOString().replace(/[:.]/g, "-");
  fs.copyFileSync(filePath, `${filePath}.bak-${stamp}`);
}

function writeJsonConfig(filePath, mcpConfig) {
  const current = readJsonObject(filePath);
  const next = {
    ...current,
    mcpServers: {
      ...(current.mcpServers || {}),
      ...mcpConfig.mcpServers
    }
  };
  fs.writeFileSync(filePath, `${JSON.stringify(next, null, 2)}\n`);
}

function readJsonObject(filePath) {
  if (!fs.existsSync(filePath)) return {};
  const raw = fs.readFileSync(filePath, "utf8").trim();
  if (!raw) return {};
  return JSON.parse(raw);
}

function writeCodexConfig(filePath, endpoint, token) {
  const current = fs.existsSync(filePath) ? fs.readFileSync(filePath, "utf8") : "";
  const withoutUseStakly = removeTomlSection(current, "mcp_servers.usestakly").trimEnd();
  const block = [
    "[mcp_servers.usestakly]",
    'type = "streamable-http"',
    `url = "${escapeToml(endpoint)}"`,
    "",
    "[mcp_servers.usestakly.headers]",
    `Authorization = "Bearer ${escapeToml(token)}"`
  ].join("\n");
  const next = withoutUseStakly ? `${withoutUseStakly}\n\n${block}\n` : `${block}\n`;
  fs.writeFileSync(filePath, next);
}

function removeTomlSection(source, sectionName) {
  const lines = source.split(/\r?\n/);
  const kept = [];
  let skipping = false;

  for (const line of lines) {
    const match = line.trim().match(/^\[([^\]]+)\]$/);
    if (match) {
      const current = match[1];
      skipping = current === sectionName || current.startsWith(`${sectionName}.`);
    }
    if (!skipping) kept.push(line);
  }

  return kept.join("\n");
}

function escapeToml(value) {
  return String(value).replaceAll("\\", "\\\\").replaceAll('"', '\\"');
}

async function initializeMcp(endpoint, token) {
  const initResponse = await sendMcpRequest(endpoint, token, {
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      protocolVersion: "2025-06-18",
      capabilities: {},
      clientInfo: { name: "usestakly-mcp-cli", version: "0.1.1" }
    }
  });
  const sessionId = initResponse.headers.get("mcp-session-id");
  const initBody = await initResponse.text();
  assertMcpResponseOk(initResponse, initBody, "MCP initialize failed");

  const toolResponse = await sendMcpRequest(
    endpoint,
    token,
    {
      jsonrpc: "2.0",
      id: 2,
      method: "tools/call",
      params: {
        name: "search_github_repos",
        arguments: {
          query: "react",
          filter: "explore",
          limit: 1
        }
      }
    },
    sessionId
  );
  const toolBody = await toolResponse.text();
  assertMcpResponseOk(toolResponse, toolBody, "MCP protected tool call failed");
}

async function sendMcpRequest(endpoint, token, body, sessionId) {
  const headers = {
    Authorization: `Bearer ${token}`,
    "Content-Type": "application/json",
    Accept: "application/json, text/event-stream"
  };
  if (sessionId) headers["mcp-session-id"] = sessionId;

  return fetch(endpoint, {
    method: "POST",
    headers,
    body: JSON.stringify(body)
  });
}

function assertMcpResponseOk(response, body, label) {
  if (!response.ok) {
    throw new Error(`${label}: HTTP ${response.status} ${body.slice(0, 200)}`);
  }

  const payload = parseMcpPayload(body);
  if (payload?.error) {
    const message =
      typeof payload.error.message === "string"
        ? payload.error.message
        : JSON.stringify(payload.error);
    throw new Error(`${label}: ${message}`);
  }
}

function parseMcpPayload(body) {
  const trimmed = body.trim();
  if (!trimmed) return null;
  if (trimmed.startsWith("data:")) {
    const dataLine = trimmed
      .split(/\r?\n/)
      .find((line) => line.startsWith("data:") && line.slice(5).trim().startsWith("{"));
    if (!dataLine) return null;
    return JSON.parse(dataLine.slice(5).trim());
  }
  return JSON.parse(trimmed);
}

function printHelp(command) {
  const common = `Options:
  --endpoint https://.../mcp
  --token-env USESTAKLY_MCP_TOKEN
  --token usk_...
  --help

Prefer interactive input or --token-env over --token to avoid shell history leaks.
`;

  if (command === "install") {
    output.write(`UseStakly MCP install

Usage:
  npx usestakly-mcp install
  npx usestakly-mcp install --client codex --token-env USESTAKLY_MCP_TOKEN

Options:
  --client codex|cursor|claude|generic
  --dry-run
${common}`);
    return;
  }

  if (command === "test") {
    output.write(`UseStakly MCP test

Usage:
  npx usestakly-mcp test
  npx usestakly-mcp test --token-env USESTAKLY_MCP_TOKEN

${common}`);
    return;
  }

  if (command === "doctor") {
    output.write(`UseStakly MCP doctor

Usage:
  npx usestakly-mcp doctor

Options:
  --endpoint https://.../mcp
  --help
`);
    return;
  }

  output.write(`UseStakly MCP CLI

Usage:
  npx usestakly-mcp install
  npx usestakly-mcp test
  npx usestakly-mcp doctor

Run a command with --help for details.
`);
}

main().catch((error) => {
  console.error(`Error: ${error.message}`);
  process.exitCode = 1;
});
