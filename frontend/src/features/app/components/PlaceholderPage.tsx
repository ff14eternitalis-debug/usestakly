import type { CopyBlock } from "../../../lib/app-types";

type PlaceholderPageProps = {
  eyebrow: string;
  title: string;
  body: string;
  copy: CopyBlock;
};

export function PlaceholderPage({
  eyebrow,
  title,
  body,
  copy
}: PlaceholderPageProps) {
  return (
    <section className="app-view-shell app-page">
      <section className="app-section">
        <div className="app-section-head">
          <div>
            <span className="workspace-status-pill">{eyebrow}</span>
            <h1 className="workspace-title workspace-title-compact">{title}</h1>
            <p>{body}</p>
          </div>
        </div>

        <div className="workspace-empty placeholder-page-box">{copy.pageInProgress}</div>
      </section>
    </section>
  );
}
