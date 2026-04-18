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
  readyReferences
}: WorkspaceScreenProps) {
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
        />
        <AssemblySection copy={copy} locale={locale} />
        <SnippetsSection copy={copy} snippets={snippets} />
        <ReferencesSection copy={copy} recentSnippets={recentSnippets} />
      </div>
    </section>
  );
}
