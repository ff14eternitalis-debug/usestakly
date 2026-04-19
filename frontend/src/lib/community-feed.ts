import type { CommunitySnippet, SnippetDetail } from "./app-types";

const COMMUNITY_SEED: CommunitySnippet[] = [
  {
    id: "community-react-hero-card",
    title: "Aurora pulse button",
    description: "An original CTA button with a soft glow ring, raised shadow, and premium hover lift.",
    fullDescription:
      "A polished public UI snippet designed for hero sections, onboarding moments, or premium pricing pages. The component keeps the markup light, uses a warm copper gradient, and adds a pulse aura behind the label to make the action feel important without becoming noisy.",
    author: "@alice",
    library: "@alice/react-ui-kit",
    language: "TypeScript",
    framework: "React + Tailwind",
    appreciation: 94,
    saves: 312,
    canonicalReference: "@alice/react-ui-kit:aurora-pulse-button",
    scope: "community",
    rawCode: `export function AuroraPulseButton() {
  return (
    <button className="group relative inline-flex items-center justify-center rounded-full border border-white/10 bg-[linear-gradient(135deg,#B67332,#93441A)] px-6 py-3 text-sm font-semibold text-stone-50 shadow-[0_18px_40px_rgba(147,68,26,0.35)] transition-transform duration-200 hover:-translate-y-0.5">
      <span className="absolute inset-0 rounded-full bg-[radial-gradient(circle_at_50%_50%,rgba(255,245,232,0.32),transparent_62%)] opacity-80 blur-md transition-opacity duration-200 group-hover:opacity-100" />
      <span className="relative">Launch workspace</span>
    </button>
  );
}`,
    previewKind: "button",
    previewLabel: "React component preview",
    previewNote: "Interactive public snippet preview built with React + Tailwind.",
    previewActionLabel: "Launch workspace"
  },
  {
    id: "community-axum-auth-guard",
    title: "Axum auth guard",
    description: "Reusable request guard for authenticated endpoints with clean extractor ergonomics.",
    fullDescription:
      "A backend-oriented public snippet showing a clean extractor pattern for authenticated routes in Axum. It is useful when you want a readable gateway between session validation and protected handlers.",
    author: "@nox",
    library: "@nox/rust-backend-core",
    language: "Rust",
    framework: "Axum",
    appreciation: 89,
    saves: 204,
    canonicalReference: "@nox/rust-backend-core:auth-guard-v1",
    scope: "community",
    rawCode: `use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let session = parts
            .headers
            .get("x-session-user")
            .and_then(|value| value.to_str().ok())
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

        Ok(Self {
            user_id: session.to_string(),
        })
    }
}`,
    previewKind: "backend",
    previewLabel: "Backend pattern preview",
    previewNote: "Read-only backend asset. The preview explains behavior instead of executing the code."
  },
  {
    id: "community-sql-migration-pack",
    title: "SQL migration pack",
    description: "Battle-tested migration starter for app schemas with users, assets, and provenance.",
    fullDescription:
      "A database starter asset for applications that need users, libraries, snippets, and provenance from day one. It is structured to stay readable for teams while leaving room for versioning and trust metadata.",
    author: "@mira",
    library: "@mira/database-patterns",
    language: "SQL",
    framework: "PostgreSQL",
    appreciation: 86,
    saves: 188,
    canonicalReference: "@mira/database-patterns:starter-schema-core",
    scope: "community",
    rawCode: `create table libraries (
  id uuid primary key,
  owner_id uuid not null,
  slug text not null,
  visibility text not null default 'private'
);

create table snippets (
  id uuid primary key,
  library_id uuid not null references libraries(id),
  slug text not null,
  language text not null,
  description text
);`,
    previewKind: "database",
    previewLabel: "Schema preview",
    previewNote: "Read-only schema asset. The preview highlights what the migration creates."
  }
];

function toCommunitySnippet(item: SnippetDetail, index: number): CommunitySnippet {
  const framework = item.snippet.framework ?? null;
  const publicScope = item.snippet.visibility === "public" ? "community" : "private";

  return {
    id: item.snippet.id,
    title: item.snippet.name,
    description: item.snippet.description ?? item.snippet.slug,
    fullDescription:
      item.snippet.description ??
      "Public snippet imported from your workspace. Open it to inspect its current raw source and canonical reference.",
    author: "@you",
    library: item.canonicalReference.split(":")[0] ?? item.snippet.slug,
    language: item.snippet.language,
    framework,
    appreciation: Math.max(72, 96 - index * 3),
    saves: Math.max(48, 160 - index * 11),
    canonicalReference: item.canonicalReference,
    scope: publicScope,
    rawCode:
      item.currentVersion?.code ??
      "// No code snapshot available yet for this public snippet.",
    previewKind: "backend",
    previewLabel: framework ?? item.snippet.language,
    previewNote: "Imported public snippet from your workspace. This preview is read-only."
  };
}

export function buildCommunityFeed(snippets: SnippetDetail[]): CommunitySnippet[] {
  const derived = snippets.slice(0, 6).map(toCommunitySnippet);
  return [...derived, ...COMMUNITY_SEED].slice(0, 8);
}

export function featuredCommunitySnippets(snippets: CommunitySnippet[]): CommunitySnippet[] {
  return [...snippets].sort((a, b) => b.appreciation - a.appreciation).slice(0, 3);
}
