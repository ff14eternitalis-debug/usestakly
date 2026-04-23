import { Button } from "../../../components/Button";
import { formatRelative } from "../../../lib/format";
import type { AgentTokenCreated, AgentTokenSummary } from "../../../lib/types";

function formatTimestamp(value: string): string {
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short"
  }).format(new Date(value));
}

type AgentTokensPanelProps = {
  label: string;
  onLabelChange(value: string): void;
  createPending: boolean;
  revokePending: boolean;
  created: AgentTokenCreated | null;
  copied: boolean;
  error: string | null;
  tokensLoading: boolean;
  tokens: AgentTokenSummary[];
  onCreate(): void;
  onCopy(): void;
  onRevoke(id: string): void;
  tokenLabel: string;
  tokenPlaceholder: string;
  creatingLabel: string;
  createLabel: string;
  tokenShownOnceLabel: string;
  tokenShownOnceHint: string;
  copiedLabel: string;
  copyLabel: string;
  createdNowLabel: string;
  activeTokensLabel: string;
  emptyTitle: string;
  emptyBody: string;
  createdAtLabel: string;
  lastUsedLabel: string;
  lastUsedNeverLabel: string;
  revokingLabel: string;
  revokeLabel: string;
  loadingLabel: string;
};

export function AgentTokensPanel({
  label,
  onLabelChange,
  createPending,
  revokePending,
  created,
  copied,
  error,
  tokensLoading,
  tokens,
  onCreate,
  onCopy,
  onRevoke,
  tokenLabel,
  tokenPlaceholder,
  creatingLabel,
  createLabel,
  tokenShownOnceLabel,
  tokenShownOnceHint,
  copiedLabel,
  copyLabel,
  createdNowLabel,
  activeTokensLabel,
  emptyTitle,
  emptyBody,
  createdAtLabel,
  lastUsedLabel,
  lastUsedNeverLabel,
  revokingLabel,
  revokeLabel,
  loadingLabel
}: AgentTokensPanelProps) {
  return (
    <div className="grid gap-4">
      <div className="surface grid gap-4 p-5">
        <div className="grid gap-1.5">
          <span className="kicker">{tokenLabel}</span>
          <input
            type="text"
            value={label}
            onChange={(e) => onLabelChange(e.target.value)}
            placeholder={tokenPlaceholder}
            className="input"
          />
        </div>
        <div className="flex flex-wrap items-center gap-3">
          <Button
            type="button"
            variant="outline"
            onClick={onCreate}
            disabled={createPending || !label.trim()}
          >
            {createPending ? creatingLabel : createLabel}
          </Button>
          {error ? (
            <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
              {error}
            </p>
          ) : null}
        </div>

        {created ? (
          <div className="grid gap-3 rounded-[8px] border border-line bg-bg-subtle p-4">
            <div className="flex flex-wrap items-center justify-between gap-2">
              <div className="grid gap-1">
                <span className="kicker">{tokenShownOnceLabel}</span>
                <p className="text-[0.86rem] text-fg-dim">{tokenShownOnceHint}</p>
              </div>
              <Button type="button" size="sm" variant="ghost" onClick={onCopy}>
                {copied ? copiedLabel : copyLabel}
              </Button>
            </div>
            <code className="block overflow-x-auto rounded-[6px] border border-line bg-surface px-3 py-3 text-[0.84rem]">
              {created.token}
            </code>
            <p className="mono text-[0.75rem] text-fg-muted">{createdNowLabel}</p>
          </div>
        ) : null}
      </div>

      <section className="grid gap-4">
        <div className="flex items-center justify-between">
          <span className="kicker">{activeTokensLabel}</span>
        </div>

        {tokensLoading ? (
          <div className="py-10 text-center">
            <span className="kicker">{loadingLabel}</span>
          </div>
        ) : tokens.length === 0 ? (
          <div className="surface grid gap-3 p-10 text-center">
            <p className="display-md !text-[1.2rem]">{emptyTitle}</p>
            <p className="mx-auto max-w-[52ch] text-[0.96rem] leading-relaxed text-fg-dim">
              {emptyBody}
            </p>
          </div>
        ) : (
          <ul className="grid gap-3">
            {tokens.map((token) => (
              <li
                key={token.id}
                className="surface grid gap-3 p-5 md:grid-cols-[1fr_auto] md:items-center"
              >
                <div className="grid gap-1.5">
                  <div className="flex flex-wrap items-center gap-2">
                    <h2 className="display-md !text-[1.08rem]">{token.label}</h2>
                    {created?.id === token.id ? (
                      <span className="kicker text-accent">{createdNowLabel}</span>
                    ) : null}
                  </div>
                  <div className="flex flex-wrap gap-x-4 gap-y-1 text-[0.86rem] text-fg-dim">
                    <span>
                      {createdAtLabel} {formatTimestamp(token.createdAt)}
                    </span>
                    <span>
                      {token.lastUsedAt
                        ? `${lastUsedLabel} ${formatRelative(token.lastUsedAt)}`
                        : lastUsedNeverLabel}
                    </span>
                  </div>
                </div>
                <Button
                  type="button"
                  variant="danger"
                  size="sm"
                  onClick={() => onRevoke(token.id)}
                  disabled={revokePending}
                >
                  {revokePending ? revokingLabel : revokeLabel}
                </Button>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}
