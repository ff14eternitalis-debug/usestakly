import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { buttonClass } from "../components/Button";
import { Chip } from "../components/Chip";
import { useT } from "../i18n";
import { listUseCaseWatches } from "../lib/api/use-cases";
import { apiDelete, apiGet, apiPatch } from "../lib/api-client";
import {
  abandonmentTone,
  flagLabel,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import type { UseCaseWatch, WatchedRepo } from "../lib/types";

function scoreColor(tone: "ok" | "warn" | "danger" | "neutral"): string {
  if (tone === "danger") return "var(--color-danger)";
  if (tone === "warn") return "var(--color-warn)";
  if (tone === "ok") return "var(--color-accent)";
  return "var(--color-fg-muted)";
}

export function WatchlistPage() {
  const t = useT();
  const queryClient = useQueryClient();
  const [confirmingRemoveId, setConfirmingRemoveId] = useState<string | null>(null);

  const query = useQuery({
    queryKey: ["watchlist"],
    queryFn: ({ signal }) => apiGet<WatchedRepo[]>("/api/watchlist", signal)
  });
  const useCaseQuery = useQuery({
    queryKey: ["use-case-watches"],
    queryFn: ({ signal }) => listUseCaseWatches(signal)
  });

  const toggleMute = useMutation({
    mutationFn: ({ id, muted }: { id: string; muted: boolean }) =>
      apiPatch(`/api/watchlist/${id}`, { muted }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["watchlist"] })
  });

  const remove = useMutation({
    mutationFn: (id: string) => apiDelete(`/api/watchlist/${id}`),
    onSuccess: () => {
      setConfirmingRemoveId(null);
      return queryClient.invalidateQueries({ queryKey: ["watchlist"] });
    }
  });

  const items = query.data ?? [];
  const useCaseWatches = useCaseQuery.data ?? [];
  const isLoading = query.isLoading || useCaseQuery.isLoading;
  const isError = query.isError || useCaseQuery.isError;
  const hasAnyWatch = items.length > 0 || useCaseWatches.length > 0;

  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.watchlist.eyebrow}</span>
        <div className="grid gap-4 md:grid-cols-[1.3fr_1fr] md:items-end">
          <h1 className="display-lg max-w-[22ch]">
            {t.watchlist.h1Part1}
            <br />
            <span className="accent">{t.watchlist.h1Accent}</span>
          </h1>
          <p className="max-w-[44ch] text-[0.96rem] leading-relaxed text-fg-dim md:text-right">
            {t.watchlist.intro}{" "}
            <Link to="/notifications" className="link-underline text-accent">
              {t.watchlist.notifications}
            </Link>
            .
          </p>
        </div>
      </header>

      {isLoading ? (
        <div className="py-10 text-center">
          <span className="kicker">{t.watchlist.loading}</span>
        </div>
      ) : isError ? (
        <div className="surface grid gap-4 p-10 text-center">
          <p className="display-md !text-[1.3rem]">{t.watchlist.loadErrorTitle}</p>
          <p className="max-w-[52ch] mx-auto text-[0.96rem] leading-relaxed text-fg-dim">
            {t.watchlist.loadErrorBody}
          </p>
          <button
            type="button"
            onClick={() => {
              void query.refetch();
              void useCaseQuery.refetch();
            }}
            className={`${buttonClass("outline")} justify-self-center mt-2`}
          >
            {t.watchlist.retry}
          </button>
        </div>
      ) : !hasAnyWatch ? (
        <div className="surface grid gap-4 p-10 text-center">
          <p className="display-md !text-[1.3rem]">{t.watchlist.emptyTitle}</p>
          <p className="max-w-[52ch] mx-auto text-[0.96rem] leading-relaxed text-fg-dim">
            {t.watchlist.emptyBody}
          </p>
          <Link
            to="/discover"
            className={`${buttonClass("primary")} justify-self-center mt-2`}
          >
            {t.watchlist.emptyAction}
            <span className="arrow">→</span>
          </Link>
        </div>
      ) : (
        <div className="grid gap-8">
          {useCaseWatches.length > 0 ? (
            <section className="grid gap-3">
              <div className="flex items-center justify-between border-b border-line pb-3">
                <div>
                  <p className="kicker">{t.watchlist.needsLabel}</p>
                  <h2 className="text-[1.05rem] font-semibold text-fg">
                    {t.watchlist.needsTitle}
                  </h2>
                </div>
                <span className="mono text-[0.78rem] text-fg-muted">
                  {useCaseWatches.length} {t.watchlist.needsCount}
                </span>
              </div>
              <ul className="grid gap-3">
                {useCaseWatches.map((watch) => (
                  <UseCaseWatchItem key={watch.id} watch={watch} />
                ))}
              </ul>
            </section>
          ) : null}

          {items.length > 0 ? (
            <section className="grid gap-3">
              <div className="flex items-center justify-between border-b border-line pb-3">
                <div>
                  <p className="kicker">{t.watchlist.reposLabel}</p>
                  <h2 className="text-[1.05rem] font-semibold text-fg">
                    {t.watchlist.reposTitle}
                  </h2>
                </div>
                <span className="mono text-[0.78rem] text-fg-muted">
                  {items.length} {t.watchlist.reposCount}
                </span>
              </div>
              <ul className="grid gap-3">
                {items.map((w, i) => {
            const overallTone = scoreTone(w.overall);
            const abTone = abandonmentTone(w.abandonment);
            const isMuting =
              toggleMute.isPending && toggleMute.variables?.id === w.artifactId;
            const isRemoving = remove.isPending && remove.variables === w.artifactId;
            const wantsRemoveConfirm = confirmingRemoveId === w.artifactId;
            return (
              <li
                key={w.id}
                className="group grid gap-4 rounded-[10px] border border-line bg-surface/40 p-5 transition-colors hover:border-line-strong md:grid-cols-[40px_1fr_auto_auto] md:items-center md:gap-6"
              >
                <span className="kicker data-value md:self-center">
                  {String(i + 1).padStart(2, "0")}
                </span>

                <div className="grid gap-2 min-w-0">
                  <Link
                    to="/repos/$id"
                    params={{ id: w.artifactId }}
                    className="inline-flex items-baseline gap-1 flex-wrap"
                  >
                    <h2 className="display-md !text-[1.1rem] truncate">
                      <span className="mono text-[0.82em] font-normal text-fg-muted mr-0.5">
                        {w.owner}/
                      </span>
                      <span className="group-hover:text-accent transition-colors">
                        {w.name}
                      </span>
                    </h2>
                  </Link>
                  <div className="flex flex-wrap items-center gap-1.5">
                    {w.language ? <Chip tone="info">{w.language}</Chip> : null}
                    {w.archived ? <Chip tone="warn">archived</Chip> : null}
                    {w.muted ? <Chip tone="neutral">muted</Chip> : null}
                    {w.flags.map((f) => (
                      <Chip
                        key={f}
                        tone={
                          f === "security-issue" || f === "broken"
                            ? "danger"
                            : "warn"
                        }
                      >
                        {flagLabel(f)}
                      </Chip>
                    ))}
                  </div>
                  <div className="flex flex-wrap items-center gap-x-4 gap-y-1 mono text-[0.8rem] text-fg-muted">
                    <span>★ {formatStars(w.starsCount)}</span>
                    <span>
                      ab{" "}
                      <span
                        className="data-value"
                        style={{ color: scoreColor(abTone) }}
                      >
                        {formatScore(w.abandonment)}
                      </span>
                    </span>
                    <span>
                      {formatRelative(w.lastCommitAt).replace(" ago", "")}
                    </span>
                    <span>
                      {t.watchlist.watched} {formatRelative(w.watchedAt)}
                    </span>
                  </div>
                </div>

                <div className="grid gap-0.5 md:text-right">
                  <span className="kicker">{t.watchlist.overall}</span>
                  <span
                    className="data-value text-[2.6rem] leading-none tracking-tight"
                    style={{ color: scoreColor(overallTone) }}
                  >
                    {formatScore(w.overall)}
                  </span>
                </div>

                <div className="flex gap-2 md:flex-col md:items-stretch">
                  <button
                    type="button"
                    onClick={() =>
                      toggleMute.mutate({ id: w.artifactId, muted: !w.muted })
                    }
                    disabled={isMuting || remove.isPending}
                    className="rounded-[6px] border border-line px-3 py-1.5 mono text-[0.74rem] uppercase tracking-[0.12em] text-fg-dim hover:border-line-strong hover:text-fg disabled:cursor-not-allowed disabled:opacity-40 transition-colors"
                  >
                    {w.muted ? t.watchlist.unmute : t.watchlist.mute}
                  </button>
                  <button
                    type="button"
                    onClick={() => {
                      if (wantsRemoveConfirm) {
                        remove.mutate(w.artifactId);
                      } else {
                        setConfirmingRemoveId(w.artifactId);
                      }
                    }}
                    disabled={isRemoving || toggleMute.isPending}
                    className="rounded-[6px] border border-[color:var(--color-danger)]/30 px-3 py-1.5 mono text-[0.74rem] uppercase tracking-[0.12em] text-[color:var(--color-danger)] hover:bg-[color:var(--color-danger)]/10 disabled:cursor-not-allowed disabled:opacity-40 transition-colors"
                  >
                    {isRemoving
                      ? t.watchlist.removing
                      : wantsRemoveConfirm
                        ? t.watchlist.confirmRemove
                        : t.watchlist.remove}
                  </button>
                  {wantsRemoveConfirm && !isRemoving ? (
                    <button
                      type="button"
                      onClick={() => setConfirmingRemoveId(null)}
                      className="rounded-[6px] border border-line px-3 py-1.5 mono text-[0.74rem] uppercase tracking-[0.12em] text-fg-muted hover:border-line-strong hover:text-fg transition-colors"
                    >
                      {t.watchlist.cancelRemove}
                    </button>
                  ) : null}
                </div>
              </li>
            );
                })}
              </ul>
            </section>
          ) : null}
        </div>
      )}
    </section>
  );
}

function UseCaseWatchItem({ watch }: { watch: UseCaseWatch }) {
  const t = useT();
  return (
    <li className="grid gap-4 rounded-[10px] border border-line bg-surface/40 p-5 transition-colors hover:border-line-strong md:grid-cols-[1fr_auto] md:items-start">
      <div className="grid gap-3 min-w-0">
        <div className="grid gap-1">
          <h3 className="display-md !text-[1.08rem]">{watch.label}</h3>
          <p className="max-w-[70ch] text-[0.9rem] leading-relaxed text-fg-dim">
            {watch.queryText}
          </p>
        </div>
        <div className="flex flex-wrap gap-1.5">
          <Chip tone="info">{watch.normalizedIntent}</Chip>
          <Chip tone="neutral">risk {watch.riskTolerance}</Chip>
          {watch.topics.slice(0, 6).map((topic) => (
            <Chip key={topic} tone="neutral">
              #{topic}
            </Chip>
          ))}
        </div>
        {watch.topMatches.length > 0 ? (
          <div className="flex flex-wrap gap-2 text-[0.82rem] text-fg-dim">
            {watch.topMatches.slice(0, 4).map((match) => (
              <Link
                key={match.artifactId}
                to="/repos/$id"
                params={{ id: match.artifactId }}
                className="rounded-[999px] border border-line bg-bg-subtle px-3 py-1.5 transition-colors hover:border-accent hover:text-accent"
              >
                {match.fullName} · {formatScore(match.qualityScore)}
              </Link>
            ))}
          </div>
        ) : null}
      </div>
      <div className="grid gap-1 md:text-right">
        <span className="kicker">{t.watchlist.matches}</span>
        <span className="data-value text-[2.2rem] leading-none text-fg">
          {watch.matchCount}
        </span>
        <span className="mono text-[0.76rem] text-fg-muted">
          {formatRelative(watch.createdAt)}
        </span>
      </div>
    </li>
  );
}
