import type { CopyBlock, CurrentUser, Locale } from "./app-types";

export const COPY: Record<Locale, CopyBlock> = {
  en: {
    authEyebrow: "Authentication",
    authTitle: "Sign in to continue",
    authBody:
      "Access your libraries, sync your identity, and start from your own codebase.",
    authGitHubButton: "Continue with GitHub",
    authDiscordButton: "Continue with Discord",
    authNotice: "GitHub and Discord are available for MVP login.",
    authSecurityLabel: "Session",
    authSecurityValue: "Secure browser session",
    authAccessLabel: "Language",
    authAccessValue: "English / French",
    loading: "Checking session...",
    language: "FR",
    connectedTitle: "You are connected",
    connectedBody: "Your session is active and ready to access UseStakly.",
    connectedLabel: "Signed in as",
    logout: "Logout",
    workspaceEyebrow: "Workspace",
    workspaceTitle: "UseStakly",
    workspaceBody:
      "Resolve your own libraries first, then compose a real app from addressable components.",
    workspaceStatus: "Private-first workspace is online",
    librariesStat: "Libraries",
    snippetsStat: "Snippets",
    publicStat: "Public assets",
    readyStat: "Assembly ready",
    librariesTitle: "Libraries",
    librariesBody: "Ownable sources the MCP can resolve before it ever generates code.",
    snippetsTitle: "Snippet inventory",
    snippetsBody: "Recent reusable building blocks already indexed in your workspace.",
    recentTitle: "Recent references",
    recentBody: "Direct references your agent can request explicitly.",
    commandTitle: "Assembly behavior",
    commandBody:
      "UseStakly should resolve exact assets first, search inside your allowed scope second, and only generate as a fallback.",
    commandModeStrict: "Strict mode",
    commandModeAuto: "Auto mode",
    commandModePrompt: "Prompt shape",
    defaultLibrary: "Default library",
    createLibraryTitle: "Create a library",
    createLibraryName: "Library name",
    createLibrarySlug: "Library slug",
    createLibraryDescription: "Description",
    createLibrarySubmit: "Create library",
    createSnippetTitle: "Create a snippet",
    createSnippetLibrary: "Library",
    createSnippetName: "Snippet name",
    createSnippetSlug: "Snippet slug",
    createSnippetDomain: "Domain",
    createSnippetKind: "Kind",
    createSnippetCategory: "Category",
    createSnippetLanguage: "Language",
    createSnippetFramework: "Framework",
    createSnippetVersion: "Version",
    createSnippetTags: "Tags",
    createSnippetCode: "Initial code",
    createSnippetSubmit: "Create snippet",
    detailTitle: "Snippet detail",
    detailEmpty: "Select a snippet to inspect its canonical reference and current code.",
    detailDescription: "Description",
    detailCode: "Current code",
    detailLibrary: "Library",
    detailRisk: "Risk",
    emptyLibraries: "No library yet. Your first library will become the initial MCP anchor.",
    emptySnippets:
      "No snippet yet. Once you add one, it will appear here with its canonical reference.",
    visibilityPrivate: "Private",
    visibilityPublic: "Public",
    trustedPrivate: "Private trust",
    trustedPublic: "Public unverified",
    referenceLabel: "Reference",
    tagsLabel: "Tags",
    versionLabel: "Version",
    scopeLabel: "Scope",
    logoutSecondary: "Sign out",
    navHome: "Home",
    navExplore: "Explore",
    navStudio: "Private studio",
    navProfile: "Profile",
    appEyebrow: "Community + private code graph",
    homeTitle: "Build from what the community already proved.",
    homeBody:
      "Start from the snippets people actually reuse, then move into your private studio when you need your own addressable assets.",
    homeFeaturedTitle: "Most appreciated snippets",
    homeFeaturedBody: "Community picks that should surface first in the product experience.",
    homeHighlightsTitle: "Why people save them",
    homeHighlightsBody:
      "High-signal assets with strong reuse potential, clean references, and stack clarity.",
    homeTrendingLabel: "Appreciation",
    homeSavedLabel: "Saves",
    homeReferenceLabel: "Reference",
    homeScopeCommunity: "Community",
    homeScopePrivate: "Private",
    homeEmpty: "No highlighted snippet yet. Publish a few strong assets and the feed will start here.",
    exploreTitle: "Explore public libraries",
    exploreBody:
      "Browse reusable blocks by stack, language, and reference quality before opening your private studio.",
    exploreEmpty: "No public snippet available yet in this MVP feed.",
    studioTitle: "Private studio",
    studioBody:
      "Your personal libraries, references, and assembly-first workspace stay here, separate from the public feed.",
    profileTitle: "Identity",
    profileBody:
      "Your social presence and your private code surface should live together, but never be confused.",
    profileIdentity: "Identity",
    profileEmail: "Email",
    profileHandle: "Handle",
    profilePresence: "Presence",
    profilePrivateLabel: "Private assets",
    profilePublicLabel: "Public assets"
  },
  fr: {
    authEyebrow: "Authentification",
    authTitle: "Connecte-toi pour continuer",
    authBody:
      "Accède à tes bibliothèques, synchronise ton identité et démarre depuis ta propre base de code.",
    authGitHubButton: "Continuer avec GitHub",
    authDiscordButton: "Continuer avec Discord",
    authNotice: "GitHub et Discord sont disponibles pour le login MVP.",
    authSecurityLabel: "Session",
    authSecurityValue: "Session navigateur sécurisée",
    authAccessLabel: "Langue",
    authAccessValue: "Français / Anglais",
    loading: "Vérification de la session...",
    language: "EN",
    connectedTitle: "Tu es connecté",
    connectedBody: "Ta session est active et prête à accéder à UseStakly.",
    connectedLabel: "Connecté en tant que",
    logout: "Se déconnecter",
    workspaceEyebrow: "Workspace",
    workspaceTitle: "UseStakly",
    workspaceBody:
      "Résous d'abord tes propres bibliothèques, puis compose une vraie app à partir de composants adressables.",
    workspaceStatus: "Workspace private-first en ligne",
    librariesStat: "Bibliothèques",
    snippetsStat: "Snippets",
    publicStat: "Assets publics",
    readyStat: "Prêt pour l’assemblage",
    librariesTitle: "Bibliothèques",
    librariesBody:
      "Des sources maîtrisées que le MCP peut résoudre avant de générer la moindre ligne.",
    snippetsTitle: "Inventaire de snippets",
    snippetsBody:
      "Les briques réutilisables les plus récentes déjà indexées dans ton workspace.",
    recentTitle: "Références récentes",
    recentBody: "Des références directes que ton agent peut demander explicitement.",
    commandTitle: "Comportement d’assemblage",
    commandBody:
      "UseStakly doit résoudre les assets exacts d’abord, chercher dans le scope autorisé ensuite, et ne générer qu’en fallback.",
    commandModeStrict: "Mode strict",
    commandModeAuto: "Mode auto",
    commandModePrompt: "Forme du prompt",
    defaultLibrary: "Bibliothèque par défaut",
    createLibraryTitle: "Créer une bibliothèque",
    createLibraryName: "Nom de la bibliothèque",
    createLibrarySlug: "Slug de la bibliothèque",
    createLibraryDescription: "Description",
    createLibrarySubmit: "Créer la bibliothèque",
    createSnippetTitle: "Créer un snippet",
    createSnippetLibrary: "Bibliothèque",
    createSnippetName: "Nom du snippet",
    createSnippetSlug: "Slug du snippet",
    createSnippetDomain: "Domaine",
    createSnippetKind: "Type",
    createSnippetCategory: "Catégorie",
    createSnippetLanguage: "Langage",
    createSnippetFramework: "Framework",
    createSnippetVersion: "Version",
    createSnippetTags: "Tags",
    createSnippetCode: "Code initial",
    createSnippetSubmit: "Créer le snippet",
    detailTitle: "Détail du snippet",
    detailEmpty: "Sélectionne un snippet pour inspecter sa référence canonique et son code courant.",
    detailDescription: "Description",
    detailCode: "Code courant",
    detailLibrary: "Bibliothèque",
    detailRisk: "Risque",
    emptyLibraries:
      "Aucune bibliothèque pour l’instant. La première deviendra l’ancre initiale du MCP.",
    emptySnippets:
      "Aucun snippet pour l’instant. Dès que tu en ajoutes un, il apparaîtra ici avec sa référence canonique.",
    visibilityPrivate: "Privée",
    visibilityPublic: "Publique",
    trustedPrivate: "Trust privée",
    trustedPublic: "Publique non vérifiée",
    referenceLabel: "Référence",
    tagsLabel: "Tags",
    versionLabel: "Version",
    scopeLabel: "Scope",
    logoutSecondary: "Déconnexion",
    navHome: "Accueil",
    navExplore: "Explorer",
    navStudio: "Studio privé",
    navProfile: "Profil",
    appEyebrow: "Graphe de code communautaire + privé",
    homeTitle: "Construis à partir de ce que la communauté a déjà validé.",
    homeBody:
      "Commence par les snippets réellement appréciés, puis bascule dans ton studio privé quand tu as besoin de tes propres assets adressables.",
    homeFeaturedTitle: "Snippets les plus appréciés",
    homeFeaturedBody: "Les picks communautaires qui doivent être mis en avant dans l’expérience produit.",
    homeHighlightsTitle: "Pourquoi ils sont sauvegardés",
    homeHighlightsBody:
      "Des assets à fort signal, avec vrai potentiel de réutilisation, références propres et stack claire.",
    homeTrendingLabel: "Appréciation",
    homeSavedLabel: "Sauvegardes",
    homeReferenceLabel: "Référence",
    homeScopeCommunity: "Communauté",
    homeScopePrivate: "Privé",
    homeEmpty:
      "Aucun snippet mis en avant pour l’instant. Publie quelques assets solides et le feed démarrera ici.",
    exploreTitle: "Explorer les bibliothèques publiques",
    exploreBody:
      "Parcours les briques réutilisables par stack, langage et qualité de référence avant d’ouvrir ton studio privé.",
    exploreEmpty: "Aucun snippet public disponible pour l’instant dans ce feed MVP.",
    studioTitle: "Studio privé",
    studioBody:
      "Tes bibliothèques personnelles, tes références et ton workspace orienté assemblage restent ici, séparés du feed public.",
    profileTitle: "Identité",
    profileBody:
      "Ta présence sociale et ta surface de code privée doivent vivre ensemble, sans jamais être confondues.",
    profileIdentity: "Identité",
    profileEmail: "Email",
    profileHandle: "Handle",
    profilePresence: "Présence",
    profilePrivateLabel: "Assets privés",
    profilePublicLabel: "Assets publics"
  }
};

export function detectInitialLocale(): Locale {
  if (typeof window === "undefined") {
    return "en";
  }
  const stored = window.localStorage.getItem("usestakly-locale");
  if (stored === "fr" || stored === "en") {
    return stored;
  }
  return window.navigator.language.toLowerCase().startsWith("fr") ? "fr" : "en";
}

export function getVisibilityLabel(copy: CopyBlock, visibility: string): string {
  return visibility === "public" ? copy.visibilityPublic : copy.visibilityPrivate;
}

export function getTrustLabel(copy: CopyBlock, trustLevel: string): string {
  return trustLevel === "public_unverified" ? copy.trustedPublic : copy.trustedPrivate;
}

export function avatarFallback(user: CurrentUser): string {
  const source = user.displayName ?? user.username ?? user.email;
  return source.slice(0, 2).toUpperCase();
}
