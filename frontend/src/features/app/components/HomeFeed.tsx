import type { CommunitySnippet, CopyBlock } from "../../../lib/app-types";

type HomeFeedProps = {
  copy: CopyBlock;
  featuredSnippets: CommunitySnippet[];
  onOpenSnippet: (snippet: CommunitySnippet) => void;
};

export function HomeFeed({ copy, featuredSnippets, onOpenSnippet }: HomeFeedProps) {
  return (
    <section className="app-view-shell app-page">
      <section className="home-hero">
        <div className="home-hero-copy">
          <h1 className="home-title">{copy.homeTitle}</h1>
          <p className="home-subcopy">{copy.homeBody}</p>
          <div className="home-hero-support">
            <strong>{copy.homeSecondaryTitle}</strong>
            <p>{copy.homeSecondaryBody}</p>
          </div>
        </div>
      </section>

      <section className="app-section">
        <div className="app-section-head">
          <div>
            <h2>{copy.homeFeaturedTitle}</h2>
            <p>{copy.homeFeaturedBody}</p>
          </div>
        </div>

        {featuredSnippets.length === 0 ? (
          <div className="workspace-empty">{copy.homeEmpty}</div>
        ) : (
          <div className="featured-snippet-grid">
            {featuredSnippets.map((item) => (
              <button
                className="featured-snippet-card"
                key={item.id}
                type="button"
                onClick={() => onOpenSnippet(item)}
              >
                <div className="featured-snippet-top">
                  <span className="workspace-chip workspace-chip-accent">
                    {item.scope === "community" ? copy.homeScopeCommunity : copy.homeScopePrivate}
                  </span>
                  <div className="featured-snippet-score">
                    <strong>{item.appreciation}</strong>
                    <span>{copy.homeTrendingLabel}</span>
                  </div>
                </div>

                <div>
                  <h3>{item.title}</h3>
                  <p>{item.description}</p>
                </div>

                <div className="community-snippet-meta">
                  <span className="workspace-chip">{item.language}</span>
                  {item.framework ? <span className="workspace-chip">{item.framework}</span> : null}
                  <span className="workspace-chip">{item.author}</span>
                </div>

                <div className="snippet-reference">
                  <span>{copy.homeReferenceLabel}</span>
                  <code>{item.canonicalReference}</code>
                </div>

                <span className="featured-snippet-link">{copy.homeOpenSnippet}</span>
              </button>
            ))}
          </div>
        )}
      </section>
    </section>
  );
}
