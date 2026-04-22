type Props = {
  scale?: "sm" | "md";
  muted?: boolean;
};

export function Wordmark({ scale = "md", muted = false }: Props) {
  const size =
    scale === "sm"
      ? "text-[1.15rem] leading-none"
      : "text-[1.45rem] leading-none";
  const color = muted ? "text-ink-dim" : "text-ink";
  return (
    <span
      className={`inline-flex items-baseline font-display ${size} ${color}`}
    >
      <span className="font-semibold italic tracking-tight">Use</span>
      <span
        className="font-mono uppercase tracking-[0.04em]"
        style={{ fontSize: "0.72em", marginLeft: "0.12em" }}
      >
        Stakly
      </span>
      <span className="ml-[0.15em] inline-block h-[0.35em] w-[0.35em] translate-y-[-0.2em] rounded-full bg-ember" />
    </span>
  );
}
