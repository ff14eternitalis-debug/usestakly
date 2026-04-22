import { formatScore } from "../lib/format";

type Tone = "ok" | "warn" | "danger" | "neutral";

type Props = {
  label: string;
  value: number | null | undefined;
  tone?: Tone;
  invert?: boolean;
  hint?: string;
};

export function ScoreBar({
  label,
  value,
  tone = "neutral",
  invert = false,
  hint
}: Props) {
  const displayed = value ?? null;
  const filled =
    displayed === null
      ? 0
      : Math.max(0, Math.min(1, invert ? 1 - displayed : displayed));

  return (
    <div className="grid gap-2">
      <div className="flex items-baseline justify-between gap-2">
        <span className="text-[0.82rem] font-medium text-fg-dim">
          {label}
        </span>
        <span className="data-value text-[0.9rem] text-fg">
          {formatScore(displayed)}
        </span>
      </div>
      <div className="score-track" data-tone={tone}>
        <span style={{ transform: `scaleX(${filled})` }} />
      </div>
      {hint ? (
        <p className="text-[0.74rem] leading-snug text-fg-muted">{hint}</p>
      ) : null}
    </div>
  );
}
