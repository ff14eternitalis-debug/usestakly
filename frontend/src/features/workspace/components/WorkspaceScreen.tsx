import { useEffect, useState } from "react";

import type {
  CopyBlock,
  LibraryRecord,
  Locale,
  SnippetDetail
} from "../../../lib/app-types";
import { AssemblySection } from "./AssemblySection";
import { LibrariesSection } from "./LibrariesSection";
import { ReferencesSection } from "./ReferencesSection";
import { SnippetDetailPanel } from "./SnippetDetailPanel";
import { SnippetsSection } from "./SnippetsSection";

type WorkspaceScreenProps = {
  copy: CopyBlock;
  libraries: LibraryRecord[];
  snippets: SnippetDetail[];
  recentSnippets: SnippetDetail[];
  workspaceLoading: boolean;
  locale: Locale;
  publicAssetCount: number;
  readyReferences: number;
  onCreateLibrary: (input: {
    name: string;
    slug: string;
    description?: string;
    visibility: "private" | "public";
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
  libraries,
  snippets,
  recentSnippets,
  workspaceLoading,
  locale,
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
    <section className="workspace-studio">
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
