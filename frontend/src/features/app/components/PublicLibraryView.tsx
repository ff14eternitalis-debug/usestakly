import { useMemo, useState } from "react";

import type { CommunitySnippet, CopyBlock, PublicLibraryProfile } from "../../../lib/app-types";

type PublicLibraryViewProps = {
  copy: CopyBlock;
  library: PublicLibraryProfile;
  onBack: () => void;
  onOpenSnippet: (snippet: CommunitySnippet) => void;
};

export function PublicLibraryView({
  copy,
  library,
  onBack,
  onOpenSnippet
}: PublicLibraryViewProps) {
  const [languageFilter, setLanguageFilter] = useState<string>("all");
  const [domainFilter, setDomainFilter] = useState<string>("all");

  const filteredSnippets = useMemo(
    () =>
      library.snippets.filter((snippet) => {
        const matchesLanguage =
          languageFilter === "all" || snippet.language === languageFilter;
        const matchesDomain = domainFilter === "all" || snippet.domain === domainFilter;
        return matchesLanguage && matchesDomain;
      }),
    [domainFilter, languageFilter, library.snippets]
  );

  return (
    <section className="app-view-shell app-page public-library-page">
      <section className="public-library-hero">
        <div className="public-library-hero-copy">
          <button className="public-snippet-back" type="button" onClick={onBack}>
            ← {copy.libraryBack}
          </button>
          <span className="workspace-status-pill">
            {library.authorName} · {copy.libraryTitleSuffix}
          </span>
          <h1 className="public-snippet-title">{library.libraryName}</h1>
          <p className="public-snippet-subcopy">{copy.libraryBody}</p>
        </div>

        <div className="public-library-hero-aside">
          <div className="public-library-identity">
            <div className="public-library-avatar public-library-avatar-large">
              {library.authorName
                .split(" ")
                .slice(0, 2)
                .map((part) => part[0]?.toUpperCase() ?? "")
                .join("")}
            </div>
            <div>
              <strong>{library.authorName}</strong>
              <span>{library.author}</span>
            </div>
          </div>
          <p>{library.bio}</p>
        </div>
      </section>

      <section className="public-library-toolbar">
        <div className="public-library-filter">
          <span>{copy.libraryFilterLanguage}</span>
          <div className="community-snippet-meta">
            <button
              className={`workspace-chip-button${languageFilter === "all" ? " workspace-chip-button-active" : ""}`}
              type="button"
              onClick={() => setLanguageFilter("all")}
            >
              {copy.libraryAll}
            </button>
            {library.languages.map((language) => (
              <button
                className={`workspace-chip-button${languageFilter === language ? " workspace-chip-button-active" : ""}`}
                key={language}
                type="button"
                onClick={() => setLanguageFilter(language)}
              >
                {language}
              </button>
            ))}
          </div>
        </div>

        <div className="public-library-filter">
          <span>{copy.libraryFilterDomain}</span>
          <div className="community-snippet-meta">
            <button
              className={`workspace-chip-button${domainFilter === "all" ? " workspace-chip-button-active" : ""}`}
              type="button"
              onClick={() => setDomainFilter("all")}
            >
              {copy.libraryAll}
            </button>
            {library.domains.map((domain) => (
              <button
                className={`workspace-chip-button${domainFilter === domain ? " workspace-chip-button-active" : ""}`}
                key={domain}
                type="button"
                onClick={() => setDomainFilter(domain)}
              >
                {domain}
              </button>
            ))}
          </div>
        </div>
      </section>

      {filteredSnippets.length === 0 ? (
        <div className="workspace-empty">{copy.libraryEmpty}</div>
      ) : (
        <div className="community-snippet-list">
          {filteredSnippets.map((snippet) => (
            <button
              className="community-snippet-card"
              key={snippet.id}
              type="button"
              onClick={() => onOpenSnippet(snippet)}
            >
              <div className="community-snippet-head">
                <div>
                  <h3>{snippet.title}</h3>
                  <p>{snippet.description}</p>
                </div>
                <span className="workspace-chip workspace-chip-accent">{snippet.domain}</span>
              </div>

              <div className="community-snippet-meta">
                <span className="workspace-chip">{snippet.language}</span>
                {snippet.framework ? <span className="workspace-chip">{snippet.framework}</span> : null}
                <span className="workspace-chip">{snippet.author}</span>
              </div>

              <div className="snippet-reference">
                <span>{copy.referenceLabel}</span>
                <code>{snippet.canonicalReference}</code>
              </div>

              <span className="featured-snippet-link">{copy.homeOpenSnippet}</span>
            </button>
          ))}
        </div>
      )}
    </section>
  );
}
