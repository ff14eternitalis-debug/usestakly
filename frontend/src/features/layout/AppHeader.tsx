import { Link } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { Wordmark } from "../../components/Wordmark";
import { apiGet, authUrl } from "../../lib/api-client";
import type { UnreadCount } from "../../lib/types";
import { useAuthStore } from "../../state/auth-store";
import { logout } from "../auth/hooks";

function Clock() {
  const now = new Date();
  const fmt = now.toLocaleDateString("en-GB", {
    day: "2-digit",
    month: "short",
    year: "numeric"
  });
  return (
    <span className="font-mono text-[0.7rem] uppercase tracking-[0.2em] text-ink-muted">
      Vol. 1 · {fmt}
    </span>
  );
}

export function AppHeader() {
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
    <header className="relative z-20">
      <div className="shell flex items-center justify-between py-3">
        <Clock />
        <span className="kicker hidden md:inline">
          the open-source observatory
        </span>
        {isAuthed ? (
          <button
            type="button"
            onClick={() => void logout()}
            className="font-mono text-[0.7rem] uppercase tracking-[0.2em] text-ink-muted hover:text-ember transition-colors"
          >
            sign out
          </button>
        ) : (
          <Link
            to="/login"
            className="font-mono text-[0.7rem] uppercase tracking-[0.2em] text-ink-muted hover:text-ink transition-colors"
          >
            sign in
          </Link>
        )}
      </div>

      <div className="masthead-rule" />

      <div className="shell py-5">
        <div className="flex flex-wrap items-end justify-between gap-4">
          <Link to="/" className="group inline-flex items-end gap-3">
            <Wordmark scale="md" />
          </Link>
          <nav className="flex flex-wrap items-center gap-7 font-sans text-[0.94rem]">
            <Link to="/discover" className="nav-link">
              Discover
            </Link>
            {isAuthed ? (
              <>
                <Link to="/watchlist" className="nav-link">
                  Watchlist
                </Link>
                <Link
                  to="/notifications"
                  className="nav-link inline-flex items-center gap-2"
                >
                  Notifications
                  {unread > 0 ? (
                    <span className="inline-flex min-w-[22px] items-center justify-center rounded-sm bg-ember px-1.5 py-[1px] font-mono text-[0.66rem] font-semibold text-paper-soft">
                      {unread > 99 ? "99+" : unread}
                    </span>
                  ) : null}
                </Link>
                <span
                  className="font-mono text-[0.74rem] uppercase tracking-[0.18em] text-ink-muted"
                  title={user?.email ?? undefined}
                >
                  @{user?.username ?? "anon"}
                </span>
              </>
            ) : (
              <a
                href={authUrl("/api/auth/github/start")}
                className="nav-link"
              >
                Sign in with GitHub
              </a>
            )}
          </nav>
        </div>
      </div>

      <hr className="rule-dashed" />
    </header>
  );
}
