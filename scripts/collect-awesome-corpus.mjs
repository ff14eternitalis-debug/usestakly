#!/usr/bin/env node
/**
 * Dry-run Awesome corpus collector. Does not call UseStakly APIs.
 */
import fs from "node:fs/promises";
import path from "node:path";
import {
  parseArgs,
  isAwesomeListRepo,
  normalizeGithubRepo,
  extractMarkdownLinks,
  rankAndCap,
  sleep,
} from "./awesome-corpus-lib.mjs";

async function main() {
  const cfg = parseArgs(process.argv);
  const allowlist = JSON.parse(await fs.readFile(cfg.allowlist, "utf8"));
  const lists = allowlist.lists ?? [];
  await fs.mkdir(cfg.cacheDir, { recursive: true });

  const stats = { rawLinks: 0, rejected: {}, duplicates: 0, awesomeListSkipped: 0 };
  const byKey = new Map();

  for (const list of lists) {
    const sourceList = `${list.owner}/${list.repo}`;
    let readme;
    try {
      readme = await fetchReadme(list.owner, list.repo, cfg);
    } catch (err) {
      console.warn(`Skipping ${sourceList}: ${err.message}`);
      continue;
    }
    const category = list.category ?? "unknown";
    for (const href of extractMarkdownLinks(readme)) {
      stats.rawLinks++;
      const norm = normalizeGithubRepo(href);
      if (norm.reject) {
        stats.rejected[norm.reject] = (stats.rejected[norm.reject] ?? 0) + 1;
        continue;
      }
      if (isAwesomeListRepo(norm.owner, norm.repo)) {
        stats.awesomeListSkipped++;
        continue;
      }
      if (byKey.has(norm.key)) {
        stats.duplicates++;
        continue;
      }
      byKey.set(norm.key, {
        owner: norm.owner,
        repo: norm.repo,
        url: norm.url,
        key: norm.key,
        sourceList,
        sourceCategory: category,
        reason: "direct_repo_link",
      });
    }
  }

  const ranked = rankAndCap([...byKey.values()], cfg.max, cfg.perSourceCap);
  const payload = {
    generatedAt: new Date().toISOString(),
    root: cfg.root,
    allowlist: cfg.allowlist,
    max: cfg.max,
    stats: {
      ...stats,
      uniqueCandidates: byKey.size,
      finalCount: ranked.length,
    },
    candidates: ranked,
  };

  await fs.mkdir(path.dirname(cfg.out), { recursive: true });
  await fs.writeFile(cfg.out, JSON.stringify(payload, null, 2));
  await fs.writeFile(cfg.summary, renderSummary(payload));
  console.log(`Wrote ${ranked.length} candidates -> ${cfg.out}`);
}

async function fetchReadme(owner, repo, cfg) {
  const cachePath = path.join(cfg.cacheDir, `${owner}__${repo}.md`);
  try {
    return await fs.readFile(cachePath, "utf8");
  } catch {
    /* miss */
  }
  const url = `https://raw.githubusercontent.com/${owner}/${repo}/HEAD/README.md`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(`README fetch failed ${owner}/${repo}: ${res.status}`);
  const text = await res.text();
  await fs.writeFile(cachePath, text);
  await sleep(cfg.delayMs);
  return text;
}

function renderSummary(payload) {
  const s = payload.stats;
  return `# Awesome corpus candidates

- Generated: ${payload.generatedAt}
- Allowlist: \`${payload.allowlist}\`
- Raw links scanned: ${s.rawLinks}
- Duplicates skipped: ${s.duplicates}
- Awesome-list repos skipped: ${s.awesomeListSkipped}
- Unique candidates before cap: ${s.uniqueCandidates}
- **Final count (cap ${payload.max}): ${s.finalCount}**

## Rejections

${Object.entries(s.rejected)
  .map(([k, v]) => `- ${k}: ${v}`)
  .join("\n")}

Review \`candidates\` in the JSON file, then copy to \`awesome-candidates-approved.json\` for import.
`;
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
