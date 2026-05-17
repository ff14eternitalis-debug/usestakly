import { useQuery } from "@tanstack/react-query";

import { useT } from "../../i18n";
import { apiGet } from "../../lib/api-client";
import {
  abandonmentTone,
  formatScore,
  scoreTone
} from "../../lib/format";
import type { RepoSearchResponse } from "../../lib/types";

import { LandingHeroChart } from "./landing-hero-chart";
import { LandingMiniStat } from "./landing-mini-stat";

export function LiveRepositoryPanel() {
  const t = useT();
  const query = useQuery({
    queryKey: ["search", "explore", "hero-panel"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&sort=score&limit=1",
        signal
      )
  });
  const repo = query.data?.items[0];
  const quality = repo?.quality ?? null;
  const overallTone = scoreTone(quality?.overall);
  const overallColor =
    overallTone === "danger"
      ? "var(--color-danger)"
      : overallTone === "warn"
        ? "var(--color-warn)"
        : overallTone === "ok"
          ? "var(--color-accent)"
          : "var(--color-fg-muted)";

  return (
    <aside className="surface relative overflow-hidden rise-in rise-in-d1">
      <div className="flex items-center justify-between border-b border-line px-5 py-3">
        <span className="kicker">{t.landing.panelLive}</span>
        <span className="inline-flex items-center gap-1.5 mono text-[0.7rem] text-fg-muted">
          <span className="dot dot-pulse text-accent" />
          {t.common.observingStatus}
        </span>
      </div>

      <div className="grid gap-5 px-6 py-7">
        <div className="flex items-end justify-between gap-4">
          <div>
            <p className="mono text-[0.74rem] uppercase tracking-[0.14em] text-fg-muted">
              {t.landing.panelSample}
            </p>
            <p className="display-md text-[1.1rem]! mt-1">
              <span className="mono text-fg-muted">
                {repo ? `${repo.owner}/` : "owner/"}
              </span>
              <span>{repo?.name ?? "repo"}</span>
            </p>
          </div>
          <div className="text-right">
            <p className="kicker">{t.landing.panelOverall}</p>
            <p
              className="data-value text-[3.6rem] leading-none tracking-tight"
              style={{ color: overallColor }}
            >
              {formatScore(quality?.overall)}
            </p>
          </div>
        </div>

        <LandingHeroChart />

        <div className="grid grid-cols-2 gap-3 text-[0.84rem]">
          <LandingMiniStat
            label={t.footer.freshness}
            value={formatScore(quality?.freshness)}
            tone={scoreTone(quality?.freshness)}
          />
          <LandingMiniStat
            label={t.footer.adoption}
            value={formatScore(quality?.adoption)}
            tone={scoreTone(quality?.adoption)}
          />
          <LandingMiniStat
            label={t.footer.reliability}
            value={formatScore(quality?.reliability)}
            tone={scoreTone(quality?.reliability)}
          />
          <LandingMiniStat
            label={t.footer.abandonment}
            value={formatScore(quality?.abandonment)}
            tone={abandonmentTone(quality?.abandonment)}
          />
        </div>
      </div>

      <div className="border-t border-line px-5 py-3 flex justify-between text-[0.74rem]">
        <span className="mono text-fg-muted">
          {quality?.formulaVersion ?? "v2.0"}
        </span>
      </div>
    </aside>
  );
}
