import { useQuery } from "@tanstack/react-query";

import { useT } from "../i18n";
import { apiGet } from "../lib/api-client";
import type { PublicStatus } from "../lib/types";

export function StatusPage() {
  const t = useT();
  const status = useQuery({
    queryKey: ["status", "public"],
    queryFn: ({ signal }) => apiGet<PublicStatus>("/api/status/public", signal),
    refetchInterval: 60_000
  });

  const checkedAt = status.data?.checkedAt
    ? new Date(status.data.checkedAt).toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit"
      })
    : new Date().toLocaleTimeString([], {
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
          state={status.isLoading ? "checking" : checkState(status.data?.api.status)}
        />
        <StatusCheck
          label={t.status.database}
          state={status.isLoading ? "checking" : checkState(status.data?.database.status)}
        />
        <StatusCheck
          label={t.status.registryRead}
          state={status.isLoading ? "checking" : checkState(status.data?.registry.status)}
          detail={
            status.data ? `${status.data.registry.repoCount} ${t.status.repos}` : undefined
          }
        />
        <StatusCheck
          label={t.status.githubIngestion}
          state={status.isLoading ? "checking" : checkState(status.data?.ingestion.status)}
          detail={status.data?.ingestion.message}
        />
        <StatusCheck
          label={t.status.mcp}
          state={status.isLoading ? "checking" : checkState(status.data?.mcp.status)}
          detail={status.data ? `${status.data.mcp.tools.length} ${t.status.tools}` : undefined}
        />
      </section>

      <section className="grid gap-4 border-t border-line pt-8 sm:grid-cols-2">
        <StatusFact label={t.status.formula} value={status.data?.formula.version ?? "v2.0"} />
        <StatusFact label={t.status.publicStatus} value={status.data?.status ?? "checking"} />
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

function checkState(status?: "ok" | "down" | "degraded" | "disabled") {
  if (status === "ok") return "online";
  if (status === "degraded" || status === "disabled") return "degraded";
  return "offline";
}

function StatusCheck({
  label,
  state,
  detail
}: {
  label: string;
  state: "checking" | "online" | "degraded" | "offline";
  detail?: string;
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
      {detail ? (
        <p className="mt-2 mono text-[0.78rem] text-fg-muted">{detail}</p>
      ) : null}
    </div>
  );
}

function StatusFact({ label, value }: { label: string; value: string }) {
  return (
    <div className="border-l border-line pl-4">
      <p className="kicker">{label}</p>
      <p className="mt-2 text-[1rem] font-semibold text-fg">{value}</p>
    </div>
  );
}
