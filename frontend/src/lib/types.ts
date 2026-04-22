export type CurrentUser = {
  id: string;
  email: string;
  username: string;
  displayName: string | null;
  avatarUrl: string | null;
};

export type SearchFilter = "auto" | "strict" | "explore";

export type QualityContext = {
  freshness: number | null;
  adoption: number | null;
  reliability: number | null;
  abandonment: number | null;
  overall: number | null;
  flags: string[];
  formulaVersion: string | null;
  computedAt: string | null;
};

export type RepoSearchResult = {
  artifactId: string;
  owner: string;
  name: string;
  fullName: string;
  htmlUrl: string;
  description: string | null;
  language: string | null;
  licenseSpdx: string | null;
  topics: string[];
  starsCount: number;
  forksCount: number;
  openIssuesCount: number;
  archived: boolean;
  lastCommitAt: string | null;
  quality: QualityContext | null;
};

export type RepoSignal = {
  signal: string;
  isPassive: boolean;
  evidenceUrl: string | null;
  evidenceDescription: string | null;
  createdAt: string;
};

export type RepoProfile = RepoSearchResult & {
  subscribersCount: number;
  defaultBranch: string | null;
  priorsFetchedAt: string | null;
  recentSignals: RepoSignal[];
};

export type RepoSearchResponse = {
  filter: SearchFilter;
  items: RepoSearchResult[];
};

export type WatchedRepo = {
  id: string;
  artifactId: string;
  owner: string;
  name: string;
  fullName: string;
  htmlUrl: string;
  language: string | null;
  starsCount: number;
  archived: boolean;
  lastCommitAt: string | null;
  muted: boolean;
  watchedAt: string;
  overall: number | null;
  abandonment: number | null;
  flags: string[];
};

export type NotificationKind =
  | "score_drop"
  | "abandonment_up"
  | "flag_added"
  | "flag_severe";

export type Notification = {
  id: string;
  artifactId: string;
  owner: string | null;
  name: string | null;
  kind: NotificationKind;
  payload: Record<string, unknown>;
  createdAt: string;
  readAt: string | null;
};

export type UnreadCount = {
  unread: number;
};
