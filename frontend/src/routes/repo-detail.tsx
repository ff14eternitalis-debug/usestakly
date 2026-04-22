import { Link, useParams } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Button, buttonClass } from "../components/Button";
import { Chip } from "../components/Chip";
import { ScoreBar } from "../components/ScoreBar";
import {
  ApiError,
  apiDelete,
  apiGet,
  apiPost
} from "../lib/api-client";
import {
  abandonmentTone,
  flagLabel,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import type { RepoProfile, WatchedRepo } from "../lib/types";
import { useAuthStore } from "../state/auth-store";

function toneFromFlag(flag: string): "danger" | "warn" | "neutral" {
  if (flag === "security-issue" || flag === "broken") return "danger";
  if (flag === "deprecated" || flag === "unmaintained" || flag === "abandoned")
    return "warn";
  return "neutral";
}

export function RepoDetailPage() {
  const { id } = useParams({ from: "/repos/$id" });
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  const queryClient = useQueryClient();

  const profile = useQuery({
    queryKey: ["repo", id],
    queryFn: ({ signal }) => apiGet<RepoProfile>(`/api/repos/${id}`, signal)
  });

  const watchlistQuery = useQuery({
    queryKey: ["watchlist"],
    queryFn: ({ signal }) => apiGet<WatchedRepo[]>("/api/watchlist", signal),
    enabled: isAuthed
  });

  const watching = (watchlistQuery.data ?? []).some(
    (w) => w.artifactId === id
  );

  const addWatch = useMutation({
    mutationFn: () => apiPost("/api/watchlist", { externalArtifactId: id }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["watchlist"] });
    }
  });

  const removeWatch = useMutation({
    mutationFn: () => apiDelete(`/api/watchlist/${id}`),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["watchlist"] });
    }
  });

  if (profile.isLoading) {
    return (
      <section className="shell py-16 text-center text-ink-muted">
        <span className="kicker">Pulling the file…</span>
      </section>
    );
  }

  if (profile.isError || !profile.data) {
    const err = profile.error;
    const notFound = err instanceof ApiError && err.status === 404;
    return (
      <section className="shell grid gap-4 py-16 text-ink-muted">
        <p className="kicker text-ember">
          {notFound ? "Not in the register" : "Observatory offline"}
        </p>
        <p className="text-[0.95rem]">
          {notFound
            ? "No profile exists under this identifier."
            : "The backend didn't answer."}
        </p>
        <Link to="/discover" className="link-underline w-fit text-ink">
          Back to discover <span className="arrow">→</span>
        </Link>
      </section>
    );
  }

  const repo = profile.data;
  const q = repo.quality;
  const overallTone = scoreTone(q?.overall);

  return (
    <article className="shell grid gap-12 py-12 md:py-16">
      <header className="grid gap-6">
        <div className="flex items-center justify-between flex-wrap gap-3">
          <Link
            to="/discover"
            className="kicker link-underline border-0"
          >
            ← Register
          </Link>
          <p className="kicker">
            formula {q?.formulaVersion ?? "—"} · computed{" "}
            {formatRelative(q?.computedAt ?? null)}
          </p>
        </div>

        <div className="grid gap-3">
          <p className="font-mono text-[0.84rem] uppercase tracking-[0.18em] text-ink-muted">
            {repo.owner} / {repo.name}
          </p>
          <h1 className="display-xl">
            <span className="italic-accent">{repo.name}</span>
          </h1>
          {repo.description ? (
            <p className="max-w-[64ch] text-[1.1rem] leading-[1.55] text-ink-dim">
              {repo.description}
            </p>
          ) : null}
        </div>

        <div className="flex flex-wrap items-center gap-2">
          {repo.language ? <Chip tone="info">{repo.language}</Chip> : null}
          {repo.licenseSpdx ? (
            <Chip tone="neutral">{repo.licenseSpdx}</Chip>
          ) : null}
          {repo.archived ? <Chip tone="warn">archived</Chip> : null}
          {q?.flags.map((flag) => (
            <Chip key={flag} tone={toneFromFlag(flag)}>
              {flagLabel(flag)}
            </Chip>
          ))}
          {repo.topics.map((t) => (
            <Chip key={t} tone="neutral">
              #{t}
            </Chip>
          ))}
        </div>

        <div className="flex flex-wrap items-center gap-3 pt-2">
          {isAuthed ? (
            watching ? (
              <Button
                variant="outline"
                onClick={() => removeWatch.mutate()}
                disabled={removeWatch.isPending}
              >
                {removeWatch.isPending ? "Unwatching…" : "Unwatch"}
              </Button>
            ) : (
              <Button
                variant="primary"
                onClick={() => addWatch.mutate()}
                disabled={addWatch.isPending}
                iconAfter={<span className="arrow">→</span>}
              >
                {addWatch.isPending ? "Adding…" : "Add to watchlist"}
              </Button>
            )
          ) : (
            <Link to="/login" className={buttonClass("outline")}>
              Sign in to watch
            </Link>
          )}
          <a
            href={repo.htmlUrl}
            target="_blank"
            rel="noreferrer"
            className="font-mono text-[0.82rem] uppercase tracking-[0.16em] text-ink-muted hover:text-ink"
          >
            github.com ↗
          </a>
        </div>
      </header>

      <hr className="rule-double" />

      <section className="grid gap-10 md:grid-cols-[260px_1fr] md:gap-16">
        <div className="grid gap-3">
          <p className="kicker">Overall verdict</p>
          <p
            className="font-display italic-accent data-value text-[6rem] leading-none"
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
            {formatScore(q?.overall)}
          </p>
          <p className="kicker">
            {overallTone === "ok"
              ? "Healthy"
              : overallTone === "warn"
                ? "Monitor"
                : overallTone === "danger"
                  ? "At risk"
                  : "Unscored"}
          </p>

          <div className="mt-4 grid gap-2 border-t border-line pt-4 text-[0.92rem]">
            <Row k="Stars" v={formatStars(repo.starsCount)} />
            <Row k="Forks" v={formatStars(repo.forksCount)} />
            <Row k="Open issues" v={String(repo.openIssuesCount)} />
            <Row k="Subscribers" v={formatStars(repo.subscribersCount)} />
            <Row
              k="Last commit"
              v={formatRelative(repo.lastCommitAt)}
            />
            <Row
              k="Priors fetched"
              v={formatRelative(repo.priorsFetchedAt)}
            />
            {repo.defaultBranch ? (
              <Row k="Default branch" v={repo.defaultBranch} mono />
            ) : null}
          </div>
        </div>

        <div className="grid gap-8">
          <div>
            <p className="kicker mb-4">Dimensions</p>
            <div className="grid gap-6 md:grid-cols-2">
              <ScoreBar
                label="Freshness"
                value={q?.freshness ?? null}
                tone={scoreTone(q?.freshness ?? null)}
                hint="Exponential decay on last_commit_at (half-life 180d)."
              />
              <ScoreBar
                label="Adoption"
                value={q?.adoption ?? null}
                tone={scoreTone(q?.adoption ?? null)}
                hint="Log-normalised resolve count (saturates at 1k)."
              />
              <ScoreBar
                label="Reliability"
                value={q?.reliability ?? null}
                tone={scoreTone(q?.reliability ?? null)}
                hint="Success / total builds. Neutral 0.5 before 5 samples."
              />
              <ScoreBar
                label="Abandonment"
                value={q?.abandonment ?? null}
                tone={abandonmentTone(q?.abandonment ?? null)}
                invert
                hint="Inverse freshness plus regret bump above threshold."
              />
            </div>
          </div>
        </div>
      </section>

      <hr className="rule-dashed" />

      <section className="grid gap-6">
        <div className="flex items-baseline justify-between">
          <h2 className="display-md">Recent signals</h2>
          <p className="kicker">
            {repo.recentSignals.length} entr
            {repo.recentSignals.length === 1 ? "y" : "ies"}
          </p>
        </div>
        {repo.recentSignals.length === 0 ? (
          <p className="text-[0.98rem] text-ink-muted border-t border-line pt-6">
            No signals reported yet. The observatory is listening.
          </p>
        ) : (
          <ul className="grid gap-3">
            {repo.recentSignals.map((signal, i) => (
              <li
                key={i}
                className="grid grid-cols-[auto_1fr_auto] items-baseline gap-4 border-t border-dashed border-line pt-3"
              >
                <Chip tone={signal.isPassive ? "neutral" : "info"} mono>
                  {signal.isPassive ? "passive" : "reported"}
                </Chip>
                <div className="grid gap-1">
                  <p className="font-mono text-[0.88rem] text-ink">
                    {signal.signal}
                  </p>
                  {signal.evidenceDescription ? (
                    <p className="text-[0.9rem] text-ink-dim">
                      {signal.evidenceDescription}
                    </p>
                  ) : null}
                </div>
                <span className="kicker">
                  {formatRelative(signal.createdAt)}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>
    </article>
  );
}

function Row({ k, v, mono = false }: { k: string; v: string; mono?: boolean }) {
  return (
    <div className="flex items-baseline justify-between gap-3">
      <span className="kicker">{k}</span>
      <span
        className={`text-ink ${mono ? "font-mono text-[0.86rem]" : "data-value text-[0.92rem]"}`}
      >
        {v}
      </span>
    </div>
  );
}
