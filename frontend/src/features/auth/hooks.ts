import { useEffect } from "react";

import { apiGet, apiPost, ApiError } from "../../lib/api-client";
import type { CurrentUser } from "../../lib/types";
import { useAuthStore } from "../../state/auth-store";

export function useHydrateAuth(): boolean {
  const setUser = useAuthStore((s) => s.setUser);
  const hydrated = useAuthStore((s) => s.hydrated);

  useEffect(() => {
    const controller = new AbortController();
    void (async () => {
      try {
        const user = await apiGet<CurrentUser>("/api/me", controller.signal);
        setUser(user);
      } catch (err) {
        if (err instanceof ApiError) {
          setUser(null);
          return;
        }
        if ((err as { name?: string })?.name !== "AbortError") {
          setUser(null);
        }
      }
    })();
    return () => controller.abort();
  }, [setUser]);

  return hydrated;
}

export async function logout(): Promise<void> {
  try {
    await apiPost("/api/auth/logout");
  } finally {
    useAuthStore.getState().reset();
  }
}
