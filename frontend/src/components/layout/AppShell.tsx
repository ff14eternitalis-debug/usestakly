import { useEffect, useMemo, useState } from "react";

import { apiGet, apiPost, authUrl } from "../../lib/api-client";

type CurrentUser = {
  id: string;
  email: string;
  username: string;
  displayName: string | null;
  avatarUrl: string | null;
};

type LibraryRecord = {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  visibility: string;
  trustLevel: string;
  isDefault: boolean;
};

type SnippetVersionRecord = {
  id: string;
  version: string;
  riskLevel: string;
};

type SnippetRecord = {
  id: string;
  libraryId: string;
  slug: string;
  domain: string;
  kind: string;
  category: string | null;
  name: string;
  description: string | null;
  language: string;
  framework: string | null;
  visibility: string;
  trustLevel: string;
  updatedAt: string;
};

type SnippetDetail = {
  snippet: SnippetRecord;
  currentVersion: SnippetVersionRecord | null;
  tags: string[];
  canonicalReference: string;
};

type LibraryListResponse = {
  items: LibraryRecord[];
};

type SnippetListResponse = {
  items: SnippetDetail[];
};

type Locale = "en" | "fr";
type Theme = "light" | "dark";

type CopyBlock = {
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
  theme: string;
  connectedTitle: string;
  connectedBody: string;
  connectedLabel: string;
  logout: string;
  workspaceEyebrow: string;
  workspaceTitle: string;
  workspaceBody: string;
  workspaceStatus: string;
  librariesStat: string;
  snippetsStat: string;
  publicStat: string;
  readyStat: string;
  librariesTitle: string;
  librariesBody: string;
  snippetsTitle: string;
  snippetsBody: string;
  recentTitle: string;
  recentBody: string;
  commandTitle: string;
  commandBody: string;
  commandModeStrict: string;
  commandModeAuto: string;
  commandModePrompt: string;
  defaultLibrary: string;
  emptyLibraries: string;
  emptySnippets: string;
  visibilityPrivate: string;
  visibilityPublic: string;
  trustedPrivate: string;
  trustedPublic: string;
  referenceLabel: string;
  tagsLabel: string;
  versionLabel: string;
  scopeLabel: string;
  logoutSecondary: string;
};

const COPY: Record<Locale, CopyBlock> = {
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
    theme: "Dark",
    connectedTitle: "You are connected",
    connectedBody: "Your session is active and ready to access UseStakly.",
    connectedLabel: "Signed in as",
    logout: "Logout",
    workspaceEyebrow: "Workspace",
    workspaceTitle: "UseStakly",
    workspaceBody:
      "Resolve your own libraries first, then compose a real app from addressable components.",
    workspaceStatus: "Private-first workspace is online",
    librariesStat: "Libraries",
    snippetsStat: "Snippets",
    publicStat: "Public assets",
    readyStat: "Assembly ready",
    librariesTitle: "Libraries",
    librariesBody: "Ownable sources the MCP can resolve before it ever generates code.",
    snippetsTitle: "Snippet inventory",
    snippetsBody: "Recent reusable building blocks already indexed in your workspace.",
    recentTitle: "Recent references",
    recentBody: "Direct references your agent can request explicitly.",
    commandTitle: "Assembly behavior",
    commandBody:
      "UseStakly should resolve exact assets first, search inside your allowed scope second, and only generate as a fallback.",
    commandModeStrict: "Strict mode",
    commandModeAuto: "Auto mode",
    commandModePrompt: "Prompt shape",
    defaultLibrary: "Default library",
    emptyLibraries: "No library yet. Your first library will become the initial MCP anchor.",
    emptySnippets:
      "No snippet yet. Once you add one, it will appear here with its canonical reference.",
    visibilityPrivate: "Private",
    visibilityPublic: "Public",
    trustedPrivate: "Private trust",
    trustedPublic: "Public unverified",
    referenceLabel: "Reference",
    tagsLabel: "Tags",
    versionLabel: "Version",
    scopeLabel: "Scope",
    logoutSecondary: "Sign out"
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
    theme: "Sombre",
    connectedTitle: "Tu es connecté",
    connectedBody: "Ta session est active et prête à accéder à UseStakly.",
    connectedLabel: "Connecté en tant que",
    logout: "Se déconnecter",
    workspaceEyebrow: "Workspace",
    workspaceTitle: "UseStakly",
    workspaceBody:
      "Résous d'abord tes propres bibliothèques, puis compose une vraie app à partir de composants adressables.",
    workspaceStatus: "Workspace private-first en ligne",
    librariesStat: "Bibliothèques",
    snippetsStat: "Snippets",
    publicStat: "Assets publics",
    readyStat: "Prêt pour l’assemblage",
    librariesTitle: "Bibliothèques",
    librariesBody:
      "Des sources maîtrisées que le MCP peut résoudre avant de générer la moindre ligne.",
    snippetsTitle: "Inventaire de snippets",
    snippetsBody:
      "Les briques réutilisables les plus récentes déjà indexées dans ton workspace.",
    recentTitle: "Références récentes",
    recentBody: "Des références directes que ton agent peut demander explicitement.",
    commandTitle: "Comportement d’assemblage",
    commandBody:
      "UseStakly doit résoudre les assets exacts d’abord, chercher dans le scope autorisé ensuite, et ne générer qu’en fallback.",
    commandModeStrict: "Mode strict",
    commandModeAuto: "Mode auto",
    commandModePrompt: "Forme du prompt",
    defaultLibrary: "Bibliothèque par défaut",
    emptyLibraries:
      "Aucune bibliothèque pour l’instant. La première deviendra l’ancre initiale du MCP.",
    emptySnippets:
      "Aucun snippet pour l’instant. Dès que tu en ajoutes un, il apparaîtra ici avec sa référence canonique.",
    visibilityPrivate: "Privée",
    visibilityPublic: "Publique",
    trustedPrivate: "Trust privée",
    trustedPublic: "Publique non vérifiée",
    referenceLabel: "Référence",
    tagsLabel: "Tags",
    versionLabel: "Version",
    scopeLabel: "Scope",
    logoutSecondary: "Déconnexion"
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

function detectInitialTheme(): Theme {
  if (typeof window === "undefined") {
    return "light";
  }

  const stored = window.localStorage.getItem("usestakly-theme");
  if (stored === "light" || stored === "dark") {
    return stored;
  }

  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function getVisibilityLabel(copy: CopyBlock, visibility: string): string {
  return visibility === "public" ? copy.visibilityPublic : copy.visibilityPrivate;
}

function getTrustLabel(copy: CopyBlock, trustLevel: string): string {
  return trustLevel === "public_unverified" ? copy.trustedPublic : copy.trustedPrivate;
}

function avatarFallback(user: CurrentUser): string {
  const source = user.displayName ?? user.username ?? user.email;
  return source.slice(0, 2).toUpperCase();
}

export function AppShell() {
  const [user, setUser] = useState<CurrentUser | null>(null);
  const [libraries, setLibraries] = useState<LibraryRecord[]>([]);
  const [snippets, setSnippets] = useState<SnippetDetail[]>([]);
  const [loading, setLoading] = useState(true);
  const [workspaceLoading, setWorkspaceLoading] = useState(false);
  const [locale, setLocale] = useState<Locale>(detectInitialLocale);
  const [theme, setTheme] = useState<Theme>(detectInitialTheme);

  useEffect(() => {
    window.localStorage.setItem("usestakly-locale", locale);
  }, [locale]);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    window.localStorage.setItem("usestakly-theme", theme);
  }, [theme]);

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

  useEffect(() => {
    let cancelled = false;

    async function loadWorkspace() {
      if (!user) {
        setLibraries([]);
        setSnippets([]);
        return;
      }

      setWorkspaceLoading(true);

      try {
        const [libraryResponse, snippetResponse] = await Promise.all([
          apiGet<LibraryListResponse>("/api/libraries"),
          apiGet<SnippetListResponse>("/api/snippets")
        ]);

        if (!cancelled) {
          setLibraries(libraryResponse.items);
          setSnippets(snippetResponse.items);
        }
      } catch {
        if (!cancelled) {
          setLibraries([]);
          setSnippets([]);
        }
      } finally {
        if (!cancelled) {
          setWorkspaceLoading(false);
        }
      }
    }

    void loadWorkspace();

    return () => {
      cancelled = true;
    };
  }, [user]);

  async function handleLogout() {
    await apiPost("/api/auth/logout");
    setUser(null);
    setLibraries([]);
    setSnippets([]);
  }

  const copy = COPY[locale];
  const publicLibraries = useMemo(
    () => libraries.filter((library) => library.visibility === "public").length,
    [libraries]
  );
  const publicSnippets = useMemo(
    () => snippets.filter((item) => item.snippet.visibility === "public").length,
    [snippets]
  );
  const readyReferences = useMemo(
    () => snippets.filter((item) => item.currentVersion !== null).length,
    [snippets]
  );
  const recentSnippets = useMemo(() => snippets.slice(0, 4), [snippets]);

  return (
    <main className={user ? "workspace-shell" : "auth-screen"}>
      <div className="auth-noise" />

      {!user ? (
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
      ) : (
        <section className="workspace-panel">
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
                <button
                  className="workspace-logout"
                  type="button"
                  onClick={() => {
                    void handleLogout();
                  }}
                >
                  {copy.logoutSecondary}
                </button>
              </div>
            </div>
          </header>

          <div className="workspace-hero">
            <div className="workspace-hero-copy">
              <span className="workspace-status-pill">{copy.workspaceStatus}</span>
              <h1 className="workspace-title">{copy.workspaceTitle}</h1>
              <p className="workspace-copy">{copy.workspaceBody}</p>
            </div>

            <div className="workspace-stats">
              <div>
                <span>{copy.librariesStat}</span>
                <strong>{libraries.length}</strong>
              </div>
              <div>
                <span>{copy.snippetsStat}</span>
                <strong>{snippets.length}</strong>
              </div>
              <div>
                <span>{copy.publicStat}</span>
                <strong>{publicLibraries + publicSnippets}</strong>
              </div>
              <div>
                <span>{copy.readyStat}</span>
                <strong>{readyReferences}</strong>
              </div>
            </div>
          </div>

          <div className="workspace-grid">
            <section className="workspace-section workspace-section-wide">
              <div className="workspace-section-head">
                <div>
                  <h2>{copy.librariesTitle}</h2>
                  <p>{copy.librariesBody}</p>
                </div>
                {workspaceLoading ? <span className="workspace-inline-note">{copy.loading}</span> : null}
              </div>

              {libraries.length === 0 ? (
                <div className="workspace-empty">{copy.emptyLibraries}</div>
              ) : (
                <div className="library-list">
                  {libraries.map((library) => (
                    <article className="library-row" key={library.id}>
                      <div className="library-row-main">
                        <div className="library-row-head">
                          <h3>{library.name}</h3>
                          {library.isDefault ? (
                            <span className="workspace-chip workspace-chip-accent">
                              {copy.defaultLibrary}
                            </span>
                          ) : null}
                        </div>
                        <p>{library.description ?? library.slug}</p>
                      </div>
                      <div className="library-row-meta">
                        <span className="workspace-chip">{library.slug}</span>
                        <span className="workspace-chip">
                          {getVisibilityLabel(copy, library.visibility)}
                        </span>
                        <span className="workspace-chip">
                          {getTrustLabel(copy, library.trustLevel)}
                        </span>
                      </div>
                    </article>
                  ))}
                </div>
              )}
            </section>

            <section className="workspace-section">
              <div className="workspace-section-head">
                <div>
                  <h2>{copy.commandTitle}</h2>
                  <p>{copy.commandBody}</p>
                </div>
              </div>

              <div className="command-stack">
                <div className="command-mode">
                  <span>{copy.commandModeStrict}</span>
                  <strong>@owner/library:snippet@version</strong>
                </div>
                <div className="command-mode">
                  <span>{copy.commandModeAuto}</span>
                  <strong>own libraries - public fallback - generate last</strong>
                </div>
                <div className="command-prompt">
                  <span>{copy.commandModePrompt}</span>
                  <code>
                    {locale === "fr"
                      ? "Construis cette feature avec React + Tailwind en résolvant d’abord @moi/ui-kit"
                      : "Build this feature with React + Tailwind and resolve @me/ui-kit first"}
                  </code>
                </div>
              </div>
            </section>

            <section className="workspace-section workspace-section-wide">
              <div className="workspace-section-head">
                <div>
                  <h2>{copy.snippetsTitle}</h2>
                  <p>{copy.snippetsBody}</p>
                </div>
              </div>

              {snippets.length === 0 ? (
                <div className="workspace-empty">{copy.emptySnippets}</div>
              ) : (
                <div className="snippet-grid">
                  {snippets.slice(0, 6).map((item) => (
                    <article className="snippet-row" key={item.snippet.id}>
                      <div className="snippet-head">
                        <div>
                          <h3>{item.snippet.name}</h3>
                          <p>{item.snippet.description ?? item.snippet.slug}</p>
                        </div>
                        <span className="workspace-chip">{item.snippet.domain}</span>
                      </div>

                      <div className="snippet-metadata">
                        <span className="workspace-chip">{item.snippet.language}</span>
                        {item.snippet.framework ? (
                          <span className="workspace-chip">{item.snippet.framework}</span>
                        ) : null}
                        <span className="workspace-chip">
                          {getVisibilityLabel(copy, item.snippet.visibility)}
                        </span>
                        <span className="workspace-chip">
                          {item.currentVersion
                            ? `${copy.versionLabel} ${item.currentVersion.version}`
                            : copy.readyStat}
                        </span>
                      </div>

                      <div className="snippet-reference">
                        <span>{copy.referenceLabel}</span>
                        <code>{item.canonicalReference}</code>
                      </div>
                    </article>
                  ))}
                </div>
              )}
            </section>

            <section className="workspace-section">
              <div className="workspace-section-head">
                <div>
                  <h2>{copy.recentTitle}</h2>
                  <p>{copy.recentBody}</p>
                </div>
              </div>

              {recentSnippets.length === 0 ? (
                <div className="workspace-empty">{copy.emptySnippets}</div>
              ) : (
                <div className="reference-list">
                  {recentSnippets.map((item) => (
                    <article className="reference-row" key={item.snippet.id}>
                      <div>
                        <strong>{item.snippet.name}</strong>
                        <span>{item.snippet.language}</span>
                      </div>
                      <code>{item.canonicalReference}</code>
                      <p>
                        {copy.tagsLabel}: {item.tags.length > 0 ? item.tags.join(", ") : "none"}
                      </p>
                    </article>
                  ))}
                </div>
              )}
            </section>
          </div>
        </section>
      )}
    </main>
  );
}
