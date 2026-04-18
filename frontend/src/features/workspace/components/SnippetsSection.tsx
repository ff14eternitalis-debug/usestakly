import { getVisibilityLabel } from "../../../lib/app-copy";
import type { CopyBlock, LibraryRecord, SnippetDetail } from "../../../lib/app-types";
import { CreateSnippetForm } from "./CreateSnippetForm";

type SnippetsSectionProps = {
  copy: CopyBlock;
  libraries: LibraryRecord[];
  snippets: SnippetDetail[];
  selectedSnippetId: string | null;
  onSelectSnippet: (snippet: SnippetDetail) => void;
  onCreateSnippet: (input: {
    libraryId: string;
    slug: string;
    name: string;
    domain: string;
    kind: string;
    category: string;
    language: string;
    framework?: string;
    tags: string[];
    version: string;
    code: string;
  }) => Promise<void>;
};

export function SnippetsSection({
  copy,
  libraries,
  snippets,
  selectedSnippetId,
  onSelectSnippet,
  onCreateSnippet
}: SnippetsSectionProps) {
  return (
    <section className="workspace-section workspace-section-wide">
      <div className="workspace-section-head">
        <div>
          <h2>{copy.snippetsTitle}</h2>
          <p>{copy.snippetsBody}</p>
        </div>
      </div>

      <div className="workspace-form-shell">
        <h3 className="workspace-subtitle">{copy.createSnippetTitle}</h3>
        <CreateSnippetForm copy={copy} libraries={libraries} onCreate={onCreateSnippet} />
      </div>

      {snippets.length === 0 ? (
        <div className="workspace-empty">{copy.emptySnippets}</div>
      ) : (
        <div className="snippet-grid">
          {snippets.slice(0, 6).map((item) => (
            <article
              className={`snippet-row${selectedSnippetId === item.snippet.id ? " snippet-row-selected" : ""}`}
              key={item.snippet.id}
              onClick={() => onSelectSnippet(item)}
              onKeyDown={(event) => {
                if (event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  onSelectSnippet(item);
                }
              }}
              role="button"
              tabIndex={0}
            >
              <div className="snippet-head">
                <div>
                  <h3>{item.snippet.name}</h3>
                  <p>{item.snippet.description ?? item.snippet.slug}</p>
                </div>
                <span className="workspace-chip">{item.snippet.domain}</span>
              </div>

              <div className="snippet-metadata">
                <span className="workspace-chip">{item.snippet.language}</span>
                {item.snippet.framework ? (
                  <span className="workspace-chip">{item.snippet.framework}</span>
                ) : null}
                <span className="workspace-chip">
                  {getVisibilityLabel(copy, item.snippet.visibility)}
                </span>
                <span className="workspace-chip">
                  {item.currentVersion
                    ? `${copy.versionLabel} ${item.currentVersion.version}`
                    : copy.readyStat}
                </span>
              </div>

              <div className="snippet-reference">
                <span>{copy.referenceLabel}</span>
                <code>{item.canonicalReference}</code>
              </div>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
