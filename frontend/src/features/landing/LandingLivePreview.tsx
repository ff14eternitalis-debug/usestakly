import { Link } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { RepoCard } from "../../components/RepoCard";
import { useT } from "../../i18n";
import { apiGet } from "../../lib/api-client";
import type { RepoSearchResponse } from "../../lib/types";

import { LandingTicker } from "./landing-ticker";

export function LandingLivePreview() {
  const t = useT();
  const query = useQuery({
    queryKey: ["search", "explore", "landing"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&limit=4",
        signal
      )
  });

  return (
    <section className="shell py-16 md:py-24">
      <header className="grid gap-4 mb-10 md:mb-12">
        <div className="flex items-end justify-between gap-4 flex-wrap">
          <div className="grid gap-3">
            <span className="kicker">{t.landing.previewEyebrow}</span>
            <h2 className="display-lg max-w-[22ch]">{t.landing.previewH2}</h2>
          </div>
          <Link
            to="/discover"
            className="link-underline text-[0.92rem] text-fg-dim hover:text-accent"
          >
            {t.landing.previewSeeAll} <span className="arrow">→</span>
          </Link>
        </div>
      </header>

      <LandingTicker />

      <div className="grid gap-5 pt-10">
        {query.isLoading ? (
          <div className="py-16 text-center">
            <span className="kicker">{t.common.tuning}</span>
          </div>
        ) : query.isError ? (
          <div className="surface grid gap-2 p-8">
            <p className="kicker" style={{ color: "var(--color-danger)" }}>
              {t.common.offline}
            </p>
            <p className="text-[0.94rem] text-fg-dim">
              {t.common.offlineHint}{" "}
              <code className="inline">{t.common.cargoRun}</code>{" "}
              {t.common.offlineFrom}{" "}
              <code className="inline">{t.common.backendDir}</code>.
            </p>
          </div>
        ) : (
          (query.data?.items ?? []).map((repo, index) => (
            <RepoCard key={repo.artifactId} repo={repo} index={index} />
          ))
        )}
      </div>
    </section>
  );
}
