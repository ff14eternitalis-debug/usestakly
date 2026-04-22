import { Link } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";

import { Chip } from "../components/Chip";
import { buttonClass } from "../components/Button";
import { RepoCard } from "../components/RepoCard";
import { apiGet } from "../lib/api-client";
import {
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import type { RepoSearchResponse } from "../lib/types";
import { useAuthStore } from "../state/auth-store";

function Hero() {
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  return (
    <section className="shell relative pt-10 pb-16 md:pt-16 md:pb-24">
      <div className="grid gap-10 md:grid-cols-[1.55fr_1fr] md:gap-16 items-end">
        <div className="grid gap-7 rise-in">
          <div className="flex items-center gap-3">
            <span className="callout-mark" />
            <p className="eyebrow">Issue&nbsp;No.&nbsp;001 · Field notes</p>
          </div>

          <h1 className="display-xl">
            The verdict on a repo
            <br />
            <span className="italic-accent">isn't its star count.</span>
          </h1>

          <p className="max-w-[52ch] text-[1.1rem] md:text-[1.22rem] leading-[1.55] text-ink-dim">
            UseStakly observes public open-source repositories through signals
            of <em className="italic-accent text-ink">actual usage</em> —
            freshness, adoption, reliability, abandonment — and raises its hand
            when a project you depend on starts to drift.
          </p>

          <div className="flex flex-wrap items-center gap-3 pt-2">
            <Link to="/discover" className={buttonClass("primary")}>
              Open the observatory
              <span className="arrow">→</span>
            </Link>
            {!isAuthed ? (
              <Link
                to="/login"
                className="link-underline font-sans text-[0.98rem] text-ink-dim"
              >
                or sign in to build a watchlist
              </Link>
            ) : (
              <Link
                to="/watchlist"
                className="link-underline font-sans text-[0.98rem] text-ink-dim"
              >
                straight to your watchlist
              </Link>
            )}
          </div>

          <div className="grid max-w-xl grid-cols-3 gap-4 pt-8 rise-in rise-in-delay-2">
            <FigureStat k="4" label="Quality dimensions" />
            <FigureStat k="v1" label="Public formula" />
            <FigureStat k="0%" label="Proprietary magic" />
          </div>
        </div>

        <aside className="card relative rise-in rise-in-delay-1">
          <div className="flex items-center justify-between border-b border-line px-5 py-3">
            <p className="kicker">Dispatch</p>
            <p className="kicker">live</p>
          </div>
          <blockquote className="px-6 py-7">
            <p className="display-md italic-accent font-display">
              “Stars measure interest.
              <br />
              Commits measure belief.
              <br />
              <span className="text-ember">Regret</span> measures truth.”
            </p>
            <footer className="mt-5 flex items-center gap-3">
              <span className="h-px w-10 bg-ink" />
              <span className="kicker">The editor's desk</span>
            </footer>
          </blockquote>
          <div className="border-t border-line px-5 py-3 font-mono text-[0.72rem] uppercase tracking-[0.18em] text-ink-muted flex justify-between">
            <span>formula_v1</span>
            <span className="text-ember">transparent · versioned · local</span>
          </div>
        </aside>
      </div>
    </section>
  );
}

function FigureStat({ k, label }: { k: string; label: string }) {
  return (
    <div className="border-t-2 border-ink pt-3">
      <p className="font-display italic-accent text-[2.4rem] leading-none">{k}</p>
      <p className="kicker mt-2">{label}</p>
    </div>
  );
}

function PillarsSection() {
  return (
    <section className="shell grid gap-10 py-14 md:py-20">
      <header className="grid gap-4">
        <p className="eyebrow">Two columns · one beat</p>
        <h2 className="display-lg max-w-[22ch]">
          What the register was missing, <span className="italic-accent">in two movements.</span>
        </h2>
      </header>

      <hr className="rule-double" />

      <div className="grid gap-10 md:grid-cols-2 md:gap-14">
        <Pillar
          ordinal="I."
          title="Discovery, scored by usage."
          body="Searches don't return hype. Each repo is measured against a transparent formula that combines commit cadence, adoption, build reliability, and signs of abandonment. The auto filter hides broken or unmaintained projects; explore mode shows everything with its receipts."
          artifactLabel="filter"
          artifactValue="auto / strict / explore"
        />
        <Pillar
          ordinal="II."
          title="Watchlist, with real alerts."
          body="Keep a short list of repositories you actually depend on. UseStakly diffs scores between recomputes and raises in-app notifications when abandonment rises, a severe flag lands, or overall quality drops below a threshold. No pull-request RSS, no maintainer silence going unnoticed."
          artifactLabel="triggers"
          artifactValue="score_drop · abandonment_up · flag_severe"
        />
      </div>
    </section>
  );
}

function Pillar({
  ordinal,
  title,
  body,
  artifactLabel,
  artifactValue
}: {
  ordinal: string;
  title: string;
  body: string;
  artifactLabel: string;
  artifactValue: string;
}) {
  return (
    <article className="grid gap-5 border-t border-line pt-8">
      <div className="flex items-baseline justify-between">
        <span className="font-display italic-accent text-[2.6rem] leading-none text-ember">
          {ordinal}
        </span>
        <span className="kicker">Pillar</span>
      </div>
      <h3 className="display-md font-display">{title}</h3>
      <p className="text-[1rem] leading-[1.65] text-ink-dim">{body}</p>
      <div className="mt-2 flex flex-wrap items-center gap-3 border-t border-dashed border-line-strong pt-4">
        <span className="kicker">{artifactLabel}</span>
        <code className="font-mono text-[0.85rem] text-ink">{artifactValue}</code>
      </div>
    </article>
  );
}

function FormulaSection() {
  return (
    <section className="shell grid gap-8 rounded-sm border border-ink bg-ink/95 px-8 py-14 text-paper-soft md:grid-cols-[1fr_1.1fr] md:gap-16 md:px-14 md:py-20">
      <div className="grid gap-6">
        <p
          className="kicker"
          style={{ color: "rgba(242, 236, 223, 0.6)" }}
        >
          formula_v1.toml · open, audited, local
        </p>
        <h2 className="display-lg font-display text-paper-soft">
          The score is a <span className="italic-accent">statement</span>, not a black box.
        </h2>
        <p
          className="max-w-[48ch] text-[1.02rem] leading-[1.7]"
          style={{ color: "rgba(242, 236, 223, 0.78)" }}
        >
          Each dimension is a named equation with a known half-life, threshold
          or log-saturation. Every score carries the formula version that
          produced it, so tomorrow's v2 doesn't quietly rewrite yesterday's
          verdict.
        </p>
      </div>

      <div className="grid gap-3 font-mono text-[0.82rem] leading-relaxed">
        <FormulaLine dim="freshness" weight="0.20" expr="0.5 ^ (age_days / 180)" />
        <FormulaLine dim="adoption" weight="0.15" expr="ln(r + 1) / ln(1001)" />
        <FormulaLine
          dim="reliability"
          weight="0.40"
          expr="success / (success + failure)"
        />
        <FormulaLine
          dim="abandonment"
          weight="0.25"
          expr="1 − freshness + regret_bump"
          tone="ember"
        />
      </div>
    </section>
  );
}

function FormulaLine({
  dim,
  weight,
  expr,
  tone
}: {
  dim: string;
  weight: string;
  expr: string;
  tone?: "ember";
}) {
  return (
    <div className="grid grid-cols-[120px_80px_1fr] items-baseline gap-3 border-b border-dashed border-[color:rgba(242,236,223,0.15)] pb-2">
      <span
        className={tone === "ember" ? "text-ember-soft" : "text-paper-soft"}
      >
        {dim}
      </span>
      <span style={{ color: "rgba(242, 236, 223, 0.5)" }}>w={weight}</span>
      <span style={{ color: "rgba(242, 236, 223, 0.82)" }}>{expr}</span>
    </div>
  );
}

function LivePreview() {
  const query = useQuery({
    queryKey: ["search", "explore", "landing"],
    queryFn: ({ signal }) =>
      apiGet<RepoSearchResponse>(
        "/api/repos/search?filter=explore&limit=4",
        signal
      )
  });

  return (
    <section className="shell grid gap-10 py-14 md:py-20">
      <header className="grid gap-4">
        <div className="flex items-center justify-between gap-4 flex-wrap">
          <p className="eyebrow">From the current issue</p>
          <Link to="/discover" className="link-underline text-[0.92rem]">
            See all entries <span className="arrow">→</span>
          </Link>
        </div>
        <h2 className="display-lg max-w-[24ch]">
          Dispatches from the
          <span className="italic-accent"> observation deck.</span>
        </h2>
      </header>

      <Ticker />

      <div className="grid gap-6">
        {query.isLoading ? (
          <div className="grid gap-4 py-10 text-center text-ink-muted">
            <span className="kicker">Tuning the instruments…</span>
          </div>
        ) : query.isError ? (
          <div className="grid gap-2 border-t border-line py-10 text-ink-muted">
            <p className="kicker text-ember">Observatory offline</p>
            <p className="text-[0.98rem]">
              The backend isn't responding. Start it with{" "}
              <code className="font-mono text-ink">cargo run</code> from{" "}
              <code className="font-mono text-ink">backend/</code>.
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
      <div className="overflow-hidden border-y border-line py-3 text-center font-mono text-[0.8rem] uppercase tracking-[0.2em] text-ink-muted">
        ————— tuning ————— tuning ————— tuning —————
      </div>
    );
  }

  return (
    <div className="overflow-hidden border-y border-line py-3">
      <div className="ticker-track gap-8 whitespace-nowrap">
        {doubled.map((repo, i) => {
          const tone = scoreTone(repo.quality?.overall);
          return (
            <span
              key={`${repo.artifactId}-${i}`}
              className="inline-flex items-baseline gap-2 font-mono text-[0.86rem]"
            >
              <span className="text-ink-muted">
                {repo.owner}/{repo.name}
              </span>
              <span
                className="data-value"
                style={{
                  color:
                    tone === "danger"
                      ? "var(--color-ember)"
                      : tone === "warn"
                        ? "var(--color-ochre)"
                        : tone === "ok"
                          ? "var(--color-moss)"
                          : "var(--color-ink)"
                }}
              >
                {formatScore(repo.quality?.overall)}
              </span>
              <span className="text-ink-muted">★{formatStars(repo.starsCount)}</span>
              <span className="text-ink-muted">
                · {formatRelative(repo.lastCommitAt).replace(" ago", "")}
              </span>
              <span className="text-line-strong">·</span>
            </span>
          );
        })}
      </div>
    </div>
  );
}

function ClosingCTA() {
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  return (
    <section className="shell grid gap-6 py-16 md:py-24">
      <hr className="rule-double" />
      <div className="grid gap-6 md:grid-cols-[1.4fr_1fr] md:items-end">
        <div className="grid gap-3">
          <p className="eyebrow">End of dispatch</p>
          <h2 className="display-lg max-w-[22ch]">
            Keep a short list. <span className="italic-accent">We'll keep watch.</span>
          </h2>
        </div>
        <div className="flex flex-wrap gap-3 md:justify-end">
          <Link to="/discover" className={buttonClass("outline")}>
            Browse repositories
            <span className="arrow">→</span>
          </Link>
          {isAuthed ? (
            <Link to="/watchlist" className={buttonClass("primary")}>
              Open watchlist
              <span className="arrow">→</span>
            </Link>
          ) : (
            <Link to="/login" className={buttonClass("primary")}>
              Sign in
              <span className="arrow">→</span>
            </Link>
          )}
        </div>
      </div>

      <div className="grid gap-3 pt-6 md:grid-cols-3">
        <Chip tone="neutral">self-hosted</Chip>
        <Chip tone="info">no SaaS lock-in</Chip>
        <Chip tone="ok">embeddings run local</Chip>
      </div>
    </section>
  );
}

export function LandingPage() {
  return (
    <>
      <Hero />
      <PillarsSection />
      <FormulaSection />
      <LivePreview />
      <ClosingCTA />
    </>
  );
}
