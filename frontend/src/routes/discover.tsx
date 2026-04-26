import { useMemo, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Link, useNavigate } from "@tanstack/react-router";

import { Button } from "../components/Button";
import { RepoCard } from "../components/RepoCard";
import { useT } from "../i18n";
import { ApiError, apiGet, apiPost } from "../lib/api-client";
import type { AddRepoResponse, RepoSearchResponse, RepoSort, SearchFilter } from "../lib/types";

const PAGE_SIZE = 20;

const LANGUAGE_OPTIONS = ["", "TypeScript", "JavaScript", "Python", "Rust", "Go"];
const TOPIC_OPTIONS = [
  "react",
  "typescript",
  "orm",
  "database",
  "table",
  "testing",
  "auth",
  "http",
  "css",
  "api"
];
const SCORE_OPTIONS: { value: number | ""; label: string }[] = [
  { value: "", label: "—" },
  { value: 0.45, label: "≥ 0.45" },
  { value: 0.6, label: "≥ 0.60" },
  { value: 0.75, label: "≥ 0.75" }
];
const RISK_OPTIONS: { value: number | ""; label: string }[] = [
  { value: "", label: "—" },
  { value: 0.35, label: "≤ 0.35" },
  { value: 0.2, label: "≤ 0.20" },
  { value: 0.1, label: "≤ 0.10" }
];

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
  const [topics, setTopics] = useState<string[]>([]);
  const [scoreMin, setScoreMin] = useState<number | "">("");
  const [abandonmentMax, setAbandonmentMax] = useState<number | "">("");
  const [includeArchived, setIncludeArchived] = useState(false);
  const [sort, setSort] = useState<RepoSort>("score");
  const [starsMin, setStarsMin] = useState<number | "">("");
  const [page, setPage] = useState(0);
  const [repoInput, setRepoInput] = useState("");
  const [addedRepo, setAddedRepo] = useState<AddRepoResponse | null>(null);
  const queryClient = useQueryClient();
  const navigate = useNavigate();

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
    if (topics.length) params.set("topics", topics.join(","));
    if (typeof scoreMin === "number") params.set("score_min", String(scoreMin));
    if (typeof abandonmentMax === "number") {
      params.set("abandonment_max", String(abandonmentMax));
    }
    if (includeArchived) params.set("include_archived", "true");
    params.set("sort", sort);
    if (typeof starsMin === "number") params.set("stars_min", String(starsMin));
    params.set("limit", String(PAGE_SIZE));
    params.set("offset", String(page * PAGE_SIZE));
    return params.toString();
  }, [
    query,
    filter,
    language,
    topics,
    scoreMin,
    abandonmentMax,
    includeArchived,
    sort,
    starsMin,
    page
  ]);

  const results = useQuery({
    queryKey: ["search", search],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(`/api/repos/search?${search}`, signal),
    placeholderData: (prev) => prev
  });

  const addRepo = useMutation({
    mutationFn: () =>
      apiPost<AddRepoResponse>("/api/repos/add", { repo: repoInput.trim() }),
    onSuccess: async (data) => {
      setAddedRepo(data);
      setRepoInput("");
      await queryClient.invalidateQueries({ queryKey: ["search"] });
      void navigate({ to: "/repos/$id", params: { id: data.artifactId } });
    }
  });

  const count = results.data?.items.length ?? 0;
  const offset = results.data?.offset ?? page * PAGE_SIZE;
  const rangeStart = count > 0 ? offset + 1 : 0;
  const rangeEnd = offset + count;
  const hasPrevious = page > 0;
  const hasNext = results.data?.hasMore ?? false;
  const activeFilter = FILTERS.find((f) => f.value === filter);
  const addRepoError =
    addRepo.error instanceof ApiError ? addRepo.error.message : null;
  const hasActiveFilters =
    Boolean(query.trim()) ||
    filter !== "explore" ||
    Boolean(language) ||
    topics.length > 0 ||
    typeof scoreMin === "number" ||
    typeof abandonmentMax === "number" ||
    includeArchived ||
    sort !== "score" ||
    typeof starsMin === "number";
  const resetPaged = (fn: () => void) => {
    fn();
    setPage(0);
  };
  const clearAllFilters = () => {
    setQuery("");
    setFilter("explore");
    setLanguage("");
    setTopics([]);
    setScoreMin("");
    setAbandonmentMax("");
    setIncludeArchived(false);
    setSort("score");
    setStarsMin("");
    setPage(0);
  };
  const toggleTopic = (topic: string) => {
    resetPaged(() => {
      setTopics((current) =>
        current.includes(topic)
          ? current.filter((item) => item !== topic)
          : [...current, topic]
      );
    });
  };

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

      <section className="grid gap-4 md:grid-cols-2">
        <div className="rounded-[8px] border border-line bg-surface/45 p-5">
          <p className="kicker">{t.repoDetail.overallVerdict}</p>
          <h2 className="mt-2 text-[1.04rem] font-semibold text-fg">
            {t.discover.scoreGuideTitle}
          </h2>
          <p className="mt-2 text-[0.9rem] leading-relaxed text-fg-dim">
            {t.discover.scoreGuideBody}
          </p>
          <Link
            to="/how-to-read"
            className="mt-4 inline-flex w-fit items-center text-[0.88rem] font-medium text-accent hover:underline"
          >
            {t.discover.scoreGuideAction} <span className="arrow">→</span>
          </Link>
        </div>
        <div className="rounded-[8px] border border-line bg-surface/45 p-5">
          <p className="kicker">{t.howToRead.corpusLabel}</p>
          <h2 className="mt-2 text-[1.04rem] font-semibold text-fg">
            {t.discover.corpusTitle}
          </h2>
          <p className="mt-2 text-[0.9rem] leading-relaxed text-fg-dim">
            {t.discover.corpusBody}
          </p>
        </div>
      </section>

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
                onChange={(e) => resetPaged(() => setQuery(e.target.value))}
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
                  onClick={() => resetPaged(() => setFilter(f.value))}
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

        <div className="grid gap-4 md:grid-cols-4">
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.languageLabel}</span>
            <select
              value={language}
              onChange={(e) => resetPaged(() => setLanguage(e.target.value))}
              className="input"
            >
              {LANGUAGE_OPTIONS.map((option) => (
                <option key={option || "any"} value={option}>
                  {option || t.discover.languageAny}
                </option>
              ))}
            </select>
          </label>
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.starsMinLabel}</span>
            <input
              type="number"
              min={0}
              placeholder={t.discover.starsMinPlaceholder}
              value={starsMin}
              onChange={(e) =>
                resetPaged(() =>
                  setStarsMin(e.target.value === "" ? "" : Number(e.target.value))
                )
              }
              className="input"
            />
          </label>
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.scoreMinLabel}</span>
            <select
              value={scoreMin}
              onChange={(e) =>
                resetPaged(() =>
                  setScoreMin(e.target.value === "" ? "" : Number(e.target.value))
                )
              }
              className="input"
            >
              {SCORE_OPTIONS.map((option) => (
                <option key={String(option.value)} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.riskMaxLabel}</span>
            <select
              value={abandonmentMax}
              onChange={(e) =>
                resetPaged(() =>
                  setAbandonmentMax(e.target.value === "" ? "" : Number(e.target.value))
                )
              }
              className="input"
            >
              {RISK_OPTIONS.map((option) => (
                <option key={String(option.value)} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
        </div>

        <div className="grid gap-4 md:grid-cols-[1fr_220px]">
          <fieldset className="grid gap-2">
            <legend className="kicker">{t.discover.topicsLabel}</legend>
            <div className="flex flex-wrap gap-2">
              {TOPIC_OPTIONS.map((topic) => (
                <button
                  key={topic}
                  type="button"
                  onClick={() => toggleTopic(topic)}
                  className={`rounded-[999px] border px-3 py-1.5 text-[0.8rem] font-medium transition-colors ${
                    topics.includes(topic)
                      ? "border-accent bg-[color:var(--color-accent-glow)] text-accent"
                      : "border-line bg-surface text-fg-dim hover:border-accent hover:text-accent"
                  }`}
                >
                  #{topic}
                </button>
              ))}
            </div>
          </fieldset>
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.sortLabel}</span>
            <select
              value={sort}
              onChange={(e) => resetPaged(() => setSort(e.target.value as RepoSort))}
              className="input"
            >
              <option value="score">{t.discover.sortScore}</option>
              <option value="stars">{t.discover.sortStars}</option>
              <option value="recency">{t.discover.sortRecency}</option>
              <option value="abandonment">{t.discover.sortAbandonment}</option>
            </select>
          </label>
        </div>

        <div className="grid gap-3 md:grid-cols-[1fr_auto] md:items-center">
          <p className="text-[0.84rem] leading-snug text-fg-dim">
            <span className="font-medium text-fg">{t.discover.hintLabel}.</span>{" "}
            {activeFilter?.hint}
          </p>
          <label className="inline-flex items-center gap-2 text-[0.86rem] text-fg-dim">
            <input
              type="checkbox"
              checked={includeArchived}
              onChange={(e) => resetPaged(() => setIncludeArchived(e.target.checked))}
              className="size-4 accent-[var(--color-accent)]"
            />
            {t.discover.includeArchived}
          </label>
        </div>

        {hasActiveFilters ? (
          <div className="flex flex-wrap items-center gap-2 border-t border-line pt-3">
            {query.trim() ? (
              <FilterChip label={`${t.discover.queryLabel}: ${query}`} onRemove={() => resetPaged(() => setQuery(""))} />
            ) : null}
            {filter !== "explore" ? (
              <FilterChip label={`${t.discover.modeLabel}: ${filter}`} onRemove={() => resetPaged(() => setFilter("explore"))} />
            ) : null}
            {language ? (
              <FilterChip label={`${t.discover.languageLabel}: ${language}`} onRemove={() => resetPaged(() => setLanguage(""))} />
            ) : null}
            {topics.map((topic) => (
              <FilterChip key={topic} label={`#${topic}`} onRemove={() => toggleTopic(topic)} />
            ))}
            {typeof starsMin === "number" ? (
              <FilterChip label={`${t.discover.starsMinLabel}: ${starsMin}`} onRemove={() => resetPaged(() => setStarsMin(""))} />
            ) : null}
            {typeof scoreMin === "number" ? (
              <FilterChip label={`${t.discover.scoreMinLabel}: ${scoreMin}`} onRemove={() => resetPaged(() => setScoreMin(""))} />
            ) : null}
            {typeof abandonmentMax === "number" ? (
              <FilterChip label={`${t.discover.riskMaxLabel}: ${abandonmentMax}`} onRemove={() => resetPaged(() => setAbandonmentMax(""))} />
            ) : null}
            {sort !== "score" ? (
              <FilterChip label={`${t.discover.sortLabel}: ${sort}`} onRemove={() => resetPaged(() => setSort("score"))} />
            ) : null}
            {includeArchived ? (
              <FilterChip label={t.discover.includeArchived} onRemove={() => resetPaged(() => setIncludeArchived(false))} />
            ) : null}
            <button
              type="button"
              onClick={clearAllFilters}
              className="ml-0 rounded-[6px] border border-line bg-surface px-3 py-1.5 text-[0.8rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent sm:ml-1"
            >
              {t.discover.clearFilters}
            </button>
          </div>
        ) : null}

        <div className="grid gap-3 border-t border-line pt-4 md:grid-cols-[1fr_auto] md:items-end">
          <label className="grid gap-1.5">
            <span className="kicker">{t.discover.addRepoLabel}</span>
            <input
              type="text"
              placeholder={t.discover.addRepoPlaceholder}
              value={repoInput}
              onChange={(e) => setRepoInput(e.target.value)}
              className="input"
            />
          </label>
          <div className="grid gap-1.5">
            <span className="kicker">&nbsp;</span>
            <Button
              type="button"
              variant="outline"
              onClick={() => addRepo.mutate()}
              disabled={addRepo.isPending || !repoInput.trim()}
            >
              {addRepo.isPending ? t.discover.addRepoPending : t.discover.addRepoAction}
            </Button>
          </div>
        </div>

        <p className="text-[0.84rem] text-fg-dim">{t.discover.addRepoHelp}</p>

        {addRepoError ? (
          <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
            {addRepoError}
          </p>
        ) : null}

        {addedRepo ? (
          <div className="rounded-[6px] border border-line bg-surface/40 px-4 py-3 text-[0.9rem] text-fg-dim">
            {(addedRepo.alreadyIndexed
              ? t.discover.addRepoExists
              : t.discover.addRepoSuccess)}{" "}
            <Link
              to="/repos/$id"
              params={{ id: addedRepo.artifactId }}
              className="text-accent hover:underline"
            >
              {addedRepo.fullName}
            </Link>
            {" · "}
            <Link
              to="/repos/$id"
              params={{ id: addedRepo.artifactId }}
              className="text-accent hover:underline"
            >
              {t.discover.addRepoOpen}
            </Link>
          </div>
        ) : null}
      </div>

      <div className="flex items-center justify-between border-b border-line pb-3">
        <div className="flex items-center gap-2">
          <span className="dot text-accent" />
          <p className="mono text-[0.8rem] text-fg-dim">
            {results.isFetching
              ? t.discover.measuring
              : results.data
                ? `${rangeStart}-${rangeEnd} · ${count} ${count === 1 ? t.discover.entriesSingle : t.discover.entriesPlural} · filter=${results.data.filter}`
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
                onClick={() => {
                  setFilter("explore");
                  setScoreMin("");
                  setAbandonmentMax("");
                  setPage(0);
                }}
                className="text-accent hover:underline"
              >
                {t.discover.exploreLink}
              </button>
              {t.discover.orLoweringStars}
            </p>
          </div>
        ) : (
          (results.data?.items ?? []).map((repo, index) => (
            <RepoCard key={repo.artifactId} repo={repo} index={offset + index} />
          ))
        )}
      </div>

      {results.data && (hasPrevious || hasNext) ? (
        <nav
          className="flex flex-col gap-3 border-t border-line pt-4 sm:flex-row sm:items-center sm:justify-between"
          aria-label={t.discover.paginationLabel}
        >
          <p className="mono text-[0.78rem] text-fg-dim">
            {t.discover.pageLabel} {page + 1} · {rangeStart}-{rangeEnd}
          </p>
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => setPage((value) => Math.max(0, value - 1))}
              disabled={!hasPrevious || results.isFetching}
              className="inline-flex min-w-28 items-center justify-center gap-2 rounded-[6px] border border-line-strong bg-surface px-3 py-2 text-[0.86rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent disabled:cursor-not-allowed disabled:opacity-40"
            >
              <span aria-hidden>←</span>
              {t.discover.previousPage}
            </button>
            <button
              type="button"
              onClick={() => setPage((value) => value + 1)}
              disabled={!hasNext || results.isFetching}
              className="inline-flex min-w-28 items-center justify-center gap-2 rounded-[6px] border border-line-strong bg-surface px-3 py-2 text-[0.86rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent disabled:cursor-not-allowed disabled:opacity-40"
            >
              {t.discover.nextPage}
              <span aria-hidden>→</span>
            </button>
          </div>
        </nav>
      ) : null}
    </section>
  );
}

function FilterChip({
  label,
  onRemove
}: {
  label: string;
  onRemove: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onRemove}
      className="inline-flex items-center gap-2 rounded-[999px] border border-line bg-bg-subtle px-3 py-1.5 text-[0.78rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent"
    >
      {label}
      <span aria-hidden>×</span>
    </button>
  );
}
