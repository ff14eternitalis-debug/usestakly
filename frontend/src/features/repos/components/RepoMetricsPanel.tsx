import { ScoreBar } from "../../../components/ScoreBar";
import {
  abandonmentTone,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../../../lib/format";
import type { RepoProfile, VitalityInputs } from "../../../lib/types";

function scoreColor(tone: "ok" | "warn" | "danger" | "neutral"): string {
  if (tone === "danger") return "var(--color-danger)";
  if (tone === "warn") return "var(--color-warn)";
  if (tone === "ok") return "var(--color-accent)";
  return "var(--color-fg-muted)";
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

type RepoMetricsPanelProps = {
  repo: RepoProfile;
  overallVerdictLabel: string;
  verdictLabel: string;
  dimensionsLabel: string;
  freshnessLabel: string;
  adoptionLabel: string;
  reliabilityLabel: string;
  abandonmentLabel: string;
  vitalityLabel: string;
  freshnessHint: string;
  adoptionHint: string;
  reliabilityHint: string;
  abandonmentHint: string;
  vitalityHint: string;
  vitalityCollectiveLabel: string;
  vitalityCadenceLabel: string;
  vitalityCiLabel: string;
  vitalityReleaseLabel: string;
  vitalityNotCapturedLabel: string;
  vitalityNeverReleasedLabel: string;
  ciYesLabel: string;
  ciNoLabel: string;
  starsLabel: string;
  forksLabel: string;
  openIssuesLabel: string;
  subscribersLabel: string;
  lastCommitLabel: string;
  priorsFetchedLabel: string;
  defaultBranchLabel: string;
};

function formatNullableNumber(value: number | null): string {
  return value === null || value === undefined ? "—" : String(value);
}

function formatBoolean(
  value: boolean | null,
  yes: string,
  no: string
): string {
  if (value === null || value === undefined) return "—";
  return value ? yes : no;
}

function VitalityBreakdown({
  inputs,
  notCapturedLabel,
  neverReleasedLabel,
  collectiveLabel,
  cadenceLabel,
  ciLabel,
  releaseLabel,
  ciYesLabel,
  ciNoLabel
}: {
  inputs: VitalityInputs;
  notCapturedLabel: string;
  neverReleasedLabel: string;
  collectiveLabel: string;
  cadenceLabel: string;
  ciLabel: string;
  releaseLabel: string;
  ciYesLabel: string;
  ciNoLabel: string;
}) {
  if (!inputs.structuralSignalsAt) {
    return (
      <p className="text-[0.84rem] text-fg-muted">{notCapturedLabel}</p>
    );
  }

  return (
    <div className="grid gap-2 text-[0.88rem] sm:grid-cols-2">
      <Row
        k={collectiveLabel}
        v={formatNullableNumber(inputs.distinctContributors90d)}
      />
      <Row k={cadenceLabel} v={formatNullableNumber(inputs.commits30d)} />
      <Row
        k={ciLabel}
        v={formatBoolean(inputs.hasCi, ciYesLabel, ciNoLabel)}
      />
      <Row
        k={releaseLabel}
        v={
          inputs.lastReleaseAt
            ? formatRelative(inputs.lastReleaseAt)
            : neverReleasedLabel
        }
      />
    </div>
  );
}

export function RepoMetricsPanel({
  repo,
  overallVerdictLabel,
  verdictLabel,
  dimensionsLabel,
  freshnessLabel,
  adoptionLabel,
  reliabilityLabel,
  abandonmentLabel,
  vitalityLabel,
  freshnessHint,
  adoptionHint,
  reliabilityHint,
  abandonmentHint,
  vitalityHint,
  vitalityCollectiveLabel,
  vitalityCadenceLabel,
  vitalityCiLabel,
  vitalityReleaseLabel,
  vitalityNotCapturedLabel,
  vitalityNeverReleasedLabel,
  ciYesLabel,
  ciNoLabel,
  starsLabel,
  forksLabel,
  openIssuesLabel,
  subscribersLabel,
  lastCommitLabel,
  priorsFetchedLabel,
  defaultBranchLabel
}: RepoMetricsPanelProps) {
  const q = repo.quality;
  const overallTone = scoreTone(q?.overall);

  return (
    <section className="grid gap-8 md:grid-cols-[280px_1fr] md:gap-14">
      <div className="grid gap-4">
        <div className="surface p-5 grid gap-4">
          <span className="kicker">{overallVerdictLabel}</span>
          <p
            className="data-value text-[5.4rem] leading-none tracking-tight"
            style={{ color: scoreColor(overallTone) }}
          >
            {formatScore(q?.overall)}
          </p>
          <div className="flex items-center gap-2">
            <span className="dot" style={{ color: scoreColor(overallTone) }} />
            <span className="text-[0.86rem] font-medium text-fg">{verdictLabel}</span>
          </div>
        </div>

        <div className="surface p-5 grid gap-2 text-[0.88rem]">
          <Row k={starsLabel} v={formatStars(repo.starsCount)} />
          <Row k={forksLabel} v={formatStars(repo.forksCount)} />
          <Row k={openIssuesLabel} v={String(repo.openIssuesCount)} />
          <Row k={subscribersLabel} v={formatStars(repo.subscribersCount)} />
          <Row k={lastCommitLabel} v={formatRelative(repo.lastCommitAt)} />
          <Row k={priorsFetchedLabel} v={formatRelative(repo.priorsFetchedAt)} />
          {repo.defaultBranch ? (
            <Row k={defaultBranchLabel} v={repo.defaultBranch} mono />
          ) : null}
        </div>
      </div>

      <div className="grid gap-6">
        <span className="kicker">{dimensionsLabel}</span>
        <div className="grid gap-x-10 gap-y-6 md:grid-cols-2">
          <ScoreBar
            label={freshnessLabel}
            value={q?.freshness ?? null}
            tone={scoreTone(q?.freshness ?? null)}
            hint={freshnessHint}
          />
          <ScoreBar
            label={adoptionLabel}
            value={q?.adoption ?? null}
            tone={scoreTone(q?.adoption ?? null)}
            hint={adoptionHint}
          />
          <ScoreBar
            label={reliabilityLabel}
            value={q?.reliability ?? null}
            tone={scoreTone(q?.reliability ?? null)}
            hint={reliabilityHint}
          />
          <ScoreBar
            label={abandonmentLabel}
            value={q?.abandonment ?? null}
            tone={abandonmentTone(q?.abandonment ?? null)}
            invert
            hint={abandonmentHint}
          />
        </div>

        <div className="surface p-5 grid gap-4">
          <ScoreBar
            label={vitalityLabel}
            value={q?.vitality ?? null}
            tone={scoreTone(q?.vitality ?? null)}
            hint={vitalityHint}
          />
          <VitalityBreakdown
            inputs={repo.vitalityInputs}
            notCapturedLabel={vitalityNotCapturedLabel}
            neverReleasedLabel={vitalityNeverReleasedLabel}
            collectiveLabel={vitalityCollectiveLabel}
            cadenceLabel={vitalityCadenceLabel}
            ciLabel={vitalityCiLabel}
            releaseLabel={vitalityReleaseLabel}
            ciYesLabel={ciYesLabel}
            ciNoLabel={ciNoLabel}
          />
        </div>
      </div>
    </section>
  );
}
