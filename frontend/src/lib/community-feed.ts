import type { CommunitySnippet, SnippetDetail } from "./app-types";

const COMMUNITY_SEED: CommunitySnippet[] = [
  {
    id: "community-react-hero-card",
    title: "React hero card",
    description: "A polished landing hero block with clean hierarchy and call-to-action slots.",
    author: "@alice",
    library: "@alice/react-ui-kit",
    language: "TypeScript",
    framework: "React + Tailwind",
    appreciation: 94,
    saves: 312,
    canonicalReference: "@alice/react-ui-kit:hero-card-premium",
    scope: "community"
  },
  {
    id: "community-axum-auth-guard",
    title: "Axum auth guard",
    description: "Reusable request guard for authenticated endpoints with clean extractor ergonomics.",
    author: "@nox",
    library: "@nox/rust-backend-core",
    language: "Rust",
    framework: "Axum",
    appreciation: 89,
    saves: 204,
    canonicalReference: "@nox/rust-backend-core:auth-guard-v1",
    scope: "community"
  },
  {
    id: "community-sql-migration-pack",
    title: "SQL migration pack",
    description: "Battle-tested migration starter for app schemas with users, assets, and provenance.",
    author: "@mira",
    library: "@mira/database-patterns",
    language: "SQL",
    framework: "PostgreSQL",
    appreciation: 86,
    saves: 188,
    canonicalReference: "@mira/database-patterns:starter-schema-core",
    scope: "community"
  }
];

function toCommunitySnippet(item: SnippetDetail, index: number): CommunitySnippet {
  const framework = item.snippet.framework ?? null;
  const publicScope = item.snippet.visibility === "public" ? "community" : "private";

  return {
    id: item.snippet.id,
    title: item.snippet.name,
    description: item.snippet.description ?? item.snippet.slug,
    author: "@you",
    library: item.canonicalReference.split(":")[0] ?? item.snippet.slug,
    language: item.snippet.language,
    framework,
    appreciation: Math.max(72, 96 - index * 3),
    saves: Math.max(48, 160 - index * 11),
    canonicalReference: item.canonicalReference,
    scope: publicScope
  };
}

export function buildCommunityFeed(snippets: SnippetDetail[]): CommunitySnippet[] {
  const derived = snippets.slice(0, 6).map(toCommunitySnippet);
  return [...derived, ...COMMUNITY_SEED].slice(0, 8);
}

export function featuredCommunitySnippets(snippets: CommunitySnippet[]): CommunitySnippet[] {
  return [...snippets].sort((a, b) => b.appreciation - a.appreciation).slice(0, 3);
}
