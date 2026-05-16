import { Link } from "@tanstack/react-router";

import { Chip } from "../../../components/Chip";
import { proofTierLabel } from "../../../lib/dimension-display";
import type { IngestionStatus, ProofTier } from "../../../lib/types";

type StructuralRefreshBannerProps = {
  proofTier: ProofTier;
  ingestionStatus: IngestionStatus;
  refreshing: boolean;
  proofTierLabels: Record<string, string>;
  refreshingLabel: string;
  incompleteLabel: string;
  howToReadLabel: string;
};

export function StructuralRefreshBanner({
  proofTier,
  ingestionStatus,
  refreshing,
  proofTierLabels,
  refreshingLabel,
  incompleteLabel,
  howToReadLabel
}: StructuralRefreshBannerProps) {
  const showRefresh =
    refreshing ||
    ingestionStatus.structuralStale ||
    !ingestionStatus.structuralComplete;

  return (
    <div className="flex flex-wrap items-center gap-2">
      <Chip tone="info">{proofTierLabel(proofTier, proofTierLabels)}</Chip>
      <Link
        to="/how-to-read"
        className="text-[0.82rem] font-medium text-accent hover:underline"
      >
        {howToReadLabel}
      </Link>
      {showRefresh ? (
        <p className="w-full text-[0.84rem] text-fg-dim">
          {refreshing ? refreshingLabel : incompleteLabel}
        </p>
      ) : null}
    </div>
  );
}
