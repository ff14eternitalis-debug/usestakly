import type { Dispatch, SetStateAction } from "react";

import { authUrl } from "../../../lib/api-client";
import type { CopyBlock, Locale, Theme } from "../../../lib/app-types";

type AuthScreenProps = {
  copy: CopyBlock;
  loading: boolean;
  setLocale: Dispatch<SetStateAction<Locale>>;
  setTheme: Dispatch<SetStateAction<Theme>>;
};

export function AuthScreen({
  copy,
  loading,
  setLocale,
  setTheme
}: AuthScreenProps) {
  return (
    <section className="auth-panel">
      <div className="auth-brand-row">
        <div>
          <p className="auth-brand-mark">UseStakly</p>
          <p className="auth-brand-subtitle">{copy.authEyebrow}</p>
        </div>
        <div className="auth-controls">
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
        </div>
      </div>

      <div className="auth-grid">
        <div className="auth-hero">
          <h1 className="auth-title">UseStakly</h1>
          <p className="auth-copy">{copy.authTitle}</p>
          <p className="auth-subcopy">{copy.authBody}</p>

          <div className="auth-meta">
            <div>
              <span>{copy.authSecurityLabel}</span>
              <strong>{copy.authSecurityValue}</strong>
            </div>
            <div>
              <span>{copy.authAccessLabel}</span>
              <strong>{copy.authAccessValue}</strong>
            </div>
          </div>
        </div>

        <aside className="auth-card">
          {loading ? (
            <div className="auth-state">
              <p className="auth-status">{copy.loading}</p>
            </div>
          ) : (
            <div className="auth-state">
              <a className="auth-primary-button" href={authUrl("/api/auth/github/start")}>
                {copy.authButton}
              </a>
              <p className="auth-card-copy">{copy.authNotice}</p>
            </div>
          )}
        </aside>
      </div>
    </section>
  );
}
