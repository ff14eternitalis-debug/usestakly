import type { CopyBlock, CurrentUser, Locale, Theme } from "./app-types";

export const COPY: Record<Locale, CopyBlock> = {
  en: {
    authEyebrow: "Authentication",
    authTitle: "Sign in to continue",
    authBody:
      "Access your libraries, sync your identity, and start from your own codebase.",
    authButton: "Continue with GitHub",
    authNotice: "Only GitHub is enabled for the MVP.",
    authSecurityLabel: "Session",
    authSecurityValue: "Secure browser session",
    authAccessLabel: "Language",
    authAccessValue: "English / French",
    loading: "Checking session...",
    language: "FR",
    theme: "Dark",
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
    logoutSecondary: "Sign out"
  },
  fr: {
    authEyebrow: "Authentification",
    authTitle: "Connecte-toi pour continuer",
    authBody:
      "Accède à tes bibliothèques, synchronise ton identité et démarre depuis ta propre base de code.",
    authButton: "Continuer avec GitHub",
    authNotice: "Seul GitHub est activé pour le MVP.",
    authSecurityLabel: "Session",
    authSecurityValue: "Session navigateur sécurisée",
    authAccessLabel: "Langue",
    authAccessValue: "Français / Anglais",
    loading: "Vérification de la session...",
    language: "EN",
    theme: "Sombre",
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
    logoutSecondary: "Déconnexion"
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

export function detectInitialTheme(): Theme {
  if (typeof window === "undefined") {
    return "light";
  }
  const stored = window.localStorage.getItem("usestakly-theme");
  if (stored === "light" || stored === "dark") {
    return stored;
  }
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
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
