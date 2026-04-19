import { useMemo, useState } from "react";
import type { CommunitySnippet, CopyBlock, SnippetFile } from "../../../lib/app-types";
import { LanguageLogo } from "../../../lib/language-logo";

type PublicSnippetViewProps = {
  copy: CopyBlock;
  snippet: CommunitySnippet;
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
    <div className="public-snippet-preview public-snippet-preview-language">
      <span>{snippet.previewLabel}</span>
      <LanguageLogo
        language={snippet.language}
        framework={snippet.framework}
        size={96}
      />
      <strong>{snippet.framework ?? snippet.language}</strong>
      <p>{snippet.previewNote}</p>
    </div>
  );
}

function resolveInitialFile(files: SnippetFile[], primaryFileId?: string): SnippetFile {
  if (primaryFileId) {
    const match = files.find((file) => file.id === primaryFileId);
    if (match) return match;
  }
  return files[0];
}

export function PublicSnippetView({ copy, snippet }: PublicSnippetViewProps) {
  const initialFile = useMemo(
    () => resolveInitialFile(snippet.files, snippet.primaryFileId),
    [snippet.files, snippet.primaryFileId]
  );
  const [activeFileId, setActiveFileId] = useState<string>(initialFile.id);
  const activeFile =
    snippet.files.find((file) => file.id === activeFileId) ?? initialFile;
  const hasMultipleFiles = snippet.files.length > 1;

  return (
    <section className="app-view-shell app-page public-snippet-page">
      <section className="public-snippet-hero">
        <div className="public-snippet-copy">
          <div className="public-snippet-actions">
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
          {hasMultipleFiles ? (
            <div className="public-snippet-tabs" role="tablist" aria-label="Snippet files">
              {snippet.files.map((file) => {
                const isActive = file.id === activeFile.id;
                return (
                  <button
                    key={file.id}
                    type="button"
                    role="tab"
                    aria-selected={isActive}
                    className={
                      isActive
                        ? "public-snippet-tab public-snippet-tab-active"
                        : "public-snippet-tab"
                    }
                    onClick={() => setActiveFileId(file.id)}
                  >
                    {file.label}
                  </button>
                );
              })}
            </div>
          ) : null}
          <pre className="public-snippet-code" data-language={activeFile.language}>
            <code>{activeFile.content}</code>
          </pre>
        </article>
      </section>
    </section>
  );
}
