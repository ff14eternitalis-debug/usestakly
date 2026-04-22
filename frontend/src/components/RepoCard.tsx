import { Link } from "@tanstack/react-router";

import type { RepoSearchResult } from "../lib/types";
import {
  abandonmentTone,
  flagLabel,
  formatRelative,
  formatScore,
  formatStars,
  scoreTone
} from "../lib/format";
import { Chip } from "./Chip";
import { ScoreBar } from "./ScoreBar";

type Props = {
  repo: RepoSearchResult;
  index?: number;
};

function toneFromFlag(flag: string): "danger" | "warn" | "neutral" {
  if (flag === "security-issue" || flag === "broken") return "danger";
  if (flag === "deprecated" || flag === "unmaintained" || flag === "abandoned")
    return "warn";
  return "neutral";
}

export function RepoCard({ repo, index }: Props) {
  const q = repo.quality;
  const overallTone = scoreTone(q?.overall);
  return (
    <article className="group relative grid gap-5 border-t border-line pt-6">
      {index !== undefined ? (
        <span className="kicker absolute right-0 top-6 data-value">
          No. {String(index + 1).padStart(2, "0")}
        </span>
      ) : null}

      <div className="grid gap-2">
        <Link
          to="/repos/$id"
          params={{ id: repo.artifactId }}
          className="block"
        >
          <h3 className="display-md font-display">
            <span className="text-ink-muted font-mono text-[0.82em] mr-2">
              {repo.owner}/
            </span>
            <span className="italic-accent">{repo.name}</span>
          </h3>
        </Link>
        {repo.description ? (
          <p className="max-w-[56ch] text-[0.98rem] leading-relaxed text-ink-dim">
            {repo.description}
          </p>
        ) : null}
      </div>

      <div className="flex flex-wrap items-center gap-2">
        {repo.language ? <Chip tone="info">{repo.language}</Chip> : null}
        {repo.licenseSpdx ? (
          <Chip tone="neutral">{repo.licenseSpdx}</Chip>
        ) : null}
        {repo.archived ? <Chip tone="warn">archived</Chip> : null}
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

      <div className="grid gap-4 md:grid-cols-[220px_1fr]">
        <div className="grid gap-2">
          <span className="kicker">Overall</span>
          <div className="flex items-baseline gap-2">
            <span
              className="font-display italic-accent data-value text-[3.4rem] leading-none"
              style={{
                color:
                  overallTone === "danger"
                    ? "var(--color-ember)"
                    : overallTone === "warn"
                      ? "var(--color-ochre)"
                      : overallTone === "ok"
                        ? "var(--color-ink)"
                        : "var(--color-ink-muted)"
              }}
            >
              {formatScore(q?.overall)}
            </span>
            <span className="kicker">
              formula {q?.formulaVersion ?? "—"}
            </span>
          </div>
          <div className="flex gap-4 pt-1">
            <span className="data-value text-[0.85rem] text-ink-dim">
              ★ {formatStars(repo.starsCount)}
            </span>
            <span className="data-value text-[0.85rem] text-ink-dim">
              commits · {formatRelative(repo.lastCommitAt)}
            </span>
          </div>
        </div>

        <div className="grid gap-3 md:grid-cols-2">
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

      <div className="flex items-center justify-between text-[0.84rem]">
        <Link
          to="/repos/$id"
          params={{ id: repo.artifactId }}
          className="link-underline text-ink group-hover:text-ember"
        >
          Read the full verdict <span className="arrow">→</span>
        </Link>
        <a
          href={repo.htmlUrl}
          target="_blank"
          rel="noreferrer"
          className="font-mono text-[0.76rem] uppercase tracking-[0.16em] text-ink-muted hover:text-ink"
        >
          github.com ↗
        </a>
      </div>
    </article>
  );
}
