import type { CopyBlock, LibraryRecord, SnippetDetail } from "../../../lib/app-types";

type SnippetDetailPanelProps = {
  copy: CopyBlock;
  libraries: LibraryRecord[];
  selectedSnippet: SnippetDetail | null;
};

export function SnippetDetailPanel({
  copy,
  libraries,
  selectedSnippet
}: SnippetDetailPanelProps) {
  if (!selectedSnippet) {
    return (
      <section className="workspace-section">
        <div className="workspace-section-head">
          <div>
            <h2>{copy.detailTitle}</h2>
            <p>{copy.detailEmpty}</p>
          </div>
        </div>
      </section>
    );
  }

  const library = libraries.find(
    (entry) => entry.id === selectedSnippet.snippet.libraryId
  );

  return (
    <section className="workspace-section">
      <div className="workspace-section-head">
        <div>
          <h2>{copy.detailTitle}</h2>
          <p>{selectedSnippet.snippet.name}</p>
        </div>
      </div>

      <div className="detail-stack">
        <div className="detail-block">
          <span>{copy.referenceLabel}</span>
          <code>{selectedSnippet.canonicalReference}</code>
        </div>
        <div className="detail-block">
          <span>{copy.detailLibrary}</span>
          <strong>{library?.name ?? selectedSnippet.snippet.libraryId}</strong>
        </div>
        <div className="detail-block">
          <span>{copy.detailDescription}</span>
          <p>{selectedSnippet.snippet.description ?? selectedSnippet.snippet.slug}</p>
        </div>
        <div className="detail-block">
          <span>{copy.detailRisk}</span>
          <strong>{selectedSnippet.currentVersion?.riskLevel ?? "safe"}</strong>
        </div>
        <div className="detail-block">
          <span>{copy.detailCode}</span>
          <pre>{selectedSnippet.currentVersion?.code ?? ""}</pre>
        </div>
      </div>
    </section>
  );
}
