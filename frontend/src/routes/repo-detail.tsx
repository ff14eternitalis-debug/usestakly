import { Link, useParams } from "@tanstack/react-router";
import { useEffect, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { ApiError } from "../lib/api-client";
import {
  createRepoSignal,
  disputeRepoSignal,
  getRepoProfile,
  getRepoViewerState,
  refreshRepoProfile
} from "../lib/api/repos";
import { formatRelative, scoreTone } from "../lib/format";
import { addRepoToWatchlist, getWatchlist, removeRepoFromWatchlist } from "../lib/api/watchlist";
import type { RepoProfile } from "../lib/types";
import { useT } from "../i18n";
import { useAuthStore } from "../state/auth-store";
import { OwnerDisputePanel } from "../features/repos/components/OwnerDisputePanel";
import { RepoHeader } from "../features/repos/components/RepoHeader";
import { RepoMetricsPanel } from "../features/repos/components/RepoMetricsPanel";
import { StructuralRefreshBanner } from "../features/repos/components/StructuralRefreshBanner";
import { RepoRecommendationExplanation } from "../features/repos/components/RepoRecommendationExplanation";
import { RepoScoreHistory } from "../features/repos/components/RepoScoreHistory";
import { RepoSignalsList } from "../features/repos/components/RepoSignalsList";
import { ReportSignalForm } from "../features/repos/components/ReportSignalForm";
import { labelsForExplanation } from "../lib/repo-explanation";
import { useSeoOverride } from "../seo/seo-context";

export function RepoDetailPage() {
  const t = useT();
  const { id } = useParams({ from: "/repos/$id" });
  const { setOverride } = useSeoOverride();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  const queryClient = useQueryClient();
  const [signal, setSignal] = useState("deprecated");
  const [evidenceUrl, setEvidenceUrl] = useState("");
  const [evidenceDescription, setEvidenceDescription] = useState("");
  const [disputeReason, setDisputeReason] = useState("");
  const [structuralRefreshing, setStructuralRefreshing] = useState(false);
  const refreshAttempted = useRef(false);

  const profile = useQuery({
    queryKey: ["repo", id],
    queryFn: ({ signal }) => getRepoProfile(id, signal)
  });

  useEffect(() => {
    setOverride(null);
  }, [id, setOverride]);

  useEffect(() => {
    const data = profile.data;
    if (!data || data.artifactId !== id || refreshAttempted.current) return;
    const needsRefresh =
      data.ingestionStatus.structuralStale ||
      !data.ingestionStatus.structuralComplete;
    if (!needsRefresh) return;
    refreshAttempted.current = true;
    setStructuralRefreshing(true);
    void refreshRepoProfile(id)
      .then(() => queryClient.invalidateQueries({ queryKey: ["repo", id] }))
      .catch(() => undefined)
      .finally(() => setStructuralRefreshing(false));
  }, [profile.data, id, queryClient]);

  useEffect(() => {
    refreshAttempted.current = false;
  }, [id]);

  useEffect(() => {
    const data = profile.data;
    if (!data || data.artifactId !== id) return;
    const desc =
      data.description?.trim().slice(0, 300) ||
      `Quality score and signals for ${data.fullName} on UseStakly.`;
    setOverride({
      title: `${data.fullName} — UseStakly`,
      description: desc,
      ogType: "article"
    });
  }, [profile.data, id, setOverride]);

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
  const explanation = labelsForExplanation(t.repoExplanation, repo.recommendationExplanation);
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

      <StructuralRefreshBanner
        proofTier={repo.proofTier}
        ingestionStatus={repo.ingestionStatus}
        refreshing={structuralRefreshing}
        proofTierLabels={t.dimensionDisplay.proofTier}
        refreshingLabel={t.repoDetail.refreshingGithub}
        incompleteLabel={t.repoDetail.structuralIncomplete}
        howToReadLabel={t.repoDetail.scoreGuideAction}
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
        vitalityLabel={t.repoDetail.vitality}
        dimensionDisplayStates={t.dimensionDisplay.states}
        vitalityCollectiveLabel={t.repoDetail.vitalityCollective}
        vitalityCadenceLabel={t.repoDetail.vitalityCadence}
        vitalityCiLabel={t.repoDetail.vitalityCi}
        vitalityReleaseLabel={t.repoDetail.vitalityRelease}
        vitalityNotCapturedLabel={t.repoDetail.vitalityNotCaptured}
        vitalityNeverReleasedLabel={t.repoDetail.vitalityNeverReleased}
        ciYesLabel={t.repoDetail.ciYes}
        ciNoLabel={t.repoDetail.ciNo}
        starsLabel={t.repoDetail.stars}
        forksLabel={t.repoDetail.forks}
        openIssuesLabel={t.repoDetail.openIssues}
        subscribersLabel={t.repoDetail.subscribers}
        lastCommitLabel={t.repoDetail.lastCommit}
        priorsFetchedLabel={t.repoDetail.priorsFetched}
        defaultBranchLabel={t.repoDetail.defaultBranch}
      />

      <section className="grid gap-4 rounded-[8px] border border-line bg-surface/45 p-5 md:grid-cols-[0.55fr_1.45fr]">
        <div className="grid content-start gap-2">
          <p className="kicker">{t.repoDetail.overallVerdict}</p>
          <h2 className="text-[1.05rem] font-semibold text-fg">
            {t.repoDetail.scoreGuideTitle}
          </h2>
          <Link
            to="/how-to-read"
            className="inline-flex w-fit items-center text-[0.86rem] font-medium text-accent hover:underline"
          >
            {t.repoDetail.scoreGuideAction} <span className="arrow">→</span>
          </Link>
        </div>
        <div className="grid gap-3">
          <p className="text-[0.92rem] leading-relaxed text-fg-dim">
            {t.repoDetail.scoreGuideBody}
          </p>
          <ul className="grid gap-2">
            {t.repoDetail.scoreGuideItems.map((item) => (
              <li
                key={item}
                className="flex gap-3 text-[0.88rem] leading-relaxed text-fg-dim"
              >
                <span className="mt-2 size-1.5 shrink-0 rounded-full bg-accent" />
                <span>{item}</span>
              </li>
            ))}
          </ul>
        </div>
      </section>

      <ScoreProvenancePanel repo={repo} />

      <RepoRecommendationExplanation
        title={t.repoExplanation.title}
        includedLabel={t.repoExplanation.included}
        caveatsLabel={t.repoExplanation.caveats}
        includedBecause={explanation.included}
        caveats={explanation.caveats}
      />

      <RepoScoreHistory
        title={t.repoDetail.scoreSnapshotTitle}
        currentLabel={t.repoDetail.scoreSnapshotCurrent}
        previousLabel={t.repoDetail.scoreSnapshotPrevious}
        computedLabel={t.repoDetail.scoreSnapshotComputed}
        snapshot={repo.scoreSnapshot}
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

function ScoreProvenancePanel({ repo }: { repo: RepoProfile }) {
  const t = useT();
  const q = repo.quality;
  const resolveCount = q?.resolveCount ?? 0;
  const buildSuccessCount = q?.buildSuccessCount ?? 0;
  const buildFailureCount = q?.buildFailureCount ?? 0;
  const regretCount = q?.regretCount ?? 0;
  const usageCount =
    resolveCount + buildSuccessCount + buildFailureCount + regretCount;
  const buildCount = buildSuccessCount + buildFailureCount;
  const volumeLabel =
    usageCount === 0
      ? t.repoDetail.signalVolumeEmpty
      : usageCount < 5 || buildCount < 5
        ? t.repoDetail.signalVolumePartial
        : t.repoDetail.signalVolumeReady;

  return (
    <section className="grid gap-5 rounded-[8px] border border-line bg-surface/45 p-5">
      <div className="grid gap-2 md:grid-cols-[0.55fr_1.45fr]">
        <div>
          <p className="kicker">{t.repoDetail.provenanceTitle}</p>
          <p className="mt-2 text-[0.88rem] leading-relaxed text-fg-dim">
            {t.repoDetail.provenanceBody}
          </p>
        </div>
        <div className="grid gap-3 sm:grid-cols-2">
          <ProvenanceCard title={t.repoDetail.githubMetadata}>
            <ProvenanceRow
              label={t.repoDetail.freshnessSource}
              value={t.repoDetail.lastCommitSource}
            />
            <ProvenanceRow
              label={t.repoDetail.lastCommit}
              value={formatRelative(repo.lastCommitAt)}
            />
            <ProvenanceRow
              label={t.repoDetail.computed}
              value={formatRelative(q?.computedAt ?? null)}
            />
          </ProvenanceCard>

          <ProvenanceCard title={t.repoDetail.usageSignals}>
            <ProvenanceRow
              label={t.repoDetail.adoptionSource}
              value={`${resolveCount} ${t.repoDetail.resolveCount}`}
            />
            <ProvenanceRow
              label={t.repoDetail.reliabilitySource}
              value={
                buildCount > 0
                  ? `${buildSuccessCount}/${buildCount}`
                  : t.repoDetail.neutralReliability
              }
            />
            <ProvenanceRow
              label={t.repoDetail.regretCount}
              value={String(regretCount)}
            />
          </ProvenanceCard>
        </div>
      </div>

      <div className="grid gap-3 border-t border-line pt-4 sm:grid-cols-4">
        <SignalCount label={t.repoDetail.resolveCount} value={resolveCount} />
        <SignalCount
          label={t.repoDetail.buildSuccessCount}
          value={buildSuccessCount}
        />
        <SignalCount
          label={t.repoDetail.buildFailureCount}
          value={buildFailureCount}
        />
        <SignalCount label={t.repoDetail.regretCount} value={regretCount} />
      </div>

      <p className="text-[0.88rem] leading-relaxed text-fg-dim">
        {volumeLabel}
      </p>
    </section>
  );
}

function ProvenanceCard({
  title,
  children
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="rounded-[8px] border border-line bg-bg-subtle p-4">
      <p className="mono text-[0.76rem] uppercase text-accent">{title}</p>
      <div className="mt-3 grid gap-2">{children}</div>
    </div>
  );
}

function ProvenanceRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-baseline justify-between gap-3">
      <span className="kicker">{label}</span>
      <span className="text-right text-[0.84rem] text-fg">{value}</span>
    </div>
  );
}

function SignalCount({ label, value }: { label: string; value: number }) {
  return (
    <div className="rounded-[8px] border border-line bg-bg-subtle px-4 py-3">
      <p className="data-value text-[1.35rem] leading-none text-fg">{value}</p>
      <p className="mt-2 mono text-[0.68rem] uppercase tracking-[0.08em] text-fg-muted">
        {label}
      </p>
    </div>
  );
}
