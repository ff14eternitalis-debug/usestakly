/** Shared parsing for collect-awesome-corpus.mjs */

const GITHUB_HOST = /github\.com/i;
const REJECT_SEGMENTS =
  /\/(issues|pull|pulls|releases|actions|wiki|discussions|sponsors|commit)(\/|$)/i;

export function parseArgs(argv) {
  const out = {
    root: "sindresorhus/awesome",
    allowlist: "docs/corpus/awesome-lists-allowlist.json",
    max: 500,
    out: "docs/corpus/awesome-candidates.json",
    summary: "docs/corpus/awesome-candidates-summary.md",
    cacheDir: ".cache/awesome-readmes",
    delayMs: 400,
    perSourceCap: 45,
  };
  for (let i = 2; i < argv.length; i++) {
    const key = argv[i];
    const val = argv[i + 1];
    if (key === "--root" && val) out.root = val, i++;
    else if (key === "--allowlist" && val) out.allowlist = val, i++;
    else if (key === "--max" && val) out.max = Number(val), i++;
    else if (key === "--out" && val) out.out = val, i++;
    else if (key === "--summary" && val) out.summary = val, i++;
    else if (key === "--delay-ms" && val) out.delayMs = Number(val), i++;
    else if (key === "--per-source-cap" && val) out.perSourceCap = Number(val), i++;
  }
  return out;
}

export function isAwesomeListRepo(owner, repo) {
  const r = repo.toLowerCase();
  return r === "awesome" || r.startsWith("awesome-") || r.startsWith("awesome_");
}

export function normalizeGithubRepo(href) {
  if (!href || !GITHUB_HOST.test(href)) return { reject: "non_github" };
  let path = href
    .replace(/^https?:\/\/([^/]+\.)?github\.com\//i, "")
    .replace(/^github\.com\//i, "")
    .split(/[?#]/)[0]
    .replace(/\.git$/, "");
  if (REJECT_SEGMENTS.test(`/${path}`)) return { reject: "non_repo_path" };
  const tree = path.match(/^([^/]+)\/([^/]+)\/(tree|blob)\//);
  if (tree) path = `${tree[1]}/${tree[2]}`;
  const parts = path.split("/").filter(Boolean);
  if (parts.length !== 2) return { reject: "invalid_path" };
  const [owner, repo] = parts;
  if (!owner || !repo) return { reject: "invalid_path" };
  return {
    owner: owner.toLowerCase(),
    repo: repo.toLowerCase(),
    key: `${owner.toLowerCase()}/${repo.toLowerCase()}`,
    url: `https://github.com/${owner}/${repo}`,
  };
}

export function extractMarkdownLinks(markdown) {
  const links = [];
  const re = /\[[^\]]*\]\(([^)]+)\)/g;
  let m;
  while ((m = re.exec(markdown)) !== null) links.push(m[1].trim());
  const bare = /https?:\/\/github\.com\/[^\s)>"']+/gi;
  for (const u of markdown.match(bare) ?? []) links.push(u.trim());
  return links;
}

export function rankAndCap(candidates, max, perSourceCap) {
  const bySource = new Map();
  for (const c of candidates) {
    const list = c.sourceList;
    if (!bySource.has(list)) bySource.set(list, []);
    bySource.get(list).push(c);
  }
  for (const arr of bySource.values()) {
    arr.sort((a, b) => a.key.localeCompare(b.key));
  }
  const sources = [...bySource.keys()].sort();
  const out = [];
  const counts = new Map(sources.map((s) => [s, 0]));
  while (out.length < max) {
    let progressed = false;
    for (const source of sources) {
      if (out.length >= max) break;
      const n = counts.get(source) ?? 0;
      if (n >= perSourceCap) continue;
      const bucket = bySource.get(source) ?? [];
      if (n >= bucket.length) continue;
      out.push(bucket[n]);
      counts.set(source, n + 1);
      progressed = true;
    }
    if (!progressed) break;
  }
  return out;
}

export function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}
