import type { CommunitySnippet, CopyBlock } from "../../../lib/app-types";

type HomeFeedProps = {
  copy: CopyBlock;
  featuredSnippets: CommunitySnippet[];
};

export function HomeFeed({ copy, featuredSnippets }: HomeFeedProps) {
  return (
    <section className="app-view-shell">
      <div className="workspace-hero">
        <div className="workspace-hero-copy">
          <span className="workspace-status-pill">{copy.homeFeaturedTitle}</span>
          <h1 className="workspace-title">{copy.homeTitle}</h1>
          <p className="workspace-copy">{copy.homeBody}</p>
        </div>

        <div className="workspace-stats">
          <div>
            <span>{copy.homeTrendingLabel}</span>
            <strong>{featuredSnippets[0]?.appreciation ?? 0}</strong>
          </div>
          <div>
            <span>{copy.homeSavedLabel}</span>
            <strong>{featuredSnippets[0]?.saves ?? 0}</strong>
          </div>
          <div>
            <span>{copy.publicStat}</span>
            <strong>{featuredSnippets.filter((item) => item.scope === "community").length}</strong>
          </div>
          <div>
            <span>{copy.readyStat}</span>
            <strong>{featuredSnippets.length}</strong>
          </div>
        </div>
      </div>

      <div className="community-grid">
        <section className="workspace-section workspace-section-wide">
          <div className="workspace-section-head">
            <div>
              <h2>{copy.homeFeaturedTitle}</h2>
              <p>{copy.homeFeaturedBody}</p>
            </div>
          </div>

          {featuredSnippets.length === 0 ? (
            <div className="workspace-empty">{copy.homeEmpty}</div>
          ) : (
            <div className="community-snippet-list">
              {featuredSnippets.map((item) => (
                <article className="community-snippet-card" key={item.id}>
                  <div className="community-snippet-head">
                    <div>
                      <h3>{item.title}</h3>
                      <p>{item.description}</p>
                    </div>
                    <span className="workspace-chip workspace-chip-accent">
                      {item.scope === "community" ? copy.homeScopeCommunity : copy.homeScopePrivate}
                    </span>
                  </div>

                  <div className="community-snippet-meta">
                    <span className="workspace-chip">{item.language}</span>
                    {item.framework ? <span className="workspace-chip">{item.framework}</span> : null}
                    <span className="workspace-chip">{item.author}</span>
                  </div>

                  <div className="community-snippet-stats">
                    <div>
                      <span>{copy.homeTrendingLabel}</span>
                      <strong>{item.appreciation}</strong>
                    </div>
                    <div>
                      <span>{copy.homeSavedLabel}</span>
                      <strong>{item.saves}</strong>
                    </div>
                  </div>

                  <div className="snippet-reference">
                    <span>{copy.homeReferenceLabel}</span>
                    <code>{item.canonicalReference}</code>
                  </div>
                </article>
              ))}
            </div>
          )}
        </section>

        <section className="workspace-section">
          <div className="workspace-section-head">
            <div>
              <h2>{copy.homeHighlightsTitle}</h2>
              <p>{copy.homeHighlightsBody}</p>
            </div>
          </div>

          <div className="community-highlight-list">
            {featuredSnippets.slice(0, 3).map((item) => (
              <article className="reference-row" key={`${item.id}-highlight`}>
                <div className="reference-row-main">
                  <span>{item.library}</span>
                  <code>{item.canonicalReference}</code>
                  <p>{item.description}</p>
                </div>
              </article>
            ))}
          </div>
        </section>
      </div>
    </section>
  );
}
