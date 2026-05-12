import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const frontendDir = path.resolve(fileURLToPath(import.meta.url), "..", "..");
const repoRoot = path.resolve(frontendDir, "..");
const backendDir = path.join(repoRoot, "backend");
const seedPath = path.join(frontendDir, "e2e", "real-api-seed.sql");
const port = process.env.E2E_PORT || "5173";
const apiPort = process.env.REAL_E2E_API_PORT || "4100";
const apiBase = `http://127.0.0.1:${apiPort}`;

let backendProcess;
let stoppingBackend = false;

try {
  await run("docker", ["compose", "up", "-d"], { cwd: repoRoot });
  backendProcess = startBackend();
  await waitForHealth(`${apiBase}/health`, 60_000);
  await seedDatabase();
  await run(process.execPath, [playwrightCli(), "test", "e2e/real-api.spec.ts"], {
    cwd: frontendDir,
    env: {
      ...process.env,
      REAL_E2E: "1",
      E2E_PORT: port,
      VITE_API_BASE_URL: apiBase
    }
  });
} finally {
  if (backendProcess) {
    await stopProcessTree(backendProcess);
  }
  await run("docker", ["compose", "stop"], { cwd: repoRoot }).catch((error) => {
    console.error(error.message);
  });
}

function startBackend() {
  const child = spawn("cargo", ["run"], {
    cwd: backendDir,
    env: {
      ...process.env,
      DATABASE_URL: "postgres://postgres:postgres@localhost:5432/project_k",
      APP_PORT: apiPort,
      DEV_USER_ID: "00000000-0000-0000-0000-000000000001",
      DEV_USER_EMAIL: "dev@usestakly.local",
      DEV_USER_USERNAME: "usestakly-dev",
      DEV_USER_DISPLAY_NAME: "UseStakly Dev",
      APP_BASE_URL: `http://localhost:${apiPort}`,
      FRONTEND_BASE_URL: `http://127.0.0.1:${port}`,
      APP_SESSION_SECRET: "local-real-e2e-session-secret",
      APP_NOTIFICATION_SECRET: "local-real-e2e-notification-secret",
      APP_SCHEDULER_ENABLED: "false",
      APP_SEMANTIC_SEARCH_ENABLED: "false",
      RUST_LOG: process.env.RUST_LOG || "info"
    },
    stdio: ["ignore", "pipe", "pipe"]
  });

  child.stdout.on("data", (chunk) => process.stdout.write(prefix(chunk, "backend")));
  child.stderr.on("data", (chunk) => process.stderr.write(prefix(chunk, "backend")));
  child.on("exit", (code, signal) => {
    if (stoppingBackend) return;
    if (code !== null && code !== 0) {
      console.error(`backend exited with code ${code}`);
    }
    if (signal) {
      console.error(`backend exited with signal ${signal}`);
    }
  });

  return child;
}

async function seedDatabase() {
  const sql = await fs.promises.readFile(seedPath, "utf8");
  await run("docker", [
    "exec",
    "-i",
    "usestakly-db",
    "psql",
    "-U",
    "postgres",
    "-d",
    "project_k",
    "-v",
    "ON_ERROR_STOP=1"
  ], {
    cwd: repoRoot,
    input: sql
  });
}

async function waitForHealth(url, timeoutMs) {
  const deadline = Date.now() + timeoutMs;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await delay(500);
  }
  throw new Error(`Timed out waiting for ${url}: ${lastError?.message || "unknown error"}`);
}

function run(command, args, options = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: options.cwd,
      env: options.env || process.env,
      stdio: ["pipe", "pipe", "pipe"]
    });
    let stderr = "";
    child.stdout.on("data", (chunk) => {
      if (!options.quiet) process.stdout.write(chunk);
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
      if (!options.quiet) process.stderr.write(chunk);
    });
    child.on("error", reject);
    child.on("exit", (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`${command} ${args.join(" ")} failed with code ${code}\n${stderr}`));
      }
    });
    if (options.input) {
      child.stdin.end(options.input);
    } else {
      child.stdin.end();
    }
  });
}

function waitForExit(child, timeoutMs) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("process did not exit")), timeoutMs);
    child.once("exit", () => {
      clearTimeout(timer);
      resolve();
    });
  });
}

async function stopProcessTree(child) {
  if (child.exitCode !== null || child.killed) return;
  stoppingBackend = true;
  if (process.platform === "win32") {
    await run("taskkill", ["/PID", String(child.pid), "/T", "/F"], { quiet: true }).catch(() => {
      child.kill();
    });
    return;
  }
  child.kill("SIGTERM");
  await waitForExit(child, 5_000).catch(() => child.kill("SIGKILL"));
}

function playwrightCli() {
  return path.join(frontendDir, "node_modules", "playwright", "cli.js");
}

function prefix(chunk, label) {
  return chunk
    .toString()
    .split(/\r?\n/)
    .filter(Boolean)
    .map((line) => `[${label}] ${line}\n`)
    .join("");
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
