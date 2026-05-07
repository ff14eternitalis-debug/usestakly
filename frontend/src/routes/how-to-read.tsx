import { Link } from "@tanstack/react-router";

import { useT } from "../i18n";

export function HowToReadPage() {
  const t = useT();

  return (
    <article className="shell grid gap-12 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.howToRead.eyebrow}</span>
        <h1 className="display-lg max-w-[18ch]">{t.howToRead.h1}</h1>
        <p className="max-w-[64ch] text-[0.98rem] leading-relaxed text-fg-dim">
          {t.howToRead.intro}
        </p>
        <div className="flex flex-wrap gap-3 pt-2">
          <Link
            to="/discover"
            className="group inline-flex items-center gap-2 rounded-[6px] border border-line-strong bg-surface px-5 py-3 text-[0.96rem] font-medium tracking-tight text-fg shadow-[0_0_0_1px_var(--color-line-strong)] transition-all duration-150 hover:border-accent hover:bg-surface hover:text-accent"
          >
            {t.howToRead.ctaDiscover}
            <span className="arrow transition-transform duration-150 group-hover:translate-x-0.5">
              →
            </span>
          </Link>
          <Link
            to="/mcp-guide"
            className="inline-flex items-center rounded-[6px] border border-line px-4 py-2 text-[0.88rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent"
          >
            {t.howToRead.ctaMcp}
          </Link>
        </div>
      </header>

      <section className="grid gap-6 border-t border-line pt-8 md:grid-cols-[0.72fr_1.28fr]">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.howToRead.scoreLabel}</span>
          <h2 className="display-md !text-[1.45rem]">
            {t.howToRead.scoreTitle}
          </h2>
        </div>
        <p className="max-w-[68ch] text-[0.96rem] leading-relaxed text-fg-dim">
          {t.howToRead.scoreBody}
        </p>
      </section>

      <section className="grid gap-6 border-t border-line pt-8 md:grid-cols-[0.72fr_1.28fr]">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.howToRead.dimensionsLabel}</span>
          <h2 className="display-md !text-[1.45rem]">
            {t.repoDetail.dimensions}
          </h2>
        </div>
        <div className="grid gap-3 sm:grid-cols-2">
          {t.howToRead.dimensions.map((dimension) => (
            <div
              key={dimension.name}
              className="rounded-[8px] border border-line bg-surface/45 p-4"
            >
              <p className="mono text-[0.78rem] uppercase text-accent">
                {dimension.name}
              </p>
              <p className="mt-2 text-[0.9rem] leading-relaxed text-fg-dim">
                {dimension.body}
              </p>
            </div>
          ))}
        </div>
      </section>

      <GuideDetails
        label={t.howToRead.formulaVersionLabel}
        title={t.howToRead.formulaVersionTitle}
      >
        <p className="max-w-[68ch] text-[0.96rem] leading-relaxed text-fg-dim">
          {t.howToRead.formulaVersionBody}
        </p>
      </GuideDetails>

      <GuideDetails
        label={t.howToRead.vitalityLimitsLabel}
        title={t.howToRead.vitalityLimitsTitle}
      >
        <p className="max-w-[68ch] text-[0.96rem] leading-relaxed text-fg-dim">
          {t.howToRead.vitalityLimitsBody}
        </p>
      </GuideDetails>

      <GuideDetails label={t.howToRead.radarLabel} title={t.howToRead.radarTitle}>
        <div className="grid gap-4">
          <p className="max-w-[68ch] text-[0.96rem] leading-relaxed text-fg-dim">
            {t.howToRead.radarBody}
          </p>
          <div className="grid gap-3">
            {t.howToRead.radarBands.map((band) => (
              <GuideRow key={band.name} name={band.name} body={band.body} />
            ))}
          </div>
        </div>
      </GuideDetails>

      <section className="grid gap-6 border-t border-line pt-8 md:grid-cols-[0.72fr_1.28fr]">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.howToRead.modesLabel}</span>
          <h2 className="display-md !text-[1.45rem]">
            {t.howToRead.modesTitle}
          </h2>
        </div>
        <div className="grid gap-3">
          {t.howToRead.modes.map((mode) => (
            <GuideRow key={mode.name} name={mode.name} body={mode.body} />
          ))}
        </div>
      </section>

      <GuideDetails label={t.howToRead.corpusLabel} title={t.howToRead.corpusTitle}>
        <div className="grid gap-4">
          <p className="text-[0.96rem] leading-relaxed text-fg-dim">
            {t.howToRead.corpusBody}
          </p>
          <ul className="grid gap-2">
            {t.howToRead.corpusItems.map((item) => (
              <li
                key={item}
                className="flex gap-3 text-[0.92rem] leading-relaxed text-fg-dim"
              >
                <span className="mt-2 size-1.5 shrink-0 rounded-full bg-accent" />
                <span>{item}</span>
              </li>
            ))}
          </ul>
        </div>
      </GuideDetails>

      <section className="grid gap-6 border-t border-line pt-8 md:grid-cols-[0.72fr_1.28fr]">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.howToRead.workflowLabel}</span>
          <h2 className="display-md !text-[1.45rem]">
            {t.howToRead.workflowTitle}
          </h2>
        </div>
        <ol className="grid gap-3">
          {t.howToRead.workflowItems.map((item, index) => (
            <li
              key={item}
              className="grid grid-cols-[2rem_1fr] gap-3 rounded-[8px] border border-line bg-surface/45 p-4"
            >
              <span className="mono text-[0.78rem] text-accent">
                {String(index + 1).padStart(2, "0")}
              </span>
              <span className="text-[0.92rem] leading-relaxed text-fg-dim">
                {item}
              </span>
            </li>
          ))}
        </ol>
      </section>
    </article>
  );
}

function GuideDetails({
  label,
  title,
  children
}: {
  label: string;
  title: string;
  children: React.ReactNode;
}) {
  return (
    <details className="group border-t border-line pt-6">
      <summary className="grid cursor-pointer list-none gap-3 md:grid-cols-[0.72fr_1.28fr]">
        <span className="kicker">{label}</span>
        <span className="flex items-center justify-between gap-4">
          <span className="display-md !text-[1.28rem]">{title}</span>
          <span className="mono text-accent transition-transform group-open:rotate-45">
            +
          </span>
        </span>
      </summary>
      <div className="grid gap-6 pt-5 md:grid-cols-[0.72fr_1.28fr]">
        <span aria-hidden />
        <div>{children}</div>
      </div>
    </details>
  );
}

function GuideRow({ name, body }: { name: string; body: string }) {
  return (
    <div className="grid gap-2 border-b border-line pb-4 last:border-b-0 last:pb-0 sm:grid-cols-[10rem_1fr]">
      <p className="mono text-[0.78rem] uppercase text-accent">{name}</p>
      <p className="text-[0.92rem] leading-relaxed text-fg-dim">{body}</p>
    </div>
  );
}
