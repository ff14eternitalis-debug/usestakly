import type { ReactNode } from "react";

type Tone = "neutral" | "danger" | "warn" | "ok" | "info";

type Props = {
  tone?: Tone;
  children: ReactNode;
  mono?: boolean;
  className?: string;
};

const tones: Record<Tone, string> = {
  neutral: "border-line text-ink-dim",
  danger: "border-ember/50 text-ember",
  warn: "border-ochre/60 text-ochre",
  ok: "border-moss/50 text-moss",
  info: "border-prussian/40 text-prussian"
};

export function Chip({
  tone = "neutral",
  children,
  mono = false,
  className = ""
}: Props) {
  return (
    <span
      className={`inline-flex items-center gap-1.5 border px-2 py-[3px] text-[0.72rem] uppercase tracking-[0.14em] ${tones[tone]} ${mono ? "font-mono" : "font-sans"} ${className}`}
      style={{ borderRadius: 2 }}
    >
      {children}
    </span>
  );
}
