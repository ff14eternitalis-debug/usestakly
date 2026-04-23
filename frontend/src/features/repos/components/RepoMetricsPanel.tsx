import { ScoreBar } from "../../../components/ScoreBar";
import {
  abandonmentTone,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../../../lib/format";
import type { RepoProfile } from "../../../lib/types";

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
  freshnessHint: string;
  adoptionHint: string;
  reliabilityHint: string;
  abandonmentHint: string;
  starsLabel: string;
  forksLabel: string;
  openIssuesLabel: string;
  subscribersLabel: string;
  lastCommitLabel: string;
  priorsFetchedLabel: string;
  defaultBranchLabel: string;
};

export function RepoMetricsPanel({
  repo,
  overallVerdictLabel,
  verdictLabel,
  dimensionsLabel,
  freshnessLabel,
  adoptionLabel,
  reliabilityLabel,
  abandonmentLabel,
  freshnessHint,
  adoptionHint,
  reliabilityHint,
  abandonmentHint,
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
      </div>
    </section>
  );
}
