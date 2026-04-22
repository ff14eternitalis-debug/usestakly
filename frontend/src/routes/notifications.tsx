import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { Chip } from "../components/Chip";
import { apiGet, apiPost } from "../lib/api-client";
import { formatRelative, notificationLabel } from "../lib/format";
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

  return (
    <section className="shell grid gap-10 py-12 md:py-16">
      <header className="grid gap-5">
        <p className="eyebrow">
          <span className="callout-mark" />
          Dispatch inbox
        </p>
        <div className="grid gap-4 md:grid-cols-[1.2fr_1fr] md:items-end">
          <h1 className="display-lg max-w-[22ch]">
            What's moved<br />
            <span className="italic-accent">since last you looked.</span>
          </h1>
          <div className="flex flex-wrap items-center gap-3 md:justify-end">
            <label className="inline-flex items-center gap-2 text-[0.92rem] text-ink-dim">
              <input
                type="checkbox"
                checked={unreadOnly}
                onChange={(e) => setUnreadOnly(e.target.checked)}
                className="accent-ember"
              />
              Unread only
            </label>
            <button
              type="button"
              onClick={() => markAll.mutate()}
              disabled={markAll.isPending || unreadCount === 0}
              className="border border-line px-3 py-2 font-mono text-[0.78rem] uppercase tracking-[0.14em] text-ink-dim hover:border-ink hover:text-ink disabled:opacity-40 disabled:cursor-not-allowed"
              style={{ borderRadius: 2 }}
            >
              mark all read
            </button>
          </div>
        </div>
      </header>

      {query.isLoading ? (
        <div className="py-10 text-center text-ink-muted">
          <span className="kicker">Sorting the mail…</span>
        </div>
      ) : items.length === 0 ? (
        <div className="grid gap-2 border-t border-line pt-10 text-ink-dim">
          <p className="display-md">All quiet on the register.</p>
          <p className="max-w-[52ch] text-[1rem] leading-relaxed">
            Nothing to report {unreadOnly ? "unread" : "recently"}. Add
            repositories to your{" "}
            <Link to="/watchlist" className="link-underline">
              watchlist
            </Link>{" "}
            so the observatory can flag drift for you.
          </p>
        </div>
      ) : (
        <ul className="grid gap-4">
          {items.map((n) => {
            const unread = !n.readAt;
            const summary = payloadSummary(n);
            return (
              <li
                key={n.id}
                className={`grid grid-cols-[auto_1fr_auto] items-baseline gap-5 border-t border-line pt-4 ${
                  unread ? "" : "opacity-60"
                }`}
              >
                <Chip tone={KIND_TONE[n.kind]}>{notificationLabel(n.kind)}</Chip>
                <div className="grid gap-1">
                  <div className="flex items-baseline gap-2">
                    {n.owner && n.name ? (
                      <Link
                        to="/repos/$id"
                        params={{ id: n.artifactId }}
                        className="link-underline font-display text-[1.12rem] italic-accent"
                      >
                        {n.owner}/{n.name}
                      </Link>
                    ) : (
                      <span className="font-mono text-[0.92rem] text-ink-muted">
                        {n.artifactId.slice(0, 8)}…
                      </span>
                    )}
                    {unread ? (
                      <span className="inline-block h-2 w-2 translate-y-[-2px] rounded-full bg-ember" />
                    ) : null}
                  </div>
                  {summary ? (
                    <p className="font-mono text-[0.85rem] text-ink-dim">
                      {summary}
                    </p>
                  ) : null}
                </div>
                <div className="grid gap-1 justify-items-end">
                  <span className="kicker">
                    {formatRelative(n.createdAt)}
                  </span>
                  {unread ? (
                    <button
                      type="button"
                      onClick={() => markRead.mutate(n.id)}
                      className="font-mono text-[0.72rem] uppercase tracking-[0.16em] text-ink-muted hover:text-ink"
                    >
                      mark read
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
