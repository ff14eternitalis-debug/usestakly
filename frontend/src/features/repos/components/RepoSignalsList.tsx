import { Chip } from "../../../components/Chip";
import { formatRelative } from "../../../lib/format";
import type { RepoSignal } from "../../../lib/types";

type RepoSignalsListProps = {
  signals: RepoSignal[];
  title: string;
  countSingleLabel: string;
  countPluralLabel: string;
  emptyLabel: string;
  passiveLabel: string;
  reportedLabel: string;
  statusLabel: string;
};

export function RepoSignalsList({
  signals,
  title,
  countSingleLabel,
  countPluralLabel,
  emptyLabel,
  passiveLabel,
  reportedLabel,
  statusLabel
}: RepoSignalsListProps) {
  return (
    <section className="grid gap-6">
      <div className="flex items-baseline justify-between">
        <h2 className="display-md">{title}</h2>
        <p className="kicker">
          {signals.length} {signals.length === 1 ? countSingleLabel : countPluralLabel}
        </p>
      </div>
      {signals.length === 0 ? (
        <p className="text-[0.94rem] text-fg-muted border-t border-line pt-6">{emptyLabel}</p>
      ) : (
        <ul className="grid gap-2">
          {signals.map((signal, index) => (
            <li
              key={signal.id ?? `${signal.signal}-${index}`}
              className="grid grid-cols-[auto_1fr_auto] items-center gap-4 rounded-[6px] border border-line bg-surface/30 px-4 py-3 hover:border-line-strong transition-colors"
            >
              <Chip tone={signal.isPassive ? "neutral" : "info"} mono>
                {signal.isPassive ? passiveLabel : reportedLabel}
              </Chip>
              <div className="grid gap-0.5">
                <p className="mono text-[0.86rem] text-fg">{signal.signal}</p>
                {signal.evidenceDescription ? (
                  <p className="text-[0.86rem] text-fg-dim">{signal.evidenceDescription}</p>
                ) : null}
                {signal.reviewStatus !== "accepted" || signal.events.length > 0 ? (
                  <div className="grid gap-1 pt-1">
                    <p className="mono text-[0.76rem] text-fg-muted">
                      {statusLabel}: {signal.reviewStatus}
                    </p>
                    {signal.events.slice(0, 5).map((event, eventIndex) => (
                      <p key={eventIndex} className="mono text-[0.74rem] text-fg-muted">
                        {event.eventKind} · {formatRelative(event.createdAt)}
                        {event.note ? ` · ${event.note}` : ""}
                      </p>
                    ))}
                  </div>
                ) : null}
              </div>
              <span className="kicker whitespace-nowrap">{formatRelative(signal.createdAt)}</span>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
