import { Button } from "../../../components/Button";
import { Chip } from "../../../components/Chip";
import type { RepoSignal } from "../../../lib/types";

type OwnerDisputePanelProps = {
  signals: RepoSignal[];
  disputeReason: string;
  isPending: boolean;
  isSuccess: boolean;
  error: string | null;
  onDisputeReasonChange(value: string): void;
  onDispute(signalId: string): void;
  title: string;
  hint: string;
  statusLabel: string;
  disputeReasonLabel: string;
  disputingLabel: string;
  disputeLabel: string;
  disputedLabel: string;
};

export function OwnerDisputePanel({
  signals,
  disputeReason,
  isPending,
  isSuccess,
  error,
  onDisputeReasonChange,
  onDispute,
  title,
  hint,
  statusLabel,
  disputeReasonLabel,
  disputingLabel,
  disputeLabel,
  disputedLabel
}: OwnerDisputePanelProps) {
  const activeSignals = signals.filter((item) => !item.isPassive);

  if (activeSignals.length === 0) {
    return null;
  }

  return (
    <section className="grid gap-4">
      <h2 className="display-md">{title}</h2>
      <p className="max-w-[66ch] text-[0.94rem] leading-relaxed text-fg-dim">{hint}</p>
      <div className="grid gap-3">
        {activeSignals.map((item) => (
          <div key={item.id} className="surface grid gap-3 p-4">
            <div className="flex flex-wrap items-center gap-2">
              <Chip tone="info" mono>
                {item.signal}
              </Chip>
              <span className="kicker">
                {statusLabel}: {item.reviewStatus}
              </span>
            </div>
            {item.evidenceDescription ? (
              <p className="text-[0.9rem] text-fg-dim">{item.evidenceDescription}</p>
            ) : null}
            <label className="grid gap-1.5">
              <span className="kicker">{disputeReasonLabel}</span>
              <textarea
                value={disputeReason}
                onChange={(e) => onDisputeReasonChange(e.target.value)}
                className="input min-h-[96px]"
              />
            </label>
            <div className="flex flex-wrap items-center gap-3">
              <Button
                type="button"
                variant="outline"
                onClick={() => onDispute(item.id)}
                disabled={isPending || disputeReason.trim().length < 10}
              >
                {isPending ? disputingLabel : disputeLabel}
              </Button>
              {error ? (
                <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
                  {error}
                </p>
              ) : isSuccess ? (
                <p className="text-[0.86rem] text-fg-dim">{disputedLabel}</p>
              ) : null}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
