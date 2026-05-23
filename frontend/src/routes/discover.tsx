import { useEffect, useMemo, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "@tanstack/react-router";

import { DiscoverFilters } from "../features/repos/components/DiscoverFilters";
import {
  DiscoverRadarSection,
  DiscoverResults
} from "../features/repos/components/DiscoverResults";
import { UseCaseSearchPanel } from "../features/repos/components/UseCaseSearchPanel";
import { PAGE_SIZE, RADAR_PAGE_SIZE } from "../features/repos/components/discover-shared";
import { useT } from "../i18n";
import { ApiError, apiGet, apiPost } from "../lib/api-client";
import { trackEvent } from "../lib/analytics";
import type { AddRepoResponse, RepoSearchResponse, RepoSort, SearchFilter } from "../lib/types";
import { useAuthStore } from "../state/auth-store";

export function DiscoverPage() {
  const t = useT();
  const [discoverMode, setDiscoverMode] = useState<"recommended" | "radar" | "advanced">(
    "recommended"
  );
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
  const [radarPage, setRadarPage] = useState(0);
  const [repoInput, setRepoInput] = useState("");
  const [addedRepo, setAddedRepo] = useState<AddRepoResponse | null>(null);
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  const authState = isAuthed ? "signed_in" : "anonymous";

  const FILTERS = [
    { value: "explore" as const, label: t.discover.modeExplore, hint: t.discover.hintExplore },
    { value: "auto" as const, label: t.discover.modeAuto, hint: t.discover.hintAuto },
    { value: "strict" as const, label: t.discover.modeStrict, hint: t.discover.hintStrict }
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

  const radarSearch = useMemo(() => {
    const params = new URLSearchParams();
    params.set("filter", "explore");
    params.set("maturity_bands", "emerging,experimental");
    params.set("sort", "trend");
    params.set("limit", String(RADAR_PAGE_SIZE));
    params.set("offset", String(radarPage * RADAR_PAGE_SIZE));
    return params.toString();
  }, [radarPage]);

  const results = useQuery({
    queryKey: ["search", search],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(`/api/repos/search?${search}`, signal),
    placeholderData: (prev) => prev,
    enabled: discoverMode === "advanced"
  });

  const radarResults = useQuery({
    queryKey: ["search", "radar", radarSearch],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(`/api/repos/search?${radarSearch}`, signal),
    placeholderData: (prev) => prev,
    enabled: discoverMode === "radar"
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
  const hasPrevious = page > 0;
  const hasNext = results.data?.hasMore ?? false;
  const radarCount = radarResults.data?.items.length ?? 0;
  const radarOffset = radarResults.data?.offset ?? radarPage * RADAR_PAGE_SIZE;
  const hasPreviousRadar = radarPage > 0;
  const hasNextRadar = radarResults.data?.hasMore ?? false;
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

  const widenFilters = () => {
    setFilter("explore");
    setScoreMin("");
    setAbandonmentMax("");
    setPage(0);
  };

  useEffect(() => {
    trackEvent("discover_open", {
      route: "/discover",
      auth_state: authState
    });
  }, [authState]);

  const switchDiscoverMode = (
    mode: "recommended" | "radar" | "advanced"
  ) => {
    setDiscoverMode(mode);
    setPage(0);
    setRadarPage(0);
    if (mode === "radar" || mode === "advanced") {
      trackEvent("discover_search_submit", {
        route: "/discover",
        radar_mode: mode === "radar" ? "emerging" : "reliable",
        auth_state: authState
      });
    }
  };

  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.discover.eyebrow}</span>
        <h1 className="display-lg max-w-[22ch]">
          {t.discover.h1Part1}{" "}
          <span className="accent">{t.discover.h1Accent}</span> {t.discover.h1Part2}
        </h1>
        <p className="max-w-[60ch] text-[0.98rem] leading-relaxed text-fg-dim">
          {t.discover.intro}
        </p>
      </header>

      <div className="flex flex-wrap gap-2 border-b border-line">
        {[
          {
            value: "recommended" as const,
            label: t.discover.recommendedTab,
            hint: t.discover.recommendedTabHint
          },
          {
            value: "radar" as const,
            label: t.discover.radarTab,
            hint: t.discover.radarTabHint
          },
          {
            value: "advanced" as const,
            label: t.discover.advancedTab,
            hint: t.discover.advancedTabHint
          }
        ].map((tab) => {
          const active = discoverMode === tab.value;
          return (
            <button
              key={tab.value}
              type="button"
              onClick={() => {
                switchDiscoverMode(tab.value);
              }}
              className={`grid gap-1 border-b-2 px-1 pb-3 text-left transition-colors sm:min-w-44 ${
                active
                  ? "border-accent text-fg"
                  : "border-transparent text-fg-muted hover:text-fg"
              }`}
            >
              <span className="text-[0.95rem] font-semibold">{tab.label}</span>
            </button>
          );
        })}
      </div>

      {discoverMode === "recommended" ? <UseCaseSearchPanel /> : null}

      {discoverMode === "radar" ? (
        <DiscoverRadarSection
          query={radarResults}
          items={radarResults.data?.items ?? []}
          offset={radarOffset}
          count={radarCount}
          page={radarPage}
          onPageChange={setRadarPage}
          hasPrevious={hasPreviousRadar}
          hasNext={hasNextRadar}
        />
      ) : null}

      {discoverMode === "advanced" ? (
        <DiscoverFilters
          filters={FILTERS}
          query={query}
          setQuery={setQuery}
          filter={filter}
          setFilter={setFilter}
          language={language}
          setLanguage={setLanguage}
          topics={topics}
          toggleTopic={toggleTopic}
          scoreMin={scoreMin}
          setScoreMin={setScoreMin}
          abandonmentMax={abandonmentMax}
          setAbandonmentMax={setAbandonmentMax}
          includeArchived={includeArchived}
          setIncludeArchived={setIncludeArchived}
          sort={sort}
          setSort={setSort}
          starsMin={starsMin}
          setStarsMin={setStarsMin}
          resetPaged={resetPaged}
          clearAllFilters={clearAllFilters}
          hasActiveFilters={hasActiveFilters}
          activeFilter={activeFilter}
          repoInput={repoInput}
          setRepoInput={setRepoInput}
          addRepoPending={addRepo.isPending}
          onAddRepo={() => {
            trackEvent("add_repo_submit", {
              route: "/discover",
              auth_state: authState
            });
            addRepo.mutate();
          }}
          addRepoError={addRepoError}
          addedRepo={addedRepo}
        />
      ) : null}

      {discoverMode === "advanced" ? (
        <DiscoverResults
          variant="advanced"
          query={results}
          items={results.data?.items ?? []}
          offset={offset}
          count={count}
          page={page}
          onPageChange={setPage}
          hasPrevious={hasPrevious}
          hasNext={hasNext}
          onWidenFilters={widenFilters}
        />
      ) : null}
    </section>
  );
}
