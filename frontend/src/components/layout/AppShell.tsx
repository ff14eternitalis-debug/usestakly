import { useEffect, useState } from "react";

import { apiPost, authUrl } from "../../lib/api-client";

type CurrentUser = {
  id: string;
  email: string;
  username: string;
  displayName: string | null;
  avatarUrl: string | null;
};

type Locale = "en" | "fr";

const COPY: Record<
  Locale,
  {
    authEyebrow: string;
    authTitle: string;
    authBody: string;
    authButton: string;
    authNotice: string;
    authSecurityLabel: string;
    authSecurityValue: string;
    authAccessLabel: string;
    authAccessValue: string;
    loading: string;
    language: string;
    connectedTitle: string;
    connectedBody: string;
    connectedLabel: string;
    logout: string;
  }
> = {
  en: {
    authEyebrow: "Authentication",
    authTitle: "Sign in to continue",
    authBody:
      "Access your libraries, sync your identity, and start from your own codebase.",
    authButton: "Continue with GitHub",
    authNotice: "Only GitHub is enabled for the MVP.",
    authSecurityLabel: "Session",
    authSecurityValue: "Secure browser session",
    authAccessLabel: "Language",
    authAccessValue: "English / French",
    loading: "Checking session...",
    language: "FR",
    connectedTitle: "You are connected",
    connectedBody: "Your session is active and ready to access UseStakly.",
    connectedLabel: "Signed in as",
    logout: "Logout"
  },
  fr: {
    authEyebrow: "Authentification",
    authTitle: "Connecte-toi pour continuer",
    authBody:
      "Accède à tes bibliothèques, synchronise ton identité et démarre depuis ta propre base de code.",
    authButton: "Continuer avec GitHub",
    authNotice: "Seul GitHub est activé pour le MVP.",
    authSecurityLabel: "Session",
    authSecurityValue: "Session navigateur sécurisée",
    authAccessLabel: "Langue",
    authAccessValue: "Français / Anglais",
    loading: "Vérification de la session...",
    language: "EN",
    connectedTitle: "Tu es connecté",
    connectedBody: "Ta session est active et prête à accéder à UseStakly.",
    connectedLabel: "Connecté en tant que",
    logout: "Se déconnecter"
  }
};

function detectInitialLocale(): Locale {
  if (typeof window === "undefined") {
    return "en";
  }

  const stored = window.localStorage.getItem("usestakly-locale");
  if (stored === "fr" || stored === "en") {
    return stored;
  }

  return window.navigator.language.toLowerCase().startsWith("fr") ? "fr" : "en";
}

export function AppShell() {
  const [user, setUser] = useState<CurrentUser | null>(null);
  const [loading, setLoading] = useState(true);
  const [locale, setLocale] = useState<Locale>(detectInitialLocale);

  useEffect(() => {
    window.localStorage.setItem("usestakly-locale", locale);
  }, [locale]);

  useEffect(() => {
    let cancelled = false;

    async function loadUser() {
      try {
        const response = await fetch(authUrl("/api/me"), {
          credentials: "include"
        });

        if (!response.ok) {
          if (!cancelled) {
            setUser(null);
          }
          return;
        }

        const data = (await response.json()) as CurrentUser;
        if (!cancelled) {
          setUser(data);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadUser();

    return () => {
      cancelled = true;
    };
  }, []);

  async function handleLogout() {
    await apiPost("/api/auth/logout");
    setUser(null);
  }

  const copy = COPY[locale];

  return (
    <main className="auth-screen">
      <div className="auth-noise" />
      <section className="auth-panel">
        <div className="auth-brand-row">
          <div>
            <p className="auth-brand-mark">UseStakly</p>
            <p className="auth-brand-subtitle">{copy.authEyebrow}</p>
          </div>
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
            ) : user ? (
              <div className="auth-state">
                <p className="auth-status">{copy.connectedTitle}</p>
                <p className="auth-card-copy">{copy.connectedBody}</p>
                <div className="identity-block">
                  <span>{copy.connectedLabel}</span>
                  <strong>{user.displayName ?? user.username}</strong>
                  <small>{user.email}</small>
                </div>
                <button
                  className="auth-secondary-button"
                  type="button"
                  onClick={() => {
                    void handleLogout();
                  }}
                >
                  {copy.logout}
                </button>
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
    </main>
  );
}
