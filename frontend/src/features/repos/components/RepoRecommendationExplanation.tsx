type RepoRecommendationExplanationProps = {
  title: string;
  includedLabel: string;
  caveatsLabel: string;
  includedBecause: string[];
  caveats: string[];
};

export function RepoRecommendationExplanation({
  title,
  includedLabel,
  caveatsLabel,
  includedBecause,
  caveats
}: RepoRecommendationExplanationProps) {
  if (includedBecause.length === 0 && caveats.length === 0) {
    return null;
  }

  return (
    <section className="grid gap-3 border-t border-line pt-4">
      <p className="kicker">{title}</p>
      {includedBecause.length > 0 ? (
        <ReasonList label={includedLabel} items={includedBecause} muted={false} />
      ) : null}
      {caveats.length > 0 ? (
        <ReasonList label={caveatsLabel} items={caveats} muted />
      ) : null}
    </section>
  );
}

function ReasonList({
  label,
  items,
  muted
}: {
  label: string;
  items: string[];
  muted: boolean;
}) {
  const textClass = muted ? "text-fg-muted" : "text-fg-dim";
  return (
    <div className="grid gap-1.5">
      <p className="text-[0.78rem] font-medium uppercase tracking-wide text-fg-muted">
        {label}
      </p>
      <ul className="grid gap-1">
        {items.map((item) => (
          <li key={item} className={`text-[0.86rem] leading-relaxed ${textClass}`}>
            {item}
          </li>
        ))}
      </ul>
    </div>
  );
}

