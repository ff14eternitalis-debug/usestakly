import { useLocation } from "@tanstack/react-router";
import { useEffect } from "react";

import { routeSeo } from "./route-meta";
import { useSeoOverride } from "./seo-context";
import { applySeoToDocument } from "./sync-meta";

export function DocumentMeta() {
  const location = useLocation();
  const { override } = useSeoOverride();

  useEffect(() => {
    const base = routeSeo(location.pathname);
    const merged = override
      ? {
          title: override.title,
          description: override.description,
          ogType: override.ogType ?? base.ogType
        }
      : base;
    applySeoToDocument(merged, location.pathname);
  }, [location.pathname, override]);

  return null;
}
