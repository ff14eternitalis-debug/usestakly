import { Link } from "@tanstack/react-router";

import { useT } from "../i18n";
import type { RepoSearchResult } from "../lib/types";
import {
  abandonmentTone,
  flagLabel,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import { radarSummary, radarTone } from "../lib/radar";
import { Chip } from "./Chip";
import { ScoreBar } from "./ScoreBar";

type Props = {
  repo: RepoSearchResult;
  index?: number;
  showRadarSummary?: boolean;
};

function toneFromFlag(flag: string): "danger" | "warn" | "neutral" {
  if (flag === "security-issue" || flag === "broken") return "danger";
  if (flag === "deprecated" || flag === "unmaintained" || flag === "abandoned")
    return "warn";
  return "neutral";
}

function scoreColor(tone: "ok" | "warn" | "danger" | "neutral"): string {
  if (tone === "danger") return "var(--color-danger)";
  if (tone === "warn") return "var(--color-warn)";
  if (tone === "ok") return "var(--color-accent)";
  return "var(--color-fg-muted)";
}

export function RepoCard({ repo, index, showRadarSummary = false }: Props) {
  const t = useT();
  const q = repo.quality;
  const overallTone = scoreTone(q?.overall);
  return (
    <Link
      to="/repos/$id"
      params={{ id: repo.artifactId }}
      className="group block"
    >
      <article className="relative grid gap-4 rounded-[10px] border border-line bg-surface/40 p-5 transition-all duration-200 hover:border-line-strong hover:bg-surface md:p-6">
        {index !== undefined ? (
          <span className="kicker absolute right-5 top-5 data-value">
            {String(index + 1).padStart(2, "0")}
          </span>
        ) : null}

        <div className="grid gap-2 pr-10">
          <div className="flex items-baseline gap-2 flex-wrap">
            <h3 className="display-md !text-[1.15rem] md:!text-[1.3rem]">
              <span className="font-mono text-[0.78em] font-normal text-fg-muted mr-1">
                {repo.owner}/
              </span>
              <span className="group-hover:text-accent transition-colors">
                {repo.name}
              </span>
            </h3>
          </div>
          {repo.description ? (
            <p className="max-w-[60ch] text-[0.94rem] leading-relaxed text-fg-dim">
              {repo.description}
            </p>
          ) : null}
        </div>

        <div className="flex flex-wrap items-center gap-1.5">
          {repo.language ? <Chip tone="info">{repo.language}</Chip> : null}
          {repo.licenseSpdx ? (
            <Chip tone="neutral">{repo.licenseSpdx}</Chip>
          ) : null}
          {repo.archived ? <Chip tone="warn">archived</Chip> : null}
          {repo.radar ? (
            <Chip tone={radarTone(repo.radar.maturityBand)}>
              {repo.radar.maturityBand}
            </Chip>
          ) : null}
          {repo.categories.slice(0, 3).map((category) => (
            <Chip key={category.category} tone="accent">
              {category.category}
            </Chip>
          ))}
          {q?.flags.map((flag) => (
            <Chip key={flag} tone={toneFromFlag(flag)}>
              {flagLabel(flag)}
            </Chip>
          ))}
          {repo.topics.slice(0, 3).map((t) => (
            <Chip key={t} tone="neutral">
              #{t}
            </Chip>
          ))}
        </div>

        {showRadarSummary && repo.radar ? (
          <p className="max-w-[70ch] border-l border-line pl-3 text-[0.86rem] leading-relaxed text-fg-dim">
            <span className="font-medium text-fg">Radar.</span>{" "}
            {radarSummary(repo.radar, t.radar)}
          </p>
        ) : null}

        <div className="grid gap-4 pt-2 md:grid-cols-[220px_1fr] md:gap-8">
          <div className="grid gap-2 border-t border-line pt-4 md:border-0 md:border-r md:pt-0 md:pr-6">
            <span className="kicker">Overall</span>
            <div className="flex items-baseline gap-3">
              <span
                className="data-value text-[3.2rem] font-medium leading-none tracking-tight"
                style={{ color: scoreColor(overallTone) }}
              >
                {formatScore(q?.overall)}
              </span>
            </div>
            <div className="flex flex-wrap gap-x-4 gap-y-1 pt-1 text-[0.82rem]">
              <span className="data-value text-fg-dim">
                ★ {formatStars(repo.starsCount)}
              </span>
              <span className="data-value text-fg-muted">
                {formatRelative(repo.lastCommitAt).replace(" ago", "")}
              </span>
              {q?.formulaVersion ? (
                <span className="mono text-fg-muted">
                  {q.formulaVersion}
                </span>
              ) : null}
            </div>
          </div>

          <div className="grid gap-x-6 gap-y-3 md:grid-cols-2">
            <ScoreBar
              label="Freshness"
              value={q?.freshness ?? null}
              tone={scoreTone(q?.freshness ?? null)}
            />
            <ScoreBar
              label="Adoption"
              value={q?.adoption ?? null}
              tone={scoreTone(q?.adoption ?? null)}
            />
            <ScoreBar
              label="Reliability"
              value={q?.reliability ?? null}
              tone={scoreTone(q?.reliability ?? null)}
            />
            <ScoreBar
              label="Abandonment"
              value={q?.abandonment ?? null}
              tone={abandonmentTone(q?.abandonment ?? null)}
              invert
            />
          </div>
        </div>

        <div className="flex items-center justify-between pt-2 text-[0.82rem]">
          <span className="text-fg-dim group-hover:text-accent transition-colors">
            View profile <span className="arrow">→</span>
          </span>
          <span
            className="mono text-[0.72rem] uppercase tracking-[0.16em] text-fg-muted hover:text-fg"
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              window.open(repo.htmlUrl, "_blank", "noreferrer");
            }}
            role="button"
            tabIndex={0}
          >
            github ↗
          </span>
        </div>
      </article>
    </Link>
  );
}
