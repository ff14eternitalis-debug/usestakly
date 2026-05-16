import { apiGet, apiPost } from "../api-client";
import type { RefreshRepoResponse, RepoProfile, RepoViewerState } from "../types";

export function getRepoProfile(id: string, signal?: AbortSignal): Promise<RepoProfile> {
  return apiGet<RepoProfile>(`/api/repos/${id}`, signal);
}

export function refreshRepoProfile(
  id: string,
  signal?: AbortSignal
): Promise<RefreshRepoResponse> {
  return apiPost<RefreshRepoResponse>(`/api/repos/${id}/refresh`, {}, signal);
}

export function getRepoViewerState(
  id: string,
  signal?: AbortSignal
): Promise<RepoViewerState> {
  return apiGet<RepoViewerState>(`/api/repos/${id}/viewer-state`, signal);
}

export function createRepoSignal(
  id: string,
  body: {
    signal: string;
    evidenceUrl?: string;
    evidenceDescription?: string;
  }
): Promise<void> {
  return apiPost(`/api/repos/${id}/signals`, body);
}

export function disputeRepoSignal(
  repoId: string,
  signalId: string,
  body: { reason: string }
): Promise<void> {
  return apiPost(`/api/repos/${repoId}/signals/${signalId}/dispute`, body);
}
