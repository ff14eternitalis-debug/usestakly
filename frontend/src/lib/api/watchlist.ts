import { apiDelete, apiGet, apiPost } from "../api-client";
import type { WatchedRepo } from "../types";

export function getWatchlist(signal?: AbortSignal): Promise<WatchedRepo[]> {
  return apiGet<WatchedRepo[]>("/api/watchlist", signal);
}

export function addRepoToWatchlist(externalArtifactId: string): Promise<void> {
  return apiPost("/api/watchlist", { externalArtifactId });
}

export function removeRepoFromWatchlist(externalArtifactId: string): Promise<void> {
  return apiDelete(`/api/watchlist/${externalArtifactId}`);
}
