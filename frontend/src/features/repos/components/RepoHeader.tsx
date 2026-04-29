import { Link } from "@tanstack/react-router";

import { Button, buttonClass } from "../../../components/Button";
import { Chip } from "../../../components/Chip";
import { flagLabel } from "../../../lib/format";
import { loginSearch } from "../../../lib/return-to";
import type { RepoProfile } from "../../../lib/types";

function toneFromFlag(flag: string): "danger" | "warn" | "neutral" {
  if (flag === "security-issue" || flag === "broken") return "danger";
  if (flag === "deprecated" || flag === "unmaintained" || flag === "abandoned") {
    return "warn";
  }
  return "neutral";
}

type RepoHeaderProps = {
  repo: RepoProfile;
  isAuthed: boolean;
  watching: boolean;
  watchPending: boolean;
  unwatchPending: boolean;
  addToWatchlist(): void;
  removeFromWatchlist(): void;
  signInToWatchLabel: string;
  signInToWatchHint: string;
  addLabel: string;
  addingLabel: string;
  unwatchLabel: string;
  unwatchingLabel: string;
  viewOnGithubLabel: string;
};

export function RepoHeader({
  repo,
  isAuthed,
  watching,
  watchPending,
  unwatchPending,
  addToWatchlist,
  removeFromWatchlist,
  signInToWatchLabel,
  signInToWatchHint,
  addLabel,
  addingLabel,
  unwatchLabel,
  unwatchingLabel,
  viewOnGithubLabel
}: RepoHeaderProps) {
  const q = repo.quality;

  return (
    <header className="grid gap-5">
      <div className="grid gap-2">
        <p className="mono text-[0.82rem] uppercase tracking-[0.14em] text-fg-muted">
          {repo.owner} / {repo.name}
        </p>
        <h1 className="display-xl !text-[clamp(2.2rem,5vw,3.8rem)]">{repo.name}</h1>
        {repo.description ? (
          <p className="max-w-[64ch] text-[1.02rem] leading-[1.6] text-fg-dim">
            {repo.description}
          </p>
        ) : null}
      </div>

      <div className="flex flex-wrap items-center gap-1.5">
        {repo.language ? <Chip tone="info">{repo.language}</Chip> : null}
        {repo.licenseSpdx ? <Chip tone="neutral">{repo.licenseSpdx}</Chip> : null}
        {repo.archived ? <Chip tone="warn">archived</Chip> : null}
        {repo.categories.map((category) => (
          <Chip key={category.category} tone="accent">
            {category.category}
          </Chip>
        ))}
        {q?.flags.map((flag) => (
          <Chip key={flag} tone={toneFromFlag(flag)}>
            {flagLabel(flag)}
          </Chip>
        ))}
        {repo.topics.map((topic) => (
          <Chip key={topic} tone="neutral">
            #{topic}
          </Chip>
        ))}
      </div>

      <div className="flex flex-wrap items-center gap-3 pt-1">
        {isAuthed ? (
          watching ? (
            <Button variant="outline" onClick={removeFromWatchlist} disabled={unwatchPending}>
              {unwatchPending ? unwatchingLabel : unwatchLabel}
            </Button>
          ) : (
            <Button
              variant="primary"
              onClick={addToWatchlist}
              disabled={watchPending}
              iconAfter={<span className="arrow">+</span>}
            >
              {watchPending ? addingLabel : addLabel}
            </Button>
          )
        ) : (
          <div className="grid gap-1.5">
            <Link
              to="/login"
              search={loginSearch()}
              className={buttonClass("outline")}
            >
              {signInToWatchLabel}
            </Link>
            <p className="text-[0.84rem] leading-snug text-fg-dim">
              {signInToWatchHint}
            </p>
          </div>
        )}
        <a
          href={repo.htmlUrl}
          target="_blank"
          rel="noreferrer"
          className="inline-flex items-center gap-1.5 text-[0.86rem] text-fg-dim hover:text-accent transition-colors"
        >
          {viewOnGithubLabel} ↗
        </a>
      </div>
    </header>
  );
}
