import { formatRelative, formatScore } from "../../../lib/format";
import { dimensionTone } from "../../../lib/dimension-display";
import type { DimensionState } from "../../../lib/types";

type DimensionScoreRowProps = {
  label: string;
  dimension?: DimensionState;
  invert?: boolean;
  compact?: boolean;
  displayStateLabels: Record<string, string>;
};

export function DimensionScoreRow({
  label,
  dimension,
  invert = false,
  compact = false,
  displayStateLabels
}: DimensionScoreRowProps) {
  const tone = dimensionTone(dimension, invert);
  const displayed = dimension?.value ?? null;
  const filled =
    displayed === null
      ? 0
      : Math.max(0, Math.min(1, invert ? 1 - displayed : displayed));
  const stateLabel = dimension
    ? displayStateLabels[dimension.displayState] ?? dimension.displayState
    : "—";

  return (
    <div className="grid gap-2">
      <div className="flex items-baseline justify-between gap-2">
        <span className="text-[0.82rem] font-medium text-fg-dim">{label}</span>
        <span className="data-value text-[0.9rem] text-fg">
          {formatScore(displayed)}
        </span>
      </div>
      <div className="flex items-baseline justify-between gap-2">
        <span
          className={`text-fg-muted ${compact ? "text-[0.72rem]" : "text-[0.76rem]"}`}
        >
          {stateLabel}
        </span>
        {dimension?.asOf ? (
          <span className="text-[0.72rem] text-fg-muted">
            {formatRelative(dimension.asOf)}
          </span>
        ) : null}
      </div>
      <div className="score-track" data-tone={tone}>
        <span style={{ transform: `scaleX(${filled})` }} />
      </div>
      {!compact && dimension?.summary ? (
        <p className="text-[0.74rem] leading-snug text-fg-muted">{dimension.summary}</p>
      ) : null}
    </div>
  );
}
