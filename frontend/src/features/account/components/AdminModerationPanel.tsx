import { Button } from "../../../components/Button";
import { formatRelative } from "../../../lib/format";
import type { PendingRepoSignal } from "../../../lib/types";

type AdminModerationPanelProps = {
  adminToken: string;
  onAdminTokenChange(value: string): void;
  loading: boolean;
  items: PendingRepoSignal[] | undefined;
  reviewPending: boolean;
  onReview(id: string, action: "approve" | "reject"): void;
  title: string;
  adminTokenLabel: string;
  adminTokenPlaceholder: string;
  loadingLabel: string;
  approveLabel: string;
  rejectLabel: string;
  reviewingLabel: string;
  emptyLabel: string;
};

export function AdminModerationPanel({
  adminToken,
  onAdminTokenChange,
  loading,
  items,
  reviewPending,
  onReview,
  title,
  adminTokenLabel,
  adminTokenPlaceholder,
  loadingLabel,
  approveLabel,
  rejectLabel,
  reviewingLabel,
  emptyLabel
}: AdminModerationPanelProps) {
  return (
    <section className="grid gap-4">
      <span className="kicker">{title}</span>
      <div className="surface grid gap-4 p-5">
        <label className="grid gap-1.5">
          <span className="kicker">{adminTokenLabel}</span>
          <input
            type="password"
            value={adminToken}
            onChange={(e) => onAdminTokenChange(e.target.value)}
            placeholder={adminTokenPlaceholder}
            className="input"
          />
        </label>

        {!adminToken.trim() ? null : loading ? (
          <p className="text-[0.9rem] text-fg-dim">{loadingLabel}</p>
        ) : items?.length ? (
          <ul className="grid gap-3">
            {items.map((item) => (
              <li key={item.id} className="grid gap-3 rounded-[8px] border border-line p-4">
                <div className="flex flex-wrap items-center gap-2">
                  <span className="mono text-[0.86rem] text-fg">
                    {item.owner}/{item.name}
                  </span>
                  <span className="kicker">{item.signal}</span>
                  <span className="kicker">{item.reviewStatus}</span>
                  {item.reporterTier ? (
                    <span className="kicker">
                      reporter {item.reporterTier} · {item.reporterScore?.toFixed(2)}
                    </span>
                  ) : null}
                  {item.hasOwnerDispute && item.ownerDisputeTier ? (
                    <span className="kicker">
                      owner dispute {item.ownerDisputeTier} · {item.ownerDisputeScore?.toFixed(2)}
                    </span>
                  ) : null}
                </div>
                {item.evidenceDescription ? (
                  <p className="text-[0.9rem] text-fg-dim">{item.evidenceDescription}</p>
                ) : null}
                <div className="flex flex-wrap items-center gap-2 text-[0.82rem] text-fg-dim">
                  <span>{item.suggestedAction}</span>
                  {item.reporterUsageSignalCount !== null ? (
                    <span>usage {item.reporterUsageSignalCount}</span>
                  ) : null}
                  {item.hasOwnerDispute && item.ownerDisputeUsageSignalCount !== null ? (
                    <span>owner usage {item.ownerDisputeUsageSignalCount}</span>
                  ) : null}
                  {item.needsStrictReview ? <span>strict review</span> : null}
                </div>
                <div className="flex flex-wrap items-center gap-3">
                  <Button
                    type="button"
                    size="sm"
                    variant="outline"
                    onClick={() => onReview(item.id, "approve")}
                    disabled={reviewPending}
                  >
                    {reviewPending ? reviewingLabel : approveLabel}
                  </Button>
                  <Button
                    type="button"
                    size="sm"
                    variant="danger"
                    onClick={() => onReview(item.id, "reject")}
                    disabled={reviewPending}
                  >
                    {reviewPending ? reviewingLabel : rejectLabel}
                  </Button>
                  <span className="kicker">{formatRelative(item.createdAt)}</span>
                </div>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-[0.9rem] text-fg-dim">{emptyLabel}</p>
        )}
      </div>
    </section>
  );
}
