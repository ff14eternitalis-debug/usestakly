import { Link } from "@tanstack/react-router";

import { Wordmark } from "../../components/Wordmark";
import { useT } from "../../i18n";

const footerLinkClass =
  "link-underline text-[0.92rem] text-fg-dim hover:text-accent";

export function SiteFooter() {
  const t = useT();
  const year = new Date().getFullYear();
  return (
    <footer className="mt-24 border-t border-line">
      <div className="shell grid gap-12 py-10 md:grid-cols-[1.5fr_1fr_1fr_1fr] md:gap-14 md:py-16">
        <div className="grid max-w-md gap-4">
          <Wordmark scale="md" />
          <p className="text-[0.9rem] leading-relaxed text-fg-dim">
            {t.footer.tagline}
          </p>
        </div>

        <FooterCol title={t.footer.product}>
          <FooterLink to="/discover">{t.nav.discover}</FooterLink>
          <FooterLink to="/how-to-read">{t.nav.howToRead}</FooterLink>
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
          <FooterLink to="/privacy">{t.footer.privacy}</FooterLink>
          <FooterLink to="/legal">{t.footer.legal}</FooterLink>
          <FooterLink to="/status">{t.footer.status}</FooterLink>
          <FooterExternal href={`mailto:${t.footer.contactEmail}`}>
            {t.footer.contact}
          </FooterExternal>
        </FooterCol>
      </div>

      <div className="shell border-t border-line py-8 md:py-10">
        <div className="grid gap-4 text-[0.78rem] md:grid-cols-[1fr_auto_1fr] md:items-center md:gap-8">
          <span className="mono text-fg-muted">
            {t.footer.copyright.replace("{year}", String(year))}
          </span>
          <FooterStatusBadge />
          <span className="mono text-fg-muted md:text-right">
            {t.footer.transparentByDesign}
          </span>
        </div>
      </div>
    </footer>
  );
}

function FooterStatusBadge() {
  const t = useT();
  return (
    <span className="inline-flex items-center gap-2 mono text-fg-dim">
      <span
        className="dot dot-pulse h-2 w-2 shrink-0 text-accent shadow-[0_0_10px_color-mix(in_srgb,var(--color-accent)_55%,transparent)]"
        aria-hidden
      />
      <span>
        {t.footer.publicBeta}
        <span className="text-fg-muted">{" · "}</span>
        {t.footer.formulaVersionLabel}
      </span>
    </span>
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
      <ul className="grid gap-2.5 text-[0.9rem]">{children}</ul>
    </div>
  );
}

function FooterLink({
  to,
  children
}: {
  to:
    | "/discover"
    | "/how-to-read"
    | "/privacy"
    | "/legal"
    | "/status"
    | "/watchlist"
    | "/notifications"
    | "/mcp-guide";
  children: React.ReactNode;
}) {
  return (
    <li>
      <Link to={to} className={footerLinkClass}>
        {children}
      </Link>
    </li>
  );
}

function FooterExternal({
  href,
  children
}: {
  href: string;
  children: React.ReactNode;
}) {
  return (
    <li>
      <a href={href} className={footerLinkClass}>
        {children}
      </a>
    </li>
  );
}

function FooterMuted({ children }: { children: React.ReactNode }) {
  return <li className="text-fg-dim">{children}</li>;
}
