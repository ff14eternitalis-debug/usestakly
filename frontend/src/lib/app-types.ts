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

export type CopyBlock = {
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
};
