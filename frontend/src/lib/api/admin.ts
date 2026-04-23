import { apiGetWithInit, apiPostWithInit } from "../api-client";
import type { PendingRepoSignal } from "../types";

export function getPendingRepoSignals(adminToken: string): Promise<PendingRepoSignal[]> {
  return apiGetWithInit<PendingRepoSignal[]>("/api/admin/repo-signals/pending", {
    headers: { "x-admin-token": adminToken.trim() }
  });
}

export function reviewPendingRepoSignal(
  id: string,
  action: "approve" | "reject",
  adminToken: string
): Promise<void> {
  return apiPostWithInit(
    `/api/admin/repo-signals/${id}/review`,
    { action },
    {
      headers: { "x-admin-token": adminToken.trim() }
    }
  );
}
