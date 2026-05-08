import { apiDelete, apiGet, apiPost, apiPut } from "../api-client";
import type {
  AccountSummary,
  AgentTokenCreated,
  AgentTokenSummary,
  DigestTimePreset,
  NotificationChannelSummary,
  NotificationPreferences,
  NotificationChannelType
} from "../types";

export function getAccountSummary(signal?: AbortSignal): Promise<AccountSummary> {
  return apiGet<AccountSummary>("/api/account/summary", signal);
}

export function getAgentTokens(signal?: AbortSignal): Promise<AgentTokenSummary[]> {
  return apiGet<AgentTokenSummary[]>("/api/agent-tokens", signal);
}

export function createAgentToken(label: string): Promise<AgentTokenCreated> {
  return apiPost<AgentTokenCreated>("/api/agent-tokens", { label });
}

export function revokeAgentToken(id: string): Promise<void> {
  return apiDelete(`/api/agent-tokens/${id}`);
}

export function getNotificationChannels(
  signal?: AbortSignal
): Promise<NotificationChannelSummary[]> {
  return apiGet<NotificationChannelSummary[]>(
    "/api/account/notification-channels",
    signal
  );
}

export function upsertNotificationChannel(body: {
  channelType: NotificationChannelType;
  label?: string;
  email?: string;
  webhookUrl?: string;
  enabled?: boolean;
  criticalAlertsEnabled?: boolean;
  dailyDigestEnabled?: boolean;
}): Promise<NotificationChannelSummary> {
  return apiPost<NotificationChannelSummary>(
    "/api/account/notification-channels",
    body
  );
}

export function deleteNotificationChannel(id: string): Promise<void> {
  return apiDelete(`/api/account/notification-channels/${id}`);
}

export function testNotificationChannel(id: string): Promise<{ ok: boolean }> {
  return apiPost<{ ok: boolean }>(
    `/api/account/notification-channels/${id}/test`
  );
}

export function getNotificationPreferences(
  signal?: AbortSignal
): Promise<NotificationPreferences> {
  return apiGet<NotificationPreferences>(
    "/api/account/notification-preferences",
    signal
  );
}

export function updateNotificationPreferences(body: {
  digestTimePreset: DigestTimePreset;
  timezone: string;
}): Promise<NotificationPreferences> {
  return apiPut<NotificationPreferences>(
    "/api/account/notification-preferences",
    body
  );
}
