import type { Dispatch, SetStateAction } from "react";

import { avatarFallback } from "../../../lib/app-copy";
import type { AppView, CopyBlock, CurrentUser, Locale } from "../../../lib/app-types";

type AppTopbarProps = {
  copy: CopyBlock;
  user: CurrentUser;
  activeView: AppView;
  setActiveView: Dispatch<SetStateAction<AppView>>;
  setLocale: Dispatch<SetStateAction<Locale>>;
};

const NAV_ITEMS: AppView[] = ["home", "explore", "documents", "forum", "studio"];

export function AppTopbar({
  copy,
  user,
  activeView,
  setActiveView,
  setLocale
}: AppTopbarProps) {
  const labels: Record<AppView, string> = {
    home: copy.navHome,
    explore: copy.navExplore,
    documents: copy.navDocuments,
    forum: copy.navForum,
    studio: copy.navStudio,
    profile: copy.navProfile,
    snippet: copy.homeOpenSnippet
  };

  return (
    <header className="app-header">
      <div className="app-header-inner">
        <div className="app-brand-block">
          <p className="auth-brand-mark">UseStakly</p>
          <p className="auth-brand-subtitle">{copy.appEyebrow}</p>
        </div>

        <nav className="app-nav" aria-label="Primary">
          {NAV_ITEMS.map((item) => (
            <button
              key={item}
              className={`app-nav-button${activeView === item ? " app-nav-button-active" : ""}`}
              type="button"
              onClick={() => setActiveView(item)}
            >
              {labels[item]}
            </button>
          ))}
        </nav>

        <div className="app-user-bar">
          <button
            className="lang-toggle"
            type="button"
            onClick={() => setLocale((current) => (current === "en" ? "fr" : "en"))}
          >
            {copy.language}
          </button>
          <button
            className="app-user-trigger"
            type="button"
            onClick={() => setActiveView("profile")}
          >
            <div className="workspace-avatar">
              {user.avatarUrl ? (
                <img alt={user.displayName ?? user.username} src={user.avatarUrl} />
              ) : (
                <span>{avatarFallback(user)}</span>
              )}
            </div>
            <div className="workspace-user-copy">
              <strong>{user.displayName ?? user.username}</strong>
              <small>{user.email}</small>
            </div>
          </button>
        </div>
      </div>
    </header>
  );
}
