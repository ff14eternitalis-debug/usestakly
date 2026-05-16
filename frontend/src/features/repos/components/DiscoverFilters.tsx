import { Link } from "@tanstack/react-router";

import { Button } from "../../../components/Button";
import { useT } from "../../../i18n";
import type { AddRepoResponse, RepoSort, SearchFilter } from "../../../lib/types";
import {
  FilterChip,
  LANGUAGE_OPTIONS,
  RISK_OPTIONS,
  SCORE_OPTIONS,
  SearchIcon,
  TOPIC_OPTIONS,
  type DiscoverFilterMode
} from "./discover-shared";

type DiscoverFiltersProps = {
  filters: DiscoverFilterMode[];
  query: string;
  setQuery: (value: string) => void;
  filter: SearchFilter;
  setFilter: (value: SearchFilter) => void;
  language: string;
  setLanguage: (value: string) => void;
  topics: string[];
  toggleTopic: (topic: string) => void;
  scoreMin: number | "";
  setScoreMin: (value: number | "") => void;
  abandonmentMax: number | "";
  setAbandonmentMax: (value: number | "") => void;
  includeArchived: boolean;
  setIncludeArchived: (value: boolean) => void;
  sort: RepoSort;
  setSort: (value: RepoSort) => void;
  starsMin: number | "";
  setStarsMin: (value: number | "") => void;
  resetPaged: (fn: () => void) => void;
  clearAllFilters: () => void;
  hasActiveFilters: boolean;
  activeFilter: DiscoverFilterMode | undefined;
  repoInput: string;
  setRepoInput: (value: string) => void;
  addRepoPending: boolean;
  onAddRepo: () => void;
  addRepoError: string | null;
  addedRepo: AddRepoResponse | null;
};

export function DiscoverFilters({
  filters,
  query,
  setQuery,
  filter,
  setFilter,
  language,
  setLanguage,
  topics,
  toggleTopic,
  scoreMin,
  setScoreMin,
  abandonmentMax,
  setAbandonmentMax,
  includeArchived,
  setIncludeArchived,
  sort,
  setSort,
  starsMin,
  setStarsMin,
  resetPaged,
  clearAllFilters,
  hasActiveFilters,
  activeFilter,
  repoInput,
  setRepoInput,
  addRepoPending,
  onAddRepo,
  addRepoError,
  addedRepo
}: DiscoverFiltersProps) {
  const t = useT();

  return (
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
            {filters.map((f) => (
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
            <option value="trend">{t.discover.sortTrend}</option>
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
            <FilterChip
              label={`${t.discover.queryLabel}: ${query}`}
              onRemove={() => resetPaged(() => setQuery(""))}
            />
          ) : null}
          {filter !== "explore" ? (
            <FilterChip
              label={`${t.discover.modeLabel}: ${filter}`}
              onRemove={() => resetPaged(() => setFilter("explore"))}
            />
          ) : null}
          {language ? (
            <FilterChip
              label={`${t.discover.languageLabel}: ${language}`}
              onRemove={() => resetPaged(() => setLanguage(""))}
            />
          ) : null}
          {topics.map((topic) => (
            <FilterChip key={topic} label={`#${topic}`} onRemove={() => toggleTopic(topic)} />
          ))}
          {typeof starsMin === "number" ? (
            <FilterChip
              label={`${t.discover.starsMinLabel}: ${starsMin}`}
              onRemove={() => resetPaged(() => setStarsMin(""))}
            />
          ) : null}
          {typeof scoreMin === "number" ? (
            <FilterChip
              label={`${t.discover.scoreMinLabel}: ${scoreMin}`}
              onRemove={() => resetPaged(() => setScoreMin(""))}
            />
          ) : null}
          {typeof abandonmentMax === "number" ? (
            <FilterChip
              label={`${t.discover.riskMaxLabel}: ${abandonmentMax}`}
              onRemove={() => resetPaged(() => setAbandonmentMax(""))}
            />
          ) : null}
          {sort !== "score" ? (
            <FilterChip
              label={`${t.discover.sortLabel}: ${sort}`}
              onRemove={() => resetPaged(() => setSort("score"))}
            />
          ) : null}
          {includeArchived ? (
            <FilterChip
              label={t.discover.includeArchived}
              onRemove={() => resetPaged(() => setIncludeArchived(false))}
            />
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
            onClick={onAddRepo}
            disabled={addRepoPending || !repoInput.trim()}
          >
            {addRepoPending ? t.discover.addRepoPending : t.discover.addRepoAction}
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
          {(addedRepo.alreadyIndexed ? t.discover.addRepoExists : t.discover.addRepoSuccess)}{" "}
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
  );
}
