import type { RecommendationExplanation, SearchFilterSummary } from "./types";

export type ExplanationCopy = {
  codes: Record<string, string>;
  filterSummary: Record<string, string>;
};

export function explainCode(copy: ExplanationCopy, code: string): string {
  return copy.codes[code] ?? code.replaceAll("_", " ");
}

export function labelsForExplanation(
  copy: ExplanationCopy,
  explanation: RecommendationExplanation | null | undefined
): { included: string[]; caveats: string[] } {
  if (!explanation) {
    return { included: [], caveats: [] };
  }
  return {
    included: explanation.includedBecause.map((code) => explainCode(copy, code)),
    caveats: explanation.caveats.map((code) => explainCode(copy, code))
  };
}

export function filterSummaryLabel(
  copy: ExplanationCopy,
  summary: SearchFilterSummary | null | undefined
): string | null {
  const code = summary?.messageCode;
  if (!code) return null;
  return copy.filterSummary[code] ?? null;
}
