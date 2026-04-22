import { useMemo, useState } from "react";
import { useQuery } from "@tanstack/react-query";

import { Chip } from "../components/Chip";
import { RepoCard } from "../components/RepoCard";
import { apiGet } from "../lib/api-client";
import type { RepoSearchResponse, SearchFilter } from "../lib/types";

const FILTERS: { value: SearchFilter; label: string; hint: string }[] = [
  {
    value: "explore",
    label: "Explore",
    hint: "Everything with its receipts — no filter."
  },
  {
    value: "auto",
    label: "Auto",
    hint: "Hides broken / unmaintained. Reliability ≥ 0.9."
  },
  {
    value: "strict",
    label: "Strict",
    hint: "Overall ≥ 0.85, zero flags, reliability ≥ 0.95."
  }
];

export function DiscoverPage() {
  const [query, setQuery] = useState("");
  const [filter, setFilter] = useState<SearchFilter>("explore");
  const [language, setLanguage] = useState("");
  const [starsMin, setStarsMin] = useState<number | "">("");

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

  return (
    <section className="shell grid gap-10 py-12 md:py-16">
      <header className="grid gap-5">
        <p className="eyebrow">
          <span className="callout-mark" />
          The register
        </p>
        <h1 className="display-lg max-w-[22ch]">
          What are you <span className="italic-accent">measuring</span> today?
        </h1>
        <p className="max-w-[60ch] text-[1rem] leading-relaxed text-ink-dim">
          Search the corpus by name, owner, description or topic. Narrow by
          language, minimum stars, or the confidence filter — all three modes
          use the same public formula, just different thresholds.
        </p>
      </header>

      <div className="card grid gap-5 p-5 md:p-6">
        <div className="grid gap-4 md:grid-cols-[1fr_auto]">
          <label className="grid gap-2">
            <span className="kicker">Query</span>
            <input
              type="search"
              placeholder="e.g. date picker, orm, htmx, zustand"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              className="border border-line bg-paper-soft px-4 py-3 text-[1rem] outline-none focus:border-ink"
              style={{ borderRadius: 2 }}
            />
          </label>
          <fieldset className="grid gap-2">
            <legend className="kicker">Filter mode</legend>
            <div className="flex gap-2">
              {FILTERS.map((f) => (
                <button
                  key={f.value}
                  type="button"
                  onClick={() => setFilter(f.value)}
                  title={f.hint}
                  className={`border px-3 py-2 text-[0.82rem] font-mono uppercase tracking-[0.14em] transition-colors ${
                    filter === f.value
                      ? "border-ink bg-ink text-paper-soft"
                      : "border-line text-ink-dim hover:border-ink-dim hover:text-ink"
                  }`}
                  style={{ borderRadius: 2 }}
                >
                  {f.label}
                </button>
              ))}
            </div>
          </fieldset>
        </div>

        <div className="grid gap-4 md:grid-cols-[1fr_1fr_1fr_auto]">
          <label className="grid gap-2">
            <span className="kicker">Language</span>
            <input
              type="text"
              placeholder="any"
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="border border-line bg-paper-soft px-3 py-2 text-[0.94rem] outline-none focus:border-ink"
              style={{ borderRadius: 2 }}
            />
          </label>
          <label className="grid gap-2">
            <span className="kicker">Stars min</span>
            <input
              type="number"
              min={0}
              placeholder="0"
              value={starsMin}
              onChange={(e) =>
                setStarsMin(e.target.value === "" ? "" : Number(e.target.value))
              }
              className="border border-line bg-paper-soft px-3 py-2 text-[0.94rem] outline-none focus:border-ink"
              style={{ borderRadius: 2 }}
            />
          </label>
          <div className="flex items-end gap-3">
            <p className="text-[0.82rem] leading-snug text-ink-muted">
              {FILTERS.find((f) => f.value === filter)?.hint}
            </p>
          </div>
          <div className="flex items-end justify-end">
            <Chip tone={results.isFetching ? "info" : "neutral"} mono>
              {results.isFetching ? "measuring" : "ready"}
            </Chip>
          </div>
        </div>
      </div>

      <div className="flex items-baseline justify-between border-b border-line pb-3">
        <p className="kicker">
          {results.data
            ? `${results.data.items.length} entr${results.data.items.length === 1 ? "y" : "ies"} · filter=${results.data.filter}`
            : "—"}
        </p>
        <p className="kicker">sorted by overall · stars · recency</p>
      </div>

      <div className="grid gap-8">
        {results.isLoading ? (
          <div className="py-16 text-center text-ink-muted">
            <span className="kicker">Tuning the instruments…</span>
          </div>
        ) : results.isError ? (
          <div className="grid gap-2 py-10 text-ink-muted">
            <p className="kicker text-ember">Observatory offline</p>
            <p className="text-[0.95rem]">
              The backend didn't answer. Run{" "}
              <code className="font-mono text-ink">cargo run</code> from{" "}
              <code className="font-mono text-ink">backend/</code>.
            </p>
          </div>
        ) : (results.data?.items ?? []).length === 0 ? (
          <div className="grid gap-3 py-16 text-center">
            <p className="kicker">No match</p>
            <p className="text-[1rem] text-ink-dim">
              Try widening the filter to <em>explore</em>, or lowering the stars floor.
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
