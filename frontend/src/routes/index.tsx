import { Link } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { buttonClass } from "../components/Button";
import { RepoCard } from "../components/RepoCard";
import { useT } from "../i18n";
import { apiGet } from "../lib/api-client";
import {
  abandonmentTone,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import type { RepoSearchResponse } from "../lib/types";
import { useAuthStore } from "../state/auth-store";

function Hero() {
  const t = useT();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  return (
    <section className="shell relative pt-16 pb-20 md:pt-24 md:pb-28">
      <div className="grid gap-12 md:grid-cols-[1.3fr_1fr] md:items-center md:gap-16">
        <div className="grid gap-8 rise-in">
          <h1 className="display-xl">
            <span className="accent">{t.landing.h1}</span>
          </h1>

          <p className="max-w-[56ch] text-[1.08rem] md:text-[1.15rem] leading-[1.55] text-fg-dim">
            {t.landing.introBefore}
            <strong className="font-semibold text-fg">{t.landing.introStrong}</strong>
            {t.landing.introAfter}
          </p>

          <div className="flex flex-wrap items-center gap-4 pt-2">
            <Link
              to="/discover"
              className={buttonClass(
                "primary",
                "md",
                "px-5 py-3 text-[0.96rem] shadow-[0_0_0_1px_var(--color-line-strong)]"
              )}
            >
              {t.landing.openObservatory}
              <span className="arrow">→</span>
            </Link>
            <Link
              to="/how-to-read"
              className="link-underline text-[0.92rem] text-fg-dim hover:text-accent"
            >
              {t.landing.readGuide}
            </Link>
            {isAuthed ? (
              <Link
                to="/watchlist"
                className="inline-flex items-center gap-1.5 text-[0.92rem] text-fg-dim hover:text-accent transition-colors"
              >
                {t.landing.myWatchlist}
                <span className="arrow">→</span>
              </Link>
            ) : null}
          </div>

          <div className="mt-6 grid grid-cols-3 gap-5 border-t border-line pt-6 rise-in rise-in-d2">
            <Kpi k="4" label={t.landing.kpi1} />
            <Kpi k="v2" label={t.landing.kpi2} />
            <Kpi k="0%" label={t.landing.kpi3} />
          </div>
        </div>

        <HeroPanel />
      </div>
    </section>
  );
}

function HeroPanel() {
  const t = useT();
  const query = useQuery({
    queryKey: ["search", "explore", "hero-panel"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&sort=score&limit=1",
        signal
      )
  });
  const repo = query.data?.items[0];
  const quality = repo?.quality ?? null;
  const overallTone = scoreTone(quality?.overall);
  const overallColor =
    overallTone === "danger"
      ? "var(--color-danger)"
      : overallTone === "warn"
        ? "var(--color-warn)"
        : overallTone === "ok"
          ? "var(--color-accent)"
          : "var(--color-fg-muted)";

  return (
    <aside className="surface relative overflow-hidden rise-in rise-in-d1">
      <div className="flex items-center justify-between border-b border-line px-5 py-3">
        <span className="kicker">{t.landing.panelLive}</span>
        <span className="inline-flex items-center gap-1.5 mono text-[0.7rem] text-fg-muted">
          <span className="dot dot-pulse text-accent" />
          {t.common.observingStatus}
        </span>
      </div>

      <div className="grid gap-5 px-6 py-7">
        <div className="flex items-end justify-between gap-4">
          <div>
            <p className="mono text-[0.74rem] uppercase tracking-[0.14em] text-fg-muted">
              {t.landing.panelSample}
            </p>
            <p className="display-md text-[1.1rem]! mt-1">
              <span className="mono text-fg-muted">
                {repo ? `${repo.owner}/` : "owner/"}
              </span>
              <span>{repo?.name ?? "repo"}</span>
            </p>
          </div>
          <div className="text-right">
            <p className="kicker">{t.landing.panelOverall}</p>
            <p
              className="data-value text-[3.6rem] leading-none tracking-tight"
              style={{ color: overallColor }}
            >
              {formatScore(quality?.overall)}
            </p>
          </div>
        </div>

        <HeroChart />

        <div className="grid grid-cols-2 gap-3 text-[0.84rem]">
          <MiniStat
            label={t.footer.freshness}
            value={formatScore(quality?.freshness)}
            tone={scoreTone(quality?.freshness)}
          />
          <MiniStat
            label={t.footer.adoption}
            value={formatScore(quality?.adoption)}
            tone={scoreTone(quality?.adoption)}
          />
          <MiniStat
            label={t.footer.reliability}
            value={formatScore(quality?.reliability)}
            tone={scoreTone(quality?.reliability)}
          />
          <MiniStat
            label={t.footer.abandonment}
            value={formatScore(quality?.abandonment)}
            tone={abandonmentTone(quality?.abandonment)}
          />
        </div>
      </div>

      <div className="border-t border-line px-5 py-3 flex justify-between text-[0.74rem]">
        <span className="mono text-fg-muted">
          {quality?.formulaVersion ?? "v2.0"}
        </span>
      </div>
    </aside>
  );
}

function HeroChart() {
  const points = [0.41, 0.46, 0.52, 0.55, 0.58, 0.62, 0.61, 0.64, 0.63, 0.65];
  const w = 300;
  const h = 60;
  const max = 1;
  const step = w / (points.length - 1);
  const path = points
    .map((p, i) => `${i === 0 ? "M" : "L"} ${i * step} ${h - (p / max) * h}`)
    .join(" ");
  const area = `${path} L ${w} ${h} L 0 ${h} Z`;
  return (
    <svg
      viewBox={`0 0 ${w} ${h}`}
      className="w-full h-[56px]"
      preserveAspectRatio="none"
      aria-hidden
    >
      <defs>
        <linearGradient id="sparkGrad" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="var(--color-accent)" stopOpacity="0.28" />
          <stop offset="100%" stopColor="var(--color-accent)" stopOpacity="0" />
        </linearGradient>
      </defs>
      <path d={area} fill="url(#sparkGrad)" />
      <path
        d={path}
        fill="none"
        stroke="var(--color-accent)"
        strokeWidth="1.6"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function MiniStat({
  label,
  value,
  tone
}: {
  label: string;
  value: string;
  tone: "ok" | "warn" | "danger" | "neutral";
}) {
  const color =
    tone === "ok"
      ? "var(--color-accent)"
      : tone === "warn"
        ? "var(--color-warn)"
        : tone === "danger"
          ? "var(--color-danger)"
          : "var(--color-fg-muted)";
  return (
    <div className="grid gap-1 border-t border-line pt-2">
      <span className="mono text-[0.68rem] uppercase tracking-[0.14em] text-fg-muted">
        {label}
      </span>
      <span className="data-value text-[0.94rem]" style={{ color }}>
        {value}
      </span>
    </div>
  );
}

function Kpi({ k, label }: { k: string; label: string }) {
  return (
    <div className="grid gap-1.5">
      <p className="data-value text-[2rem] leading-none tracking-tight text-fg">
        {k}
      </p>
      <p className="text-[0.78rem] text-fg-muted">{label}</p>
    </div>
  );
}

function DataQualitySection() {
  const t = useT();
  return (
    <section className="shell py-14 md:py-20">
      <div className="grid gap-8 border-y border-line py-10 md:grid-cols-[0.8fr_1.2fr] md:gap-14">
        <div className="grid content-start gap-4">
          <span className="kicker">{t.landing.dataEyebrow}</span>
          <h2 className="display-lg max-w-[20ch]">{t.landing.dataH2}</h2>
          <p className="max-w-[52ch] text-[0.96rem] leading-relaxed text-fg-dim">
            {t.landing.dataBody}
          </p>
          <Link
            to="/how-to-read"
            className="inline-flex w-fit items-center gap-1.5 text-[0.92rem] font-medium text-accent hover:gap-2.5 transition-all"
          >
            {t.landing.dataCta}
            <span className="arrow">→</span>
          </Link>
        </div>
        <div className="grid gap-4">
          {t.landing.dataItems.map((item, index) => (
            <div
              key={item.title}
              className="grid gap-3 border-b border-line pb-5 last:border-b-0 last:pb-0 sm:grid-cols-[3rem_1fr]"
            >
              <span className="data-value text-[0.9rem] text-accent">
                {String(index + 1).padStart(2, "0")}
              </span>
              <div className="grid gap-1.5">
                <h3 className="text-[1rem] font-semibold text-fg">
                  {item.title}
                </h3>
                <p className="text-[0.92rem] leading-relaxed text-fg-dim">
                  {item.body}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function PillarsSection() {
  const t = useT();
  return (
    <section className="shell py-16 md:py-24">
      <header className="grid gap-4 mb-12 md:mb-16">
        <span className="kicker">{t.landing.pillarsEyebrow}</span>
        <h2 className="display-lg max-w-[24ch]">{t.landing.pillarsH2}</h2>
      </header>

      <div className="grid gap-6 md:grid-cols-2 md:gap-8">
        <Pillar
          index="01"
          pillarLabel={t.landing.pillar}
          title={t.landing.pillar1Title}
          body={t.landing.pillar1Body}
          cta={{ to: "/discover", label: t.landing.pillar1Cta }}
          artifact={{
            label: t.landing.pillar1Artifact,
            value: "auto / strict / explore"
          }}
        />
        <Pillar
          index="02"
          pillarLabel={t.landing.pillar}
          title={t.landing.pillar2Title}
          body={t.landing.pillar2Body}
          cta={{ to: "/watchlist", label: t.landing.pillar2Cta }}
          artifact={{
            label: t.landing.pillar2Artifact,
            value: "score_drop · abandonment_up · flag_severe"
          }}
        />
      </div>
    </section>
  );
}

function Pillar({
  index,
  pillarLabel,
  title,
  body,
  cta,
  artifact
}: {
  index: string;
  pillarLabel: string;
  title: string;
  body: string;
  cta: { to: "/discover" | "/watchlist"; label: string };
  artifact: { label: string; value: string };
}) {
  return (
    <article className="surface p-6 md:p-8 grid gap-4 hover:border-line-strong transition-colors">
      <div className="flex items-center justify-between">
        <span className="data-value text-accent text-[0.86rem] tracking-wider">
          {index}
        </span>
        <span className="kicker">{pillarLabel}</span>
      </div>
      <h3 className="display-md">{title}</h3>
      <p className="text-[0.98rem] leading-[1.65] text-fg-dim">{body}</p>
      <div className="flex items-baseline gap-3 border-t border-line pt-4 mt-2">
        <span className="kicker">{artifact.label}</span>
        <code className="mono text-[0.82rem] text-fg-dim">
          {artifact.value}
        </code>
      </div>
      <Link
        to={cta.to}
        className="inline-flex items-center gap-1.5 mt-2 text-accent text-[0.92rem] font-medium hover:gap-2.5 transition-all"
      >
        {cta.label}
        <span className="arrow">→</span>
      </Link>
    </article>
  );
}

function FormulaSection() {
  const t = useT();
  return (
    <section className="shell py-16 md:py-24">
      <div className="surface relative overflow-hidden">
        <div
          className="absolute -top-20 -right-20 h-[280px] w-[280px] rounded-full opacity-30"
          style={{
            background:
              "radial-gradient(closest-side, var(--color-accent-glow), transparent)"
          }}
          aria-hidden
        />
        <div className="relative grid gap-10 p-8 md:grid-cols-[1fr_1.1fr] md:gap-16 md:p-14">
          <div className="grid gap-5 content-start">
            <span className="kicker accent">{t.landing.formulaEyebrow}</span>
            <h2 className="display-lg">{t.landing.formulaH2}</h2>
            <p className="max-w-[48ch] text-[0.98rem] leading-[1.7] text-fg-dim">
              {t.landing.formulaBody}
            </p>
          </div>

          <div className="grid gap-2 mono text-[0.84rem] leading-relaxed">
            <FormulaLine
              dim={t.footer.freshness.toLowerCase()}
              weight="0.20"
              expr="0.5 ^ (age_days / 180)"
            />
            <FormulaLine
              dim={t.footer.adoption.toLowerCase()}
              weight="0.15"
              expr="ln(r + 1) / ln(1001)"
            />
            <FormulaLine
              dim={t.footer.reliability.toLowerCase()}
              weight="0.40"
              expr="success / (success + failure)"
            />
            <FormulaLine
              dim={t.footer.abandonment.toLowerCase()}
              weight="0.25"
              expr="1 − freshness + regret_bump"
              highlight
            />
          </div>
        </div>
      </div>
    </section>
  );
}

function FormulaLine({
  dim,
  weight,
  expr,
  highlight = false
}: {
  dim: string;
  weight: string;
  expr: string;
  highlight?: boolean;
}) {
  return (
    <div
      className={`grid grid-cols-[110px_72px_1fr] items-baseline gap-3 border-b border-line py-2 ${
        highlight ? "text-accent" : ""
      }`}
    >
      <span className={highlight ? "text-accent" : "text-fg"}>{dim}</span>
      <span className="text-fg-muted">w={weight}</span>
      <span className={highlight ? "text-accent" : "text-fg-dim"}>{expr}</span>
    </div>
  );
}

function LivePreview() {
  const t = useT();
  const query = useQuery({
    queryKey: ["search", "explore", "landing"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&limit=4",
        signal
      )
  });

  return (
    <section className="shell py-16 md:py-24">
      <header className="grid gap-4 mb-10 md:mb-12">
        <div className="flex items-end justify-between gap-4 flex-wrap">
          <div className="grid gap-3">
            <span className="kicker">{t.landing.previewEyebrow}</span>
            <h2 className="display-lg max-w-[22ch]">{t.landing.previewH2}</h2>
          </div>
          <Link
            to="/discover"
            className="link-underline text-[0.92rem] text-fg-dim hover:text-accent"
          >
            {t.landing.previewSeeAll} <span className="arrow">→</span>
          </Link>
        </div>
      </header>

      <Ticker />

      <div className="grid gap-5 pt-10">
        {query.isLoading ? (
          <div className="py-16 text-center">
            <span className="kicker">{t.common.tuning}</span>
          </div>
        ) : query.isError ? (
          <div className="surface grid gap-2 p-8">
            <p className="kicker" style={{ color: "var(--color-danger)" }}>
              {t.common.offline}
            </p>
            <p className="text-[0.94rem] text-fg-dim">
              {t.common.offlineHint}{" "}
              <code className="inline">{t.common.cargoRun}</code>{" "}
              {t.common.offlineFrom}{" "}
              <code className="inline">{t.common.backendDir}</code>.
            </p>
          </div>
        ) : (
          (query.data?.items ?? []).map((repo, index) => (
            <RepoCard key={repo.artifactId} repo={repo} index={index} />
          ))
        )}
      </div>
    </section>
  );
}

function Ticker() {
  const t = useT();
  const query = useQuery({
    queryKey: ["search", "explore", "ticker"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&limit=12",
        signal
      )
  });
  const items = query.data?.items ?? [];
  const doubled = [...items, ...items];

  if (!items.length) {
    return (
      <div className="overflow-hidden rounded-md border border-line bg-surface/40 py-3 text-center mono text-[0.78rem] uppercase tracking-[0.18em] text-fg-muted">
        {t.landing.tickerTuning}
      </div>
    );
  }

  return (
    <div className="overflow-hidden rounded-md border border-line bg-surface/40 py-3">
      <div className="marquee-track gap-10 whitespace-nowrap">
        {doubled.map((repo, i) => {
          const tone = scoreTone(repo.quality?.overall);
          const color =
            tone === "danger"
              ? "var(--color-danger)"
              : tone === "warn"
                ? "var(--color-warn)"
                : tone === "ok"
                  ? "var(--color-accent)"
                  : "var(--color-fg-dim)";
          return (
            <span
              key={`${repo.artifactId}-${i}`}
              className="inline-flex items-baseline gap-2 mono text-[0.84rem]"
            >
              <span className="text-fg-dim">
                {repo.owner}/{repo.name}
              </span>
              <span className="data-value" style={{ color }}>
                {formatScore(repo.quality?.overall)}
              </span>
              <span className="text-fg-muted">
                ★{formatStars(repo.starsCount)}
              </span>
              <span className="text-fg-muted">
                · {formatRelative(repo.lastCommitAt).replace(" ago", "")}
              </span>
              <span className="text-fg-faint">·</span>
            </span>
          );
        })}
      </div>
    </div>
  );
}

function ClosingCTA() {
  const t = useT();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  return (
    <section className="shell py-20 md:py-28">
      <div className="surface relative overflow-hidden p-8 md:p-14">
        <div
          className="absolute inset-0 opacity-40"
          style={{
            background:
              "radial-gradient(800px 240px at 50% 100%, var(--color-accent-glow), transparent 70%)"
          }}
          aria-hidden
        />
        <div className="relative grid gap-8 md:grid-cols-[1.5fr_1fr] md:items-end">
          <div className="grid gap-4">
            <span className="kicker">{t.landing.closingEyebrow}</span>
            <h2 className="display-lg max-w-[20ch]">
              {t.landing.closingH2Part1}
              {" "}
              <br />
              <span className="accent">{t.landing.closingH2Part2}</span>
            </h2>
          </div>
          <div className="flex flex-wrap gap-3 md:justify-end">
            <Link to="/discover" className={buttonClass("outline")}>
              {t.landing.closingBrowse}
            </Link>
            {isAuthed ? (
              <Link to="/watchlist" className={buttonClass("primary")}>
                {t.landing.closingWatchlist}
                <span className="arrow">→</span>
              </Link>
            ) : (
              <Link to="/login" className={buttonClass("primary")}>
                {t.landing.closingStart}
                <span className="arrow">→</span>
              </Link>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}

export function LandingPage() {
  return (
    <>
      <Hero />
      <DataQualitySection />
      <LivePreview />
      <ClosingCTA />
    </>
  );
}
