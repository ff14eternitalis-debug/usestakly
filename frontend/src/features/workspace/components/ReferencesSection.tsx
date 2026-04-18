import type { CopyBlock, SnippetDetail } from "../../../lib/app-types";

type ReferencesSectionProps = {
  copy: CopyBlock;
  recentSnippets: SnippetDetail[];
};

export function ReferencesSection({
  copy,
  recentSnippets
}: ReferencesSectionProps) {
  return (
    <section className="workspace-section">
      <div className="workspace-section-head">
        <div>
          <h2>{copy.recentTitle}</h2>
          <p>{copy.recentBody}</p>
        </div>
      </div>

      {recentSnippets.length === 0 ? (
        <div className="workspace-empty">{copy.emptySnippets}</div>
      ) : (
        <div className="reference-list">
          {recentSnippets.map((item) => (
            <article className="reference-row" key={item.snippet.id}>
              <div>
                <strong>{item.snippet.name}</strong>
                <span>{item.snippet.language}</span>
              </div>
              <code>{item.canonicalReference}</code>
              <p>
                {copy.tagsLabel}: {item.tags.length > 0 ? item.tags.join(", ") : "none"}
              </p>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
