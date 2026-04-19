import type { CommunitySnippet, CopyBlock } from "../../../lib/app-types";

type HomeFeedProps = {
  copy: CopyBlock;
  featuredSnippets: CommunitySnippet[];
};

export function HomeFeed({ copy, featuredSnippets }: HomeFeedProps) {
  return (
    <section className="app-view-shell app-page">
      <section className="home-hero">
        <div className="home-hero-copy">
          <span className="workspace-status-pill">{copy.homeFeaturedTitle}</span>
          <h1 className="home-title">{copy.homeTitle}</h1>
          <p className="home-subcopy">{copy.homeBody}</p>
        </div>

        <div className="home-hero-aside">
          <h2>{copy.homeHighlightsTitle}</h2>
          <p>{copy.homeHighlightsBody}</p>
          <div className="home-onboarding-list">
            <article className="home-onboarding-item">
              <strong>{copy.navExplore}</strong>
              <p>{copy.homeOnboardingExplore}</p>
            </article>
            <article className="home-onboarding-item">
              <strong>{copy.navStudio}</strong>
              <p>{copy.homeOnboardingStudio}</p>
            </article>
            <article className="home-onboarding-item">
              <strong>{copy.navProfile}</strong>
              <p>{copy.homeOnboardingProfile}</p>
            </article>
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
              <article className="featured-snippet-card" key={item.id}>
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
              </article>
            ))}
          </div>
        )}
      </section>
    </section>
  );
}
