export function getSiteOrigin(): string {
  const raw = import.meta.env.VITE_SITE_ORIGIN ?? "http://127.0.0.1:5173";
  return raw.replace(/\/+$/, "");
}

export function absoluteUrl(path: string): string {
  const origin = getSiteOrigin();
  const p = path.startsWith("/") ? path : `/${path}`;
  return `${origin}${p}`;
}
