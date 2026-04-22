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
  useHydrateAuth();
  return null;
}

export function AppProviders() {
  return (
    <QueryClientProvider client={queryClient}>
      <HydrateAuth />
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
}
