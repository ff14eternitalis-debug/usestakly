import { useState, type FormEvent } from "react";
import { useMutation } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";

import { Button } from "../../../components/Button";
import { Chip } from "../../../components/Chip";
import { useT } from "../../../i18n";
import { createUseCaseWatch, recommendUseCase } from "../../../lib/api/use-cases";
import { formatScore, formatStars } from "../../../lib/format";
import { useAuthStore } from "../../../state/auth-store";

export function UseCaseSearchPanel() {
  const t = useT();
  const authStatus = useAuthStore((state) => state.status);
  const [query, setQuery] = useState("");
  const [riskTolerance, setRiskTolerance] = useState<"low" | "medium" | "high">(
    "medium"
  );

  const recommendation = useMutation({
    mutationFn: () =>
      recommendUseCase({
        query: query.trim(),
        riskTolerance,
        limit: 6
      })
  });
  const watch = useMutation({
    mutationFn: () =>
      createUseCaseWatch({
        query: recommendation.data?.query ?? query.trim(),
        riskTolerance
      })
  });

  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!query.trim()) return;
    recommendation.mutate();
  };

  return (
    <section className="surface grid gap-4 p-4 md:p-5">
      <div className="grid gap-2">
        <span className="kicker">{t.discover.intentEyebrow}</span>
        <div className="grid gap-2 md:grid-cols-[1fr_auto] md:items-end">
          <div className="grid gap-1">
            <h2 className="text-[1.1rem] font-semibold text-fg">
              {t.discover.intentTitle}
            </h2>
            <p className="max-w-[70ch] text-[0.9rem] leading-relaxed text-fg-dim">
              {t.discover.intentBody}
            </p>
          </div>
        </div>
      </div>

      <form
        onSubmit={submit}
        className="grid gap-3 md:grid-cols-[1fr_180px_auto] md:items-end"
      >
        <label className="grid gap-1.5">
          <span className="kicker">{t.discover.intentQueryLabel}</span>
          <input
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder={t.discover.intentPlaceholder}
            className="input"
          />
        </label>
        <label className="grid gap-1.5">
          <span className="kicker">{t.discover.intentRiskLabel}</span>
          <select
            value={riskTolerance}
            onChange={(event) =>
              setRiskTolerance(event.target.value as "low" | "medium" | "high")
            }
            className="input"
          >
            <option value="medium">{t.discover.intentRiskMedium}</option>
            <option value="low">{t.discover.intentRiskLow}</option>
            <option value="high">{t.discover.intentRiskHigh}</option>
          </select>
        </label>
        <Button
          type="submit"
          variant="primary"
          disabled={recommendation.isPending || !query.trim()}
        >
          {recommendation.isPending
            ? t.discover.intentSearching
            : t.discover.intentAction}
        </Button>
      </form>

      {recommendation.isError ? (
        <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
          {t.discover.intentError}
        </p>
      ) : null}

      {recommendation.data ? (
        <div className="grid gap-4 border-t border-line pt-4">
          <div className="grid gap-3 md:grid-cols-[1fr_auto] md:items-start">
            <div className="grid gap-2">
              <p className="text-[0.95rem] font-medium text-fg">
                {t.discover.intentDetected}:{" "}
                <span className="text-accent">
                  {recommendation.data.intent.label}
                </span>
              </p>
              <div className="flex flex-wrap gap-1.5">
                {recommendation.data.intent.categories.map((category) => (
                  <Chip key={category} tone="info">
                    {category}
                  </Chip>
                ))}
                {recommendation.data.intent.topics.slice(0, 8).map((topic) => (
                  <Chip key={topic} tone="neutral">
                    #{topic}
                  </Chip>
                ))}
              </div>
            </div>
            <span className="mono rounded-[6px] border border-line bg-bg-subtle px-2.5 py-1 text-[0.72rem] uppercase tracking-[0.14em] text-fg-muted">
              confidence={recommendation.data.intent.confidence}
            </span>
          </div>

          <div className="flex flex-wrap items-center gap-3">
            {authStatus === "authenticated" ? (
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => watch.mutate()}
                disabled={watch.isPending}
              >
                {watch.isPending
                  ? t.discover.intentWatchPending
                  : t.discover.intentWatchAction}
              </Button>
            ) : (
              <Link
                to="/login"
                search={{ returnTo: "/discover" }}
                className="rounded-[6px] border border-line bg-surface px-3 py-1.5 text-[0.82rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent"
              >
                {t.discover.intentWatchSignIn}
              </Link>
            )}
            {watch.isSuccess ? (
              <p className="text-[0.84rem] text-accent">
                {t.discover.intentWatchCreated}
              </p>
            ) : null}
            {watch.isError ? (
              <p className="text-[0.84rem]" style={{ color: "var(--color-danger)" }}>
                {t.discover.intentWatchError}
              </p>
            ) : null}
          </div>

          {recommendation.data.recommendations.length ? (
            <div className="grid gap-3">
              {recommendation.data.recommendations.map((repo, index) => (
                <Link
                  key={repo.artifactId}
                  to="/repos/$id"
                  params={{ id: repo.artifactId }}
                  className="group grid gap-3 rounded-[8px] border border-line bg-surface/45 p-4 transition-colors hover:border-accent/80"
                >
                  <div className="grid gap-2 md:grid-cols-[1fr_auto] md:items-start">
                    <div className="grid gap-1">
                      <p className="text-[0.98rem] font-semibold text-fg">
                        <span className="mono mr-2 text-[0.75rem] text-fg-muted">
                          {String(index + 1).padStart(2, "0")}
                        </span>
                        <span className="group-hover:text-accent">
                          {repo.fullName}
                        </span>
                      </p>
                      <p className="max-w-[76ch] text-[0.86rem] leading-relaxed text-fg-dim">
                        {repo.reason}
                      </p>
                    </div>
                    <div className="grid grid-cols-3 gap-2 text-right sm:min-w-[260px]">
                      <Metric label={t.discover.intentQuality} value={formatScore(repo.quality?.overall)} />
                      <Metric label={t.discover.intentMatch} value={formatScore(repo.matchScore)} />
                      <Metric label={t.discover.intentStars} value={formatStars(repo.starsCount)} />
                    </div>
                  </div>
                  <div className="flex flex-wrap gap-1.5">
                    {repo.language ? <Chip tone="info">{repo.language}</Chip> : null}
                    {repo.categories.slice(0, 3).map((category) => (
                      <Chip key={category.category} tone="accent">
                        {category.category}
                      </Chip>
                    ))}
                    <Chip tone={repo.risk === "low" ? "info" : repo.risk === "high" ? "warn" : "neutral"}>
                      risk {repo.risk}
                    </Chip>
                    {repo.matchedTopics.slice(0, 5).map((topic) => (
                      <Chip key={topic} tone="neutral">
                        #{topic}
                      </Chip>
                    ))}
                    {repo.quality?.formulaVersion ? (
                      <Chip tone="neutral">{repo.quality.formulaVersion}</Chip>
                    ) : null}
                  </div>
                </Link>
              ))}
            </div>
          ) : (
            <div className="rounded-[8px] border border-line bg-bg-subtle p-4">
              <p className="text-[0.9rem] font-medium text-fg">
                {t.discover.intentNoResult}
              </p>
              {recommendation.data.fallbackCandidates.length ? (
                <p className="mt-2 text-[0.86rem] leading-relaxed text-fg-dim">
                  {t.discover.intentFallback}{" "}
                  {recommendation.data.fallbackCandidates.join(", ")}
                </p>
              ) : null}
            </div>
          )}
        </div>
      ) : null}
    </section>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="grid gap-0.5">
      <span className="kicker">{label}</span>
      <span className="data-value text-[1.05rem] text-fg">{value}</span>
    </div>
  );
}
