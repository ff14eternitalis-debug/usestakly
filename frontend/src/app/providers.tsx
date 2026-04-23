import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider } from "@tanstack/react-router";

import { useHydrateAuth } from "../features/auth/hooks";
import { router } from "./router";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
      staleTime: 30_000
    }
  }
});

function HydrateAuth() {
  const hydrated = useHydrateAuth();

  if (hydrated) {
    return <RouterProvider router={router} />;
  }

  return (
    <div className="grid min-h-screen place-items-center bg-[color:var(--color-bg)] px-6 text-center">
      <div className="grid gap-3">
        <span className="kicker">AUTH</span>
        <p className="mono text-[0.82rem] uppercase tracking-[0.14em] text-fg-dim">
          Checking session…
        </p>
      </div>
    </div>
  );
}

export function AppProviders() {
  return (
    <QueryClientProvider client={queryClient}>
      <HydrateAuth />
    </QueryClientProvider>
  );
}
