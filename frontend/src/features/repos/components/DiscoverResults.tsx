import type { UseQueryResult } from "@tanstack/react-query";

import { RepoCard } from "../../../components/RepoCard";
import { useT } from "../../../i18n";
import { filterSummaryLabel } from "../../../lib/repo-explanation";
import type { RepoSearchResponse, RepoSearchResult } from "../../../lib/types";

type DiscoverResultsProps = {
  variant: "radar" | "advanced";
  query: UseQueryResult<RepoSearchResponse, Error>;
  items: RepoSearchResult[];
  offset: number;
  count: number;
  page: number;
  onPageChange: (updater: (value: number) => number) => void;
  hasPrevious: boolean;
  hasNext: boolean;
  onWidenFilters?: () => void;
};

export function DiscoverRadarSection({
  query,
  items,
  offset,
  count,
  page,
  onPageChange,
  hasPrevious,
  hasNext
}: Omit<DiscoverResultsProps, "variant" | "onWidenFilters">) {
  const t = useT();
  const rangeStart = count > 0 ? offset + 1 : 0;
  const rangeEnd = offset + count;

  return (
    <>
      <section className="grid gap-3 border-b border-line pb-5">
        <span className="kicker">Radar</span>
        <div className="grid gap-2 md:grid-cols-[1fr_auto] md:items-end">
          <div className="grid gap-2">
            <h2 className="text-[1.1rem] font-semibold text-fg">{t.discover.radarTitle}</h2>
            <p className="max-w-[72ch] text-[0.92rem] leading-relaxed text-fg-dim">
              {t.discover.radarBody}
            </p>
          </div>
          <p className="mono text-[0.78rem] text-fg-dim">
            {query.isFetching
              ? t.discover.measuring
              : query.data
                ? `${rangeStart}-${rangeEnd} · ${count} ${
                    count === 1 ? t.discover.entriesSingle : t.discover.entriesPlural
                  } · sort=trend`
                : "—"}
          </p>
        </div>
      </section>

      <DiscoverResultsList
        variant="radar"
        query={query}
        items={items}
        offset={offset}
        count={count}
        page={page}
        onPageChange={onPageChange}
        hasPrevious={hasPrevious}
        hasNext={hasNext}
      />
    </>
  );
}

export function DiscoverResults({
  variant,
  query,
  items,
  offset,
  count,
  page,
  onPageChange,
  hasPrevious,
  hasNext,
  onWidenFilters
}: DiscoverResultsProps) {
  const t = useT();
  const rangeStart = count > 0 ? offset + 1 : 0;
  const rangeEnd = offset + count;

  return (
    <>
      {variant === "advanced" ? (
        <div className="flex items-center justify-between border-b border-line pb-3">
          <div className="grid gap-2">
            <div className="flex items-center gap-2">
              <span className="dot text-accent" />
              <p className="mono text-[0.8rem] text-fg-dim">
                {query.isFetching
                  ? t.discover.measuring
                  : query.data
                    ? `${rangeStart}-${rangeEnd} · ${count} ${
                        count === 1 ? t.discover.entriesSingle : t.discover.entriesPlural
                      } · filter=${query.data.filter}`
                    : "—"}
              </p>
            </div>
            {filterSummaryLabel(t.repoExplanation, query.data?.filterSummary) ? (
              <p className="max-w-[70ch] text-[0.82rem] text-fg-muted">
                {filterSummaryLabel(t.repoExplanation, query.data?.filterSummary)}
              </p>
            ) : null}
          </div>
          <p className="kicker hidden sm:inline">{t.discover.sortedBy}</p>
        </div>
      ) : null}

      <DiscoverResultsList
        variant={variant}
        query={query}
        items={items}
        offset={offset}
        count={count}
        page={page}
        onPageChange={onPageChange}
        hasPrevious={hasPrevious}
        hasNext={hasNext}
        onWidenFilters={onWidenFilters}
      />
    </>
  );
}

function DiscoverResultsList({
  variant,
  query,
  items,
  offset,
  count,
  page,
  onPageChange,
  hasPrevious,
  hasNext,
  onWidenFilters
}: DiscoverResultsProps) {
  const t = useT();
  const rangeStart = count > 0 ? offset + 1 : 0;
  const rangeEnd = offset + count;
  const emptyLabel = variant === "radar" ? t.discover.radarEmpty : t.common.noMatch;

  return (
    <>
      <div className="grid gap-4">
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
              <code className="inline">{t.common.cargoRun}</code> {t.common.offlineFrom}{" "}
              <code className="inline">{t.common.backendDir}</code>.
            </p>
          </div>
        ) : count === 0 ? (
          <div className="surface grid gap-3 p-10 text-center">
            <p className="display-md !text-[1.2rem]">{emptyLabel}</p>
            {variant === "advanced" && onWidenFilters ? (
              <p className="text-[0.94rem] text-fg-dim">
                {t.discover.tryWidening}{" "}
                <button type="button" onClick={onWidenFilters} className="text-accent hover:underline">
                  {t.discover.exploreLink}
                </button>
                {t.discover.orLoweringStars}
              </p>
            ) : (
              <p className="text-[0.94rem] text-fg-dim">{t.discover.addRepoHelp}</p>
            )}
          </div>
        ) : (
          items.map((repo, index) => (
            <RepoCard
              key={repo.artifactId}
              repo={repo}
              index={offset + index}
              showRadarSummary={variant === "radar"}
              showRecommendationExplanation={variant === "advanced"}
            />
          ))
        )}
      </div>

      {query.data && (hasPrevious || hasNext) ? (
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
              onClick={() => onPageChange((value) => Math.max(0, value - 1))}
              disabled={!hasPrevious || query.isFetching}
              className="inline-flex min-w-28 items-center justify-center gap-2 rounded-[6px] border border-line-strong bg-surface px-3 py-2 text-[0.86rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent disabled:cursor-not-allowed disabled:opacity-40"
            >
              <span aria-hidden>←</span>
              {t.discover.previousPage}
            </button>
            <button
              type="button"
              onClick={() => onPageChange((value) => value + 1)}
              disabled={!hasNext || query.isFetching}
              className="inline-flex min-w-28 items-center justify-center gap-2 rounded-[6px] border border-line-strong bg-surface px-3 py-2 text-[0.86rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent disabled:cursor-not-allowed disabled:opacity-40"
            >
              {t.discover.nextPage}
              <span aria-hidden>→</span>
            </button>
          </div>
        </nav>
      ) : null}
    </>
  );
}
