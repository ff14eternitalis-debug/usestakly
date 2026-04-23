import { Link, useParams } from "@tanstack/react-router";
import { useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { ApiError } from "../lib/api-client";
import {
  createRepoSignal,
  disputeRepoSignal,
  getRepoProfile,
  getRepoViewerState
} from "../lib/api/repos";
import { formatRelative, scoreTone } from "../lib/format";
import { addRepoToWatchlist, getWatchlist, removeRepoFromWatchlist } from "../lib/api/watchlist";
import { useT } from "../i18n";
import { useAuthStore } from "../state/auth-store";
import { OwnerDisputePanel } from "../features/repos/components/OwnerDisputePanel";
import { RepoHeader } from "../features/repos/components/RepoHeader";
import { RepoMetricsPanel } from "../features/repos/components/RepoMetricsPanel";
import { RepoSignalsList } from "../features/repos/components/RepoSignalsList";
import { ReportSignalForm } from "../features/repos/components/ReportSignalForm";

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
    queryFn: ({ signal }) => getRepoProfile(id, signal)
  });

  const watchlistQuery = useQuery({
    queryKey: ["watchlist"],
    queryFn: ({ signal }) => getWatchlist(signal),
    enabled: isAuthed
  });

  const viewerState = useQuery({
    queryKey: ["repo-viewer-state", id],
    queryFn: ({ signal }) => getRepoViewerState(id, signal),
    enabled: isAuthed
  });

  const watching = (watchlistQuery.data ?? []).some((w) => w.artifactId === id);

  const addWatch = useMutation({
    mutationFn: () => addRepoToWatchlist(id),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["watchlist"] });
    }
  });

  const removeWatch = useMutation({
    mutationFn: () => removeRepoFromWatchlist(id),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["watchlist"] });
    }
  });

  const createSignal = useMutation({
    mutationFn: () =>
      createRepoSignal(id, {
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
      disputeRepoSignal(id, signalId, { reason: disputeReason.trim() }),
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
        <Link to="/discover" className="link-underline w-fit text-accent">
          {t.repoDetail.backToDiscover} <span className="arrow">→</span>
        </Link>
      </section>
    );
  }

  const repo = profile.data;
  const q = repo.quality;
  const signalError = createSignal.error instanceof ApiError ? createSignal.error.message : null;
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
          {t.repoDetail.formula} {q?.formulaVersion ?? "—"} · {t.repoDetail.computed}{" "}
          {formatRelative(q?.computedAt ?? null)}
        </p>
      </div>

      <RepoHeader
        repo={repo}
        isAuthed={isAuthed}
        watching={watching}
        watchPending={addWatch.isPending}
        unwatchPending={removeWatch.isPending}
        addToWatchlist={() => addWatch.mutate()}
        removeFromWatchlist={() => removeWatch.mutate()}
        signInToWatchLabel={t.repoDetail.signInToWatch}
        signInToWatchHint={t.repoDetail.signInToWatchHint}
        addLabel={t.repoDetail.addToWatchlist}
        addingLabel={t.repoDetail.adding}
        unwatchLabel={t.repoDetail.unwatch}
        unwatchingLabel={t.repoDetail.unwatching}
        viewOnGithubLabel={t.common.viewOnGithub}
      />

      <hr className="hairline" />

      <RepoMetricsPanel
        repo={repo}
        overallVerdictLabel={t.repoDetail.overallVerdict}
        verdictLabel={verdictLabel(scoreTone(q?.overall))}
        dimensionsLabel={t.repoDetail.dimensions}
        freshnessLabel={t.repoDetail.freshness}
        adoptionLabel={t.repoDetail.adoption}
        reliabilityLabel={t.repoDetail.reliability}
        abandonmentLabel={t.repoDetail.abandonment}
        freshnessHint={t.repoDetail.freshnessHint}
        adoptionHint={t.repoDetail.adoptionHint}
        reliabilityHint={t.repoDetail.reliabilityHint}
        abandonmentHint={t.repoDetail.abandonmentHint}
        starsLabel={t.repoDetail.stars}
        forksLabel={t.repoDetail.forks}
        openIssuesLabel={t.repoDetail.openIssues}
        subscribersLabel={t.repoDetail.subscribers}
        lastCommitLabel={t.repoDetail.lastCommit}
        priorsFetchedLabel={t.repoDetail.priorsFetched}
        defaultBranchLabel={t.repoDetail.defaultBranch}
      />

      <hr className="hairline" />

      <RepoSignalsList
        signals={repo.recentSignals}
        title={t.repoDetail.recentSignals}
        countSingleLabel={t.repoDetail.entrySingle}
        countPluralLabel={t.repoDetail.entriesPlural}
        emptyLabel={t.repoDetail.noSignals}
        passiveLabel={t.repoDetail.passive}
        reportedLabel={t.repoDetail.reported}
        statusLabel={t.signals.status}
      />

      {isAuthed ? (
        <>
          <hr className="hairline" />
          <ReportSignalForm
            title={t.signals.title}
            hint={t.signals.hint}
            signal={signal}
            evidenceUrl={evidenceUrl}
            evidenceDescription={evidenceDescription}
            isPending={createSignal.isPending}
            isSuccess={createSignal.isSuccess}
            error={signalError}
            onSignalChange={setSignal}
            onEvidenceUrlChange={setEvidenceUrl}
            onEvidenceDescriptionChange={setEvidenceDescription}
            onSubmit={() => createSignal.mutate()}
            signalLabel={t.signals.signalLabel}
            evidenceUrlLabel={t.signals.evidenceUrlLabel}
            evidenceDescriptionLabel={t.signals.evidenceDescriptionLabel}
            submittingLabel={t.signals.submitting}
            submitLabel={t.signals.submit}
            successLabel={t.signals.success}
          />
        </>
      ) : null}

      {isAuthed && viewerState.data?.canDisputeSignals ? (
        <>
          <hr className="hairline" />
          <OwnerDisputePanel
            signals={viewerState.data.visibleSignals}
            disputeReason={disputeReason}
            isPending={disputeSignal.isPending}
            isSuccess={disputeSignal.isSuccess}
            error={disputeError}
            onDisputeReasonChange={setDisputeReason}
            onDispute={(signalId) => disputeSignal.mutate(signalId)}
            title={t.signals.ownerTitle}
            hint={t.signals.ownerHint}
            statusLabel={t.signals.status}
            disputeReasonLabel={t.signals.disputeReasonLabel}
            disputingLabel={t.signals.disputing}
            disputeLabel={t.signals.dispute}
            disputedLabel={t.signals.disputed}
          />
        </>
      ) : null}
    </article>
  );
}
