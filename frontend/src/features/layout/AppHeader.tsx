import { Link, useNavigate } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";

import { Wordmark } from "../../components/Wordmark";
import { useT } from "../../i18n";
import { apiGet } from "../../lib/api-client";
import { loginSearch } from "../../lib/return-to";
import type { UnreadCount } from "../../lib/types";
import { useAuthStore } from "../../state/auth-store";
import { logout } from "../auth/hooks";

export function AppHeader() {
  const t = useT();
  const navigate = useNavigate();
  const { status, user } = useAuthStore();
  const isAuthed = status === "authenticated";
  const [profileMenuOpen, setProfileMenuOpen] = useState(false);
  const profileMenuRef = useRef<HTMLDivElement | null>(null);

  const unreadQuery = useQuery({
    queryKey: ["notifications", "unread"],
    queryFn: ({ signal }) =>
      apiGet<UnreadCount>("/api/notifications/unread-count", signal),
    enabled: isAuthed,
    refetchInterval: 60_000,
    staleTime: 30_000
  });
  const unread = unreadQuery.data?.unread ?? 0;
  const displayName = user?.displayName ?? user?.username ?? "UseStakly";
  const accountLabel = user?.username ? `@${user.username}` : displayName;
  const avatarFallback = displayName.trim().charAt(0).toUpperCase() || "U";

  useEffect(() => {
    if (!profileMenuOpen) {
      return;
    }

    function handlePointerDown(event: MouseEvent) {
      if (
        profileMenuRef.current &&
        !profileMenuRef.current.contains(event.target as Node)
      ) {
        setProfileMenuOpen(false);
      }
    }

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        setProfileMenuOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [profileMenuOpen]);

  async function handleSignOut() {
    setProfileMenuOpen(false);
    await logout();
    await navigate({ to: "/" });
  }

  return (
    <header className="sticky top-0 z-30 border-b border-line bg-[color:var(--color-bg)]/80 backdrop-blur-xl backdrop-saturate-150">
      <div className="shell flex h-[62px] items-center justify-between gap-6">
        <Link to="/" className="group inline-flex items-center gap-2">
          <Wordmark scale="md" />
        </Link>

        <nav className="hidden md:flex items-center gap-7">
          <Link to="/how-to-read" className="nav-link">
            {t.nav.howToRead}
          </Link>
          <Link to="/discover" className="nav-link">
            {t.nav.discover}
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
            </>
          ) : null}
          <Link to="/mcp-guide" className="nav-link">
            {t.nav.mcpGuide}
          </Link>
        </nav>

        <div className="flex items-center gap-4 sm:gap-5">
          {isAuthed ? (
            <div ref={profileMenuRef} className="relative">
              <button
                type="button"
                aria-haspopup="menu"
                aria-expanded={profileMenuOpen}
                aria-label={`${t.nav.profile}: ${accountLabel}`}
                onClick={() => setProfileMenuOpen((open) => !open)}
                className="inline-flex size-10 items-center justify-center rounded-full border border-line bg-surface p-0.5 transition-colors hover:border-accent hover:bg-[color:var(--color-accent-glow)] focus-visible:border-accent focus-visible:outline-none"
                title={user?.email ?? accountLabel}
              >
                {user?.avatarUrl ? (
                  <img
                    src={user.avatarUrl}
                    alt=""
                    referrerPolicy="no-referrer"
                    className="size-full rounded-full object-cover"
                  />
                ) : (
                  <span className="inline-flex size-full items-center justify-center rounded-full bg-accent font-mono text-[0.9rem] font-semibold text-[color:var(--color-accent-ink)]">
                    {avatarFallback}
                  </span>
                )}
              </button>

              {profileMenuOpen ? (
                <div
                  role="menu"
                  className="absolute right-0 top-[calc(100%+0.65rem)] z-50 w-60 rounded-[8px] border border-line bg-surface p-2 shadow-[0_18px_50px_rgba(0,0,0,0.45)]"
                >
                  <div className="flex items-center gap-3 border-b border-line px-2 pb-3 pt-1">
                    <div className="inline-flex size-9 shrink-0 items-center justify-center overflow-hidden rounded-full border border-line bg-bg">
                      {user?.avatarUrl ? (
                        <img
                          src={user.avatarUrl}
                          alt=""
                          referrerPolicy="no-referrer"
                          className="size-full object-cover"
                        />
                      ) : (
                        <span className="font-mono text-[0.8rem] font-semibold text-accent">
                          {avatarFallback}
                        </span>
                      )}
                    </div>
                    <div className="min-w-0">
                      <p className="truncate text-[0.86rem] font-medium text-fg">
                        {displayName}
                      </p>
                      <p className="truncate text-[0.74rem] text-fg-muted">
                        {accountLabel}
                      </p>
                    </div>
                  </div>

                  <Link
                    to="/account"
                    role="menuitem"
                    onClick={() => setProfileMenuOpen(false)}
                    className="mt-2 flex w-full items-center rounded-[6px] px-3 py-2 text-left text-[0.86rem] text-fg-dim transition-colors hover:bg-[color:var(--color-accent-glow)] hover:text-accent"
                  >
                    {t.nav.profile}
                  </Link>
                  <button
                    type="button"
                    role="menuitem"
                    onClick={() => {
                      void handleSignOut();
                    }}
                    className="mono flex w-full items-center rounded-[6px] px-3 py-2 text-left text-[0.74rem] uppercase tracking-[0.14em] text-fg-muted transition-colors hover:bg-[color:var(--color-accent-glow)] hover:text-accent"
                  >
                    {t.nav.signOut}
                  </button>
                </div>
              ) : null}
            </div>
          ) : (
            <Link
              to="/login"
              search={loginSearch()}
              className="inline-flex items-center gap-2 rounded-[6px] border border-line-strong bg-surface px-4 py-2 text-[0.86rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent"
            >
              {t.header.signIn}
            </Link>
          )}
        </div>
      </div>

      {/* Mobile nav */}
      <nav className="md:hidden border-t border-line">
        <div className="shell flex items-center gap-5 overflow-x-auto py-2.5">
          <Link to="/how-to-read" className="nav-link">
            {t.nav.howToRead}
          </Link>
          <Link to="/discover" className="nav-link">
            {t.nav.discover}
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
            </>
          ) : null}
          <Link to="/mcp-guide" className="nav-link">
            {t.nav.mcpGuide}
          </Link>
        </div>
      </nav>
    </header>
  );
}
