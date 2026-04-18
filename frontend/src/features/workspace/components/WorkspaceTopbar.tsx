import type { Dispatch, SetStateAction } from "react";

import { avatarFallback } from "../../../lib/app-copy";
import type { CopyBlock, CurrentUser, Locale, Theme } from "../../../lib/app-types";

type WorkspaceTopbarProps = {
  copy: CopyBlock;
  user: CurrentUser;
  setLocale: Dispatch<SetStateAction<Locale>>;
  setTheme: Dispatch<SetStateAction<Theme>>;
  onLogout: () => void;
};

export function WorkspaceTopbar({
  copy,
  user,
  setLocale,
  setTheme,
  onLogout
}: WorkspaceTopbarProps) {
  return (
    <header className="workspace-topbar">
      <div className="workspace-brand-block">
        <p className="auth-brand-mark">UseStakly</p>
        <p className="auth-brand-subtitle">{copy.workspaceEyebrow}</p>
      </div>

      <div className="workspace-actions">
        <button
          className="lang-toggle"
          type="button"
          onClick={() => {
            setTheme((current) => (current === "light" ? "dark" : "light"));
          }}
        >
          {copy.theme}
        </button>
        <button
          className="lang-toggle"
          type="button"
          onClick={() => {
            setLocale((current) => (current === "en" ? "fr" : "en"));
          }}
        >
          {copy.language}
        </button>
        <div className="workspace-user">
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
          <button className="workspace-logout" type="button" onClick={onLogout}>
            {copy.logoutSecondary}
          </button>
        </div>
      </div>
    </header>
  );
}
