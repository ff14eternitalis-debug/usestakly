import type { CommunitySnippet, CopyBlock } from "../../../lib/app-types";

type ExploreFeedProps = {
  copy: CopyBlock;
  communitySnippets: CommunitySnippet[];
};

export function ExploreFeed({ copy, communitySnippets }: ExploreFeedProps) {
  return (
    <section className="app-view-shell">
      <div className="workspace-section workspace-section-wide">
        <div className="workspace-section-head">
          <div>
            <h1 className="workspace-title workspace-title-compact">{copy.exploreTitle}</h1>
            <p>{copy.exploreBody}</p>
          </div>
        </div>

        {communitySnippets.length === 0 ? (
          <div className="workspace-empty">{copy.exploreEmpty}</div>
        ) : (
          <div className="community-snippet-list">
            {communitySnippets.map((item) => (
              <article className="community-snippet-card" key={item.id}>
                <div className="community-snippet-head">
                  <div>
                    <h3>{item.title}</h3>
                    <p>{item.description}</p>
                  </div>
                  <span className="workspace-chip">
                    {item.scope === "community" ? copy.homeScopeCommunity : copy.homeScopePrivate}
                  </span>
                </div>

                <div className="community-snippet-meta">
                  <span className="workspace-chip">{item.library}</span>
                  <span className="workspace-chip">{item.language}</span>
                  {item.framework ? <span className="workspace-chip">{item.framework}</span> : null}
                </div>

                <div className="snippet-reference">
                  <span>{copy.referenceLabel}</span>
                  <code>{item.canonicalReference}</code>
                </div>
              </article>
            ))}
          </div>
        )}
      </div>
    </section>
  );
}
