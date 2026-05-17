export function LandingMiniStat({
  label,
  value,
  tone
}: {
  label: string;
  value: string;
  tone: "ok" | "warn" | "danger" | "neutral";
}) {
  const color =
    tone === "ok"
      ? "var(--color-accent)"
      : tone === "warn"
        ? "var(--color-warn)"
        : tone === "danger"
          ? "var(--color-danger)"
          : "var(--color-fg-muted)";
  return (
    <div className="grid gap-1 border-t border-line pt-2">
      <span className="mono text-[0.68rem] uppercase tracking-[0.14em] text-fg-muted">
        {label}
      </span>
      <span className="data-value text-[0.94rem]" style={{ color }}>
        {value}
      </span>
    </div>
  );
}
