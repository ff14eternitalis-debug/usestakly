export function LandingKpi({ k, label }: { k: string; label: string }) {
  return (
    <div className="grid gap-1.5">
      <p className="data-value text-[2rem] leading-none tracking-tight text-fg">
        {k}
      </p>
      <p className="text-[0.78rem] text-fg-muted">{label}</p>
    </div>
  );
}
