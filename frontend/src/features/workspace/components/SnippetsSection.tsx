import { getVisibilityLabel } from "../../../lib/app-copy";
import type { CopyBlock, SnippetDetail } from "../../../lib/app-types";

type SnippetsSectionProps = {
  copy: CopyBlock;
  snippets: SnippetDetail[];
};

export function SnippetsSection({ copy, snippets }: SnippetsSectionProps) {
  return (
    <section className="workspace-section workspace-section-wide">
      <div className="workspace-section-head">
        <div>
          <h2>{copy.snippetsTitle}</h2>
          <p>{copy.snippetsBody}</p>
        </div>
      </div>

      {snippets.length === 0 ? (
        <div className="workspace-empty">{copy.emptySnippets}</div>
      ) : (
        <div className="snippet-grid">
          {snippets.slice(0, 6).map((item) => (
            <article className="snippet-row" key={item.snippet.id}>
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
