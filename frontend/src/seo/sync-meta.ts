import { getSiteOrigin } from "./site-origin";
import type { SeoPayload } from "./route-meta";

function upsertMeta(attr: "name" | "property", key: string, content: string) {
  const selector = `meta[${attr}="${key}"]`;
  let el = document.head.querySelector(selector);
  if (!el) {
    el = document.createElement("meta");
    el.setAttribute(attr, key);
    document.head.appendChild(el);
  }
  el.setAttribute("content", content);
}

function upsertLink(rel: string, href: string) {
  const selector = `link[rel="${rel}"]`;
  let el = document.head.querySelector(selector) as HTMLLinkElement | null;
  if (!el) {
    el = document.createElement("link");
    el.rel = rel;
    document.head.appendChild(el);
  }
  el.href = href;
}

export function applySeoToDocument(
  payload: SeoPayload,
  canonicalPath: string,
  imagePath = "/og-image.png"
) {
  const origin = getSiteOrigin();
  const canonical = `${origin}${canonicalPath.startsWith("/") ? canonicalPath : `/${canonicalPath}`}`;
  const imageUrl = imagePath.startsWith("http") ? imagePath : `${origin}${imagePath.startsWith("/") ? imagePath : `/${imagePath}`}`;

  document.title = payload.title;

  upsertMeta("name", "description", payload.description);

  upsertMeta("property", "og:site_name", "UseStakly");
  upsertMeta("property", "og:title", payload.title);
  upsertMeta("property", "og:description", payload.description);
  upsertMeta("property", "og:url", canonical);
  upsertMeta("property", "og:type", payload.ogType);
  upsertMeta("property", "og:image", imageUrl);
  upsertMeta("property", "og:image:width", "1200");
  upsertMeta("property", "og:image:height", "630");
  upsertMeta(
    "property",
    "og:image:alt",
    "UseStakly — open-source observatory for GitHub quality scores"
  );
  upsertMeta("property", "og:locale", "en_US");

  upsertMeta("name", "twitter:card", "summary_large_image");
  upsertMeta("name", "twitter:title", payload.title);
  upsertMeta("name", "twitter:description", payload.description);
  upsertMeta("name", "twitter:image", imageUrl);

  upsertLink("canonical", canonical);
}
