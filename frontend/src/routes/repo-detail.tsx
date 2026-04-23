import { Link, useParams } from "@tanstack/react-router";
import { useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Button, buttonClass } from "../components/Button";
import { Chip } from "../components/Chip";
import { ScoreBar } from "../components/ScoreBar";
import { useT } from "../i18n";
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
import type { RepoProfile, RepoViewerState, WatchedRepo } from "../lib/types";
import { useAuthStore } from "../state/auth-store";

function toneFromFlag(flag: string): "danger" | "warn" | "neutral" {
  if (flag === "security-issue" || flag === "broken") return "danger";
  if (flag === "deprecated" || flag === "unmaintained" || flag === "abandoned")
    return "warn";
  return "neutral";
}

function scoreColor(tone: "ok" | "warn" | "danger" | "neutral"): string {
  if (tone === "danger") return "var(--color-danger)";
  if (tone === "warn") return "var(--color-warn)";
  if (tone === "ok") return "var(--color-accent)";
  return "var(--color-fg-muted)";
}

export function RepoDetailPage() {
  const t = useT();
  const { id } = useParams({ from: "/repos/$id" });
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  const queryClient = useQueryClient();
  const [signal, setSignal] = useState("deprecated");
  const [evidenceUrl, setEvidenceUrl] = useState("");
  const [evidenceDescription, setEvidenceDescription] = useState("");
  const [disputeReason, setDisputeReason] = useState("");

  const profile = useQuery({
    queryKey: ["repo", id],
    queryFn: ({ signal }) => apiGet<RepoProfile>(`/api/repos/${id}`, signal)
  });

  const watchlistQuery = useQuery({
    queryKey: ["watchlist"],
    queryFn: ({ signal }) => apiGet<WatchedRepo[]>("/api/watchlist", signal),
    enabled: isAuthed
  });

  const viewerState = useQuery({
    queryKey: ["repo-viewer-state", id],
    queryFn: ({ signal }) =>
      apiGet<RepoViewerState>(`/api/repos/${id}/viewer-state`, signal),
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

  const createSignal = useMutation({
    mutationFn: () =>
      apiPost(`/api/repos/${id}/signals`, {
        signal,
        evidenceUrl: evidenceUrl.trim() || undefined,
        evidenceDescription: evidenceDescription.trim() || undefined
      }),
    onSuccess: async () => {
      setEvidenceUrl("");
      setEvidenceDescription("");
      await queryClient.invalidateQueries({ queryKey: ["repo", id] });
    }
  });

  const disputeSignal = useMutation({
    mutationFn: (signalId: string) =>
      apiPost(`/api/repos/${id}/signals/${signalId}/dispute`, {
        reason: disputeReason.trim()
      }),
    onSuccess: async () => {
      setDisputeReason("");
      await queryClient.invalidateQueries({ queryKey: ["repo-viewer-state", id] });
      await queryClient.invalidateQueries({ queryKey: ["repo", id] });
    }
  });

  function verdictLabel(tone: "ok" | "warn" | "danger" | "neutral"): string {
    if (tone === "ok") return t.repoDetail.healthy;
    if (tone === "warn") return t.repoDetail.monitor;
    if (tone === "danger") return t.repoDetail.atRisk;
    return t.repoDetail.unscored;
  }

  if (profile.isLoading) {
    return (
      <section className="shell py-16 text-center">
        <span className="kicker">{t.repoDetail.loading}</span>
      </section>
    );
  }

  if (profile.isError || !profile.data) {
    const err = profile.error;
    const notFound = err instanceof ApiError && err.status === 404;
    return (
      <section className="shell grid gap-4 py-16">
        <p className="kicker" style={{ color: "var(--color-danger)" }}>
          {notFound ? t.repoDetail.notFound : t.common.offline}
        </p>
        <p className="text-[0.94rem] text-fg-dim">
          {notFound ? t.repoDetail.notFoundBody : t.repoDetail.offlineBody}
        </p>
        <Link
          to="/discover"
          className="link-underline w-fit text-accent"
        >
          {t.repoDetail.backToDiscover} <span className="arrow">→</span>
        </Link>
      </section>
    );
  }

  const repo = profile.data;
  const q = repo.quality;
  const overallTone = scoreTone(q?.overall);
  const signalError =
    createSignal.error instanceof ApiError ? createSignal.error.message : null;
  const disputeError =
    disputeSignal.error instanceof ApiError ? disputeSignal.error.message : null;

  return (
    <article className="shell grid gap-10 py-10 md:py-14">
      <div className="flex items-center justify-between flex-wrap gap-2">
        <Link
          to="/discover"
          className="inline-flex items-center gap-1.5 text-[0.88rem] text-fg-dim hover:text-accent transition-colors"
        >
          <span>←</span> {t.repoDetail.back}
        </Link>
        <p className="mono text-[0.76rem] text-fg-muted">
          {t.repoDetail.formula} {q?.formulaVersion ?? "—"} ·{" "}
          {t.repoDetail.computed} {formatRelative(q?.computedAt ?? null)}
        </p>
      </div>

      <header className="grid gap-5">
        <div className="grid gap-2">
          <p className="mono text-[0.82rem] uppercase tracking-[0.14em] text-fg-muted">
            {repo.owner} / {repo.name}
          </p>
          <h1 className="display-xl !text-[clamp(2.2rem,5vw,3.8rem)]">
            {repo.name}
          </h1>
          {repo.description ? (
            <p className="max-w-[64ch] text-[1.02rem] leading-[1.6] text-fg-dim">
              {repo.description}
            </p>
          ) : null}
        </div>

        <div className="flex flex-wrap items-center gap-1.5">
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
          {repo.topics.map((tp) => (
            <Chip key={tp} tone="neutral">
              #{tp}
            </Chip>
          ))}
        </div>

        <div className="flex flex-wrap items-center gap-3 pt-1">
          {isAuthed ? (
            watching ? (
              <Button
                variant="outline"
                onClick={() => removeWatch.mutate()}
                disabled={removeWatch.isPending}
              >
                {removeWatch.isPending
                  ? t.repoDetail.unwatching
                  : t.repoDetail.unwatch}
              </Button>
            ) : (
              <Button
                variant="primary"
                onClick={() => addWatch.mutate()}
                disabled={addWatch.isPending}
                iconAfter={<span className="arrow">+</span>}
              >
                {addWatch.isPending
                  ? t.repoDetail.adding
                  : t.repoDetail.addToWatchlist}
              </Button>
            )
          ) : (
            <Link to="/login" className={buttonClass("outline")}>
              {t.repoDetail.signInToWatch}
            </Link>
          )}
          <a
            href={repo.htmlUrl}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1.5 text-[0.86rem] text-fg-dim hover:text-accent transition-colors"
          >
            {t.common.viewOnGithub} ↗
          </a>
        </div>
      </header>

      <hr className="hairline" />

      <section className="grid gap-8 md:grid-cols-[280px_1fr] md:gap-14">
        <div className="grid gap-4">
          <div className="surface p-5 grid gap-4">
            <span className="kicker">{t.repoDetail.overallVerdict}</span>
            <p
              className="data-value text-[5.4rem] leading-none tracking-tight"
              style={{ color: scoreColor(overallTone) }}
            >
              {formatScore(q?.overall)}
            </p>
            <div className="flex items-center gap-2">
              <span
                className="dot"
                style={{ color: scoreColor(overallTone) }}
              />
              <span className="text-[0.86rem] font-medium text-fg">
                {verdictLabel(overallTone)}
              </span>
            </div>
          </div>

          <div className="surface p-5 grid gap-2 text-[0.88rem]">
            <Row k={t.repoDetail.stars} v={formatStars(repo.starsCount)} />
            <Row k={t.repoDetail.forks} v={formatStars(repo.forksCount)} />
            <Row k={t.repoDetail.openIssues} v={String(repo.openIssuesCount)} />
            <Row k={t.repoDetail.subscribers} v={formatStars(repo.subscribersCount)} />
            <Row k={t.repoDetail.lastCommit} v={formatRelative(repo.lastCommitAt)} />
            <Row
              k={t.repoDetail.priorsFetched}
              v={formatRelative(repo.priorsFetchedAt)}
            />
            {repo.defaultBranch ? (
              <Row k={t.repoDetail.defaultBranch} v={repo.defaultBranch} mono />
            ) : null}
          </div>
        </div>

        <div className="grid gap-6">
          <span className="kicker">{t.repoDetail.dimensions}</span>
          <div className="grid gap-x-10 gap-y-6 md:grid-cols-2">
            <ScoreBar
              label={t.repoDetail.freshness}
              value={q?.freshness ?? null}
              tone={scoreTone(q?.freshness ?? null)}
              hint={t.repoDetail.freshnessHint}
            />
            <ScoreBar
              label={t.repoDetail.adoption}
              value={q?.adoption ?? null}
              tone={scoreTone(q?.adoption ?? null)}
              hint={t.repoDetail.adoptionHint}
            />
            <ScoreBar
              label={t.repoDetail.reliability}
              value={q?.reliability ?? null}
              tone={scoreTone(q?.reliability ?? null)}
              hint={t.repoDetail.reliabilityHint}
            />
            <ScoreBar
              label={t.repoDetail.abandonment}
              value={q?.abandonment ?? null}
              tone={abandonmentTone(q?.abandonment ?? null)}
              invert
              hint={t.repoDetail.abandonmentHint}
            />
          </div>
        </div>
      </section>

      <hr className="hairline" />

      <section className="grid gap-6">
        <div className="flex items-baseline justify-between">
          <h2 className="display-md">{t.repoDetail.recentSignals}</h2>
          <p className="kicker">
            {repo.recentSignals.length}{" "}
            {repo.recentSignals.length === 1
              ? t.repoDetail.entrySingle
              : t.repoDetail.entriesPlural}
          </p>
        </div>
        {repo.recentSignals.length === 0 ? (
          <p className="text-[0.94rem] text-fg-muted border-t border-line pt-6">
            {t.repoDetail.noSignals}
          </p>
        ) : (
          <ul className="grid gap-2">
            {repo.recentSignals.map((signal, i) => (
              <li
                key={i}
                className="grid grid-cols-[auto_1fr_auto] items-center gap-4 rounded-[6px] border border-line bg-surface/30 px-4 py-3 hover:border-line-strong transition-colors"
              >
                <Chip tone={signal.isPassive ? "neutral" : "info"} mono>
                  {signal.isPassive ? t.repoDetail.passive : t.repoDetail.reported}
                </Chip>
                <div className="grid gap-0.5">
                  <p className="mono text-[0.86rem] text-fg">
                    {signal.signal}
                  </p>
                  {signal.evidenceDescription ? (
                    <p className="text-[0.86rem] text-fg-dim">
                      {signal.evidenceDescription}
                    </p>
                  ) : null}
                  {signal.reviewStatus !== "accepted" || signal.events.length > 0 ? (
                    <div className="grid gap-1 pt-1">
                      <p className="mono text-[0.76rem] text-fg-muted">
                        {t.signals.status}: {signal.reviewStatus}
                      </p>
                      {signal.events.slice(0, 3).map((event, idx) => (
                        <p key={idx} className="mono text-[0.74rem] text-fg-muted">
                          {event.eventKind} · {formatRelative(event.createdAt)}
                          {event.note ? ` · ${event.note}` : ""}
                        </p>
                      ))}
                    </div>
                  ) : null}
                </div>
                <span className="kicker whitespace-nowrap">
                  {formatRelative(signal.createdAt)}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>

      {isAuthed ? (
        <>
          <hr className="hairline" />
          <section className="grid gap-4">
            <h2 className="display-md">{t.signals.title}</h2>
            <p className="max-w-[66ch] text-[0.94rem] leading-relaxed text-fg-dim">
              {t.signals.hint}
            </p>
            <div className="surface grid gap-4 p-5 md:grid-cols-2">
              <label className="grid gap-1.5">
                <span className="kicker">{t.signals.signalLabel}</span>
                <select
                  value={signal}
                  onChange={(e) => setSignal(e.target.value)}
                  className="input"
                >
                  <option value="deprecated">deprecated</option>
                  <option value="broken">broken</option>
                  <option value="security_issue">security_issue</option>
                  <option value="doesnt_match_claim">doesnt_match_claim</option>
                </select>
              </label>
              <label className="grid gap-1.5">
                <span className="kicker">{t.signals.evidenceUrlLabel}</span>
                <input
                  type="url"
                  value={evidenceUrl}
                  onChange={(e) => setEvidenceUrl(e.target.value)}
                  className="input"
                  placeholder="https://..."
                />
              </label>
              <label className="grid gap-1.5 md:col-span-2">
                <span className="kicker">{t.signals.evidenceDescriptionLabel}</span>
                <textarea
                  value={evidenceDescription}
                  onChange={(e) => setEvidenceDescription(e.target.value)}
                  className="input min-h-[120px]"
                />
              </label>
              <div className="flex flex-wrap items-center gap-3 md:col-span-2">
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => createSignal.mutate()}
                  disabled={createSignal.isPending}
                >
                  {createSignal.isPending ? t.signals.submitting : t.signals.submit}
                </Button>
                {signalError ? (
                  <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
                    {signalError}
                  </p>
                ) : createSignal.isSuccess ? (
                  <p className="text-[0.86rem] text-fg-dim">{t.signals.success}</p>
                ) : null}
              </div>
            </div>
          </section>
        </>
      ) : null}

      {isAuthed && viewerState.data?.canDisputeSignals ? (
        <>
          <hr className="hairline" />
          <section className="grid gap-4">
            <h2 className="display-md">{t.signals.ownerTitle}</h2>
            <p className="max-w-[66ch] text-[0.94rem] leading-relaxed text-fg-dim">
              {t.signals.ownerHint}
            </p>
            <div className="grid gap-3">
              {viewerState.data.visibleSignals
                .filter((item) => !item.isPassive)
                .map((item) => (
                  <div key={item.id} className="surface grid gap-3 p-4">
                    <div className="flex flex-wrap items-center gap-2">
                      <Chip tone="info" mono>
                        {item.signal}
                      </Chip>
                      <span className="kicker">
                        {t.signals.status}: {item.reviewStatus}
                      </span>
                    </div>
                    {item.evidenceDescription ? (
                      <p className="text-[0.9rem] text-fg-dim">{item.evidenceDescription}</p>
                    ) : null}
                    <label className="grid gap-1.5">
                      <span className="kicker">{t.signals.disputeReasonLabel}</span>
                      <textarea
                        value={disputeReason}
                        onChange={(e) => setDisputeReason(e.target.value)}
                        className="input min-h-[96px]"
                      />
                    </label>
                    <div className="flex flex-wrap items-center gap-3">
                      <Button
                        type="button"
                        variant="outline"
                        onClick={() => disputeSignal.mutate(item.id)}
                        disabled={disputeSignal.isPending || disputeReason.trim().length < 10}
                      >
                        {disputeSignal.isPending ? t.signals.disputing : t.signals.dispute}
                      </Button>
                      {disputeError ? (
                        <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
                          {disputeError}
                        </p>
                      ) : disputeSignal.isSuccess ? (
                        <p className="text-[0.86rem] text-fg-dim">{t.signals.disputed}</p>
                      ) : null}
                    </div>
                  </div>
                ))}
            </div>
          </section>
        </>
      ) : null}
    </article>
  );
}

function Row({ k, v, mono = false }: { k: string; v: string; mono?: boolean }) {
  return (
    <div className="flex items-baseline justify-between gap-3">
      <span className="kicker">{k}</span>
      <span
        className={`text-fg ${mono ? "mono text-[0.84rem]" : "data-value text-[0.88rem]"}`}
      >
        {v}
      </span>
    </div>
  );
}
