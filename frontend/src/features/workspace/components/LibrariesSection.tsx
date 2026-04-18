import { getTrustLabel, getVisibilityLabel } from "../../../lib/app-copy";
import type { CopyBlock, LibraryRecord } from "../../../lib/app-types";

type LibrariesSectionProps = {
  copy: CopyBlock;
  libraries: LibraryRecord[];
  workspaceLoading: boolean;
};

export function LibrariesSection({
  copy,
  libraries,
  workspaceLoading
}: LibrariesSectionProps) {
  return (
    <section className="workspace-section workspace-section-wide">
      <div className="workspace-section-head">
        <div>
          <h2>{copy.librariesTitle}</h2>
          <p>{copy.librariesBody}</p>
        </div>
        {workspaceLoading ? <span className="workspace-inline-note">{copy.loading}</span> : null}
      </div>

      {libraries.length === 0 ? (
        <div className="workspace-empty">{copy.emptyLibraries}</div>
      ) : (
        <div className="library-list">
          {libraries.map((library) => (
            <article className="library-row" key={library.id}>
              <div className="library-row-main">
                <div className="library-row-head">
                  <h3>{library.name}</h3>
                  {library.isDefault ? (
                    <span className="workspace-chip workspace-chip-accent">
                      {copy.defaultLibrary}
                    </span>
                  ) : null}
                </div>
                <p>{library.description ?? library.slug}</p>
              </div>
              <div className="library-row-meta">
                <span className="workspace-chip">{library.slug}</span>
                <span className="workspace-chip">
                  {getVisibilityLabel(copy, library.visibility)}
                </span>
                <span className="workspace-chip">
                  {getTrustLabel(copy, library.trustLevel)}
                </span>
              </div>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
