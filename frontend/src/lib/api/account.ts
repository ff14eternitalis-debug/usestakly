import { apiDelete, apiGet, apiPost } from "../api-client";
import type { AccountSummary, AgentTokenCreated, AgentTokenSummary } from "../types";

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
