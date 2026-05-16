import type { SearchFilter } from "../../../lib/types";

export const PAGE_SIZE = 20;

export const LANGUAGE_OPTIONS = ["", "TypeScript", "JavaScript", "Python", "Rust", "Go"];
export const TOPIC_OPTIONS = [
  "react",
  "typescript",
  "orm",
  "database",
  "table",
  "testing",
  "auth",
  "http",
  "css",
  "api"
];
export const SCORE_OPTIONS: { value: number | ""; label: string }[] = [
  { value: "", label: "—" },
  { value: 0.45, label: "≥ 0.45" },
  { value: 0.6, label: "≥ 0.60" },
  { value: 0.75, label: "≥ 0.75" }
];
export const RISK_OPTIONS: { value: number | ""; label: string }[] = [
  { value: "", label: "—" },
  { value: 0.35, label: "≤ 0.35" },
  { value: 0.2, label: "≤ 0.20" },
  { value: 0.1, label: "≤ 0.10" }
];

export const SearchIcon = (
  <svg
    width="16"
    height="16"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    aria-hidden
  >
    <circle cx="11" cy="11" r="7" />
    <line x1="21" y1="21" x2="16.65" y2="16.65" />
  </svg>
);

export function FilterChip({
  label,
  onRemove
}: {
  label: string;
  onRemove: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onRemove}
      className="inline-flex items-center gap-2 rounded-[999px] border border-line bg-bg-subtle px-3 py-1.5 text-[0.78rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent"
    >
      {label}
      <span aria-hidden>×</span>
    </button>
  );
}

export type DiscoverFilterMode = {
  value: SearchFilter;
  label: string;
  hint: string;
};
