import { HOME_META_DESCRIPTION } from "./site-default-description";

export type SeoPayload = {
  title: string;
  description: string;
  ogType: "website" | "article";
};

const HOME: SeoPayload = {
  title: "UseStakly — the open-source observatory",
  description: HOME_META_DESCRIPTION,
  ogType: "website"
};

const BY_PATH: Record<string, SeoPayload> = {
  "/discover": {
    title: "Discover — UseStakly",
    description:
      "Search and explore public GitHub repositories ranked by UseStakly quality scores, not stars alone.",
    ogType: "website"
  },
  "/how-to-read": {
    title: "How to read scores — UseStakly",
    description:
      "How to interpret UseStakly quality scores, flags, provenance, and formula versions for OSS dependencies.",
    ogType: "website"
  },
  "/mcp-guide": {
    title: "MCP for agents — UseStakly",
    description:
      "Install the UseStakly Model Context Protocol endpoint in Cursor, Codex, or other MCP clients.",
    ogType: "website"
  },
  "/login": {
    title: "Sign in — UseStakly",
    description: "Sign in with GitHub or Discord to use the watchlist, notifications, and agent tokens.",
    ogType: "website"
  },
  "/privacy": {
    title: "Privacy — UseStakly",
    description: "What data UseStakly collects, how it is used, and how long it is retained.",
    ogType: "website"
  },
  "/legal": {
    title: "Terms — UseStakly",
    description: "Terms of use for the UseStakly public beta and OSS observatory.",
    ogType: "website"
  },
  "/status": {
    title: "Service status — UseStakly",
    description: "Public health and availability information for the UseStakly service.",
    ogType: "website"
  },
  "/watchlist": {
    title: "Watchlist — UseStakly",
    description: "Repositories you follow for quality drift and notifications on UseStakly.",
    ogType: "website"
  },
  "/notifications": {
    title: "Notifications — UseStakly",
    description: "In-app notifications from your UseStakly watchlist and account activity.",
    ogType: "website"
  },
  "/account": {
    title: "Account — UseStakly",
    description: "Manage your UseStakly profile, OAuth connections, and MCP agent tokens.",
    ogType: "website"
  }
};

export function routeSeo(pathname: string): SeoPayload {
  if (pathname === "/" || pathname === "") {
    return HOME;
  }
  if (pathname.startsWith("/repos/")) {
    return {
      title: "Repository — UseStakly",
      description:
        "Quality score, signals, radar, and provenance for a public GitHub repository on UseStakly.",
      ogType: "article"
    };
  }
  return BY_PATH[pathname] ?? HOME;
}
