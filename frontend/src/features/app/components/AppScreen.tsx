import type { Dispatch, SetStateAction } from "react";

import type {
  AppView,
  CommunitySnippet,
  CopyBlock,
  CurrentUser,
  LibraryRecord,
  Locale,
  SnippetDetail
} from "../../../lib/app-types";
import { ExploreFeed } from "./ExploreFeed";
import { HomeFeed } from "./HomeFeed";
import { PlaceholderPage } from "./PlaceholderPage";
import { ProfileView } from "./ProfileView";
import { AppTopbar } from "./AppTopbar";
import { WorkspaceScreen } from "../../workspace/components/WorkspaceScreen";

type AppScreenProps = {
  copy: CopyBlock;
  user: CurrentUser;
  libraries: LibraryRecord[];
  snippets: SnippetDetail[];
  recentSnippets: SnippetDetail[];
  featuredSnippets: CommunitySnippet[];
  communitySnippets: CommunitySnippet[];
  activeView: AppView;
  setActiveView: Dispatch<SetStateAction<AppView>>;
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
  activeView,
  setActiveView,
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

  return (
    <section className="app-shell">
      <AppTopbar
        copy={copy}
        user={user}
        activeView={activeView}
        setActiveView={setActiveView}
        setLocale={setLocale}
        onLogout={onLogout}
      />

      <div className="app-main">
        {activeView === "home" ? (
          <HomeFeed copy={copy} featuredSnippets={featuredSnippets} />
        ) : null}

        {activeView === "explore" ? (
          <ExploreFeed copy={copy} communitySnippets={communitySnippets} />
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
          />
        ) : null}
      </div>
    </section>
  );
}
