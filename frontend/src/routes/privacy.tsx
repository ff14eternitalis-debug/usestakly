import { useT } from "../i18n";

export function PrivacyPage() {
  const t = useT();

  return (
    <article className="shell-narrow grid gap-12 py-12 md:py-16">
      <header className="grid gap-4">
        <span className="kicker">{t.privacy.eyebrow}</span>
        <h1 className="display-lg max-w-[20ch]">{t.privacy.h1}</h1>
        <p className="max-w-[64ch] text-[0.98rem] leading-relaxed text-fg-dim">
          {t.privacy.intro}
        </p>
      </header>

      <section className="grid gap-5 border-t border-line pt-8">
        {t.privacy.sections.map((section) => (
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

      <p className="rounded-[8px] border border-line bg-surface/45 p-5 text-[0.94rem] leading-relaxed text-fg-dim">
        {t.privacy.closing}
      </p>
    </article>
  );
}
