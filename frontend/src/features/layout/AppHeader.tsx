import { Link } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { LocaleSwitch } from "../../components/LocaleSwitch";
import { Wordmark } from "../../components/Wordmark";
import { useT } from "../../i18n";
import { apiGet } from "../../lib/api-client";
import { loginSearch } from "../../lib/return-to";
import type { UnreadCount } from "../../lib/types";
import { useAuthStore } from "../../state/auth-store";
import { logout } from "../auth/hooks";

export function AppHeader() {
  const t = useT();
  const { status, user } = useAuthStore();
  const isAuthed = status === "authenticated";

  const unreadQuery = useQuery({
    queryKey: ["notifications", "unread"],
    queryFn: ({ signal }) =>
      apiGet<UnreadCount>("/api/notifications/unread-count", signal),
    enabled: isAuthed,
    refetchInterval: 60_000,
    staleTime: 30_000
  });
  const unread = unreadQuery.data?.unread ?? 0;

  return (
    <header className="sticky top-0 z-30 border-b border-line bg-[color:var(--color-bg)]/80 backdrop-blur-xl backdrop-saturate-150">
      <div className="shell flex h-[62px] items-center justify-between gap-6">
        <Link to="/" className="group inline-flex items-center gap-2">
          <Wordmark scale="md" />
        </Link>

        <nav className="hidden md:flex items-center gap-7">
          <Link to="/discover" className="nav-link">
            {t.nav.discover}
          </Link>
          <Link to="/how-to-read" className="nav-link">
            {t.nav.howToRead}
          </Link>
          <Link to="/mcp-guide" className="nav-link">
            {t.nav.mcpGuide}
          </Link>
          {isAuthed ? (
            <>
              <Link to="/watchlist" className="nav-link">
                {t.nav.watchlist}
              </Link>
              <Link
                to="/notifications"
                className="nav-link inline-flex items-center gap-1.5"
              >
                {t.nav.notifications}
                {unread > 0 ? (
                  <span className="inline-flex min-w-[20px] items-center justify-center rounded-full bg-accent px-1.5 py-[1px] font-mono text-[0.64rem] font-semibold text-[color:var(--color-accent-ink)] leading-none">
                    {unread > 99 ? "99+" : unread}
                  </span>
                ) : null}
              </Link>
              <Link to="/account" className="nav-link">
                {t.nav.account}
              </Link>
            </>
          ) : null}
        </nav>

        <div className="flex items-center gap-3">
          <LocaleSwitch />
          {isAuthed ? (
            <>
              <span
                className="hidden sm:inline mono text-[0.78rem] text-fg-muted"
                title={user?.email ?? undefined}
              >
                @{user?.username ?? "anon"}
              </span>
              <button
                type="button"
                onClick={() => void logout()}
                className="mono rounded-[5px] border border-transparent px-2.5 py-1 text-[0.74rem] uppercase tracking-[0.14em] text-fg-muted transition-colors hover:border-accent hover:bg-[color:var(--color-accent-glow)]"
              >
                {t.nav.signOut}
              </button>
            </>
          ) : (
            <Link
              to="/login"
              search={loginSearch()}
              className="inline-flex items-center gap-2 rounded-[6px] border border-line-strong bg-surface px-3.5 py-1.5 text-[0.84rem] font-medium text-fg hover:border-accent hover:text-accent transition-colors"
            >
              {t.header.signIn}
            </Link>
          )}
        </div>
      </div>

      {/* Mobile nav */}
      <nav className="md:hidden border-t border-line">
        <div className="shell flex items-center gap-5 overflow-x-auto py-2.5">
          <Link to="/discover" className="nav-link">
            {t.nav.discover}
          </Link>
          <Link to="/how-to-read" className="nav-link">
            {t.nav.howToRead}
          </Link>
          <Link to="/mcp-guide" className="nav-link">
            {t.nav.mcpGuide}
          </Link>
          {isAuthed ? (
            <>
              <Link to="/watchlist" className="nav-link">
                {t.nav.watchlist}
              </Link>
              <Link
                to="/notifications"
                className="nav-link inline-flex items-center gap-1.5"
              >
                {t.nav.notifications}
                {unread > 0 ? (
                  <span className="inline-flex min-w-[20px] items-center justify-center rounded-full bg-accent px-1.5 py-[1px] font-mono text-[0.64rem] font-semibold text-[color:var(--color-accent-ink)] leading-none">
                    {unread > 99 ? "99+" : unread}
                  </span>
                ) : null}
              </Link>
              <Link to="/account" className="nav-link">
                {t.nav.account}
              </Link>
            </>
          ) : null}
        </div>
      </nav>
    </header>
  );
}
