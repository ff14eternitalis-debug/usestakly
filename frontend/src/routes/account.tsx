import { useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Button } from "../components/Button";
import { useT } from "../i18n";
import { ApiError, apiDelete, apiGet, apiGetWithInit, apiPost, apiPostWithInit } from "../lib/api-client";
import { formatRelative } from "../lib/format";
import type {
  AccountSummary,
  AgentTokenCreated,
  AgentTokenSummary,
  PendingRepoSignal
} from "../lib/types";
import { useAuthStore } from "../state/auth-store";

function formatTimestamp(value: string): string {
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short"
  }).format(new Date(value));
}

export function AccountPage() {
  const t = useT();
  const queryClient = useQueryClient();
  const user = useAuthStore((s) => s.user);
  const [label, setLabel] = useState("");
  const [created, setCreated] = useState<AgentTokenCreated | null>(null);
  const [copied, setCopied] = useState(false);
  const [adminToken, setAdminToken] = useState("");
  const summary = useQuery({
    queryKey: ["account-summary"],
    queryFn: ({ signal }) => apiGet<AccountSummary>("/api/account/summary", signal)
  });

  const tokens = useQuery({
    queryKey: ["agent-tokens"],
    queryFn: ({ signal }) => apiGet<AgentTokenSummary[]>("/api/agent-tokens", signal)
  });

  const pendingSignals = useQuery({
    queryKey: ["admin-pending-signals", adminToken],
    queryFn: () =>
      apiGetWithInit<PendingRepoSignal[]>("/api/admin/repo-signals/pending", {
        headers: { "x-admin-token": adminToken.trim() }
      }),
    enabled: adminToken.trim().length > 0
  });

  const createToken = useMutation({
    mutationFn: () =>
      apiPost<AgentTokenCreated>("/api/agent-tokens", { label: label.trim() }),
    onSuccess: async (token) => {
      setCreated(token);
      setCopied(false);
      setLabel("");
      await queryClient.invalidateQueries({ queryKey: ["agent-tokens"] });
    }
  });

  const revokeToken = useMutation({
    mutationFn: (id: string) => apiDelete(`/api/agent-tokens/${id}`),
    onSuccess: async (_, id) => {
      if (created?.id === id) {
        setCreated(null);
        setCopied(false);
      }
      await queryClient.invalidateQueries({ queryKey: ["agent-tokens"] });
    }
  });

  const reviewSignal = useMutation({
    mutationFn: ({ id, action }: { id: string; action: "approve" | "reject" }) =>
      apiPostWithInit(
        `/api/admin/repo-signals/${id}/review`,
        { action },
        {
          headers: { "x-admin-token": adminToken.trim() }
        }
      ),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ["admin-pending-signals", adminToken] });
    }
  });

  const error =
    createToken.error instanceof ApiError
      ? createToken.error.message
      : revokeToken.error instanceof ApiError
        ? revokeToken.error.message
        : null;
  const items = tokens.data ?? [];

  async function copyToken(): Promise<void> {
    if (!created?.token) return;
    await navigator.clipboard.writeText(created.token);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1500);
  }

  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.account.eyebrow}</span>
        <div className="grid gap-4 md:grid-cols-[1.2fr_1fr] md:items-end">
          <div className="grid gap-3">
            <h1 className="display-lg max-w-[24ch]">
              {t.account.h1Part1}
              <br />
              <span className="accent">{t.account.h1Accent}</span>
            </h1>
            <p className="max-w-[62ch] text-[0.98rem] leading-relaxed text-fg-dim">
              {t.account.intro}
            </p>
          </div>
          <div className="surface grid gap-2 p-4">
            <span className="kicker">{user?.username ? `@${user.username}` : "@"}</span>
            <p className="text-[0.9rem] text-fg-dim">{user?.email}</p>
          </div>
        </div>
      </header>

      <div className="grid gap-4 md:grid-cols-[1.1fr_0.9fr]">
        <div className="surface grid gap-4 p-5">
          <div className="grid gap-1.5">
            <span className="kicker">{t.account.tokenLabel}</span>
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              placeholder={t.account.tokenPlaceholder}
              className="input"
            />
          </div>
          <div className="flex flex-wrap items-center gap-3">
            <Button
              type="button"
              variant="outline"
              onClick={() => createToken.mutate()}
              disabled={createToken.isPending || !label.trim()}
            >
              {createToken.isPending ? t.account.creating : t.account.create}
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
                  <span className="kicker">{t.account.tokenShownOnce}</span>
                  <p className="text-[0.86rem] text-fg-dim">
                    {t.account.tokenShownOnceHint}
                  </p>
                </div>
                <Button type="button" size="sm" variant="ghost" onClick={() => void copyToken()}>
                  {copied ? t.account.copied : t.account.copy}
                </Button>
              </div>
              <code className="block overflow-x-auto rounded-[6px] border border-line bg-surface px-3 py-3 text-[0.84rem]">
                {created.token}
              </code>
              <p className="mono text-[0.75rem] text-fg-muted">
                {t.account.createdNow}
              </p>
            </div>
          ) : null}
        </div>

        <div className="surface grid gap-2 p-5">
          <span className="kicker">{t.account.quotaTitle}</span>
          <p className="text-[0.94rem] leading-relaxed text-fg-dim">
            {t.account.quotaBody}
          </p>
          {summary.data ? (
            <div className="grid gap-1 border-t border-line pt-3 text-[0.88rem] text-fg-dim">
              <span>
                {t.account.reputation}:{" "}
                <span className="data-value text-fg">
                  {summary.data.reputation.score.toFixed(2)}
                </span>
              </span>
              <span>
                {t.account.passiveSignals}: {summary.data.reputation.passiveSignalCount}
              </span>
              <span>
                {t.account.eligibility}:{" "}
                {summary.data.reputation.activeSignalEligible
                  ? t.account.eligible
                  : t.account.notEligible}
              </span>
            </div>
          ) : null}
        </div>
      </div>

      <section className="grid gap-4">
        <div className="flex items-center justify-between">
          <span className="kicker">{t.account.activeTokens}</span>
        </div>

        {tokens.isLoading ? (
          <div className="py-10 text-center">
            <span className="kicker">{t.common.tuning}</span>
          </div>
        ) : items.length === 0 ? (
          <div className="surface grid gap-3 p-10 text-center">
            <p className="display-md !text-[1.2rem]">{t.account.emptyTitle}</p>
            <p className="mx-auto max-w-[52ch] text-[0.96rem] leading-relaxed text-fg-dim">
              {t.account.emptyBody}
            </p>
          </div>
        ) : (
          <ul className="grid gap-3">
            {items.map((token) => (
              <li
                key={token.id}
                className="surface grid gap-3 p-5 md:grid-cols-[1fr_auto] md:items-center"
              >
                <div className="grid gap-1.5">
                  <div className="flex flex-wrap items-center gap-2">
                    <h2 className="display-md !text-[1.08rem]">{token.label}</h2>
                    {created?.id === token.id ? (
                      <span className="kicker text-accent">{t.account.createdNow}</span>
                    ) : null}
                  </div>
                  <div className="flex flex-wrap gap-x-4 gap-y-1 text-[0.86rem] text-fg-dim">
                    <span>
                      {t.account.createdAt} {formatTimestamp(token.createdAt)}
                    </span>
                    <span>
                      {token.lastUsedAt
                        ? `${t.account.lastUsed} ${formatRelative(token.lastUsedAt)}`
                        : t.account.lastUsedNever}
                    </span>
                  </div>
                </div>
                <Button
                  type="button"
                  variant="danger"
                  size="sm"
                  onClick={() => revokeToken.mutate(token.id)}
                  disabled={revokeToken.isPending}
                >
                  {revokeToken.isPending ? t.account.revoking : t.account.revoke}
                </Button>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section className="grid gap-4">
        <span className="kicker">{t.account.adminTitle}</span>
        <div className="surface grid gap-4 p-5">
          <label className="grid gap-1.5">
            <span className="kicker">{t.account.adminTokenLabel}</span>
            <input
              type="password"
              value={adminToken}
              onChange={(e) => setAdminToken(e.target.value)}
              placeholder={t.account.adminTokenPlaceholder}
              className="input"
            />
          </label>

          {!adminToken.trim() ? null : pendingSignals.isLoading ? (
            <p className="text-[0.9rem] text-fg-dim">{t.common.tuning}</p>
          ) : pendingSignals.data?.length ? (
            <ul className="grid gap-3">
              {pendingSignals.data.map((item) => (
                <li key={item.id} className="grid gap-3 rounded-[8px] border border-line p-4">
                  <div className="flex flex-wrap items-center gap-2">
                    <span className="mono text-[0.86rem] text-fg">
                      {item.owner}/{item.name}
                    </span>
                    <span className="kicker">{item.signal}</span>
                    <span className="kicker">{item.reviewStatus}</span>
                  </div>
                  {item.evidenceDescription ? (
                    <p className="text-[0.9rem] text-fg-dim">{item.evidenceDescription}</p>
                  ) : null}
                  <div className="flex flex-wrap items-center gap-3">
                    <Button
                      type="button"
                      size="sm"
                      variant="outline"
                      onClick={() => reviewSignal.mutate({ id: item.id, action: "approve" })}
                      disabled={reviewSignal.isPending}
                    >
                      {reviewSignal.isPending ? t.account.adminReviewing : t.account.adminApprove}
                    </Button>
                    <Button
                      type="button"
                      size="sm"
                      variant="danger"
                      onClick={() => reviewSignal.mutate({ id: item.id, action: "reject" })}
                      disabled={reviewSignal.isPending}
                    >
                      {reviewSignal.isPending ? t.account.adminReviewing : t.account.adminReject}
                    </Button>
                    <span className="kicker">{formatRelative(item.createdAt)}</span>
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-[0.9rem] text-fg-dim">{t.account.adminEmpty}</p>
          )}
        </div>
      </section>
    </section>
  );
}
