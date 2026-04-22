import { Wordmark } from "../../components/Wordmark";

export function SiteFooter() {
  const year = new Date().getFullYear();
  return (
    <footer className="mt-24 border-t border-line bg-paper-soft/60">
      <div className="shell grid gap-10 py-12 md:grid-cols-[1.2fr_1fr_1fr]">
        <div className="grid gap-4">
          <Wordmark scale="md" />
          <p className="max-w-sm text-[0.95rem] leading-relaxed text-ink-dim">
            A quality-scored observatory of public open-source repositories.
            Scoring formula is public, versioned, and never proprietary.
          </p>
          <p className="kicker">
            &copy; {year} UseStakly · self-hosted · transparent by design
          </p>
        </div>

        <div className="grid gap-3">
          <p className="kicker">Product</p>
          <ul className="grid gap-2 text-[0.95rem] text-ink-dim">
            <li>Discover</li>
            <li>Watchlist</li>
            <li>Notifications</li>
            <li>
              <span className="inline-block h-[0.4em] w-[0.4em] translate-y-[-1px] rounded-full bg-ochre" />{" "}
              MCP (coming)
            </li>
          </ul>
        </div>

        <div className="grid gap-3">
          <p className="kicker">Signals</p>
          <ul className="grid gap-2 text-[0.95rem] text-ink-dim">
            <li>Freshness · commit cadence</li>
            <li>Adoption · resolve count</li>
            <li>Reliability · build outcomes</li>
            <li>Abandonment · decay + regret</li>
          </ul>
        </div>
      </div>
    </footer>
  );
}
