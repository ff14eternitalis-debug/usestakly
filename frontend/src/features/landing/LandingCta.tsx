import { Link } from "@tanstack/react-router";

import { buttonClass } from "../../components/Button";
import { useT } from "../../i18n";
import { useAuthStore } from "../../state/auth-store";

export function LandingCta() {
  const t = useT();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  return (
    <section className="shell py-20 md:py-28">
      <div className="surface relative overflow-hidden p-8 md:p-14">
        <div
          className="absolute inset-0 opacity-40"
          style={{
            background:
              "radial-gradient(800px 240px at 50% 100%, var(--color-accent-glow), transparent 70%)"
          }}
          aria-hidden
        />
        <div className="relative grid gap-8 md:grid-cols-[1.5fr_1fr] md:items-end">
          <div className="grid gap-4">
            <span className="kicker">{t.landing.closingEyebrow}</span>
            <h2 className="display-lg max-w-[20ch]">
              {t.landing.closingH2Part1}
              {" "}
              <br />
              <span className="accent">{t.landing.closingH2Part2}</span>
            </h2>
          </div>
          <div className="flex flex-wrap gap-3 md:justify-end">
            <Link to="/discover" className={buttonClass("outline")}>
              {t.landing.closingBrowse}
            </Link>
            {isAuthed ? (
              <Link to="/watchlist" className={buttonClass("primary")}>
                {t.landing.closingWatchlist}
                <span className="arrow">→</span>
              </Link>
            ) : (
              <Link to="/login" className={buttonClass("primary")}>
                {t.landing.closingStart}
                <span className="arrow">→</span>
              </Link>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}
