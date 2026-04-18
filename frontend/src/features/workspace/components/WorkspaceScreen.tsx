import { useEffect, useState } from "react";
import type { Dispatch, SetStateAction } from "react";

import type {
  CopyBlock,
  CurrentUser,
  LibraryRecord,
  Locale,
  SnippetDetail,
  Theme
} from "../../../lib/app-types";
import { AssemblySection } from "./AssemblySection";
import { LibrariesSection } from "./LibrariesSection";
import { ReferencesSection } from "./ReferencesSection";
import { SnippetDetailPanel } from "./SnippetDetailPanel";
import { SnippetsSection } from "./SnippetsSection";
import { WorkspaceTopbar } from "./WorkspaceTopbar";

type WorkspaceScreenProps = {
  copy: CopyBlock;
  user: CurrentUser;
  libraries: LibraryRecord[];
  snippets: SnippetDetail[];
  recentSnippets: SnippetDetail[];
  workspaceLoading: boolean;
  locale: Locale;
  setLocale: Dispatch<SetStateAction<Locale>>;
  setTheme: Dispatch<SetStateAction<Theme>>;
  onLogout: () => void;
  publicAssetCount: number;
  readyReferences: number;
  onCreateLibrary: (input: {
    name: string;
    slug: string;
    description?: string;
  }) => Promise<void>;
  onCreateSnippet: (input: {
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
  }) => Promise<void>;
};

export function WorkspaceScreen({
  copy,
  user,
  libraries,
  snippets,
  recentSnippets,
  workspaceLoading,
  locale,
  setLocale,
  setTheme,
  onLogout,
  publicAssetCount,
  readyReferences,
  onCreateLibrary,
  onCreateSnippet
}: WorkspaceScreenProps) {
  const [selectedSnippetId, setSelectedSnippetId] = useState<string | null>(
    snippets[0]?.snippet.id ?? null
  );

  useEffect(() => {
    if (snippets.length === 0) {
      setSelectedSnippetId(null);
      return;
    }

    if (!selectedSnippetId || !snippets.some((item) => item.snippet.id === selectedSnippetId)) {
      setSelectedSnippetId(snippets[0].snippet.id);
    }
  }, [selectedSnippetId, snippets]);

  const selectedSnippet =
    snippets.find((item) => item.snippet.id === selectedSnippetId) ?? null;

  return (
    <section className="workspace-panel">
      <WorkspaceTopbar
        copy={copy}
        user={user}
        setLocale={setLocale}
        setTheme={setTheme}
        onLogout={onLogout}
      />

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
            <strong>{publicAssetCount}</strong>
          </div>
          <div>
            <span>{copy.readyStat}</span>
            <strong>{readyReferences}</strong>
          </div>
        </div>
      </div>

      <div className="workspace-grid">
        <LibrariesSection
          copy={copy}
          libraries={libraries}
          workspaceLoading={workspaceLoading}
          onCreateLibrary={onCreateLibrary}
        />
        <AssemblySection copy={copy} locale={locale} />
        <SnippetsSection
          copy={copy}
          libraries={libraries}
          snippets={snippets}
          selectedSnippetId={selectedSnippetId}
          onSelectSnippet={(snippet) => setSelectedSnippetId(snippet.snippet.id)}
          onCreateSnippet={onCreateSnippet}
        />
        <SnippetDetailPanel
          copy={copy}
          libraries={libraries}
          selectedSnippet={selectedSnippet}
        />
        <ReferencesSection copy={copy} recentSnippets={recentSnippets} />
      </div>
    </section>
  );
}
