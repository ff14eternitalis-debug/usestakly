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
    privacy: "Privacy",
    status: "Status",
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
    eyebrow: "Public beta · formula_v1",
    h1Part1: "UseStakly",
    h1Part2: "Choose GitHub OSS with a transparent quality score.",
    intro:
      "UseStakly helps developers and coding agents compare public GitHub repositories with visible scoring, provenance, and watchlist alerts.",
    openObservatory: "Explore repositories",
    readGuide: "Read UseStakly",
    signInForWatchlist: "Sign in for watchlist",
    myWatchlist: "My watchlist",
    kpi1: "Signals scored",
    kpi2: "Public formula",
    kpi3: "Black box",
    panelLive: "Live verdict",
    panelSample: "sample · facebook/react",
    panelOverall: "Overall",
    dataEyebrow: "Data quality",
    dataH2: "Real metadata, progressive usage signals.",
    dataBody:
      "GitHub metadata is fetched during ingestion. Adoption and reliability become stronger as MCP agents and users report real outcomes.",
    dataItems: [
      {
        title: "GitHub metadata is real",
        body:
          "Stars, forks, issues, topics, language, archived state, and last push come from GitHub."
      },
      {
        title: "Usage signals are progressive",
        body:
          "resolve, build_success, build_failure, and regret are recorded through UseStakly signals."
      },
      {
        title: "Beta by design",
        body:
          "The corpus is curated and growing. Scores should be read with provenance, not treated as hidden truth."
      }
    ],
    dataCta: "How to read the score",
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
  privacy: {
    eyebrow: "Privacy",
    h1: "Data UseStakly stores",
    intro:
      "UseStakly keeps the minimum data needed for GitHub repo scoring, watchlists, notifications, and MCP access.",
    sections: [
      {
        title: "OAuth identity",
        body:
          "GitHub or Discord OAuth is used for login. UseStakly stores your user id, username, avatar, and email when the provider returns one. It does not run a marketing mailing list."
      },
      {
        title: "Watchlist and notifications",
        body:
          "Repos you watch and notification read state are stored so the app can alert you when scores drift."
      },
      {
        title: "MCP tokens",
        body:
          "Agent tokens use the usk_ format. Plaintext is shown once, then only a SHA-256 hash is stored server-side."
      },
      {
        title: "Usage signals",
        body:
          "MCP log_usage and user reports store repo owner/name, outcome, timestamp, token owner, and optional notes so scores can improve with real usage."
      }
    ],
    closing:
      "Public repo metadata comes from GitHub. Private source code is not ingested by UseStakly."
  },
  status: {
    eyebrow: "Status",
    h1: "UseStakly service status",
    intro:
      "A lightweight production check for the public beta: API health and registry read path.",
    apiHealth: "API health",
    database: "Database",
    registryRead: "Registry read",
    mcp: "MCP tools",
    formula: "Formula",
    publicStatus: "Public status",
    repos: "repos",
    tools: "tools",
    checking: "Checking",
    online: "Online",
    degraded: "Degraded",
    offline: "Offline",
    lastChecked: "Last checked",
    betaTitle: "Public beta scope",
    betaBody:
      "Coolify health checks cover the running containers. This page adds user-facing API, database, registry, MCP, and formula checks, but it is not a full incident system yet."
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
    intentEyebrow: "Need search",
    intentTitle: "Start from what you want to build",
    intentBody:
      "Describe the tool you need. UseStakly turns the intent into topics, then ranks matching repos with the quality score.",
    intentQueryLabel: "Need",
    intentPlaceholder: "e.g. reliable TypeScript ORM, training video tool",
    intentRiskLabel: "Risk",
    intentRiskLow: "Low",
    intentRiskMedium: "Medium",
    intentRiskHigh: "High",
    intentAction: "Recommend",
    intentSearching: "Searching…",
    intentError: "Use-case recommendation is unavailable right now.",
    intentDetected: "Intent detected",
    intentQuality: "Quality",
    intentMatch: "Match",
    intentStars: "Stars",
    intentNoResult: "No scored repo in the corpus matches this need yet.",
    intentFallback: "Good candidates to add to the corpus:",
    intentWatchAction: "Create watch",
    intentWatchPending: "Creating…",
    intentWatchCreated: "Need watch created.",
    intentWatchError: "Could not create this watch.",
    intentWatchSignIn: "Sign in to watch this need",
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
    scoreMinLabel: "Score min",
    riskMaxLabel: "Risk max",
    topicsLabel: "Topics",
    sortLabel: "Sort",
    sortScore: "Score",
    sortStars: "Stars",
    sortRecency: "Recency",
    sortAbandonment: "Abandonment risk",
    includeArchived: "Include archived",
    clearFilters: "Clear filters",
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
    paginationLabel: "Results pagination",
    pageLabel: "page",
    previousPage: "Previous",
    nextPage: "Next",
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
    vitality: "Vitality",
    freshnessHint: "Exponential decay on last_commit_at (half-life 180d).",
    adoptionHint: "Log-normalised resolve count (saturates at 1k).",
    reliabilityHint: "Success / total builds. Neutral 0.5 before 5 samples.",
    abandonmentHint: "Inverse freshness plus regret bump above threshold; coupled with vitality.",
    vitalityHint:
      "Structural maintainer signals captured at ingestion: contributors, cadence, CI, releases.",
    vitalityCollective: "Distinct contributors (90d)",
    vitalityCadence: "Commits (30d)",
    vitalityCi: "Continuous integration",
    vitalityRelease: "Last release",
    vitalityNotCaptured: "Structural signals not yet captured.",
    ciYes: "Yes",
    ciNo: "No",
    vitalityNeverReleased: "no release",
    scoreGuideTitle: "How to read this score",
    scoreGuideBody:
      "Use the overall verdict as a first pass, then inspect the dimensions. A strong repo can still need monitoring if freshness decays or abandonment risk rises.",
    scoreGuideAction: "Read the full guide",
    scoreGuideItems: [
      "Freshness and reliability are the quickest risk checks before adopting a dependency.",
      "Adoption is capped so large projects do not win only because they are famous.",
      "Abandonment is a risk score: lower is better, and high values pull the overall verdict down."
    ],
    provenanceTitle: "Score provenance",
    provenanceBody:
      "GitHub metadata is live at ingestion time. Usage counts come from UseStakly signals, mostly MCP log_usage events, and stay low until agents or users report real outcomes.",
    githubMetadata: "GitHub metadata",
    usageSignals: "Usage signals",
    freshnessSource: "Freshness source",
    lastCommitSource: "Last GitHub push",
    adoptionSource: "Adoption source",
    reliabilitySource: "Reliability source",
    neutralReliability: "neutral until 5 build samples",
    resolveCount: "resolve",
    buildSuccessCount: "build success",
    buildFailureCount: "build failure",
    regretCount: "regret",
    signalVolumeEmpty:
      "No usage signals yet. Adoption is still empty and reliability stays at its neutral default.",
    signalVolumePartial:
      "Usage signal volume is still thin. Treat the score as directional until more MCP outcomes arrive.",
    signalVolumeReady:
      "Usage signal volume is present. Reliability and adoption are now backed by recorded outcomes.",
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
        "Open a repo profile or create a need watch from Discover. You'll be pinged here when a score drops, abandonment rises, or a better candidate appears.",
      emptyAction: "Open discover",
      needsLabel: "Need watches",
      needsTitle: "Needs under observation",
      needsCount: "needs",
      reposLabel: "Repo watches",
      reposTitle: "Repositories under observation",
      reposCount: "repos",
      matches: "matches",
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
    installAssistantLabel: "Install assistant",
    installAssistantBody:
      "Create a token here, choose your MCP client, copy the complete config, then test the endpoint.",
    signInToCreate:
      "Sign in to create an MCP token and generate a ready-to-copy client config.",
    tokenLabel: "Token label",
    tokenPlaceholder: "e.g. codex-local, cursor, claude-desktop",
    createTokenInline: "Create token",
    creatingToken: "Creating...",
    tokenReady:
      "Token created. Plaintext is included in the config below and will not be shown again after you leave this page.",
    chooseClientLabel: "Client",
    clientCodex: "Codex",
    clientCursor: "Cursor",
    clientClaude: "Claude Desktop",
    clientGeneric: "Generic MCP",
    configReadyTitle: "Copy a complete client config",
    configReadyBody:
      "Client schemas vary, but most Streamable HTTP clients need the same three fields: type, URL, and Authorization Bearer header.",
    copyConfig: "Copy config",
    copied: "copied",
    testToken: "Test token",
    testingToken: "Testing...",
    testOk: "Token valid. MCP initialize answered successfully.",
    testFail:
      "Token test failed. Verify the token was just created, then retry or revoke it from Account.",
    endpointLabel: "Server endpoint",
    endpointBody:
      "Use this URL in clients that support MCP Streamable HTTP. Send the token as a Bearer credential on every request.",
    cliLabel: "One-command install",
    cliTitle: "Let the CLI write the client config",
    cliBody:
      "The npm installer asks for your client and token, backs up the config file, writes UseStakly, then lets you test the transport.",
    cliInstallCommand: "npx usestakly-mcp install",
    cliTestCommand: "npx usestakly-mcp test",
    tryLabel: "Try it with your agent",
    tryTitle: "Ask for an explained recommendation",
    tryBody:
      "After installation, ask your agent for a dependency shortlist, then let it inspect provenance and log the outcome after testing.",
    tryPrompts: [
      "Find a reliable React table library with UseStakly. Explain the score, caveats, and provenance, then log_usage after the test.",
      "I need a TypeScript ORM. Recommend GitHub repos with UseStakly and compare reliability, freshness, and abandonment risk.",
      "Before adding this dependency, use UseStakly to inspect the repo detail and add it to my watchlist if it looks healthy."
    ],
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
        name: "recommend_github_repos",
        body:
          "Returns a short explained shortlist for a dependency need, with score-based reasons, caveats, next actions, and provenance."
      },
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
          "Estimates risk. Lower is better. In v2 it is coupled with vitality, so a fresh push alone can no longer mask a degraded maintainer structure."
      },
      {
        name: "Vitality",
        body:
          "Structural maintainer signals captured directly from GitHub at ingestion: distinct contributors over 90 days, commit cadence over 30 days, presence of CI workflows, and recency of releases. This is the anti-slop floor: a freshly-pushed solo repo with no CI and no release cannot dominate the ranking on freshness alone."
      }
    ],
    formulaVersionLabel: "Formula version",
    formulaVersionTitle: "v2.0 adds vitality and couples it with abandonment",
    formulaVersionBody:
      "Formula v2.0 (April 2026) introduced the vitality dimension on top of the existing four. Weights: freshness 0.15, adoption 0.10, reliability 0.30, abandonment 0.20, vitality 0.25. Old v1.1 scores remain readable in the audit history under their original formula tag.",
    vitalityLimitsLabel: "What vitality does not measure",
    vitalityLimitsTitle: "Honest limits",
    vitalityLimitsBody:
      "Vitality is structural and passive. It cannot tell whether code is well written, whether the maintainers are responsive, or whether a niche solo project of high quality deserves more credit than its sub-signals suggest. Legitimate solo tools and personal scripts will be penalised — that is an explicit tradeoff against the slop risk. Read the dimension as a floor, not a verdict on craft.",
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
      "Treat high Abandonment, low Freshness, or low Vitality as a reason to inspect before adopting.",
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
