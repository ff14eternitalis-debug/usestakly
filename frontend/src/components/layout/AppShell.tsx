import { useEffect, useMemo, useState } from "react";

import { AuthScreen } from "../../features/auth/components/AuthScreen";
import { WorkspaceScreen } from "../../features/workspace/components/WorkspaceScreen";
import { COPY, detectInitialLocale } from "../../lib/app-copy";
import { apiGet, apiPost, apiPostJson, authUrl } from "../../lib/api-client";
import type {
  CurrentUser,
  LibraryListResponse,
  LibraryRecord,
  Locale,
  SnippetDetail,
  SnippetListResponse
} from "../../lib/app-types";

export function AppShell() {
  const [user, setUser] = useState<CurrentUser | null>(null);
  const [libraries, setLibraries] = useState<LibraryRecord[]>([]);
  const [snippets, setSnippets] = useState<SnippetDetail[]>([]);
  const [loading, setLoading] = useState(true);
  const [workspaceLoading, setWorkspaceLoading] = useState(false);
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

  async function refreshWorkspace() {
    if (!user) {
      return;
    }

    setWorkspaceLoading(true);
    try {
      const [libraryResponse, snippetResponse] = await Promise.all([
        apiGet<LibraryListResponse>("/api/libraries"),
        apiGet<SnippetListResponse>("/api/snippets")
      ]);
      setLibraries(libraryResponse.items);
      setSnippets(snippetResponse.items);
    } finally {
      setWorkspaceLoading(false);
    }
  }

  async function handleLogout() {
    await apiPost("/api/auth/logout");
    setUser(null);
    setLibraries([]);
    setSnippets([]);
  }

  async function handleCreateLibrary(input: {
    name: string;
    slug: string;
    description?: string;
  }) {
    await apiPostJson("/api/libraries", {
      ...input,
      visibility: "private",
      isDefault: libraries.length === 0
    });
    await refreshWorkspace();
  }

  async function handleCreateSnippet(input: {
    libraryId: string;
    slug: string;
    name: string;
    domain: string;
    kind: string;
    category: string;
    language: string;
    framework?: string;
    tags: string[];
    version: string;
    code: string;
  }) {
    await apiPostJson("/api/snippets", {
      libraryId: input.libraryId,
      slug: input.slug,
      name: input.name,
      domain: input.domain,
      kind: input.kind,
      category: input.category,
      language: input.language,
      framework: input.framework,
      visibility: "private",
      tags: input.tags,
      initialVersion: {
        version: input.version,
        code: input.code,
        riskLevel: "safe"
      }
    });
    await refreshWorkspace();
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

      {user ? (
        <WorkspaceScreen
          copy={copy}
          user={user}
          libraries={libraries}
          snippets={snippets}
          recentSnippets={recentSnippets}
          workspaceLoading={workspaceLoading}
          locale={locale}
          setLocale={setLocale}
          onLogout={() => {
            void handleLogout();
          }}
          publicAssetCount={publicLibraries + publicSnippets}
          readyReferences={readyReferences}
          onCreateLibrary={handleCreateLibrary}
          onCreateSnippet={handleCreateSnippet}
        />
      ) : (
        <AuthScreen
          copy={copy}
          loading={loading}
          setLocale={setLocale}
        />
      )}
    </main>
  );
}
