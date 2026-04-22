import { formatScore } from "../lib/format";

type Tone = "ok" | "warn" | "danger" | "neutral";

type Props = {
  label: string;
  value: number | null | undefined;
  tone?: Tone;
  invert?: boolean;
  hint?: string;
};

export function ScoreBar({ label, value, tone = "neutral", invert = false, hint }: Props) {
  const displayed = value ?? null;
  const filled =
    displayed === null
      ? 0
      : Math.max(0, Math.min(1, invert ? 1 - displayed : displayed));
  const dataTone = tone === "neutral" ? undefined : tone;

  return (
    <div className="grid gap-1.5">
      <div className="flex items-baseline justify-between gap-2">
        <span className="kicker">{label}</span>
        <span className="data-value text-[0.95rem] text-ink">
          {formatScore(displayed)}
        </span>
      </div>
      <div className="score-track" data-tone={dataTone}>
        <span style={{ transform: `scaleX(${filled})` }} />
      </div>
      {hint ? (
        <p className="text-[0.74rem] leading-snug text-ink-muted">{hint}</p>
      ) : null}
    </div>
  );
}
