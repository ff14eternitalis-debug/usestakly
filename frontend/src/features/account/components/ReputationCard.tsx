import type { AccountSummary } from "../../../lib/types";

type ReputationCardProps = {
  summary: AccountSummary | undefined;
  quotaTitle: string;
  quotaBody: string;
  reputationLabel: string;
  tierLabel: string;
  usageSignalsLabel: string;
  successRatioLabel: string;
  buildReliabilityLabel: string;
  regretRatioLabel: string;
  passiveSignalsLabel: string;
  eligibilityLabel: string;
  eligibleLabel: string;
  notEligibleLabel: string;
};

export function ReputationCard({
  summary,
  quotaTitle,
  quotaBody,
  reputationLabel,
  tierLabel,
  usageSignalsLabel,
  successRatioLabel,
  buildReliabilityLabel,
  regretRatioLabel,
  passiveSignalsLabel,
  eligibilityLabel,
  eligibleLabel,
  notEligibleLabel
}: ReputationCardProps) {
  return (
    <div className="surface grid gap-2 p-5">
      <span className="kicker">{quotaTitle}</span>
      <p className="text-[0.94rem] leading-relaxed text-fg-dim">{quotaBody}</p>
      {summary ? (
        <div className="grid gap-1 border-t border-line pt-3 text-[0.88rem] text-fg-dim">
          <span>
            {reputationLabel}: <span className="data-value text-fg">{summary.reputation.score.toFixed(2)}</span>
          </span>
          <span>
            {tierLabel}: <span className="text-fg">{summary.reputation.tier}</span>
          </span>
          <span>
            {passiveSignalsLabel}: {summary.reputation.passiveSignalCount}
          </span>
          <span>
            {usageSignalsLabel}: {summary.reputation.usageSignalCount}
          </span>
          <span>
            {successRatioLabel}: {(summary.reputation.successfulOutcomeRatio * 100).toFixed(0)}%
          </span>
          <span>
            {buildReliabilityLabel}: {(summary.reputation.buildReliabilityRatio * 100).toFixed(0)}%
          </span>
          <span>
            {regretRatioLabel}: {(summary.reputation.regretRatio * 100).toFixed(0)}%
          </span>
          <span>
            {eligibilityLabel}:{" "}
            {summary.reputation.activeSignalEligible ? eligibleLabel : notEligibleLabel}
          </span>
        </div>
      ) : null}
    </div>
  );
}
