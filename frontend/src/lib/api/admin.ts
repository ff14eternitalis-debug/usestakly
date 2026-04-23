import { apiGetWithInit, apiPostWithInit } from "../api-client";
import type { McpMetricsReport, McpMetricsWindow, PendingRepoSignal } from "../types";

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

export function getMcpMetrics(
  adminToken: string,
  window: McpMetricsWindow
): Promise<McpMetricsReport> {
  return apiGetWithInit<McpMetricsReport>(
    `/api/admin/mcp/metrics?window=${window}`,
    {
      headers: { "x-admin-token": adminToken.trim() }
    }
  );
}
