import { Link } from "@tanstack/react-router";

import { useT } from "../../i18n";

export function LandingPrinciples() {
  const t = useT();
  return (
    <section className="shell py-14 md:py-20">
      <div className="grid gap-8 border-y border-line py-10 md:grid-cols-[0.8fr_1.2fr] md:gap-14">
        <div className="grid content-start gap-4">
          <span className="kicker">{t.landing.dataEyebrow}</span>
          <h2 className="display-lg max-w-[20ch]">{t.landing.dataH2}</h2>
          <p className="max-w-[52ch] text-[0.96rem] leading-relaxed text-fg-dim">
            {t.landing.dataBody}
          </p>
          <Link
            to="/how-to-read"
            className="inline-flex w-fit items-center gap-1.5 text-[0.92rem] font-medium text-accent hover:gap-2.5 transition-all"
          >
            {t.landing.dataCta}
            <span className="arrow">→</span>
          </Link>
        </div>
        <div className="grid gap-4">
          {t.landing.dataItems.map((item, index) => (
            <div
              key={item.title}
              className="grid gap-3 border-b border-line pb-5 last:border-b-0 last:pb-0 sm:grid-cols-[3rem_1fr]"
            >
              <span className="data-value text-[0.9rem] text-accent">
                {String(index + 1).padStart(2, "0")}
              </span>
              <div className="grid gap-1.5">
                <h3 className="text-[1rem] font-semibold text-fg">
                  {item.title}
                </h3>
                <p className="text-[0.92rem] leading-relaxed text-fg-dim">
                  {item.body}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
