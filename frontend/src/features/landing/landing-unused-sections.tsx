/**
 * Preserved from pre-refactor landing; not mounted on `/` (same as before split).
 */
import { Link } from "@tanstack/react-router";

import { useT } from "../../i18n";

function LandingPillarCard({
  index,
  pillarLabel,
  title,
  body,
  cta,
  artifact
}: {
  index: string;
  pillarLabel: string;
  title: string;
  body: string;
  cta: { to: "/discover" | "/watchlist"; label: string };
  artifact: { label: string; value: string };
}) {
  return (
    <article className="surface p-6 md:p-8 grid gap-4 hover:border-line-strong transition-colors">
      <div className="flex items-center justify-between">
        <span className="data-value text-accent text-[0.86rem] tracking-wider">
          {index}
        </span>
        <span className="kicker">{pillarLabel}</span>
      </div>
      <h3 className="display-md">{title}</h3>
      <p className="text-[0.98rem] leading-[1.65] text-fg-dim">{body}</p>
      <div className="flex items-baseline gap-3 border-t border-line pt-4 mt-2">
        <span className="kicker">{artifact.label}</span>
        <code className="mono text-[0.82rem] text-fg-dim">
          {artifact.value}
        </code>
      </div>
      <Link
        to={cta.to}
        className="inline-flex items-center gap-1.5 mt-2 text-accent text-[0.92rem] font-medium hover:gap-2.5 transition-all"
      >
        {cta.label}
        <span className="arrow">→</span>
      </Link>
    </article>
  );
}

function LandingFormulaLine({
  dim,
  weight,
  expr,
  highlight = false
}: {
  dim: string;
  weight: string;
  expr: string;
  highlight?: boolean;
}) {
  return (
    <div
      className={`grid grid-cols-[110px_72px_1fr] items-baseline gap-3 border-b border-line py-2 ${
        highlight ? "text-accent" : ""
      }`}
    >
      <span className={highlight ? "text-accent" : "text-fg"}>{dim}</span>
      <span className="text-fg-muted">w={weight}</span>
      <span className={highlight ? "text-accent" : "text-fg-dim"}>{expr}</span>
    </div>
  );
}

export function LandingPillarsSection() {
  const t = useT();
  return (
    <section className="shell py-16 md:py-24">
      <header className="grid gap-4 mb-12 md:mb-16">
        <span className="kicker">{t.landing.pillarsEyebrow}</span>
        <h2 className="display-lg max-w-[24ch]">{t.landing.pillarsH2}</h2>
      </header>

      <div className="grid gap-6 md:grid-cols-2 md:gap-8">
        <LandingPillarCard
          index="01"
          pillarLabel={t.landing.pillar}
          title={t.landing.pillar1Title}
          body={t.landing.pillar1Body}
          cta={{ to: "/discover", label: t.landing.pillar1Cta }}
          artifact={{
            label: t.landing.pillar1Artifact,
            value: "auto / strict / explore"
          }}
        />
        <LandingPillarCard
          index="02"
          pillarLabel={t.landing.pillar}
          title={t.landing.pillar2Title}
          body={t.landing.pillar2Body}
          cta={{ to: "/watchlist", label: t.landing.pillar2Cta }}
          artifact={{
            label: t.landing.pillar2Artifact,
            value: "score_drop · abandonment_up · flag_severe"
          }}
        />
      </div>
    </section>
  );
}

export function LandingFormulaSection() {
  const t = useT();
  return (
    <section className="shell py-16 md:py-24">
      <div className="surface relative overflow-hidden">
        <div
          className="absolute -top-20 -right-20 h-[280px] w-[280px] rounded-full opacity-30"
          style={{
            background:
              "radial-gradient(closest-side, var(--color-accent-glow), transparent)"
          }}
          aria-hidden
        />
        <div className="relative grid gap-10 p-8 md:grid-cols-[1fr_1.1fr] md:gap-16 md:p-14">
          <div className="grid gap-5 content-start">
            <span className="kicker accent">{t.landing.formulaEyebrow}</span>
            <h2 className="display-lg">{t.landing.formulaH2}</h2>
            <p className="max-w-[48ch] text-[0.98rem] leading-[1.7] text-fg-dim">
              {t.landing.formulaBody}
            </p>
          </div>

          <div className="grid gap-2 mono text-[0.84rem] leading-relaxed">
            <LandingFormulaLine
              dim={t.footer.freshness.toLowerCase()}
              weight="0.20"
              expr="0.5 ^ (age_days / 180)"
            />
            <LandingFormulaLine
              dim={t.footer.adoption.toLowerCase()}
              weight="0.15"
              expr="ln(r + 1) / ln(1001)"
            />
            <LandingFormulaLine
              dim={t.footer.reliability.toLowerCase()}
              weight="0.40"
              expr="success / (success + failure)"
            />
            <LandingFormulaLine
              dim={t.footer.abandonment.toLowerCase()}
              weight="0.25"
              expr="1 − freshness + regret_bump"
              highlight
            />
          </div>
        </div>
      </div>
    </section>
  );
}
