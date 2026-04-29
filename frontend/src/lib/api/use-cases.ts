import { apiGet, apiPost } from "../api-client";
import type {
  CreateUseCaseWatchResponse,
  UseCaseRecommendationReport,
  UseCaseWatch
} from "../types";

export function recommendUseCase(body: {
  query: string;
  riskTolerance?: "low" | "medium" | "high";
  limit?: number;
}): Promise<UseCaseRecommendationReport> {
  return apiPost<UseCaseRecommendationReport>("/api/use-cases/recommend", body);
}

export function createUseCaseWatch(body: {
  query: string;
  label?: string;
  riskTolerance?: "low" | "medium" | "high";
}): Promise<CreateUseCaseWatchResponse> {
  return apiPost<CreateUseCaseWatchResponse>("/api/use-cases/watch", body);
}

export function listUseCaseWatches(signal?: AbortSignal): Promise<UseCaseWatch[]> {
  return apiGet<UseCaseWatch[]>("/api/use-cases/watch", signal);
}
