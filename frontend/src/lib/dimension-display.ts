import type { DimensionState } from "./types";
import { abandonmentTone, scoreTone } from "./format";

export type ScoreTone = "ok" | "warn" | "danger" | "neutral";

export function dimensionTone(
  state: DimensionState | undefined,
  invert = false
): ScoreTone {
  if (!state) return "neutral";
  switch (state.displayState) {
    case "awaiting_community":
    case "neutral_default":
    case "not_captured":
      return "neutral";
    case "partial":
    case "missing_commit":
      return state.displayState === "missing_commit" ? "danger" : "warn";
    case "measured":
    case "growing":
      if (invert) {
        return abandonmentTone(state.value);
      }
      return scoreTone(state.value);
    default:
      return "neutral";
  }
}

export function stateForKey(
  states: DimensionState[] | undefined,
  key: string
): DimensionState | undefined {
  return states?.find((s) => s.key === key);
}

export function proofTierLabel(
  tier: string,
  labels: Record<string, string>
): string {
  return labels[tier] ?? tier;
}
