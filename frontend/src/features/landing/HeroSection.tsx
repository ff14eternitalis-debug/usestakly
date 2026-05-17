import { Link } from "@tanstack/react-router";

import { buttonClass } from "../../components/Button";
import { useT } from "../../i18n";
import { useAuthStore } from "../../state/auth-store";

import { LandingMetrics } from "./LandingMetrics";
import { LiveRepositoryPanel } from "./LiveRepositoryPanel";

export function HeroSection() {
  const t = useT();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  return (
    <section className="shell relative pt-16 pb-20 md:pt-24 md:pb-28">
      <div className="grid gap-12 md:grid-cols-[1.3fr_1fr] md:items-center md:gap-16">
        <div className="grid gap-8 rise-in">
          <h1 className="display-xl">
            <span className="accent">{t.landing.h1}</span>
          </h1>

          <p className="max-w-[56ch] text-[1.08rem] md:text-[1.15rem] leading-[1.55] text-fg-dim">
            {t.landing.introBefore}
            <strong className="font-semibold text-fg">{t.landing.introStrong}</strong>
            {t.landing.introAfter}
          </p>

          <div className="flex flex-wrap items-center gap-4 pt-2">
            <Link
              to="/discover"
              className={buttonClass(
                "primary",
                "md",
                "px-5 py-3 text-[0.96rem] shadow-[0_0_0_1px_var(--color-line-strong)]"
              )}
            >
              {t.landing.openObservatory}
              <span className="arrow">→</span>
            </Link>
            <Link
              to="/how-to-read"
              className="link-underline text-[0.92rem] text-fg-dim hover:text-accent"
            >
              {t.landing.readGuide}
            </Link>
            {isAuthed ? (
              <Link
                to="/watchlist"
                className="inline-flex items-center gap-1.5 text-[0.92rem] text-fg-dim hover:text-accent transition-colors"
              >
                {t.landing.myWatchlist}
                <span className="arrow">→</span>
              </Link>
            ) : null}
          </div>

          <LandingMetrics />
        </div>

        <LiveRepositoryPanel />
      </div>
    </section>
  );
}
