import { useT } from "../i18n";

export function LegalPage() {
  const t = useT();

  return (
    <article className="shell-narrow grid gap-10 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.legal.eyebrow}</span>
        <h1 className="display-lg max-w-[18ch]">{t.legal.h1}</h1>
        <p className="max-w-[64ch] text-[0.98rem] leading-relaxed text-fg-dim">
          {t.legal.intro}
        </p>
      </header>

      <section className="grid gap-5 border-t border-line pt-8">
        {t.legal.sections.map((section) => (
          <div
            key={section.title}
            className="grid gap-3 border-b border-line pb-5 last:border-b-0 last:pb-0 md:grid-cols-[13rem_1fr]"
          >
            <h2 className="text-[1rem] font-semibold text-fg">
              {section.title}
            </h2>
            <p className="text-[0.94rem] leading-relaxed text-fg-dim">
              {section.body}
            </p>
          </div>
        ))}
      </section>

      <section className="rounded-[8px] border border-line bg-surface/45 p-5">
        <p className="kicker mb-3">{t.legal.contactTitle}</p>
        <p className="text-[0.94rem] leading-relaxed text-fg-dim">
          {t.legal.contactBody}{" "}
          <a
            className="text-fg underline decoration-line underline-offset-4 transition-colors hover:text-accent"
            href={`mailto:${t.legal.contactEmail}`}
          >
            {t.legal.contactEmail}
          </a>
        </p>
      </section>
    </article>
  );
}
