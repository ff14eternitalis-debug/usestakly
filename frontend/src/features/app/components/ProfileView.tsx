import type { CopyBlock, CurrentUser } from "../../../lib/app-types";

type ProfileViewProps = {
  copy: CopyBlock;
  user: CurrentUser;
  privateAssetCount: number;
  publicAssetCount: number;
  onLogout: () => void;
};

export function ProfileView({
  copy,
  user,
  privateAssetCount,
  publicAssetCount,
  onLogout
}: ProfileViewProps) {
  return (
    <section className="app-view-shell">
      <div className="community-grid">
        <section className="workspace-section workspace-section-wide">
          <div className="workspace-section-head">
            <div>
              <h1 className="workspace-title workspace-title-compact">{copy.profileTitle}</h1>
              <p>{copy.profileBody}</p>
            </div>
          </div>

          <div className="profile-grid">
            <article className="reference-row">
              <div className="reference-row-main">
                <span>{copy.profileIdentity}</span>
                <code>{user.displayName ?? user.username}</code>
                <p>{copy.profilePresence}</p>
              </div>
            </article>
            <article className="reference-row">
              <div className="reference-row-main">
                <span>{copy.profileEmail}</span>
                <code>{user.email}</code>
              </div>
            </article>
            <article className="reference-row">
              <div className="reference-row-main">
                <span>{copy.profileHandle}</span>
                <code>@{user.username}</code>
              </div>
            </article>
          </div>
        </section>

        <section className="workspace-section">
          <div className="workspace-section-head">
            <div>
              <h2>{copy.profilePresence}</h2>
              <p>{copy.connectedBody}</p>
            </div>
          </div>

          <div className="workspace-stats profile-stats">
            <div>
              <span>{copy.profilePrivateLabel}</span>
              <strong>{privateAssetCount}</strong>
            </div>
            <div>
              <span>{copy.profilePublicLabel}</span>
              <strong>{publicAssetCount}</strong>
            </div>
          </div>

          <div className="profile-actions">
            <button className="workspace-logout" type="button" onClick={onLogout}>
              {copy.logoutSecondary}
            </button>
          </div>
        </section>
      </div>
    </section>
  );
}
