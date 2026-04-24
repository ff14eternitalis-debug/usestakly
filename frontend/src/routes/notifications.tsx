import { Fragment, useState } from "react";
import { Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Chip } from "../components/Chip";
import { useT } from "../i18n";
import { apiGet, apiPost } from "../lib/api-client";
import { formatRelative } from "../lib/format";
import type { Notification, NotificationKind } from "../lib/types";

const KIND_TONE: Record<NotificationKind, "danger" | "warn" | "info" | "neutral"> = {
  flag_severe: "danger",
  score_drop: "warn",
  abandonment_up: "warn",
  flag_added: "info"
};

function payloadSummary(notif: Notification): string | null {
  const p = notif.payload as Record<string, unknown>;
  if (notif.kind === "score_drop") {
    const prev = p.prev_overall ?? p.previous;
    const next = p.new_overall ?? p.current;
    if (typeof prev === "number" && typeof next === "number") {
      return `overall ${prev.toFixed(2)} → ${next.toFixed(2)}`;
    }
  }
  if (notif.kind === "abandonment_up") {
    const prev = p.prev_abandonment;
    const next = p.new_abandonment;
    if (typeof prev === "number" && typeof next === "number") {
      return `abandonment ${prev.toFixed(2)} → ${next.toFixed(2)}`;
    }
  }
  if (notif.kind === "flag_added" || notif.kind === "flag_severe") {
    const flag = p.flag;
    if (typeof flag === "string") return `flag: ${flag}`;
  }
  return null;
}

export function NotificationsPage() {
  const t = useT();
  const [unreadOnly, setUnreadOnly] = useState(false);
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["notifications", unreadOnly],
    queryFn: ({ signal }) =>
      apiGet<Notification[]>(
        `/api/notifications?unread=${unreadOnly}`,
        signal
      )
  });

  const markRead = useMutation({
    mutationFn: (id: string) => apiPost(`/api/notifications/${id}/read`),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["notifications"] });
    }
  });

  const markAll = useMutation({
    mutationFn: () => apiPost("/api/notifications/read-all"),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["notifications"] });
    }
  });

  const items = query.data ?? [];
  const unreadCount = items.filter((n) => !n.readAt).length;

  function labelFor(kind: NotificationKind): string {
    if (kind === "score_drop") return t.notifications.labelScoreDrop;
    if (kind === "abandonment_up") return t.notifications.labelAbandonmentUp;
    if (kind === "flag_added") return t.notifications.labelFlagAdded;
    return t.notifications.labelFlagSevere;
  }

  const emptyBody = unreadOnly
    ? t.notifications.emptyBodyUnread
    : t.notifications.emptyBodyRecent;
  const emptyParts = emptyBody.split("{watchlistLink}");

  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.notifications.eyebrow}</span>
        <div className="grid gap-4 md:grid-cols-[1.2fr_1fr] md:items-end">
          <h1 className="display-lg max-w-[22ch]">
            {t.notifications.h1Part1}
            <br />
            <span className="accent">{t.notifications.h1Accent}</span>
          </h1>
          <div className="flex flex-wrap items-center gap-3 md:justify-end">
            <label className="inline-flex items-center gap-2 text-[0.88rem] text-fg-dim cursor-pointer">
              <input
                type="checkbox"
                checked={unreadOnly}
                onChange={(e) => setUnreadOnly(e.target.checked)}
                className="h-4 w-4 accent-[color:var(--color-accent)] cursor-pointer"
              />
              {t.notifications.unreadOnly}
            </label>
            <button
              type="button"
              onClick={() => markAll.mutate()}
              disabled={markAll.isPending || unreadCount === 0}
              className="rounded-[6px] border border-line px-3 py-1.5 mono text-[0.74rem] uppercase tracking-[0.12em] text-fg-dim hover:border-line-strong hover:text-fg disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            >
              {t.notifications.markAllRead}
            </button>
          </div>
        </div>
      </header>

      {query.isLoading ? (
        <div className="py-10 text-center">
          <span className="kicker">{t.notifications.loading}</span>
        </div>
      ) : query.isError ? (
        <div className="surface grid gap-4 p-10 text-center">
          <p className="display-md !text-[1.3rem]">
            {t.notifications.loadErrorTitle}
          </p>
          <p className="max-w-[52ch] mx-auto text-[0.96rem] leading-relaxed text-fg-dim">
            {t.notifications.loadErrorBody}
          </p>
          <button
            type="button"
            onClick={() => void query.refetch()}
            className="justify-self-center rounded-[6px] border border-line-strong bg-surface px-3.5 py-1.5 text-[0.84rem] font-medium text-fg hover:border-accent hover:text-accent transition-colors"
          >
            {t.notifications.retry}
          </button>
        </div>
      ) : items.length === 0 ? (
        <div className="surface grid gap-3 p-10 text-center">
          <p className="display-md !text-[1.3rem]">
            {t.notifications.emptyTitle}
          </p>
          <p className="max-w-[52ch] mx-auto text-[0.96rem] leading-relaxed text-fg-dim">
            {emptyParts.map((part, i) => (
              <Fragment key={i}>
                {part}
                {i < emptyParts.length - 1 ? (
                  <Link
                    to="/watchlist"
                    className="link-underline text-accent"
                  >
                    {t.notifications.watchlist}
                  </Link>
                ) : null}
              </Fragment>
            ))}
          </p>
          <Link
            to="/watchlist"
            className="justify-self-center rounded-[6px] border border-line-strong bg-surface px-3.5 py-1.5 text-[0.84rem] font-medium text-fg hover:border-accent hover:text-accent transition-colors"
          >
            {t.notifications.watchlistAction}
          </Link>
        </div>
      ) : (
        <ul className="grid gap-2">
          {items.map((n) => {
            const unread = !n.readAt;
            const summary = payloadSummary(n);
            return (
              <li
                key={n.id}
                className={`grid grid-cols-[auto_1fr_auto] items-center gap-4 rounded-[8px] border px-4 py-3.5 transition-colors ${
                  unread
                    ? "border-line-strong bg-surface"
                    : "border-line bg-surface/30 opacity-70"
                }`}
              >
                <Chip tone={KIND_TONE[n.kind]}>{labelFor(n.kind)}</Chip>
                <div className="grid gap-0.5 min-w-0">
                  <div className="flex items-baseline gap-2 flex-wrap">
                    {n.owner && n.name ? (
                      <Link
                        to="/repos/$id"
                        params={{ id: n.artifactId }}
                        onClick={() => {
                          if (unread) {
                            markRead.mutate(n.id);
                          }
                        }}
                        className="font-medium text-fg hover:text-accent transition-colors truncate"
                      >
                        <span className="mono text-fg-muted">
                          {n.owner}/
                        </span>
                        {n.name}
                      </Link>
                    ) : (
                      <span className="mono text-[0.9rem] text-fg-muted">
                        {n.artifactId.slice(0, 8)}…
                      </span>
                    )}
                    {unread ? <span className="dot text-accent" /> : null}
                  </div>
                  {summary ? (
                    <p className="mono text-[0.82rem] text-fg-dim">
                      {summary}
                    </p>
                  ) : null}
                </div>
                <div className="grid gap-1 justify-items-end">
                  <span className="kicker whitespace-nowrap">
                    {formatRelative(n.createdAt)}
                  </span>
                  {unread ? (
                    <button
                      type="button"
                      onClick={() => markRead.mutate(n.id)}
                      disabled={markRead.isPending && markRead.variables === n.id}
                      className="mono text-[0.7rem] uppercase tracking-[0.14em] text-fg-muted hover:text-accent disabled:cursor-not-allowed disabled:opacity-40 transition-colors"
                    >
                      {markRead.isPending && markRead.variables === n.id
                        ? t.notifications.markingRead
                        : t.notifications.markRead}
                    </button>
                  ) : null}
                </div>
              </li>
            );
          })}
        </ul>
      )}
    </section>
  );
}
