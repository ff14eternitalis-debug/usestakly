import type {
  McpDailyBucket,
  McpMetricsReport,
  McpMetricsWindow,
  McpOutcomeBucket,
  McpRejectionBucket,
  McpRepoVolume,
  McpUserVolume
} from "../../../lib/types";

type Labels = {
  title: string;
  intro: string;
  windowLabel: string;
  window24h: string;
  window7d: string;
  window30d: string;
  loading: string;
  totalLogUsage: string;
  totalWatchRepo: string;
  totalRejections: string;
  distinctTokens: string;
  distinctUsers: string;
  distinctRepos: string;
  outcomeTitle: string;
  rejectionTitle: string;
  topReposTitle: string;
  topUsersTitle: string;
  dailyTitle: string;
  empty: string;
};

type AdminMcpObservabilityPanelProps = {
  adminToken: string;
  window: McpMetricsWindow;
  onWindowChange(value: McpMetricsWindow): void;
  loading: boolean;
  report: McpMetricsReport | undefined;
  labels: Labels;
};

export function AdminMcpObservabilityPanel({
  adminToken,
  window,
  onWindowChange,
  loading,
  report,
  labels
}: AdminMcpObservabilityPanelProps) {
  const totalActivity = report
    ? report.totals.logUsage + report.totals.watchRepo + report.totals.rejections
    : 0;

  return (
    <section className="grid gap-4">
      <span className="kicker">{labels.title}</span>
      <div className="surface grid gap-4 p-5">
        <p className="text-[0.9rem] text-fg-dim">{labels.intro}</p>

        <div className="flex flex-wrap items-center gap-2">
          <span className="kicker">{labels.windowLabel}</span>
          <WindowButton
            active={window === "24h"}
            onClick={() => onWindowChange("24h")}
            label={labels.window24h}
          />
          <WindowButton
            active={window === "7d"}
            onClick={() => onWindowChange("7d")}
            label={labels.window7d}
          />
          <WindowButton
            active={window === "30d"}
            onClick={() => onWindowChange("30d")}
            label={labels.window30d}
          />
        </div>

        {!adminToken.trim() ? null : loading ? (
          <p className="text-[0.9rem] text-fg-dim">{labels.loading}</p>
        ) : !report || totalActivity === 0 ? (
          <p className="text-[0.9rem] text-fg-dim">{labels.empty}</p>
        ) : (
          <div className="grid gap-5">
            <TotalsGrid report={report} labels={labels} />
            <OutcomeDistribution buckets={report.outcomeDistribution} title={labels.outcomeTitle} />
            <RejectionBreakdown buckets={report.rejectionBreakdown} title={labels.rejectionTitle} />
            <TopRepos repos={report.topRepos} title={labels.topReposTitle} />
            <TopUsers users={report.topUsers} title={labels.topUsersTitle} />
            <DailyVolume buckets={report.dailyVolume} title={labels.dailyTitle} />
          </div>
        )}
      </div>
    </section>
  );
}

function WindowButton({
  active,
  onClick,
  label
}: {
  active: boolean;
  onClick(): void;
  label: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={
        active
          ? "rounded-[6px] border border-accent px-2.5 py-1 text-[0.82rem] text-accent"
          : "rounded-[6px] border border-line px-2.5 py-1 text-[0.82rem] text-fg-dim hover:border-accent hover:text-accent"
      }
    >
      {label}
    </button>
  );
}

function TotalsGrid({ report, labels }: { report: McpMetricsReport; labels: Labels }) {
  const cells: Array<{ label: string; value: number }> = [
    { label: labels.totalLogUsage, value: report.totals.logUsage },
    { label: labels.totalWatchRepo, value: report.totals.watchRepo },
    { label: labels.totalRejections, value: report.totals.rejections },
    { label: labels.distinctTokens, value: report.totals.distinctTokens },
    { label: labels.distinctUsers, value: report.totals.distinctUsers },
    { label: labels.distinctRepos, value: report.totals.distinctRepos }
  ];
  return (
    <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
      {cells.map((cell) => (
        <div key={cell.label} className="rounded-[8px] border border-line p-3">
          <div className="kicker">{cell.label}</div>
          <div className="mono text-[1.1rem] text-fg">{cell.value.toLocaleString()}</div>
        </div>
      ))}
    </div>
  );
}

function OutcomeDistribution({
  buckets,
  title
}: {
  buckets: McpOutcomeBucket[];
  title: string;
}) {
  if (buckets.length === 0) return null;
  return (
    <div className="grid gap-2">
      <span className="kicker">{title}</span>
      <ul className="grid gap-1.5">
        {buckets.map((b) => (
          <li key={b.outcome} className="flex items-center justify-between gap-3 text-[0.9rem]">
            <span className="mono text-fg">{b.outcome}</span>
            <span className="text-fg-dim">{b.count.toLocaleString()}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function RejectionBreakdown({
  buckets,
  title
}: {
  buckets: McpRejectionBucket[];
  title: string;
}) {
  if (buckets.length === 0) return null;
  return (
    <div className="grid gap-2">
      <span className="kicker">{title}</span>
      <ul className="grid gap-1.5">
        {buckets.map((b) => (
          <li
            key={`${b.tool}:${b.reason}`}
            className="flex flex-wrap items-center justify-between gap-3 text-[0.9rem]"
          >
            <span className="mono text-fg">
              {b.tool} · {b.reason}
            </span>
            <span className="text-fg-dim">{b.count.toLocaleString()}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function TopRepos({ repos, title }: { repos: McpRepoVolume[]; title: string }) {
  if (repos.length === 0) return null;
  return (
    <div className="grid gap-2">
      <span className="kicker">{title}</span>
      <ul className="grid gap-1.5">
        {repos.map((r) => (
          <li
            key={`${r.owner}/${r.name}`}
            className="flex flex-wrap items-center justify-between gap-3 text-[0.9rem]"
          >
            <span className="mono text-fg">
              {r.owner}/{r.name}
            </span>
            <span className="text-fg-dim">
              {r.logUsage} log · {r.watchRepo} watch · {r.rejections} rej
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function TopUsers({ users, title }: { users: McpUserVolume[]; title: string }) {
  if (users.length === 0) return null;
  return (
    <div className="grid gap-2">
      <span className="kicker">{title}</span>
      <ul className="grid gap-1.5">
        {users.map((u) => (
          <li
            key={u.userId}
            className="flex flex-wrap items-center justify-between gap-3 text-[0.9rem]"
          >
            <span className="mono text-fg">{u.userId.slice(0, 8)}…</span>
            <span className="text-fg-dim">
              {u.logUsage} log · {u.watchRepo} watch · {u.rejections} rej
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}

function DailyVolume({ buckets, title }: { buckets: McpDailyBucket[]; title: string }) {
  if (buckets.length === 0) return null;
  return (
    <div className="grid gap-2">
      <span className="kicker">{title}</span>
      <ul className="grid gap-1.5">
        {buckets.map((b) => (
          <li
            key={b.bucket}
            className="flex flex-wrap items-center justify-between gap-3 text-[0.86rem]"
          >
            <span className="mono text-fg-dim">{b.bucket.slice(0, 10)}</span>
            <span className="text-fg-dim">
              {b.logUsage} log · {b.watchRepo} watch · {b.rejections} rej
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}
