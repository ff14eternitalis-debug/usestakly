import { Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { buttonClass } from "../components/Button";
import { Chip } from "../components/Chip";
import { useT } from "../i18n";
import { apiDelete, apiGet, apiPatch } from "../lib/api-client";
import {
  abandonmentTone,
  flagLabel,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import type { WatchedRepo } from "../lib/types";

function scoreColor(tone: "ok" | "warn" | "danger" | "neutral"): string {
  if (tone === "danger") return "var(--color-danger)";
  if (tone === "warn") return "var(--color-warn)";
  if (tone === "ok") return "var(--color-accent)";
  return "var(--color-fg-muted)";
}

export function WatchlistPage() {
  const t = useT();
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["watchlist"],
    queryFn: ({ signal }) => apiGet<WatchedRepo[]>("/api/watchlist", signal)
  });

  const toggleMute = useMutation({
    mutationFn: ({ id, muted }: { id: string; muted: boolean }) =>
      apiPatch(`/api/watchlist/${id}`, { muted }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["watchlist"] })
  });

  const remove = useMutation({
    mutationFn: (id: string) => apiDelete(`/api/watchlist/${id}`),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["watchlist"] })
  });

  const items = query.data ?? [];

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

      {query.isLoading ? (
        <div className="py-10 text-center">
          <span className="kicker">{t.watchlist.loading}</span>
        </div>
      ) : items.length === 0 ? (
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
        <ul className="grid gap-3">
          {items.map((w, i) => {
            const overallTone = scoreTone(w.overall);
            const abTone = abandonmentTone(w.abandonment);
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
                    className="rounded-[6px] border border-line px-3 py-1.5 mono text-[0.74rem] uppercase tracking-[0.12em] text-fg-dim hover:border-line-strong hover:text-fg transition-colors"
                  >
                    {w.muted ? t.watchlist.unmute : t.watchlist.mute}
                  </button>
                  <button
                    type="button"
                    onClick={() => remove.mutate(w.artifactId)}
                    className="rounded-[6px] border border-[color:var(--color-danger)]/30 px-3 py-1.5 mono text-[0.74rem] uppercase tracking-[0.12em] text-[color:var(--color-danger)] hover:bg-[color:var(--color-danger)]/10 transition-colors"
                  >
                    {t.watchlist.remove}
                  </button>
                </div>
              </li>
            );
          })}
        </ul>
      )}
    </section>
  );
}
