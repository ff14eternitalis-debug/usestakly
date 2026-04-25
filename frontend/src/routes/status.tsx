import { useQuery } from "@tanstack/react-query";

import { useT } from "../i18n";
import { apiGet } from "../lib/api-client";
import type { RepoSearchResponse } from "../lib/types";

type HealthResponse = {
  status: string;
};

export function StatusPage() {
  const t = useT();
  const health = useQuery({
    queryKey: ["status", "health"],
    queryFn: ({ signal }) => apiGet<HealthResponse>("/health", signal),
    refetchInterval: 60_000
  });
  const registry = useQuery({
    queryKey: ["status", "registry"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>("/api/repos/search?filter=explore&limit=1", signal),
    refetchInterval: 60_000
  });

  const checkedAt = new Date().toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit"
  });

  return (
    <article className="shell-narrow grid gap-10 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.status.eyebrow}</span>
        <h1 className="display-lg max-w-[18ch]">{t.status.h1}</h1>
        <p className="max-w-[62ch] text-[0.98rem] leading-relaxed text-fg-dim">
          {t.status.intro}
        </p>
      </header>

      <section className="grid gap-4 sm:grid-cols-2">
        <StatusCheck
          label={t.status.apiHealth}
          state={
            health.isLoading
              ? "checking"
              : health.isSuccess && health.data.status === "ok"
                ? "online"
                : "offline"
          }
        />
        <StatusCheck
          label={t.status.registryRead}
          state={
            registry.isLoading
              ? "checking"
              : registry.isSuccess
                ? "online"
                : "degraded"
          }
        />
      </section>

      <section className="grid gap-3 border-t border-line pt-8">
        <p className="kicker">{t.status.betaTitle}</p>
        <p className="text-[0.94rem] leading-relaxed text-fg-dim">
          {t.status.betaBody}
        </p>
        <p className="mono text-[0.78rem] text-fg-muted">
          {t.status.lastChecked}: {checkedAt}
        </p>
      </section>
    </article>
  );
}

function StatusCheck({
  label,
  state
}: {
  label: string;
  state: "checking" | "online" | "degraded" | "offline";
}) {
  const t = useT();
  const statusLabel =
    state === "checking"
      ? t.status.checking
      : state === "online"
        ? t.status.online
        : state === "degraded"
          ? t.status.degraded
          : t.status.offline;
  const color =
    state === "online"
      ? "var(--color-accent)"
      : state === "checking"
        ? "var(--color-fg-muted)"
        : state === "degraded"
          ? "var(--color-warn)"
          : "var(--color-danger)";

  return (
    <div className="rounded-[8px] border border-line bg-surface/45 p-5">
      <div className="flex items-center justify-between gap-4">
        <p className="text-[0.98rem] font-semibold text-fg">{label}</p>
        <span className="dot" style={{ color }} />
      </div>
      <p className="mt-5 data-value text-[1.5rem] leading-none" style={{ color }}>
        {statusLabel}
      </p>
    </div>
  );
}
