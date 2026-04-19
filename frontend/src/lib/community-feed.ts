import type {
  CommunitySnippet,
  PublicLibraryProfile,
  SnippetDetail,
  SnippetFile
} from "./app-types";

const COMMUNITY_SEED: CommunitySnippet[] = [
  {
    id: "alice-aurora-pulse-button",
    title: "Aurora pulse button",
    description: "An original CTA button with a soft glow ring, raised shadow, and premium hover lift.",
    fullDescription:
      "A polished public UI snippet designed for hero sections, onboarding moments, or premium pricing pages. The component keeps the markup light, uses a warm copper gradient, and adds a pulse aura behind the label to make the action feel important without becoming noisy.",
    author: "@alice",
    authorName: "Alice Rowan",
    library: "@alice/react-ui-kit",
    libraryName: "React UI Kit",
    language: "TypeScript",
    framework: "React + Tailwind",
    domain: "Front-end",
    appreciation: 94,
    saves: 312,
    canonicalReference: "@alice/react-ui-kit:aurora-pulse-button",
    scope: "community",
    files: [
      {
        id: "aurora-html",
        label: "HTML",
        language: "html",
        content: `<button class="aurora-button">
  <span class="aurora-button__aura"></span>
  <span class="aurora-button__label">Launch workspace</span>
</button>`
      },
      {
        id: "aurora-css",
        label: "CSS",
        language: "css",
        content: `.aurora-button {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: linear-gradient(135deg, #B67332, #93441A);
  padding: 0.75rem 1.5rem;
  font-size: 0.875rem;
  font-weight: 600;
  color: #f5f5f4;
  box-shadow: 0 18px 40px rgba(147, 68, 26, 0.35);
  transition: transform 200ms ease;
  cursor: pointer;
}

.aurora-button:hover {
  transform: translateY(-2px);
}

.aurora-button__aura {
  position: absolute;
  inset: 0;
  border-radius: 9999px;
  background: radial-gradient(circle at 50% 50%, rgba(255, 245, 232, 0.32), transparent 62%);
  opacity: 0.8;
  filter: blur(10px);
  transition: opacity 200ms ease;
}

.aurora-button:hover .aurora-button__aura {
  opacity: 1;
}

.aurora-button__label {
  position: relative;
}`
      },
      {
        id: "aurora-tsx",
        label: "React",
        language: "tsx",
        content: `export function AuroraPulseButton() {
  return (
    <button className="group relative inline-flex items-center justify-center rounded-full border border-white/10 bg-[linear-gradient(135deg,#B67332,#93441A)] px-6 py-3 text-sm font-semibold text-stone-50 shadow-[0_18px_40px_rgba(147,68,26,0.35)] transition-transform duration-200 hover:-translate-y-0.5">
      <span className="absolute inset-0 rounded-full bg-[radial-gradient(circle_at_50%_50%,rgba(255,245,232,0.32),transparent_62%)] opacity-80 blur-md transition-opacity duration-200 group-hover:opacity-100" />
      <span className="relative">Launch workspace</span>
    </button>
  );
}`
      }
    ],
    primaryFileId: "aurora-tsx",
    previewKind: "button",
    previewLabel: "React component preview",
    previewNote: "Interactive public snippet preview built with React + Tailwind.",
    previewActionLabel: "Launch workspace"
  },
  {
    id: "alice-signal-pill-toggle",
    title: "Signal pill toggle",
    description: "A segmented toggle with soft copper selection and compact state labels.",
    fullDescription:
      "A front-end control designed for dashboards and builders that need a compact segmented switch. It keeps the clickable zones generous and uses a discreet state glow so the active option reads instantly.",
    author: "@alice",
    authorName: "Alice Rowan",
    library: "@alice/react-ui-kit",
    libraryName: "React UI Kit",
    language: "TypeScript",
    framework: "React + Tailwind",
    domain: "Front-end",
    appreciation: 91,
    saves: 241,
    canonicalReference: "@alice/react-ui-kit:signal-pill-toggle",
    scope: "community",
    files: [
      {
        id: "signal-html",
        label: "HTML",
        language: "html",
        content: `<div class="signal-pill" role="tablist">
  <button class="signal-pill__option is-active" role="tab" aria-selected="true">public</button>
  <button class="signal-pill__option" role="tab" aria-selected="false">private</button>
</div>`
      },
      {
        id: "signal-css",
        label: "CSS",
        language: "css",
        content: `.signal-pill {
  display: inline-flex;
  border-radius: 9999px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(12, 10, 9, 0.6);
  padding: 4px;
}

.signal-pill__option {
  border: 0;
  background: transparent;
  color: #d6d3d1;
  padding: 0.5rem 1rem;
  border-radius: 9999px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: background 150ms ease, color 150ms ease;
}

.signal-pill__option.is-active {
  background: linear-gradient(135deg, #B67332, #93441A);
  color: #ffffff;
}`
      },
      {
        id: "signal-tsx",
        label: "React",
        language: "tsx",
        content: `type SignalPillToggleProps = {
  value: "public" | "private";
  onChange: (value: "public" | "private") => void;
};

export function SignalPillToggle({ value, onChange }: SignalPillToggleProps) {
  return (
    <div className="inline-flex rounded-full border border-white/10 bg-stone-950/60 p-1">
      {["public", "private"].map((item) => (
        <button
          key={item}
          onClick={() => onChange(item as "public" | "private")}
          className={\`rounded-full px-4 py-2 text-sm transition \${value === item ? "bg-[linear-gradient(135deg,#B67332,#93441A)] text-white" : "text-stone-300"}\`}
        >
          {item}
        </button>
      ))}
    </div>
  );
}`
      }
    ],
    primaryFileId: "signal-tsx",
    previewKind: "button",
    previewLabel: "React component preview",
    previewNote: "Interactive public snippet preview built with React + Tailwind.",
    previewActionLabel: "Public"
  },
  {
    id: "nox-axum-auth-guard",
    title: "Axum auth guard",
    description: "Reusable request guard for authenticated endpoints with clean extractor ergonomics.",
    fullDescription:
      "A backend-oriented public snippet showing a clean extractor pattern for authenticated routes in Axum. It is useful when you want a readable gateway between session validation and protected handlers.",
    author: "@nox",
    authorName: "Nox Mercer",
    library: "@nox/rust-backend-core",
    libraryName: "Rust Backend Core",
    language: "Rust",
    framework: "Axum",
    domain: "Back-end",
    appreciation: 89,
    saves: 204,
    canonicalReference: "@nox/rust-backend-core:auth-guard-v1",
    scope: "community",
    files: [
      {
        id: "auth-guard-rs",
        label: "auth_guard.rs",
        language: "rust",
        content: `use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

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
}`
      }
    ],
    previewKind: "backend",
    previewLabel: "Rust backend asset",
    previewNote: "Read-only backend asset. The preview shows the language emblem instead of live UI."
  },
  {
    id: "nox-session-cookie-config",
    title: "Session cookie config",
    description: "A compact cookie/session setup for browser-safe auth flows in Rust APIs.",
    fullDescription:
      "A reusable back-end snippet to centralize cookie lifetime, same-site mode, and secure defaults. It is meant for apps that need a clear auth foundation before layering providers or public/private scopes on top.",
    author: "@nox",
    authorName: "Nox Mercer",
    library: "@nox/rust-backend-core",
    libraryName: "Rust Backend Core",
    language: "Rust",
    framework: "Axum",
    domain: "Back-end",
    appreciation: 84,
    saves: 166,
    canonicalReference: "@nox/rust-backend-core:session-cookie-config",
    scope: "community",
    files: [
      {
        id: "session-cookie-rs",
        label: "session_cookie.rs",
        language: "rust",
        content: `pub fn build_session_cookie(name: &str, value: &str) -> Cookie<'static> {
    Cookie::build((name.to_string(), value.to_string()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7))
        .build()
}`
      }
    ],
    previewKind: "backend",
    previewLabel: "Rust backend asset",
    previewNote: "Read-only backend asset. The preview shows the language emblem instead of live UI."
  },
  {
    id: "mira-sql-migration-pack",
    title: "SQL migration pack",
    description: "Battle-tested migration starter for app schemas with users, assets, and provenance.",
    fullDescription:
      "A database starter asset for applications that need users, libraries, snippets, and provenance from day one. It is structured to stay readable for teams while leaving room for versioning and trust metadata.",
    author: "@mira",
    authorName: "Mira Solberg",
    library: "@mira/database-patterns",
    libraryName: "Database Patterns",
    language: "SQL",
    framework: "PostgreSQL",
    domain: "Database",
    appreciation: 86,
    saves: 188,
    canonicalReference: "@mira/database-patterns:starter-schema-core",
    scope: "community",
    files: [
      {
        id: "starter-schema-sql",
        label: "starter_schema.sql",
        language: "sql",
        content: `create table libraries (
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
);`
      }
    ],
    previewKind: "database",
    previewLabel: "Database asset",
    previewNote: "Read-only database asset. The preview shows the language emblem instead of live UI."
  },
  {
    id: "mira-query-audit-trigger",
    title: "Query audit trigger",
    description: "A PostgreSQL audit trigger pattern to keep provenance traces on sensitive tables.",
    fullDescription:
      "A database-side snippet that captures write activity into an audit table without spreading the logic through application code. It is useful when public/private assets need change provenance across a team.",
    author: "@mira",
    authorName: "Mira Solberg",
    library: "@mira/database-patterns",
    libraryName: "Database Patterns",
    language: "SQL",
    framework: "PostgreSQL",
    domain: "Database",
    appreciation: 82,
    saves: 149,
    canonicalReference: "@mira/database-patterns:query-audit-trigger",
    scope: "community",
    files: [
      {
        id: "audit-trigger-sql",
        label: "audit_trigger.sql",
        language: "sql",
        content: `create function log_library_change() returns trigger as $$
begin
  insert into audit_log(table_name, row_id, action)
  values (tg_table_name, new.id, tg_op);
  return new;
end;
$$ language plpgsql;`
      }
    ],
    previewKind: "database",
    previewLabel: "Database asset",
    previewNote: "Read-only database asset. The preview shows the language emblem instead of live UI."
  }
];

function mapLanguageToFileLanguage(language: string): SnippetFile["language"] {
  const normalized = language.trim().toLowerCase();
  switch (normalized) {
    case "rust":
      return "rust";
    case "sql":
    case "postgresql":
      return "sql";
    case "typescript":
    case "ts":
      return "ts";
    case "tsx":
      return "tsx";
    case "javascript":
    case "js":
      return "js";
    case "jsx":
      return "jsx";
    case "html":
      return "html";
    case "css":
      return "css";
    case "bash":
    case "sh":
    case "shell":
      return "bash";
    case "yaml":
    case "yml":
      return "yaml";
    case "json":
      return "json";
    default:
      return "plaintext";
  }
}

function toCommunitySnippet(item: SnippetDetail, index: number): CommunitySnippet {
  const framework = item.snippet.framework ?? null;
  const publicScope = item.snippet.visibility === "public" ? "community" : "private";
  const libraryRef = item.canonicalReference.split(":")[0] ?? item.snippet.slug;
  const fileLanguage = mapLanguageToFileLanguage(item.snippet.language);
  const fileId = `${item.snippet.id}-source`;

  return {
    id: item.snippet.id,
    title: item.snippet.name,
    description: item.snippet.description ?? item.snippet.slug,
    fullDescription:
      item.snippet.description ??
      "Public snippet imported from your workspace. Open it to inspect its current raw source and canonical reference.",
    author: "@you",
    authorName: "You",
    library: libraryRef,
    libraryName: item.snippet.name,
    language: item.snippet.language,
    framework,
    domain: item.snippet.domain,
    appreciation: Math.max(72, 96 - index * 3),
    saves: Math.max(48, 160 - index * 11),
    canonicalReference: item.canonicalReference,
    scope: publicScope,
    files: [
      {
        id: fileId,
        label: framework ?? item.snippet.language,
        language: fileLanguage,
        content:
          item.currentVersion?.code ??
          "// No code snapshot available yet for this public snippet."
      }
    ],
    primaryFileId: fileId,
    previewKind: "backend",
    previewLabel: framework ?? item.snippet.language,
    previewNote: "Imported public snippet from your workspace. This preview is read-only."
  };
}

export function buildCommunityFeed(snippets: SnippetDetail[]): CommunitySnippet[] {
  const derived = snippets
    .filter((item) => item.snippet.visibility === "public")
    .slice(0, 6)
    .map(toCommunitySnippet);

  return [...derived, ...COMMUNITY_SEED];
}

export function featuredCommunitySnippets(snippets: CommunitySnippet[]): CommunitySnippet[] {
  return [...snippets].sort((a, b) => b.appreciation - a.appreciation).slice(0, 3);
}

export function buildPublicLibraries(snippets: CommunitySnippet[]): PublicLibraryProfile[] {
  const grouped = new Map<string, PublicLibraryProfile>();

  for (const snippet of snippets) {
    const key = snippet.library;
    const existing = grouped.get(key);

    if (!existing) {
      grouped.set(key, {
        id: key,
        author: snippet.author,
        authorName: snippet.authorName,
        library: snippet.library,
        libraryName: snippet.libraryName,
        bio: `${snippet.authorName} shares reusable ${snippet.domain.toLowerCase()} assets for teams that want to start from proven building blocks.`,
        snippetCount: 1,
        languages: [snippet.language],
        domains: [snippet.domain],
        snippets: [snippet]
      });
      continue;
    }

    existing.snippetCount += 1;
    existing.snippets.push(snippet);
    if (!existing.languages.includes(snippet.language)) {
      existing.languages.push(snippet.language);
    }
    if (!existing.domains.includes(snippet.domain)) {
      existing.domains.push(snippet.domain);
    }
  }

  return [...grouped.values()].sort((a, b) => b.snippetCount - a.snippetCount);
}
