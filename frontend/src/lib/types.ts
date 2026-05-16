export type CurrentUser = {
  id: string;
  email: string;
  username: string;
  displayName: string | null;
  avatarUrl: string | null;
};

export type SearchFilter = "auto" | "strict" | "explore";
export type RepoSort = "score" | "stars" | "recency" | "abandonment" | "trend";

export type QualityContext = {
  freshness: number | null;
  adoption: number | null;
  reliability: number | null;
  abandonment: number | null;
  vitality: number | null;
  overall: number | null;
  resolveCount: number;
  buildSuccessCount: number;
  buildFailureCount: number;
  regretCount: number;
  flags: string[];
  formulaVersion: string | null;
  computedAt: string | null;
};

export type RepoCategory = {
  category: string;
  confidence: number;
  source: string;
  evidence: unknown;
};

export type RepoRadarSnapshot = {
  maturityBand: "established" | "emerging" | "experimental" | "stale" | "noisy";
  radarRelevance: number;
  trendSignal: number;
  explanation: unknown;
};

export type RecommendationExplanation = {
  includedBecause: string[];
  caveats: string[];
};

export type ScoreSnapshot = {
  formulaVersion: string;
  overall: number | null;
  freshness: number | null;
  adoption: number | null;
  reliability: number | null;
  abandonment: number | null;
  vitality: number | null;
  computedAt: string;
  previousFormulaVersion: string | null;
  previousOverall: number | null;
};

export type SearchFilterSummary = {
  messageCode: string | null;
};

export type VitalityInputs = {
  structuralSignalsAt: string | null;
  distinctContributors90d: number | null;
  commits30d: number | null;
  hasCi: boolean | null;
  releasesCount: number | null;
  lastReleaseAt: string | null;
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
  categories: RepoCategory[];
  radar: RepoRadarSnapshot | null;
  recommendationExplanation?: RecommendationExplanation | null;
};

export type RepoSignalEvent = {
  eventKind: string;
  note: string | null;
  createdAt: string;
};

export type RepoSignal = {
  id: string;
  signal: string;
  isPassive: boolean;
  evidenceUrl: string | null;
  evidenceDescription: string | null;
  reviewStatus: string;
  reviewNote: string | null;
  disputedAt: string | null;
  disputeReason: string | null;
  createdAt: string;
  events: RepoSignalEvent[];
};

export type RepoProfile = RepoSearchResult & {
  subscribersCount: number;
  defaultBranch: string | null;
  priorsFetchedAt: string | null;
  vitalityInputs: VitalityInputs;
  recentSignals: RepoSignal[];
  scoreSnapshot?: ScoreSnapshot | null;
};

export type RepoViewerState = {
  canDisputeSignals: boolean;
  visibleSignals: RepoSignal[];
};

export type RepoSearchResponse = {
  filter: SearchFilter;
  sort: RepoSort;
  limit: number;
  offset: number;
  count: number;
  hasMore: boolean;
  items: RepoSearchResult[];
  filterSummary?: SearchFilterSummary | null;
};

export type IntentConfidence = "high" | "medium" | "low";

export type UseCaseIntent = {
  label: string;
  confidence: IntentConfidence;
  categories: string[];
  topics: string[];
  languages: string[];
};

export type UseCaseRecommendation = RepoSearchResult & {
  matchScore: number;
  recommendationScore: number;
  risk: "low" | "medium" | "high" | string;
  reason: string;
  matchedTopics: string[];
};

export type UseCaseRecommendationReport = {
  query: string;
  riskTolerance: string;
  intent: UseCaseIntent;
  recommendations: UseCaseRecommendation[];
  fallbackCandidates: string[];
};

export type UseCaseWatchMatch = {
  artifactId: string;
  fullName: string;
  language: string | null;
  matchScore: number;
  qualityScore: number | null;
};

export type UseCaseWatch = {
  id: string;
  queryText: string;
  label: string;
  normalizedIntent: string;
  categories: string[];
  topics: string[];
  languages: string[];
  riskTolerance: string;
  enabled: boolean;
  matchCount: number;
  topMatches: UseCaseWatchMatch[];
  createdAt: string;
};

export type CreateUseCaseWatchResponse = {
  watchId: string;
  watch: UseCaseWatch;
};

export type AddRepoResponse = {
  artifactId: string;
  alreadyIndexed: boolean;
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
  subscribersCount: number;
  archived: boolean;
  defaultBranch: string | null;
  lastCommitAt: string | null;
  formulaVersion: string;
  categories: RepoCategory[];
};

export type AgentTokenSummary = {
  id: string;
  label: string;
  createdAt: string;
  lastUsedAt: string | null;
};

export type AgentTokenCreated = AgentTokenSummary & {
  token: string;
};

export type NotificationChannelType = "email" | "discord_webhook";

export type NotificationChannelSummary = {
  id: string;
  channelType: NotificationChannelType;
  label: string;
  destination: string;
  enabled: boolean;
  criticalAlertsEnabled: boolean;
  dailyDigestEnabled: boolean;
  lastTestedAt: string | null;
  lastError: string | null;
  createdAt: string;
};

export type DigestTimePreset = "morning" | "noon" | "evening" | "night";

export type NotificationPreferences = {
  digestTimePreset: DigestTimePreset;
  digestTimeLocal: string;
  timezone: string;
  emailLocale: "en";
};

export type UserReputationSummary = {
  userId: string;
  score: number;
  tier: string;
  accountAgeDays: number;
  passiveSignalCount: number;
  resolveCount: number;
  reResolveCount: number;
  buildSuccessCount: number;
  buildFailureCount: number;
  regretCount: number;
  usageSignalCount: number;
  successfulOutcomeRatio: number;
  buildReliabilityRatio: number;
  regretRatio: number;
  activeSignalEligible: boolean;
};

export type AccountSummary = {
  reputation: UserReputationSummary;
  activeSignalMinReputation: number;
  activeSignalDefaultConsensus: number;
  activeSignalSevereConsensus: number;
};

export type PendingRepoSignal = {
  id: string;
  repoId: string;
  owner: string;
  name: string;
  signal: string;
  reviewStatus: string;
  actorUserId: string | null;
  disputedByUserId: string | null;
  evidenceUrl: string | null;
  evidenceDescription: string | null;
  createdAt: string;
  reporterScore: number | null;
  reporterTier: string | null;
  reporterUsageSignalCount: number | null;
  ownerDisputeScore: number | null;
  ownerDisputeTier: string | null;
  ownerDisputeUsageSignalCount: number | null;
  hasOwnerDispute: boolean;
  needsStrictReview: boolean;
  suggestedAction: string;
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
  | "flag_severe"
  | "use_case_new_candidate"
  | "use_case_best_candidate_changed"
  | "use_case_quality_drop"
  | "use_case_flag_added";

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

export type McpMetricsWindow = "24h" | "7d" | "30d";

export type McpMetricsTotals = {
  logUsage: number;
  watchRepo: number;
  rejections: number;
  distinctTokens: number;
  distinctUsers: number;
  distinctRepos: number;
};

export type McpOutcomeBucket = {
  outcome: string;
  count: number;
};

export type McpRejectionBucket = {
  tool: string;
  reason: string;
  count: number;
};

export type McpRepoVolume = {
  owner: string;
  name: string;
  logUsage: number;
  watchRepo: number;
  rejections: number;
};

export type McpUserVolume = {
  userId: string;
  logUsage: number;
  watchRepo: number;
  rejections: number;
};

export type McpDailyBucket = {
  bucket: string;
  logUsage: number;
  watchRepo: number;
  rejections: number;
};

export type McpMetricsReport = {
  window: McpMetricsWindow;
  totals: McpMetricsTotals;
  outcomeDistribution: McpOutcomeBucket[];
  rejectionBreakdown: McpRejectionBucket[];
  topRepos: McpRepoVolume[];
  topUsers: McpUserVolume[];
  dailyVolume: McpDailyBucket[];
};

export type PublicStatus = {
  status: "ok" | "degraded";
  api: { status: "ok" | "down" | "degraded" };
  database: { status: "ok" | "down" | "degraded" };
  registry: { status: "ok" | "down" | "degraded"; repoCount: number };
  mcp: { status: "ok" | "down" | "degraded"; tools: string[] };
  formula: { status: "ok" | "down" | "degraded"; version: string };
  checkedAt: string;
};
