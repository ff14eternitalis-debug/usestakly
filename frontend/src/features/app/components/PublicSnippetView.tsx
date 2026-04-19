import type { CommunitySnippet, CopyBlock } from "../../../lib/app-types";

type PublicSnippetViewProps = {
  copy: CopyBlock;
  snippet: CommunitySnippet;
  onBack: () => void;
};

function PublicSnippetRender({ copy, snippet }: { copy: CopyBlock; snippet: CommunitySnippet }) {
  if (snippet.previewKind === "button") {
    return (
      <div className="public-snippet-preview public-snippet-preview-button">
        <div className="public-snippet-preview-copy">
          <span>{snippet.previewLabel}</span>
          <p>{copy.snippetRenderBody}</p>
        </div>
        <button className="public-snippet-demo-button" type="button">
          <span className="public-snippet-demo-glow" />
          <span>{snippet.previewActionLabel ?? snippet.title}</span>
        </button>
      </div>
    );
  }

  return (
    <div className="public-snippet-preview public-snippet-preview-readonly">
      <span>{snippet.previewLabel}</span>
      <div className="public-snippet-language-mark" aria-hidden="true">
        {snippet.language.slice(0, 3).toUpperCase()}
      </div>
      <strong>{snippet.title}</strong>
      <p>{snippet.previewNote}</p>
    </div>
  );
}

export function PublicSnippetView({ copy, snippet, onBack }: PublicSnippetViewProps) {
  return (
    <section className="app-view-shell app-page public-snippet-page">
      <section className="public-snippet-hero">
        <div className="public-snippet-copy">
          <div className="public-snippet-actions">
            <button className="public-snippet-back" type="button" onClick={onBack}>
              ← {copy.snippetBack}
            </button>
            <div className="public-snippet-pills">
              <span className="workspace-status-pill">{copy.snippetReadonly}</span>
              <span className="workspace-status-pill">
                {snippet.scope === "community" ? copy.homeScopeCommunity : copy.homeScopePrivate}
              </span>
            </div>
          </div>
          <h1 className="public-snippet-title">{snippet.title}</h1>
          <p className="public-snippet-subcopy">{snippet.description}</p>

          <div className="public-snippet-meta">
            <div>
              <span>{copy.snippetAuthorLabel}</span>
              <strong>{snippet.author}</strong>
            </div>
            <div>
              <span>{copy.snippetLibraryLabel}</span>
              <strong>{snippet.library}</strong>
            </div>
            <div>
              <span>{copy.snippetStackLabel}</span>
              <strong>{snippet.framework ?? snippet.language}</strong>
            </div>
          </div>
        </div>

        <div className="public-snippet-reference-card">
          <span>{copy.snippetOpenSourceLabel}</span>
          <code>{snippet.canonicalReference}</code>
          <p>{snippet.previewNote}</p>
        </div>
      </section>

      <section className="public-snippet-layout public-snippet-layout-three">
        <article className="public-snippet-section">
          <div className="app-section-head">
            <div>
              <h2>{copy.snippetRenderTitle}</h2>
              <p>{copy.snippetRenderBody}</p>
            </div>
          </div>
          <PublicSnippetRender copy={copy} snippet={snippet} />
        </article>

        <article className="public-snippet-section">
          <div className="app-section-head">
            <div>
              <h2>{copy.snippetSummaryTitle}</h2>
            </div>
          </div>
          <div className="public-snippet-summary">
            <p>{snippet.fullDescription}</p>
          </div>
        </article>

        <article className="public-snippet-section">
          <div className="app-section-head">
            <div>
              <h2>{copy.snippetCodeTitle}</h2>
            </div>
          </div>
          <pre className="public-snippet-code">
            <code>{snippet.rawCode}</code>
          </pre>
        </article>
      </section>
    </section>
  );
}
