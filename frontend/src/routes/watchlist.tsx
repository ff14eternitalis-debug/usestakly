import { Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Chip } from "../components/Chip";
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

export function WatchlistPage() {
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
    <section className="shell grid gap-10 py-12 md:py-16">
      <header className="grid gap-4">
        <p className="eyebrow">
          <span className="callout-mark" />
          Your watchlist
        </p>
        <div className="grid gap-4 md:grid-cols-[1.3fr_1fr] md:items-end">
          <h1 className="display-lg max-w-[22ch]">
            The short list,
            <br />
            <span className="italic-accent">under observation.</span>
          </h1>
          <p className="max-w-[44ch] text-[1rem] leading-relaxed text-ink-dim md:justify-self-end md:text-right">
            We diff scores between recomputes. If a repo drifts, you'll see it
            in{" "}
            <Link to="/notifications" className="link-underline">
              notifications
            </Link>
            .
          </p>
        </div>
      </header>

      {query.isLoading ? (
        <div className="py-10 text-center text-ink-muted">
          <span className="kicker">Pulling the file…</span>
        </div>
      ) : items.length === 0 ? (
        <div className="grid gap-4 border-t border-line pt-10 text-ink-dim">
          <p className="display-md">Nothing on watch yet.</p>
          <p className="max-w-[52ch] text-[1rem] leading-relaxed">
            Open a repo's profile from the{" "}
            <Link to="/discover" className="link-underline">
              register
            </Link>{" "}
            and tap <em>Add to watchlist</em>. You'll be pinged here when a score drops, abandonment rises, or a severe flag lands.
          </p>
        </div>
      ) : (
        <ul className="grid gap-8">
          {items.map((w, i) => {
            const overallTone = scoreTone(w.overall);
            return (
              <li
                key={w.id}
                className="grid gap-5 border-t border-line pt-6 md:grid-cols-[1fr_auto]"
              >
                <div className="grid gap-3">
                  <Link
                    to="/repos/$id"
                    params={{ id: w.artifactId }}
                    className="group inline-flex items-baseline gap-3"
                  >
                    <span className="kicker data-value">
                      {String(i + 1).padStart(2, "0")}.
                    </span>
                    <h2 className="display-md font-display">
                      <span className="font-mono text-[0.78em] text-ink-muted mr-1">
                        {w.owner}/
                      </span>
                      <span className="italic-accent">{w.name}</span>
                    </h2>
                  </Link>
                  <div className="flex flex-wrap items-center gap-2">
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
                    <span className="kicker">
                      watched {formatRelative(w.watchedAt)}
                    </span>
                  </div>
                  <div className="flex flex-wrap items-center gap-6 pt-1 font-mono text-[0.84rem] text-ink-dim">
                    <span>★ {formatStars(w.starsCount)}</span>
                    <span>
                      abandonment{" "}
                      <span
                        className="data-value"
                        style={{
                          color:
                            abandonmentTone(w.abandonment) === "danger"
                              ? "var(--color-ember)"
                              : abandonmentTone(w.abandonment) === "warn"
                                ? "var(--color-ochre)"
                                : "var(--color-ink)"
                        }}
                      >
                        {formatScore(w.abandonment)}
                      </span>
                    </span>
                    <span>
                      commits · {formatRelative(w.lastCommitAt).replace(" ago", "")}
                    </span>
                  </div>
                </div>

                <div className="grid gap-3 md:justify-items-end md:text-right">
                  <div>
                    <p className="kicker">Overall</p>
                    <p
                      className="font-display italic-accent data-value text-[3.2rem] leading-none"
                      style={{
                        color:
                          overallTone === "danger"
                            ? "var(--color-ember)"
                            : overallTone === "warn"
                              ? "var(--color-ochre)"
                              : overallTone === "ok"
                                ? "var(--color-ink)"
                                : "var(--color-ink-muted)"
                      }}
                    >
                      {formatScore(w.overall)}
                    </p>
                  </div>
                  <div className="flex gap-2">
                    <button
                      type="button"
                      onClick={() =>
                        toggleMute.mutate({ id: w.artifactId, muted: !w.muted })
                      }
                      className="border border-line px-3 py-1.5 font-mono text-[0.78rem] uppercase tracking-[0.14em] text-ink-dim hover:border-ink-dim hover:text-ink"
                      style={{ borderRadius: 2 }}
                    >
                      {w.muted ? "unmute" : "mute"}
                    </button>
                    <button
                      type="button"
                      onClick={() => remove.mutate(w.artifactId)}
                      className="border border-ember/40 px-3 py-1.5 font-mono text-[0.78rem] uppercase tracking-[0.14em] text-ember hover:bg-ember hover:text-paper-soft"
                      style={{ borderRadius: 2 }}
                    >
                      remove
                    </button>
                  </div>
                </div>
              </li>
            );
          })}
        </ul>
      )}
    </section>
  );
}
