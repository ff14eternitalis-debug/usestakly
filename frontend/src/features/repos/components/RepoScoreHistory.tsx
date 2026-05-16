import { formatRelative, formatScore } from "../../../lib/format";
import type { ScoreSnapshot } from "../../../lib/types";

type RepoScoreHistoryProps = {
  title: string;
  currentLabel: string;
  previousLabel: string;
  computedLabel: string;
  snapshot: ScoreSnapshot | null | undefined;
};

export function RepoScoreHistory({
  title,
  currentLabel,
  previousLabel,
  computedLabel,
  snapshot
}: RepoScoreHistoryProps) {
  if (!snapshot) {
    return null;
  }

  const currentOverall = snapshot.overall;
  const previousOverall = snapshot.previousOverall;
  const delta =
    currentOverall != null && previousOverall != null
      ? currentOverall - previousOverall
      : null;

  return (
    <section className="grid gap-4 border-t border-line pt-6">
      <div className="flex items-baseline justify-between gap-4">
        <h2 className="display-md">{title}</h2>
        <p className="kicker">
          {snapshot.formulaVersion}
          {snapshot.computedAt
            ? ` · ${computedLabel} ${formatRelative(snapshot.computedAt)}`
            : null}
        </p>
      </div>
      <div className="grid gap-4 sm:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)]">
        <ScoreBarRow label={currentLabel} value={currentOverall} />
        {previousOverall != null ? (
          <ScoreBarRow label={previousLabel} value={previousOverall} muted />
        ) : null}
      </div>
      {delta != null ? (
        <p className="text-[0.88rem] text-fg-dim">
          Δ vs {snapshot.previousFormulaVersion ?? "previous"}:{" "}
          <span className="mono font-medium text-fg">
            {delta >= 0 ? "+" : ""}
            {formatScore(delta)}
          </span>
        </p>
      ) : null}
    </section>
  );
}

function ScoreBarRow({
  label,
  value,
  muted = false
}: {
  label: string;
  value: number | null | undefined;
  muted?: boolean;
}) {
  const pct = value != null ? Math.round(value * 100) : 0;
  return (
    <div className="grid gap-2">
      <div className="flex items-baseline justify-between gap-2">
        <span className={`text-[0.82rem] ${muted ? "text-fg-muted" : "text-fg-dim"}`}>
          {label}
        </span>
        <span className="mono text-[0.92rem] text-fg">{formatScore(value)}</span>
      </div>
      <div className="h-2 overflow-hidden rounded-full bg-surface">
        <div
          className={`h-full rounded-full ${muted ? "bg-fg-muted/40" : "bg-accent"}`}
          style={{ width: `${Math.min(100, Math.max(0, pct))}%` }}
        />
      </div>
    </div>
  );
}
