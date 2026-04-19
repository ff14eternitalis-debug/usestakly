export type CurrentUser = {
  id: string;
  email: string;
  username: string;
  displayName: string | null;
  avatarUrl: string | null;
};

export type LibraryRecord = {
  id: string;
  slug: string;
  name: string;
  description: string | null;
  visibility: string;
  trustLevel: string;
  isDefault: boolean;
};

export type SnippetVersionRecord = {
  id: string;
  version: string;
  riskLevel: string;
  code: string;
};

export type SnippetRecord = {
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

export type SnippetDetail = {
  snippet: SnippetRecord;
  currentVersion: SnippetVersionRecord | null;
  tags: string[];
  canonicalReference: string;
};

export type LibraryListResponse = {
  items: LibraryRecord[];
};

export type SnippetListResponse = {
  items: SnippetDetail[];
};

export type Locale = "en" | "fr";

export type AppView =
  | "home"
  | "explore"
  | "library"
  | "documents"
  | "forum"
  | "studio"
  | "profile"
  | "snippet";

export type CommunitySnippetPreviewKind = "button" | "backend" | "database";

export type SnippetFileLanguage =
  | "html"
  | "css"
  | "js"
  | "ts"
  | "jsx"
  | "tsx"
  | "rust"
  | "sql"
  | "bash"
  | "yaml"
  | "json"
  | "plaintext";

export type SnippetFile = {
  id: string;
  label: string;
  language: SnippetFileLanguage;
  content: string;
};

export type CommunitySnippet = {
  id: string;
  title: string;
  description: string;
  fullDescription: string;
  author: string;
  authorName: string;
  library: string;
  libraryName: string;
  language: string;
  framework: string | null;
  domain: string;
  appreciation: number;
  saves: number;
  canonicalReference: string;
  scope: "community" | "private";
  files: SnippetFile[];
  primaryFileId?: string;
  previewKind: CommunitySnippetPreviewKind;
  previewLabel: string;
  previewNote: string;
  previewActionLabel?: string;
};

export type PublicLibraryProfile = {
  id: string;
  author: string;
  authorName: string;
  library: string;
  libraryName: string;
  bio: string;
  snippetCount: number;
  languages: string[];
  domains: string[];
  snippets: CommunitySnippet[];
};

export type CopyBlock = {
  authEyebrow: string;
  authTitle: string;
  authBody: string;
  authGitHubButton: string;
  authDiscordButton: string;
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
  createLibraryTitle: string;
  createLibraryName: string;
  createLibrarySlug: string;
  createLibraryDescription: string;
  createLibraryVisibility: string;
  createLibrarySubmit: string;
  createSnippetTitle: string;
  createSnippetLibrary: string;
  createSnippetName: string;
  createSnippetSlug: string;
  createSnippetDomain: string;
  createSnippetKind: string;
  createSnippetCategory: string;
  createSnippetLanguage: string;
  createSnippetFramework: string;
  createSnippetVersion: string;
  createSnippetTags: string;
  createSnippetCode: string;
  createSnippetSubmit: string;
  detailTitle: string;
  detailEmpty: string;
  detailDescription: string;
  detailCode: string;
  detailLibrary: string;
  detailRisk: string;
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
  navHome: string;
  navExplore: string;
  navLibrary: string;
  navDocuments: string;
  navForum: string;
  navStudio: string;
  navProfile: string;
  appEyebrow: string;
  homeTitle: string;
  homeBody: string;
  homeFeaturedTitle: string;
  homeFeaturedBody: string;
  homeSecondaryTitle: string;
  homeSecondaryBody: string;
  homeTrendingLabel: string;
  homeSavedLabel: string;
  homeReferenceLabel: string;
  homeScopeCommunity: string;
  homeScopePrivate: string;
  homeEmpty: string;
  homeOpenSnippet: string;
  exploreTitle: string;
  exploreBody: string;
  exploreEmpty: string;
  exploreOpenLibrary: string;
  libraryBack: string;
  libraryTitleSuffix: string;
  libraryBody: string;
  libraryFilterLanguage: string;
  libraryFilterDomain: string;
  libraryAll: string;
  libraryEmpty: string;
  snippetBack: string;
  snippetReadonly: string;
  snippetRenderTitle: string;
  snippetRenderBody: string;
  snippetSummaryTitle: string;
  snippetCodeTitle: string;
  snippetStackLabel: string;
  snippetAuthorLabel: string;
  snippetLibraryLabel: string;
  snippetOpenSourceLabel: string;
  documentsTitle: string;
  documentsBody: string;
  forumTitle: string;
  forumBody: string;
  pageInProgress: string;
  studioTitle: string;
  studioBody: string;
  profileTitle: string;
  profileBody: string;
  profileIdentity: string;
  profileEmail: string;
  profileHandle: string;
  profilePresence: string;
  profilePrivateLabel: string;
  profilePublicLabel: string;
};
