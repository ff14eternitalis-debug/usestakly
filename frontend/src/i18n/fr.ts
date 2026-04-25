import type { Dict } from "./en";

export const fr: Dict = {
  nav: {
    discover: "Explorer",
    howToRead: "Lire UseStakly",
    mcpGuide: "Guide MCP",
    watchlist: "Veille",
    notifications: "Notifications",
    account: "Compte",
    signIn: "Connexion",
    signOut: "déconnexion"
  },
  common: {
    offline: "Observatoire hors ligne",
    offlineHint: "Lance",
    offlineFrom: "depuis",
    backendDir: "backend/",
    cargoRun: "cargo run",
    tuning: "Calibrage des instruments…",
    noMatch: "Aucun résultat",
    browseWithoutSignIn: "Consulter sans se connecter",
    viewOnGithub: "Voir sur GitHub",
    github: "github",
    github2: "GitHub",
    arrowNext: "→",
    readyStatus: "prêt",
    checkingStatus: "vérification",
    observingStatus: "observation"
  },
  header: {
    signIn: "Connexion"
  },
  footer: {
    tagline:
      "Un observatoire à score qualité des dépôts open-source publics. Formule de scoring publique, versionnée, locale.",
    product: "Produit",
    signals: "Signaux",
    about: "À propos",
    mcp: "MCP",
    privacy: "Données",
    status: "Status",
    freshness: "Fraîcheur",
    adoption: "Adoption",
    reliability: "Fiabilité",
    abandonment: "Abandon",
    selfHosted: "Auto-hébergé",
    publicFormula: "Formule publique",
    localEmbeddings: "Embeddings locaux",
    copyright: "© {year} UseStakly",
    tagFormula: "formula_v1 · transparent par conception"
  },
  landing: {
    eyebrow: "Beta publique · formula_v1",
    h1Part1: "UseStakly",
    h1Part2: "Choisis tes repos GitHub OSS avec un score qualité transparent.",
    intro:
      "UseStakly aide les devs et les agents de code à comparer des repos GitHub publics avec scoring visible, provenance et alertes de veille.",
    openObservatory: "Explorer les repos",
    readGuide: "Lire UseStakly",
    signInForWatchlist: "Se connecter pour la veille",
    myWatchlist: "Ma veille",
    kpi1: "Signaux notés",
    kpi2: "Formule publique",
    kpi3: "Boîte noire",
    panelLive: "Verdict en direct",
    panelSample: "exemple · facebook/react",
    panelOverall: "Global",
    dataEyebrow: "Qualité des données",
    dataH2: "Métadonnées réelles, signaux d'usage progressifs.",
    dataBody:
      "Les métadonnées GitHub sont récupérées à l'ingestion. Adoption et fiabilité deviennent plus solides quand les agents MCP et les users remontent des outcomes réels.",
    dataItems: [
      {
        title: "Les métadonnées GitHub sont réelles",
        body:
          "Étoiles, forks, issues, topics, langage, état archivé et dernier push viennent de GitHub."
      },
      {
        title: "Les signaux d'usage sont progressifs",
        body:
          "resolve, build_success, build_failure et regret sont enregistrés via les signaux UseStakly."
      },
      {
        title: "Beta assumée",
        body:
          "Le corpus est curé et grandit. Les scores se lisent avec leur provenance, pas comme une vérité cachée."
      }
    ],
    dataCta: "Comment lire le score",
    pillarsEyebrow: "Ce que ça fait",
    pillarsH2: "Deux outils, une formule, aucune boîte noire.",
    pillar: "Pilier",
    pillar1Title: "Découverte, notée à l'usage.",
    pillar1Body:
      "Chaque dépôt est mesuré via une formule transparente combinant cadence de commits, adoption, fiabilité de build et signaux d'abandon. Trois modes — auto, strict, explorer — même score, seuils différents.",
    pillar1Artifact: "modes de filtre",
    pillar1Cta: "Essayer l'exploration",
    pillar2Title: "Veille, alertes réelles.",
    pillar2Body:
      "Épingle les dépôts dont tu dépends. On compare les scores entre recalculs et on déclenche des notifications in-app quand l'abandon grimpe, qu'un flag sévère tombe ou que la qualité globale chute. Pas de flux RSS de PR, pas de silence radio.",
    pillar2Artifact: "déclencheurs",
    pillar2Cta: "Ouvrir la veille",
    formulaEyebrow: "formula_v1.toml",
    formulaH2: "Le score est une affirmation, pas une boîte noire.",
    formulaBody:
      "Chaque dimension est une équation nommée avec une demi-vie ou un seuil connu. Chaque score porte la version de la formule qui l'a produit : v2 ne réécrit jamais le verdict d'hier.",
    previewEyebrow: "Depuis le registre",
    previewH2: "Aperçus en direct de l'observatoire.",
    previewSeeAll: "Voir toutes les entrées",
    tickerTuning: "─── calibrage ─── calibrage ─── calibrage ───",
    closingEyebrow: "Garde une liste courte",
    closingH2Part1: "Épingle les dépôts dont tu dépends.",
    closingH2Part2: "On veille pour toi.",
    closingBrowse: "Parcourir les dépôts",
    closingWatchlist: "Ouvrir la veille",
    closingStart: "Commencer"
  },
  privacy: {
    eyebrow: "Données",
    h1: "Ce que UseStakly stocke",
    intro:
      "UseStakly garde le minimum nécessaire pour scorer les repos GitHub, gérer la veille, les notifications et l'accès MCP.",
    sections: [
      {
        title: "Identité OAuth",
        body:
          "GitHub ou Discord OAuth sert à la connexion. UseStakly stocke ton id user, pseudo, avatar et email quand le provider le renvoie. Il n'y a pas de liste marketing."
      },
      {
        title: "Veille et notifications",
        body:
          "Les repos suivis et l'état lu/non lu des notifications sont stockés pour t'alerter quand les scores dérivent."
      },
      {
        title: "Tokens MCP",
        body:
          "Les tokens agent utilisent le format usk_. La valeur en clair est affichée une seule fois, puis seul un hash SHA-256 est stocké côté serveur."
      },
      {
        title: "Signaux d'usage",
        body:
          "MCP log_usage et les reports user stockent owner/name du repo, outcome, timestamp, propriétaire du token et notes optionnelles pour améliorer les scores avec de vrais usages."
      }
    ],
    closing:
      "Les métadonnées publiques viennent de GitHub. Le code source privé n'est pas ingéré par UseStakly."
  },
  status: {
    eyebrow: "Status",
    h1: "État du service UseStakly",
    intro:
      "Un contrôle léger pour la beta publique : santé API et lecture du registre.",
    apiHealth: "Santé API",
    database: "Base de données",
    registryRead: "Lecture registre",
    mcp: "Outils MCP",
    formula: "Formule",
    publicStatus: "Status public",
    repos: "repos",
    tools: "outils",
    checking: "Vérification",
    online: "En ligne",
    degraded: "Dégradé",
    offline: "Hors ligne",
    lastChecked: "Dernier check",
    betaTitle: "Périmètre beta publique",
    betaBody:
      "Les health checks Coolify couvrent les conteneurs. Cette page ajoute des checks visibles API, DB, registre, MCP et formule, mais ce n'est pas encore un système d'incident complet."
  },
  discover: {
    eyebrow: "Explorer",
    h1Part1: "Que veux-tu",
    h1Accent: "mesurer",
    h1Part2: "aujourd'hui ?",
    intro:
      "Cherche le corpus par nom, propriétaire, description ou topic. Affine par langage, nombre minimum d'étoiles ou confiance. Même formule, seuils différents.",
    scoreGuideTitle: "Lis le score avant les étoiles",
    scoreGuideBody:
      "Le global combine fraîcheur, adoption, fiabilité et risque d'abandon dans un verdict de 0 à 1. C'est un signal de dépendance, pas un classement de popularité.",
    scoreGuideAction: "Comment lire UseStakly",
    corpusTitle: "Corpus initial",
    corpusBody:
      "Le MVP démarre avec une sélection crédible de repos OSS publics : références actives, contre-exemples dépréciés et tooling que les agents recommandent souvent. Ajoute n'importe quel repo GitHub pour le scorer à la demande.",
    queryLabel: "Requête",
    queryPlaceholder: "ex. date picker, orm, htmx, zustand",
    modeLabel: "Mode",
    modeExplore: "Explorer",
    modeAuto: "Auto",
    modeStrict: "Strict",
    hintExplore: "Tout avec ses preuves — aucun filtre.",
    hintAuto: "Garde les dépôts scorés au-dessus du plancher. Masque les cassés et risques sévères.",
    hintStrict: "Zéro flag, assez frais, faible abandon, barre globale plus haute.",
    languageLabel: "Langage",
    languageAny: "tous",
    starsMinLabel: "Étoiles min",
    starsMinPlaceholder: "0",
    hintLabel: "Indice",
    addRepoLabel: "Ajouter un dépôt GitHub",
    addRepoPlaceholder: "owner/repo ou https://github.com/owner/repo",
    addRepoAction: "Ajouter le dépôt",
    addRepoPending: "Ajout…",
    addRepoHelp: "Colle une URL GitHub ou owner/repo pour l'ajouter tout de suite à l'observatoire.",
    addRepoSuccess: "Dépôt ajouté au registre :",
    addRepoExists: "Dépôt déjà indexé. Métadonnées et score rafraîchis :",
    addRepoOpen: "Ouvrir le profil",
    measuring: "mesure en cours…",
    entriesSingle: "entrée",
    entriesPlural: "entrées",
    sortedBy: "tri par global · étoiles · récence",
    tryWidening: "Essaye d'élargir vers",
    exploreLink: "explorer",
    orLoweringStars: ", ou baisser le seuil d'étoiles."
  },
  repoDetail: {
    back: "Explorer",
    formula: "formule",
    computed: "calculé",
    loading: "Récupération du dossier…",
    notFound: "Absent du registre",
    notFoundBody: "Aucun profil n'existe sous cet identifiant.",
    offlineBody: "Le backend n'a pas répondu.",
    backToDiscover: "Retour à l'exploration",
    addToWatchlist: "Ajouter à la veille",
    adding: "Ajout…",
    unwatch: "Retirer",
    unwatching: "Retrait…",
    signInToWatch: "Se connecter pour suivre ce dépôt",
    signInToWatchHint:
      "Reçois une alerte quand le score baisse, que l'abandon grimpe ou qu'un flag sévère tombe.",
    overallVerdict: "Verdict global",
    healthy: "En forme",
    monitor: "À surveiller",
    atRisk: "En danger",
    unscored: "Non noté",
    stars: "Étoiles",
    forks: "Forks",
    openIssues: "Issues ouvertes",
    subscribers: "Abonnés",
    lastCommit: "Dernier commit",
    priorsFetched: "Priors récupérés",
    defaultBranch: "Branche par défaut",
    dimensions: "Dimensions",
    freshness: "Fraîcheur",
    adoption: "Adoption",
    reliability: "Fiabilité",
    abandonment: "Abandon",
    freshnessHint: "Décroissance exponentielle sur last_commit_at (demi-vie 180j).",
    adoptionHint: "Nombre de résolutions log-normalisé (sature à 1k).",
    reliabilityHint: "Succès / total builds. Neutre 0.5 avant 5 échantillons.",
    abandonmentHint: "Inverse fraîcheur plus bump de regret au-dessus du seuil.",
    scoreGuideTitle: "Comment lire ce score",
    scoreGuideBody:
      "Utilise le verdict global comme premier tri, puis regarde les dimensions. Un bon repo peut quand même demander de la veille si la fraîcheur baisse ou si l'abandon grimpe.",
    scoreGuideAction: "Lire le guide complet",
    scoreGuideItems: [
      "Fraîcheur et fiabilité sont les premiers contrôles de risque avant d'adopter une dépendance.",
      "L'adoption est plafonnée pour qu'un gros projet ne gagne pas seulement parce qu'il est connu.",
      "L'abandon est un score de risque : plus bas est meilleur, et une valeur haute tire le verdict global vers le bas."
    ],
    provenanceTitle: "Provenance du score",
    provenanceBody:
      "Les métadonnées GitHub sont réelles au moment de l'ingestion. Les counts d'usage viennent des signaux UseStakly, surtout des événements MCP log_usage, et restent faibles tant que des agents ou users ne remontent pas d'outcomes réels.",
    githubMetadata: "Métadonnées GitHub",
    usageSignals: "Signaux d'usage",
    freshnessSource: "Source fraîcheur",
    lastCommitSource: "Dernier push GitHub",
    adoptionSource: "Source adoption",
    reliabilitySource: "Source fiabilité",
    neutralReliability: "neutre avant 5 échantillons build",
    resolveCount: "resolve",
    buildSuccessCount: "build success",
    buildFailureCount: "build failure",
    regretCount: "regret",
    signalVolumeEmpty:
      "Aucun signal d'usage pour l'instant. L'adoption reste vide et la fiabilité garde sa valeur neutre.",
    signalVolumePartial:
      "Le volume de signaux est encore fin. Lis le score comme une direction tant que plus d'outcomes MCP n'arrivent pas.",
    signalVolumeReady:
      "Le volume de signaux est présent. Fiabilité et adoption reposent maintenant sur des outcomes enregistrés.",
    recentSignals: "Signaux récents",
    entrySingle: "entrée",
    entriesPlural: "entrées",
    noSignals: "Aucun signal rapporté. L'observatoire écoute.",
    passive: "passif",
    reported: "rapporté"
  },
  watchlist: {
    eyebrow: "Veille",
    h1Part1: "La liste courte,",
    h1Accent: "sous observation.",
    intro:
      "On compare les scores entre recalculs. Si un dépôt dérive, tu le verras dans",
    notifications: "notifications",
    loading: "Récupération du dossier…",
    loadErrorTitle: "Veille indisponible.",
    loadErrorBody:
      "Impossible de charger tes dépôts suivis. Ta liste existe toujours ; réessaie quand la session ou le réseau revient.",
    retry: "réessayer",
    emptyTitle: "Rien en veille pour l'instant.",
    emptyBody:
      "Ouvre le profil d'un dépôt depuis le registre et clique Ajouter à la veille. Tu seras notifié ici quand un score chute, que l'abandon grimpe ou qu'un flag sévère tombe.",
    emptyAction: "Ouvrir l'exploration",
    watched: "suivi",
    overall: "Global",
    mute: "muter",
    unmute: "démuter",
    remove: "retirer",
    removing: "retrait…",
    confirmRemove: "confirmer le retrait",
    cancelRemove: "annuler"
  },
  notifications: {
    eyebrow: "Notifications",
    h1Part1: "Ce qui a bougé",
    h1Accent: "depuis ton dernier passage.",
    unreadOnly: "Non lues seulement",
    markAllRead: "tout marquer lu",
    markRead: "marquer lu",
    loading: "Tri du courrier…",
    loadErrorTitle: "Notifications indisponibles.",
    loadErrorBody:
      "Impossible de charger tes notifications. Réessaie quand la session ou le réseau revient.",
    retry: "réessayer",
    emptyTitle: "Tout est calme sur le registre.",
    emptyBodyUnread:
      "Rien de non lu à signaler. Ajoute des dépôts à ta {watchlistLink} pour que l'observatoire te signale les dérives.",
    emptyBodyRecent:
      "Rien de récent à signaler. Ajoute des dépôts à ta {watchlistLink} pour que l'observatoire te signale les dérives.",
    watchlist: "veille",
    watchlistAction: "Ouvrir la veille",
    labelScoreDrop: "chute de score",
    labelAbandonmentUp: "abandon qui grimpe",
    labelFlagAdded: "nouveau flag",
    labelFlagSevere: "flag sévère",
    markingRead: "marquage…"
  },
  login: {
    eyebrow: "Connexion",
    h1Part1: "Connexion à",
    h1Accent: "l'observatoire.",
    body:
      "Une session est requise pour tenir une veille, flagger un dépôt avec des preuves ou brancher un agent MCP. La lecture du registre est libre — pas de compte requis.",
    browseWithoutSignIn: "Consulter sans se connecter",
    continueGithub: "Continuer avec GitHub",
    continueDiscord: "Continuer avec Discord",
    privacy:
      "Aucun e-mail envoyé, aucune liste marketing. OAuth est toute la poignée de main — on apprend ton pseudo et ton avatar, rien de plus."
  },
  mcpGuide: {
    eyebrow: "Guide MCP",
    h1: "Installer UseStakly dans ton agent",
    intro:
      "Branche un agent de code compatible MCP sur le même registre GitHub scoré que l'app web. Crée un token par agent, colle la config Streamable HTTP, puis demande des recommandations de repos avec provenance.",
    createTokenAction: "Créer un token MCP",
    createTokenHint:
      "Les tokens vivent dans Compte, ne sont affichés qu'une fois et se révoquent sans toucher à ta session web.",
    installAssistantLabel: "Assistant d'installation",
    installAssistantBody:
      "Crée un token ici, choisis ton client MCP, copie la config complète, puis teste l'endpoint.",
    signInToCreate:
      "Connecte-toi pour créer un token MCP et générer une config client prête à coller.",
    tokenLabel: "Label du token",
    tokenPlaceholder: "ex. codex-local, cursor, claude-desktop",
    createTokenInline: "Créer le token",
    creatingToken: "Création...",
    tokenReady:
      "Token créé. La valeur en clair est incluse dans la config ci-dessous et ne sera plus affichée après avoir quitté cette page.",
    chooseClientLabel: "Client",
    clientCodex: "Codex",
    clientCursor: "Cursor",
    clientClaude: "Claude Desktop",
    clientGeneric: "MCP générique",
    configReadyTitle: "Copier une config client complète",
    configReadyBody:
      "Les schémas varient selon les clients, mais la plupart des clients Streamable HTTP demandent les mêmes champs : type, URL et header Authorization Bearer.",
    copyConfig: "Copier la config",
    copied: "copié",
    testToken: "Tester le token",
    testingToken: "Test...",
    testOk: "Token valide. MCP initialize a répondu correctement.",
    testFail:
      "Le test du token a échoué. Vérifie que le token vient d'être créé, puis réessaie ou révoque-le depuis Compte.",
    endpointLabel: "Endpoint serveur",
    endpointBody:
      "Utilise cette URL dans les clients qui supportent MCP Streamable HTTP. Envoie le token en Bearer sur chaque requête.",
    cliLabel: "Installation en une commande",
    cliTitle: "Laisse le CLI écrire la config client",
    cliBody:
      "L'installeur npm demande ton client et ton token, sauvegarde le fichier de config, écrit UseStakly, puis te laisse tester le transport.",
    cliInstallCommand: "npx usestakly-mcp install",
    cliTestCommand: "npx usestakly-mcp test",
    tryLabel: "À essayer avec ton agent",
    tryTitle: "Demande une recommandation expliquée",
    tryBody:
      "Après installation, demande à ton agent une shortlist de dépendances, puis laisse-le inspecter la provenance et logger l'outcome après test.",
    tryPrompts: [
      "Cherche une lib React table fiable avec UseStakly. Explique le score, les caveats et la provenance, puis log_usage après le test.",
      "J'ai besoin d'un ORM TypeScript. Recommande des repos GitHub avec UseStakly et compare fiabilité, fraîcheur et risque d'abandon.",
      "Avant d'ajouter cette dépendance, utilise UseStakly pour inspecter le détail repo et ajoute-le à ma veille s'il paraît sain."
    ],
    stepsLabel: "Installation",
    steps: [
      {
        title: "Connecte-toi et crée un token",
        body:
          "Ouvre Compte, choisis un label comme codex-local ou claude-desktop, puis crée un token. Copie tout de suite la valeur en clair."
      },
      {
        title: "Ajoute UseStakly à ton client MCP",
        body:
          "Colle l'endpoint et le header Authorization dans la configuration du client. Garde un token par machine ou agent pour une révocation précise."
      },
      {
        title: "Redémarre le client et teste une recherche",
        body:
          "Demande à ton agent de chercher une catégorie de repo via UseStakly, puis vérifie le score, la version de formule et la provenance retournés."
      }
    ],
    clientConfigLabel: "Config client",
    clientConfigTitle: "Configuration Streamable HTTP",
    clientConfigBody:
      "Les schémas varient selon les clients, mais les champs requis restent les mêmes : type Streamable HTTP, URL /mcp et header Authorization Bearer.",
    smokeTestLabel: "Test rapide",
    smokeTestTitle: "Vérifier le transport avant de brancher un agent",
    smokeTestBody:
      "Cette requête initialize doit renvoyer une réponse MCP. Si elle échoue, vérifie le préfixe du token, l'URL endpoint et l'accessibilité du backend.",
    toolsLabel: "Outils disponibles",
    toolsTitle: "Ce que l'agent peut faire",
    toolsBody:
      "Les outils read servent aux recommandations. Les outils write attachent des signaux d'usage ou des entrées de veille au user propriétaire du token.",
    tools: [
      {
        name: "recommend_github_repos",
        body:
          "Retourne une shortlist expliquée pour un besoin de dépendance, avec raisons liées au score, caveats, prochaines actions et provenance."
      },
      {
        name: "search_github_repos",
        body:
          "Cherche dans le registre scoré par requête, mode de filtre, langage, seuil d'étoiles et limite."
      },
      {
        name: "get_repo_quality_context",
        body:
          "Retourne le profil qualité complet : dimensions, flags, signaux récents, version de formule et provenance."
      },
      {
        name: "log_usage",
        body:
          "Enregistre un outcome d'usage passif comme build_success, build_failure, regret, resolve ou re_resolve."
      },
      {
        name: "watch_repo",
        body:
          "Ajoute un repo à la veille du propriétaire du token pour que UseStakly alerte sur les futures dérives."
      }
    ],
    securityLabel: "Sécurité",
    securityTitle: "Gestion des tokens",
    securityItems: [
      "Les tokens utilisent le format usk_<64 hex> et sont stockés hashés côté serveur.",
      "La valeur en clair n'est affichée qu'à la création. Stocke-la dans le client MCP, pas dans des captures ou docs partagées.",
      "Révoque les anciens tokens depuis Compte quand une machine, un client ou un coéquipier n'a plus besoin d'accès.",
      "Les outils write sont limités par token et protégés contre les doublons ou signaux négatifs répétés."
    ]
  },
  howToRead: {
    eyebrow: "Guide de lecture",
    h1: "Comment lire UseStakly",
    intro:
      "UseStakly sert aux décisions de dépendances. Le score aide à comparer des repos GitHub publics par maintenance, confiance d'usage et risque, sans laisser les étoiles décider seules.",
    scoreLabel: "Score",
    scoreTitle: "Le global est un verdict de dépendance entre 0 et 1",
    scoreBody:
      "Un score proche de 1 indique qu'un repo semble sain à adopter aujourd'hui. Un score proche de 0 indique qu'il faut enquêter ou l'éviter. La valeur garde toujours une version de formule et une date de calcul.",
    dimensionsLabel: "Dimensions",
    dimensions: [
      {
        name: "Fraîcheur",
        body:
          "Regarde l'activité récente du dépôt. Les derniers commits anciens décayent avec le temps, donc un repo célèbre mais silencieux perd en confiance."
      },
      {
        name: "Adoption",
        body:
          "Mesure les signaux d'usage et de résolution, puis plafonne l'effet pour que la popularité n'écrase pas la qualité."
      },
      {
        name: "Fiabilité",
        body:
          "Suit les usages positifs face aux échecs. Le score reste neutre tant qu'il n'y a pas assez d'échantillons."
      },
      {
        name: "Abandon",
        body:
          "Estime le risque. Plus bas est meilleur. Un abandon élevé peut tirer vers le bas un repo pourtant populaire."
      }
    ],
    modesLabel: "Modes",
    modesTitle: "Même formule, seuils différents",
    modes: [
      {
        name: "Explorer",
        body: "Affiche tout avec les preuves. Utile pour auditer ou chercher des signaux faibles."
      },
      {
        name: "Auto",
        body: "Liste courte par défaut. Masque les entrées cassées ou à risque sévère sans trop fermer la découverte."
      },
      {
        name: "Strict",
        body: "Demande un profil plus propre : aucun flag sévère accepté, meilleure fraîcheur et barre globale plus haute."
      }
    ],
    corpusLabel: "Corpus",
    corpusTitle: "Le corpus MVP est curé, puis grandit à la demande",
    corpusBody:
      "Le seed initial mélange des références actives en JS/TS, Rust, Python et Go avec des exemples dépréciés comme request et des projets en maintenance comme moment. Les démos restent honnêtes : les bons repos scorent bien, les repos dormants doivent s'expliquer.",
    corpusItems: [
      "Les repos seed sont des projets GitHub publics ingérés par le même pipeline de scoring.",
      "Tout repo peut être ajouté depuis Explorer avec owner/repo ou une URL GitHub.",
      "La veille et les signaux MCP rendent le corpus plus utile au fil du temps."
    ],
    workflowLabel: "Workflow",
    workflowTitle: "Un ordre de lecture pratique",
    workflowItems: [
      "Démarre en mode Auto et cherche la catégorie dont tu as besoin.",
      "Compare le Global, puis ouvre le détail repo pour les dimensions et flags.",
      "Traite un Abandon haut ou une Fraîcheur basse comme un signal à inspecter avant adoption.",
      "Ajoute tes vraies dépendances à la veille pour rendre les dérives visibles ensuite."
    ],
    ctaDiscover: "Ouvrir Explorer",
    ctaMcp: "Installer MCP"
  },
  account: {
    eyebrow: "Compte",
    h1Part1: "Tokens agent,",
    h1Accent: "sous contrôle.",
    intro:
      "Crée des tokens MCP pour tes agents de code, révoque ceux que tu ne veux plus garder, et garde une écriture bornée. Les tokens ne sont affichés en clair qu'une seule fois.",
    tokenLabel: "Label du nouveau token",
    tokenPlaceholder: "ex. claude-desktop, cursor, codex",
    create: "Créer le token",
    creating: "Création…",
    activeTokens: "Tokens MCP actifs",
    emptyTitle: "Aucun token MCP pour l'instant.",
    emptyBody:
      "Crée un token par agent ou machine pour que la révocation reste chirurgicale. Tous les write tools sont limités par token.",
    createdNow: "Créé à l'instant",
    lastUsedNever: "Jamais utilisé",
    lastUsed: "Dernier usage",
    createdAt: "Créé",
    revoke: "révoquer",
    revoking: "révocation…",
    tokenShownOnce: "Token en clair",
    tokenShownOnceHint: "Cette valeur n'est affichée qu'une fois. Enregistre-la maintenant dans ton client MCP.",
    copy: "copier",
    copied: "copié",
    quotaTitle: "Sécurité d'écriture",
    quotaBody:
      "Les write tools MCP sont limités par token, les appels log_usage dupliqués sont ralentis, et les outcomes négatifs répétés sont refroidis pour réduire le poisoning.",
    reputation: "Réputation",
    tier: "Niveau",
    passiveSignals: "Signaux passifs",
    usageSignals: "Signaux d'usage",
    successRatio: "Ratio positif",
    buildReliability: "Fiabilité build",
    regretRatio: "Ratio regret",
    eligibility: "Signals actifs",
    eligible: "éligible",
    notEligible: "pas encore éligible",
    adminTitle: "File de modération",
    adminTokenLabel: "Token admin",
    adminTokenPlaceholder: "Colle x-admin-token",
    adminLoad: "Charger les pending",
    adminApprove: "approuver",
    adminReject: "rejeter",
    adminEmpty: "Aucun signal repo en attente.",
    adminReviewing: "review…",
    mcpObservabilityTitle: "Observabilité MCP",
    mcpObservabilityIntro:
      "Vue agrégée des events agent_token_events : volume log_usage, watch_repo et refus des guards sur une fenêtre choisie.",
    mcpWindowLabel: "Fenêtre",
    mcpWindow24h: "24 h",
    mcpWindow7d: "7 j",
    mcpWindow30d: "30 j",
    mcpLoading: "Chargement des metrics…",
    mcpTotalLogUsage: "log_usage",
    mcpTotalWatchRepo: "watch_repo",
    mcpTotalRejections: "Refus guards",
    mcpDistinctTokens: "Tokens distincts",
    mcpDistinctUsers: "Users distincts",
    mcpDistinctRepos: "Repos touchés",
    mcpOutcomeTitle: "Distribution des outcomes log_usage",
    mcpRejectionTitle: "Refus par raison",
    mcpTopReposTitle: "Top repos",
    mcpTopUsersTitle: "Top users",
    mcpDailyTitle: "Volume quotidien",
    mcpEmpty: "Aucune activité MCP sur cette fenêtre."
  },
  signals: {
    title: "Signaler un problème",
    hint:
      "Les signals actifs demandent une preuve et assez de réputation. Les flags sévères ne sortent publiquement qu'après accord de plusieurs comptes de confiance.",
    signalLabel: "Signal",
    evidenceUrlLabel: "URL de preuve",
    evidenceDescriptionLabel: "Résumé de la preuve",
    submit: "Envoyer le signal",
    submitting: "Envoi…",
    success: "Signal enregistré. Les flags publics ne bougent qu'après consensus de comptes fiables.",
    ownerTitle: "Review owner",
    ownerHint:
      "Si ce repo appartient à ton compte GitHub, tu peux contester ici un signal actif pending ou accepté.",
    disputeReasonLabel: "Raison de la contestation",
    dispute: "Contester le signal",
    disputing: "Contestation…",
    disputed: "Signal contesté. Il repasse en review.",
    status: "statut"
  }
};
