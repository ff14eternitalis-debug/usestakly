import { Link } from "@tanstack/react-router";

import { Wordmark } from "../../components/Wordmark";
import { useT } from "../../i18n";

export function SiteFooter() {
  const t = useT();
  const year = new Date().getFullYear();
  return (
    <footer className="mt-24 border-t border-line">
      <div className="shell grid gap-10 py-12 md:grid-cols-[1.4fr_1fr_1fr_1fr]">
        <div className="grid gap-4">
          <Wordmark scale="md" />
          <p className="max-w-sm text-[0.9rem] leading-relaxed text-fg-dim">
            {t.footer.tagline}
          </p>
        </div>

        <FooterCol title={t.footer.product}>
          <FooterLink to="/discover">{t.nav.discover}</FooterLink>
          <FooterLink to="/watchlist">{t.nav.watchlist}</FooterLink>
          <FooterLink to="/notifications">{t.nav.notifications}</FooterLink>
          <FooterLink to="/mcp-guide">{t.footer.mcp}</FooterLink>
        </FooterCol>

        <FooterCol title={t.footer.signals}>
          <FooterMuted>{t.footer.freshness}</FooterMuted>
          <FooterMuted>{t.footer.adoption}</FooterMuted>
          <FooterMuted>{t.footer.reliability}</FooterMuted>
          <FooterMuted>{t.footer.abandonment}</FooterMuted>
        </FooterCol>

        <FooterCol title={t.footer.about}>
          <FooterMuted>{t.footer.selfHosted}</FooterMuted>
          <FooterMuted>{t.footer.publicFormula}</FooterMuted>
          <FooterMuted>{t.footer.localEmbeddings}</FooterMuted>
        </FooterCol>
      </div>
      <div className="shell flex items-center justify-between border-t border-line py-5 text-[0.78rem]">
        <span className="mono text-fg-muted">
          {t.footer.copyright.replace("{year}", String(year))}
        </span>
        <span className="mono text-fg-muted">{t.footer.tagFormula}</span>
      </div>
    </footer>
  );
}

function FooterCol({
  title,
  children
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="grid gap-3">
      <p className="kicker">{title}</p>
      <ul className="grid gap-2 text-[0.9rem]">{children}</ul>
    </div>
  );
}

function FooterLink({
  to,
  children
}: {
  to: "/discover" | "/watchlist" | "/notifications" | "/mcp-guide";
  children: React.ReactNode;
}) {
  return (
    <li>
      <Link
        to={to}
        className="text-fg-dim hover:text-accent transition-colors"
      >
        {children}
      </Link>
    </li>
  );
}

function FooterMuted({ children }: { children: React.ReactNode }) {
  return <li className="text-fg-dim">{children}</li>;
}
