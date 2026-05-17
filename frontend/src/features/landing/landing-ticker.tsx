import { useQuery } from "@tanstack/react-query";

import { useT } from "../../i18n";
import { apiGet } from "../../lib/api-client";
import { formatRelative, formatScore, formatStars, scoreTone } from "../../lib/format";
import type { RepoSearchResponse } from "../../lib/types";

export function LandingTicker() {
  const t = useT();
  const query = useQuery({
    queryKey: ["search", "explore", "ticker"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&limit=12",
        signal
      )
  });
  const items = query.data?.items ?? [];
  const doubled = [...items, ...items];

  if (!items.length) {
    return (
      <div className="overflow-hidden rounded-md border border-line bg-surface/40 py-3 text-center mono text-[0.78rem] uppercase tracking-[0.18em] text-fg-muted">
        {t.landing.tickerTuning}
      </div>
    );
  }

  return (
    <div className="overflow-hidden rounded-md border border-line bg-surface/40 py-3">
      <div className="marquee-track gap-10 whitespace-nowrap">
        {doubled.map((repo, i) => {
          const tone = scoreTone(repo.quality?.overall);
          const color =
            tone === "danger"
              ? "var(--color-danger)"
              : tone === "warn"
                ? "var(--color-warn)"
                : tone === "ok"
                  ? "var(--color-accent)"
                  : "var(--color-fg-dim)";
          return (
            <span
              key={`${repo.artifactId}-${i}`}
              className="inline-flex items-baseline gap-2 mono text-[0.84rem]"
            >
              <span className="text-fg-dim">
                {repo.owner}/{repo.name}
              </span>
              <span className="data-value" style={{ color }}>
                {formatScore(repo.quality?.overall)}
              </span>
              <span className="text-fg-muted">
                ★{formatStars(repo.starsCount)}
              </span>
              <span className="text-fg-muted">
                · {formatRelative(repo.lastCommitAt).replace(" ago", "")}
              </span>
              <span className="text-fg-faint">·</span>
            </span>
          );
        })}
      </div>
    </div>
  );
}
