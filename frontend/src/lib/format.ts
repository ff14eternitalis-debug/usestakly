export function formatScore(value: number | null | undefined): string {
  if (value === null || value === undefined || Number.isNaN(value)) return "—";
  return value.toFixed(2).replace(/^0/, "·");
}

export function scoreTone(
  overall: number | null | undefined
): "ok" | "warn" | "danger" | "neutral" {
  if (overall === null || overall === undefined || Number.isNaN(overall))
    return "neutral";
  if (overall >= 0.7) return "ok";
  if (overall >= 0.45) return "warn";
  return "danger";
}

export function abandonmentTone(
  value: number | null | undefined
): "ok" | "warn" | "danger" | "neutral" {
  if (value === null || value === undefined || Number.isNaN(value))
    return "neutral";
  if (value <= 0.3) return "ok";
  if (value <= 0.6) return "warn";
  return "danger";
}

export function formatStars(count: number): string {
  if (count >= 1000) return `${(count / 1000).toFixed(count >= 10000 ? 0 : 1)}k`;
  return String(count);
}

export function formatRelative(
  isoDate: string | null | undefined,
  reference: Date = new Date()
): string {
  if (!isoDate) return "—";
  const then = new Date(isoDate);
  if (Number.isNaN(then.getTime())) return "—";
  const deltaSec = Math.round((reference.getTime() - then.getTime()) / 1000);
  const abs = Math.abs(deltaSec);
  const suffix = deltaSec >= 0 ? "ago" : "from now";
  if (abs < 60) return `${abs}s ${suffix}`;
  const minutes = Math.round(abs / 60);
  if (minutes < 60) return `${minutes}m ${suffix}`;
  const hours = Math.round(minutes / 60);
  if (hours < 48) return `${hours}h ${suffix}`;
  const days = Math.round(hours / 24);
  if (days < 30) return `${days}d ${suffix}`;
  const months = Math.round(days / 30);
  if (months < 24) return `${months}mo ${suffix}`;
  const years = Math.round(months / 12);
  return `${years}y ${suffix}`;
}

const FLAG_LABELS: Record<string, string> = {
  "security-issue": "security issue",
  broken: "broken",
  deprecated: "deprecated",
  unmaintained: "unmaintained",
  abandoned: "abandoned"
};

export function flagLabel(flag: string): string {
  return FLAG_LABELS[flag] ?? flag.replace(/-/g, " ");
}

const NOTIFICATION_LABELS: Record<string, string> = {
  score_drop: "score drop",
  abandonment_up: "abandonment rising",
  flag_added: "new flag",
  flag_severe: "severe flag"
};

export function notificationLabel(kind: string): string {
  return NOTIFICATION_LABELS[kind] ?? kind.replace(/_/g, " ");
}
