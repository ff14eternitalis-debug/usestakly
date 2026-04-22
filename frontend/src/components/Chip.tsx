import type { ReactNode } from "react";

type Tone = "neutral" | "danger" | "warn" | "ok" | "info" | "accent";

type Props = {
  tone?: Tone;
  children: ReactNode;
  mono?: boolean;
  className?: string;
};

const tones: Record<Tone, string> = {
  neutral: "border-line text-fg-dim bg-surface/60",
  danger:
    "border-[color:var(--color-danger)]/30 text-[color:var(--color-danger)] bg-[color:var(--color-danger)]/5",
  warn: "border-[color:var(--color-warn)]/30 text-[color:var(--color-warn)] bg-[color:var(--color-warn)]/5",
  ok: "border-[color:var(--color-ok)]/30 text-[color:var(--color-ok)] bg-[color:var(--color-ok)]/5",
  info: "border-[color:var(--color-info)]/30 text-[color:var(--color-info)] bg-[color:var(--color-info)]/5",
  accent:
    "border-accent/40 text-accent bg-[color:var(--color-accent-glow)]"
};

export function Chip({
  tone = "neutral",
  children,
  mono = false,
  className = ""
}: Props) {
  return (
    <span
      className={`inline-flex items-center gap-1.5 border px-2 py-[3px] text-[0.7rem] uppercase tracking-[0.12em] rounded-[4px] ${tones[tone]} ${mono ? "font-mono" : "font-sans font-medium"} ${className}`}
    >
      {children}
    </span>
  );
}
