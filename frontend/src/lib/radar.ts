import type { RepoRadarSnapshot } from "./types";

type Tone = "accent" | "info" | "warn" | "danger" | "neutral";

export type RadarCopy = {
  established: string;
  emerging: string;
  experimental: string;
  stale: string;
  noisy: string;
  trendStrong: string;
  trendModerate: string;
};

export function radarTone(band: string): Tone {
  if (band === "established") return "accent";
  if (band === "emerging") return "info";
  if (band === "experimental") return "warn";
  if (band === "stale") return "danger";
  return "neutral";
}

export function radarSummary(
  radar: RepoRadarSnapshot,
  copy: RadarCopy
): string {
  const base =
    copy[radar.maturityBand as keyof Omit<RadarCopy, "trendStrong" | "trendModerate">] ??
    copy.noisy;

  if (radar.trendSignal >= 0.85) {
    return `${base} ${copy.trendStrong}`;
  }

  if (radar.trendSignal >= 0.55 && radar.maturityBand !== "established") {
    return `${base} ${copy.trendModerate}`;
  }

  return base;
}
