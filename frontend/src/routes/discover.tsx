import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";

import { RepoCard } from "../components/RepoCard";
import { useT } from "../i18n";
import { apiGet } from "../lib/api-client";
import type { RepoSearchResponse, SearchFilter } from "../lib/types";

const SearchIcon = (
  <svg
    width="16"
    height="16"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    aria-hidden
  >
    <circle cx="11" cy="11" r="7" />
    <line x1="21" y1="21" x2="16.65" y2="16.65" />
  </svg>
);

export function DiscoverPage() {
  const t = useT();
  const [query, setQuery] = useState("");
  const [filter, setFilter] = useState<SearchFilter>("explore");
  const [language, setLanguage] = useState("");
  const [starsMin, setStarsMin] = useState<number | "">("");

  const FILTERS: { value: SearchFilter; label: string; hint: string }[] = [
    { value: "explore", label: t.discover.modeExplore, hint: t.discover.hintExplore },
    { value: "auto", label: t.discover.modeAuto, hint: t.discover.hintAuto },
    { value: "strict", label: t.discover.modeStrict, hint: t.discover.hintStrict }
  ];

  const search = useMemo(() => {
    const params = new URLSearchParams();
    if (query.trim()) params.set("q", query.trim());
    params.set("filter", filter);
    if (language.trim()) params.set("language", language.trim());
    if (typeof starsMin === "number") params.set("stars_min", String(starsMin));
    params.set("limit", "40");
    return params.toString();
  }, [query, filter, language, starsMin]);

  const results = useQuery({
    queryKey: ["search", search],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(`/api/repos/search?${search}`, signal),
    placeholderData: (prev) => prev
  });

  const count = results.data?.items.length ?? 0;
  const activeFilter = FILTERS.find((f) => f.value === filter);

  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.discover.eyebrow}</span>
        <h1 className="display-lg max-w-[22ch]">
          {t.discover.h1Part1}{" "}
          <span className="accent">{t.discover.h1Accent}</span>{" "}
          {t.discover.h1Part2}
        </h1>
        <p className="max-w-[60ch] text-[0.98rem] leading-relaxed text-fg-dim">
          {t.discover.intro}
        </p>
      </header>

      <div className="surface grid gap-4 p-4 md:p-5">
        <div className="grid gap-4 md:grid-cols-[1fr_auto] md:items-end">
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.queryLabel}</span>
            <div className="relative">
              <span className="pointer-events-none absolute left-3.5 top-1/2 -translate-y-1/2 text-fg-muted">
                {SearchIcon}
              </span>
              <input
                type="search"
                placeholder={t.discover.queryPlaceholder}
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                className="input pl-10"
              />
            </div>
          </label>
          <fieldset className="grid gap-1.5">
            <legend className="kicker">{t.discover.modeLabel}</legend>
            <div className="inline-flex rounded-[6px] border border-line bg-bg-subtle p-1">
              {FILTERS.map((f) => (
                <button
                  key={f.value}
                  type="button"
                  onClick={() => setFilter(f.value)}
                  title={f.hint}
                  className={`px-3 py-1.5 text-[0.82rem] font-medium rounded-[4px] transition-all ${
                    filter === f.value
                      ? "bg-surface-elev text-fg shadow-[0_0_0_1px_var(--color-line-strong)]"
                      : "text-fg-muted hover:text-fg"
                  }`}
                >
                  {f.label}
                </button>
              ))}
            </div>
          </fieldset>
        </div>

        <div className="grid gap-4 md:grid-cols-[1fr_1fr_2fr]">
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.languageLabel}</span>
            <input
              type="text"
              placeholder={t.discover.languageAny}
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="input"
            />
          </label>
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.starsMinLabel}</span>
            <input
              type="number"
              min={0}
              placeholder={t.discover.starsMinPlaceholder}
              value={starsMin}
              onChange={(e) =>
                setStarsMin(e.target.value === "" ? "" : Number(e.target.value))
              }
              className="input"
            />
          </label>
          <div className="grid gap-1.5">
            <span className="kicker">{t.discover.hintLabel}</span>
            <p className="text-[0.84rem] leading-snug text-fg-dim self-center">
              {activeFilter?.hint}
            </p>
          </div>
        </div>
      </div>

      <div className="flex items-center justify-between border-b border-line pb-3">
        <div className="flex items-center gap-2">
          <span className="dot text-accent" />
          <p className="mono text-[0.8rem] text-fg-dim">
            {results.isFetching
              ? t.discover.measuring
              : results.data
                ? `${count} ${count === 1 ? t.discover.entriesSingle : t.discover.entriesPlural} · filter=${results.data.filter}`
                : "—"}
          </p>
        </div>
        <p className="kicker hidden sm:inline">{t.discover.sortedBy}</p>
      </div>

      <div className="grid gap-4">
        {results.isLoading ? (
          <div className="py-16 text-center">
            <span className="kicker">{t.common.tuning}</span>
          </div>
        ) : results.isError ? (
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
        ) : count === 0 ? (
          <div className="surface grid gap-3 p-10 text-center">
            <p className="display-md !text-[1.2rem]">{t.common.noMatch}</p>
            <p className="text-[0.94rem] text-fg-dim">
              {t.discover.tryWidening}{" "}
              <button
                type="button"
                onClick={() => setFilter("explore")}
                className="text-accent hover:underline"
              >
                {t.discover.exploreLink}
              </button>
              {t.discover.orLoweringStars}
            </p>
          </div>
        ) : (
          (results.data?.items ?? []).map((repo, index) => (
            <RepoCard key={repo.artifactId} repo={repo} index={index} />
          ))
        )}
      </div>
    </section>
  );
}
