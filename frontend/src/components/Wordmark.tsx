type Props = {
  scale?: "sm" | "md" | "lg";
  muted?: boolean;
};

export function Wordmark({ scale = "md", muted = false }: Props) {
  const size =
    scale === "sm"
      ? "text-[1rem]"
      : scale === "lg"
        ? "text-[1.6rem]"
        : "text-[1.2rem]";
  const color = muted ? "text-fg-dim" : "text-fg";
  return (
    <span
      className={`inline-flex items-center gap-[0.35em] font-sans font-semibold tracking-tight leading-none ${size} ${color}`}
    >
      <span>UseStakly</span>
      <span className="inline-block h-[0.42em] w-[0.42em] rounded-full bg-accent shadow-[0_0_12px_0_var(--color-accent-glow)]" />
    </span>
  );
}
