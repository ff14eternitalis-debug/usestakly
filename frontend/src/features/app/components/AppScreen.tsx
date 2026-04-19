import type { Dispatch, SetStateAction } from "react";

import type {
  AppView,
  CommunitySnippet,
  CopyBlock,
  CurrentUser,
  LibraryRecord,
  Locale,
  PublicLibraryProfile,
  SnippetDetail
} from "../../../lib/app-types";
import { ExploreFeed } from "./ExploreFeed";
import { HomeFeed } from "./HomeFeed";
import { PlaceholderPage } from "./PlaceholderPage";
import { ProfileView } from "./ProfileView";
import { AppTopbar } from "./AppTopbar";
import { PublicLibraryView } from "./PublicLibraryView";
import { PublicSnippetView } from "./PublicSnippetView";
import { WorkspaceScreen } from "../../workspace/components/WorkspaceScreen";

type AppScreenProps = {
  copy: CopyBlock;
  user: CurrentUser;
  libraries: LibraryRecord[];
  snippets: SnippetDetail[];
  recentSnippets: SnippetDetail[];
  featuredSnippets: CommunitySnippet[];
  communitySnippets: CommunitySnippet[];
  publicLibraries: PublicLibraryProfile[];
  selectedLibrary: PublicLibraryProfile | null;
  selectedSnippet: CommunitySnippet | null;
  activeView: AppView;
  setActiveView: Dispatch<SetStateAction<AppView>>;
  setSelectedLibraryId: Dispatch<SetStateAction<string | null>>;
  setSelectedSnippetId: Dispatch<SetStateAction<string | null>>;
  snippetSourceView: "home" | "explore" | "library";
  setSnippetSourceView: Dispatch<SetStateAction<"home" | "explore" | "library">>;
  workspaceLoading: boolean;
  locale: Locale;
  setLocale: Dispatch<SetStateAction<Locale>>;
  onLogout: () => void;
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

export function AppScreen({
  copy,
  user,
  libraries,
  snippets,
  recentSnippets,
  featuredSnippets,
  communitySnippets,
  publicLibraries,
  selectedLibrary,
  selectedSnippet,
  activeView,
  setActiveView,
  setSelectedLibraryId,
  setSelectedSnippetId,
  snippetSourceView,
  setSnippetSourceView,
  workspaceLoading,
  locale,
  setLocale,
  onLogout,
  publicAssetCount,
  readyReferences,
  onCreateLibrary,
  onCreateSnippet
}: AppScreenProps) {
  const privateAssetCount =
    libraries.filter((library) => library.visibility !== "public").length +
    snippets.filter((item) => item.snippet.visibility !== "public").length;

  function handleOpenSnippet(
    snippet: CommunitySnippet,
    sourceView: "home" | "explore" | "library"
  ) {
    setSelectedLibraryId(snippet.library);
    setSelectedSnippetId(snippet.id);
    setSnippetSourceView(sourceView);
    setActiveView("snippet");
  }

  function handleOpenLibrary(library: PublicLibraryProfile) {
    setSelectedLibraryId(library.id);
    setActiveView("library");
  }

  return (
    <section className="app-shell">
      <AppTopbar
        copy={copy}
        user={user}
        activeView={activeView}
        setActiveView={setActiveView}
        setLocale={setLocale}
      />

      <div className="app-main">
        {activeView === "home" ? (
          <HomeFeed
            copy={copy}
            featuredSnippets={featuredSnippets}
            onOpenSnippet={(snippet) => handleOpenSnippet(snippet, "home")}
          />
        ) : null}

        {activeView === "explore" ? (
          <ExploreFeed
            copy={copy}
            publicLibraries={publicLibraries}
            onOpenLibrary={handleOpenLibrary}
          />
        ) : null}

        {activeView === "library" && selectedLibrary ? (
          <PublicLibraryView
            copy={copy}
            library={selectedLibrary}
            onOpenSnippet={(snippet) => handleOpenSnippet(snippet, "library")}
          />
        ) : null}

        {activeView === "snippet" && selectedSnippet ? (
          <PublicSnippetView
            copy={copy}
            snippet={selectedSnippet}
          />
        ) : null}

        {activeView === "documents" ? (
          <PlaceholderPage
            eyebrow={copy.pageInProgress}
            title={copy.documentsTitle}
            body={copy.documentsBody}
            copy={copy}
          />
        ) : null}

        {activeView === "forum" ? (
          <PlaceholderPage
            eyebrow={copy.pageInProgress}
            title={copy.forumTitle}
            body={copy.forumBody}
            copy={copy}
          />
        ) : null}

        {activeView === "studio" ? (
          <WorkspaceScreen
            copy={copy}
            libraries={libraries}
            snippets={snippets}
            recentSnippets={recentSnippets}
            workspaceLoading={workspaceLoading}
            locale={locale}
            publicAssetCount={publicAssetCount}
            readyReferences={readyReferences}
            onCreateLibrary={onCreateLibrary}
            onCreateSnippet={onCreateSnippet}
          />
        ) : null}

        {activeView === "profile" ? (
          <ProfileView
            copy={copy}
            user={user}
            privateAssetCount={privateAssetCount}
            publicAssetCount={publicAssetCount}
            onLogout={onLogout}
          />
        ) : null}
      </div>
    </section>
  );
}
