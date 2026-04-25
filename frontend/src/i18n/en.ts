export const en = {
  nav: {
    discover: "Discover",
    howToRead: "How to read",
    mcpGuide: "MCP guide",
    watchlist: "Watchlist",
    notifications: "Notifications",
    account: "Account",
    signIn: "Sign in",
    signOut: "sign out"
  },
  common: {
    offline: "Observatory offline",
    offlineHint: "Run",
    offlineFrom: "from",
    backendDir: "backend/",
    cargoRun: "cargo run",
    tuning: "Tuning the instruments…",
    noMatch: "No match",
    browseWithoutSignIn: "Browse without signing in",
    viewOnGithub: "View on GitHub",
    github: "github",
    github2: "GitHub",
    arrowNext: "→",
    readyStatus: "ready",
    checkingStatus: "checking",
    observingStatus: "observing"
  },
  header: {
    signIn: "Sign in"
  },
  footer: {
    tagline:
      "A quality-scored observatory of public open-source repositories. Scoring formula is public, versioned, local.",
    product: "Product",
    signals: "Signals",
    about: "About",
    mcp: "MCP",
    freshness: "Freshness",
    adoption: "Adoption",
    reliability: "Reliability",
    abandonment: "Abandonment",
    selfHosted: "Self-hosted",
    publicFormula: "Public formula",
    localEmbeddings: "Local embeddings",
    copyright: "© {year} UseStakly",
    tagFormula: "formula_v1 · transparent by design"
  },
  landing: {
    eyebrow: "Open-source observatory · v0.1",
    h1Part1: "GitHub stars measure interest.",
    h1Part2: "We measure what matters.",
    intro:
      "UseStakly scores public repositories on four signals — freshness, adoption, reliability, abandonment — and alerts you when a project you depend on starts to drift. Transparent formula, versioned, self-hosted.",
    openObservatory: "Open the observatory",
    signInForWatchlist: "Sign in for watchlist",
    myWatchlist: "My watchlist",
    kpi1: "Signals scored",
    kpi2: "Public formula",
    kpi3: "Black box",
    panelLive: "Live verdict",
    panelSample: "sample · facebook/react",
    panelOverall: "Overall",
    pillarsEyebrow: "What it does",
    pillarsH2: "Two tools, one formula, zero black-box.",
    pillar: "Pillar",
    pillar1Title: "Discovery, scored by usage.",
    pillar1Body:
      "Each repo is measured against a transparent formula combining commit cadence, adoption, build reliability, and abandonment signals. Three modes — auto, strict, explore — same scoring, different thresholds.",
    pillar1Artifact: "filter modes",
    pillar1Cta: "Try discover",
    pillar2Title: "Watchlist, real alerts.",
    pillar2Body:
      "Pin the repos you depend on. We diff scores between recomputes and raise in-app notifications when abandonment rises, a severe flag lands, or overall quality drops. No pull-request RSS, no silence.",
    pillar2Artifact: "triggers",
    pillar2Cta: "Open watchlist",
    formulaEyebrow: "formula_v1.toml",
    formulaH2: "The score is a statement, not a black box.",
    formulaBody:
      "Each dimension is a named equation with a known half-life or threshold. Every score carries the formula version that produced it, so v2 never rewrites yesterday's verdict.",
    previewEyebrow: "From the register",
    previewH2: "Live snapshots from the observatory.",
    previewSeeAll: "See all entries",
    tickerTuning: "─── tuning ─── tuning ─── tuning ───",
    closingEyebrow: "Keep a short list",
    closingH2Part1: "Pin the repos you depend on.",
    closingH2Part2: "We'll keep watch.",
    closingBrowse: "Browse repositories",
    closingWatchlist: "Open watchlist",
    closingStart: "Get started"
  },
  discover: {
    eyebrow: "Discover",
    h1Part1: "What are you",
    h1Accent: "measuring",
    h1Part2: "today?",
    intro:
      "Search the corpus by name, owner, description or topic. Narrow by language, minimum stars, or confidence. Same formula, different thresholds.",
    scoreGuideTitle: "Read the score before the stars",
    scoreGuideBody:
      "Overall combines freshness, adoption, reliability and abandonment risk into a 0-1 verdict. It is a dependency signal, not a popularity chart.",
    scoreGuideAction: "How to read UseStakly",
    corpusTitle: "Initial corpus",
    corpusBody:
      "The MVP starts with a curated set of credible public OSS repos: active references, deprecated counterexamples, and tooling agents often recommend. Add any GitHub repo to score it on demand.",
    queryLabel: "Query",
    queryPlaceholder: "e.g. date picker, orm, htmx, zustand",
    modeLabel: "Mode",
    modeExplore: "Explore",
    modeAuto: "Auto",
    modeStrict: "Strict",
    hintExplore: "Everything with its receipts — no filter.",
    hintAuto: "Keeps scored repos above the floor. Hides broken and severe-risk entries.",
    hintStrict: "Zero flags, fresh enough, low abandonment, higher overall bar.",
    languageLabel: "Language",
    languageAny: "any",
    starsMinLabel: "Stars min",
    starsMinPlaceholder: "0",
    hintLabel: "Hint",
    addRepoLabel: "Add GitHub repo",
    addRepoPlaceholder: "owner/repo or https://github.com/owner/repo",
    addRepoAction: "Add repo",
    addRepoPending: "Adding…",
    addRepoHelp: "Paste a GitHub URL or owner/repo to add it to the observatory now.",
    addRepoSuccess: "Repo added to the registry:",
    addRepoExists: "Repo already indexed. Refreshed its metadata and score:",
    addRepoOpen: "Open profile",
    measuring: "measuring…",
    entriesSingle: "entry",
    entriesPlural: "entries",
    sortedBy: "sorted by overall · stars · recency",
    tryWidening: "Try widening to",
    exploreLink: "explore",
    orLoweringStars: ", or lowering the stars floor."
  },
  repoDetail: {
    back: "Discover",
    formula: "formula",
    computed: "computed",
    loading: "Pulling the file…",
    notFound: "Not in the register",
    notFoundBody: "No profile exists under this identifier.",
    offlineBody: "The backend didn't answer.",
    backToDiscover: "Back to discover",
    addToWatchlist: "Add to watchlist",
    adding: "Adding…",
    unwatch: "Unwatch",
    unwatching: "Unwatching…",
    signInToWatch: "Sign in to watch this repo",
    signInToWatchHint:
      "Get alerts when the score drops, abandonment rises, or a severe flag lands.",
    overallVerdict: "Overall verdict",
    healthy: "Healthy",
    monitor: "Monitor",
    atRisk: "At risk",
    unscored: "Unscored",
    stars: "Stars",
    forks: "Forks",
    openIssues: "Open issues",
    subscribers: "Subscribers",
    lastCommit: "Last commit",
    priorsFetched: "Priors fetched",
    defaultBranch: "Default branch",
    dimensions: "Dimensions",
    freshness: "Freshness",
    adoption: "Adoption",
    reliability: "Reliability",
    abandonment: "Abandonment",
    freshnessHint: "Exponential decay on last_commit_at (half-life 180d).",
    adoptionHint: "Log-normalised resolve count (saturates at 1k).",
    reliabilityHint: "Success / total builds. Neutral 0.5 before 5 samples.",
    abandonmentHint: "Inverse freshness plus regret bump above threshold.",
    scoreGuideTitle: "How to read this score",
    scoreGuideBody:
      "Use the overall verdict as a first pass, then inspect the dimensions. A strong repo can still need monitoring if freshness decays or abandonment risk rises.",
    scoreGuideAction: "Read the full guide",
    scoreGuideItems: [
      "Freshness and reliability are the quickest risk checks before adopting a dependency.",
      "Adoption is capped so large projects do not win only because they are famous.",
      "Abandonment is a risk score: lower is better, and high values pull the overall verdict down."
    ],
    recentSignals: "Recent signals",
    entrySingle: "entry",
    entriesPlural: "entries",
    noSignals: "No signals reported yet. The observatory is listening.",
    passive: "passive",
    reported: "reported"
  },
  watchlist: {
    eyebrow: "Watchlist",
    h1Part1: "The short list,",
    h1Accent: "under observation.",
    intro: "We diff scores between recomputes. If a repo drifts, you'll see it in",
    notifications: "notifications",
    loading: "Pulling the file…",
    loadErrorTitle: "Watchlist unavailable.",
    loadErrorBody:
      "We could not load your watched repositories. Your list is still there; retry once the session or network is back.",
    retry: "retry",
    emptyTitle: "Nothing on watch yet.",
    emptyBody:
      "Open a repo's profile from the register and tap Add to watchlist. You'll be pinged here when a score drops, abandonment rises, or a severe flag lands.",
    emptyAction: "Open discover",
    watched: "watched",
    overall: "Overall",
    mute: "mute",
    unmute: "unmute",
    remove: "remove",
    removing: "removing…",
    confirmRemove: "confirm remove",
    cancelRemove: "cancel"
  },
  notifications: {
    eyebrow: "Notifications",
    h1Part1: "What's moved",
    h1Accent: "since last you looked.",
    unreadOnly: "Unread only",
    markAllRead: "mark all read",
    markRead: "mark read",
    loading: "Sorting the mail…",
    loadErrorTitle: "Notifications unavailable.",
    loadErrorBody:
      "We could not load your notifications. Retry once the session or network is back.",
    retry: "retry",
    emptyTitle: "All quiet on the register.",
    emptyBodyUnread:
      "Nothing to report unread. Add repositories to your {watchlistLink} so the observatory can flag drift for you.",
    emptyBodyRecent:
      "Nothing to report recently. Add repositories to your {watchlistLink} so the observatory can flag drift for you.",
    watchlist: "watchlist",
    watchlistAction: "Open watchlist",
    labelScoreDrop: "score drop",
    labelAbandonmentUp: "abandonment rising",
    labelFlagAdded: "new flag",
    labelFlagSevere: "severe flag",
    markingRead: "marking…"
  },
  login: {
    eyebrow: "Sign in",
    h1Part1: "Sign in to the",
    h1Accent: "observatory.",
    body:
      "A session is required to keep a watchlist, flag a repo with evidence, or connect an MCP agent. Reading the register is open — no account needed.",
    browseWithoutSignIn: "Browse without signing in",
    continueGithub: "Continue with GitHub",
    continueDiscord: "Continue with Discord",
    privacy:
      "No emails are sent, no marketing list. OAuth is the entire handshake — we learn your username and avatar, nothing more."
  },
  mcpGuide: {
    eyebrow: "MCP guide",
    h1: "Install UseStakly in your agent",
    intro:
      "Connect an MCP-capable coding agent to the same quality-scored GitHub registry used by the web app. Create one token per agent, paste the Streamable HTTP config, then ask for scored repo recommendations with provenance.",
    createTokenAction: "Create MCP token",
    createTokenHint:
      "Tokens live in Account, are shown once, and can be revoked without touching your login session.",
    endpointLabel: "Server endpoint",
    endpointBody:
      "Use this URL in clients that support MCP Streamable HTTP. Send the token as a Bearer credential on every request.",
    stepsLabel: "Install flow",
    steps: [
      {
        title: "Sign in and create a token",
        body:
          "Open Account, choose a label such as codex-local or claude-desktop, then create a token. Copy the plaintext value immediately."
      },
      {
        title: "Add UseStakly to your MCP client",
        body:
          "Paste the endpoint and Authorization header into the client configuration. Keep one token per machine or agent so revocation stays precise."
      },
      {
        title: "Restart the client and test a search",
        body:
          "Ask your agent to search UseStakly for a repo category, then inspect the returned score, formula version, and provenance."
      }
    ],
    clientConfigLabel: "Client config",
    clientConfigTitle: "Streamable HTTP configuration",
    clientConfigBody:
      "Client schemas vary, but the required fields are stable: type Streamable HTTP, the /mcp URL, and an Authorization Bearer header.",
    smokeTestLabel: "Smoke test",
    smokeTestTitle: "Check the transport before wiring an agent",
    smokeTestBody:
      "This initialize request should return an MCP response. If it fails, verify the token prefix, endpoint URL, and that the backend is reachable.",
    toolsLabel: "Available tools",
    toolsTitle: "What the agent can do",
    toolsBody:
      "Read tools are safe for recommendations. Write tools attach usage signals or watchlist entries to the user who owns the token.",
    tools: [
      {
        name: "search_github_repos",
        body:
          "Searches the scored registry by query, filter mode, language, stars floor, and limit."
      },
      {
        name: "get_repo_quality_context",
        body:
          "Returns the full repo quality profile: dimensions, flags, recent signals, formula version, and provenance."
      },
      {
        name: "log_usage",
        body:
          "Records a passive usage outcome such as build_success, build_failure, regret, resolve, or re_resolve."
      },
      {
        name: "watch_repo",
        body:
          "Adds a repo to the token owner's watchlist so UseStakly can alert on future drift."
      }
    ],
    securityLabel: "Security",
    securityTitle: "Token handling",
    securityItems: [
      "Tokens use the format usk_<64 hex> and are stored hashed server-side.",
      "Plaintext is shown once at creation. Store it in the MCP client, not in screenshots or shared docs.",
      "Revoke old tokens from Account when a machine, client, or teammate no longer needs access.",
      "Write tools are rate-limited per token and guarded against duplicate or repeated negative signals."
    ]
  },
  howToRead: {
    eyebrow: "Reading guide",
    h1: "How to read UseStakly",
    intro:
      "UseStakly is built for dependency decisions. The score helps you compare public GitHub repos by maintenance, usage confidence and risk, without treating stars as the final answer.",
    scoreLabel: "Score",
    scoreTitle: "Overall is a 0-1 dependency verdict",
    scoreBody:
      "A score near 1 means the repo currently looks healthy for adoption. A score near 0 means the repo needs investigation or should usually be avoided. The value is always tied to a formula version and a computed date.",
    dimensionsLabel: "Dimensions",
    dimensions: [
      {
        name: "Freshness",
        body:
          "Looks at recent repository activity. Old last commits decay over time, so a famous but quiet repo can lose confidence."
      },
      {
        name: "Adoption",
        body:
          "Measures usage and resolution signals, then caps the effect so popularity does not drown out quality."
      },
      {
        name: "Reliability",
        body:
          "Tracks positive versus failed usage outcomes. It stays neutral until there are enough samples."
      },
      {
        name: "Abandonment",
        body:
          "Estimates risk. Lower is better. High abandonment can pull down an otherwise popular repo."
      }
    ],
    modesLabel: "Modes",
    modesTitle: "Same formula, different thresholds",
    modes: [
      {
        name: "Explore",
        body: "Shows everything with receipts. Useful for audits and weak-signal research."
      },
      {
        name: "Auto",
        body: "Default shortlist. Hides broken or severe-risk entries while keeping discovery broad."
      },
      {
        name: "Strict",
        body: "Requires a cleaner profile: no accepted severe flags, better freshness and a higher overall bar."
      }
    ],
    corpusLabel: "Corpus",
    corpusTitle: "The MVP corpus is curated, then grows on demand",
    corpusBody:
      "The initial seed mixes active references across JS/TS, Rust, Python and Go with deprecated examples such as request and maintenance-mode examples such as moment. This makes demos honest: good repos score well, stale repos have to explain themselves.",
    corpusItems: [
      "Seed repos are public GitHub projects ingested through the same scoring pipeline.",
      "Any repo can be added from Discover with owner/repo or a GitHub URL.",
      "Watchlist refresh and MCP usage signals make the corpus more useful over time."
    ],
    workflowLabel: "Workflow",
    workflowTitle: "A practical reading order",
    workflowItems: [
      "Start in Auto mode and search the category you need.",
      "Compare Overall, then open the repo detail for dimensions and flags.",
      "Treat high Abandonment or low Freshness as a reason to inspect before adopting.",
      "Add real dependencies to the watchlist so drift becomes visible later."
    ],
    ctaDiscover: "Open discover",
    ctaMcp: "Install MCP"
  },
  account: {
    eyebrow: "Account",
    h1Part1: "Agent tokens,",
    h1Accent: "under control.",
    intro:
      "Create MCP tokens for your coding agents, revoke the ones you no longer trust, and keep write access constrained. Tokens are shown in plaintext once.",
    tokenLabel: "New token label",
    tokenPlaceholder: "e.g. claude-desktop, cursor, codex",
    create: "Create token",
    creating: "Creating…",
    activeTokens: "Active MCP tokens",
    emptyTitle: "No MCP token yet.",
    emptyBody:
      "Create one token per agent or machine so revocation stays surgical. All write tools are rate-limited per token.",
    createdNow: "Created just now",
    lastUsedNever: "Never used",
    lastUsed: "Last used",
    createdAt: "Created",
    revoke: "revoke",
    revoking: "revoking…",
    tokenShownOnce: "Plaintext token",
    tokenShownOnceHint: "This value is shown once. Store it in your MCP client now.",
    copy: "copy",
    copied: "copied",
    quotaTitle: "Write safety",
    quotaBody:
      "MCP write tools are limited per token, duplicate log_usage calls are throttled, and repeated negative outcomes are cooled down to reduce poisoning.",
    reputation: "Reputation",
    tier: "Tier",
    passiveSignals: "Passive signals",
    usageSignals: "Usage signals",
    successRatio: "Positive ratio",
    buildReliability: "Build reliability",
    regretRatio: "Regret ratio",
    eligibility: "Active signals",
    eligible: "eligible",
    notEligible: "not yet eligible",
    adminTitle: "Moderation queue",
    adminTokenLabel: "Admin token",
    adminTokenPlaceholder: "Paste x-admin-token",
    adminLoad: "Load pending",
    adminApprove: "approve",
    adminReject: "reject",
    adminEmpty: "No pending repo signal.",
    adminReviewing: "reviewing…",
    mcpObservabilityTitle: "MCP observability",
    mcpObservabilityIntro:
      "Aggregated view of agent_token_events: log_usage, watch_repo and guard rejection volume over the chosen window.",
    mcpWindowLabel: "Window",
    mcpWindow24h: "24h",
    mcpWindow7d: "7d",
    mcpWindow30d: "30d",
    mcpLoading: "Loading metrics…",
    mcpTotalLogUsage: "log_usage",
    mcpTotalWatchRepo: "watch_repo",
    mcpTotalRejections: "Guard rejections",
    mcpDistinctTokens: "Distinct tokens",
    mcpDistinctUsers: "Distinct users",
    mcpDistinctRepos: "Repos touched",
    mcpOutcomeTitle: "log_usage outcome distribution",
    mcpRejectionTitle: "Rejections by reason",
    mcpTopReposTitle: "Top repos",
    mcpTopUsersTitle: "Top users",
    mcpDailyTitle: "Daily volume",
    mcpEmpty: "No MCP activity in this window."
  },
  signals: {
    title: "Report a signal",
    hint:
      "Active signals require evidence and enough reputation. Severe flags only surface publicly once multiple trusted users agree.",
    signalLabel: "Signal",
    evidenceUrlLabel: "Evidence URL",
    evidenceDescriptionLabel: "Evidence summary",
    submit: "Submit signal",
    submitting: "Submitting…",
    success: "Signal recorded. Public flags update only after trusted consensus.",
    ownerTitle: "Owner review",
    ownerHint:
      "If this repo belongs to your GitHub account, you can dispute a pending or accepted active signal here.",
    disputeReasonLabel: "Dispute reason",
    dispute: "Dispute signal",
    disputing: "Disputing…",
    disputed: "Signal disputed. It now waits for review.",
    status: "status"
  }
} as const;

type Loose<T> = T extends string
  ? string
  : T extends readonly (infer U)[]
    ? ReadonlyArray<Loose<U>>
  : { -readonly [K in keyof T]: Loose<T[K]> };

export type Dict = Loose<typeof en>;
