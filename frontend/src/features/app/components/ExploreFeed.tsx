import type { CopyBlock, PublicLibraryProfile } from "../../../lib/app-types";

type ExploreFeedProps = {
  copy: CopyBlock;
  publicLibraries: PublicLibraryProfile[];
  onOpenLibrary: (library: PublicLibraryProfile) => void;
};

function avatarMonogram(name: string) {
  return name
    .split(" ")
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? "")
    .join("");
}

export function ExploreFeed({ copy, publicLibraries, onOpenLibrary }: ExploreFeedProps) {
  return (
    <section className="app-view-shell app-page">
      <div className="workspace-section workspace-section-wide explore-shell">
        <div className="workspace-section-head">
          <div>
            <h1 className="workspace-title workspace-title-compact">{copy.exploreTitle}</h1>
            <p>{copy.exploreBody}</p>
          </div>
        </div>

        {publicLibraries.length === 0 ? (
          <div className="workspace-empty">{copy.exploreEmpty}</div>
        ) : (
          <div className="public-library-grid">
            {publicLibraries.map((library) => (
              <article className="public-library-card" key={library.id}>
                <button
                  className="public-library-author"
                  type="button"
                  onClick={() => onOpenLibrary(library)}
                >
                  <div className="public-library-avatar">{avatarMonogram(library.authorName)}</div>
                  <div className="public-library-author-copy">
                    <strong>{library.authorName}</strong>
                    <span>{library.author}</span>
                  </div>
                </button>

                <div className="public-library-card-copy">
                  <h3>{library.libraryName}</h3>
                  <p>{library.bio}</p>
                </div>

                <div className="community-snippet-meta">
                  {library.languages.map((language) => (
                    <span className="workspace-chip" key={language}>
                      {language}
                    </span>
                  ))}
                  {library.domains.map((domain) => (
                    <span className="workspace-chip" key={domain}>
                      {domain}
                    </span>
                  ))}
                </div>

                <div className="public-library-footer">
                  <div className="featured-snippet-score">
                    <strong>{library.snippetCount}</strong>
                    <span>{copy.snippetsStat}</span>
                  </div>
                  <button
                    className="public-library-open"
                    type="button"
                    onClick={() => onOpenLibrary(library)}
                  >
                    {copy.exploreOpenLibrary}
                  </button>
                </div>
              </article>
            ))}
          </div>
        )}
      </div>
    </section>
  );
}
