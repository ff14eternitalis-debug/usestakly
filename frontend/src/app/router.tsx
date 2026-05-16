import {
  Outlet,
  createRootRoute,
  createRoute,
  createRouter,
  redirect
} from "@tanstack/react-router";
import { Suspense, lazy } from "react";

import { AppHeader } from "../features/layout/AppHeader";
import { SiteFooter } from "../features/layout/SiteFooter";
import { DocumentMeta } from "../seo/DocumentMeta";
import { SeoOverrideProvider } from "../seo/seo-context";
import { currentReturnTo } from "../lib/return-to";
import { useAuthStore } from "../state/auth-store";

const AccountPage = lazy(() =>
  import("../routes/account").then((module) => ({ default: module.AccountPage }))
);
const DiscoverPage = lazy(() =>
  import("../routes/discover").then((module) => ({ default: module.DiscoverPage }))
);
const HowToReadPage = lazy(() =>
  import("../routes/how-to-read").then((module) => ({ default: module.HowToReadPage }))
);
const LandingPage = lazy(() =>
  import("../routes/index").then((module) => ({ default: module.LandingPage }))
);
const LegalPage = lazy(() =>
  import("../routes/legal").then((module) => ({ default: module.LegalPage }))
);
const LoginPage = lazy(() =>
  import("../routes/login").then((module) => ({ default: module.LoginPage }))
);
const McpGuidePage = lazy(() =>
  import("../routes/mcp-guide").then((module) => ({ default: module.McpGuidePage }))
);
const NotificationsPage = lazy(() =>
  import("../routes/notifications").then((module) => ({
    default: module.NotificationsPage
  }))
);
const PrivacyPage = lazy(() =>
  import("../routes/privacy").then((module) => ({ default: module.PrivacyPage }))
);
const RepoDetailPage = lazy(() =>
  import("../routes/repo-detail").then((module) => ({ default: module.RepoDetailPage }))
);
const StatusPage = lazy(() =>
  import("../routes/status").then((module) => ({ default: module.StatusPage }))
);
const WatchlistPage = lazy(() =>
  import("../routes/watchlist").then((module) => ({ default: module.WatchlistPage }))
);

const rootRoute = createRootRoute({
  component: () => (
    <SeoOverrideProvider>
      <DocumentMeta />
      <div className="min-h-screen flex flex-col">
        <AppHeader />
        <main className="flex-1">
          <Suspense
            fallback={
              <div className="shell py-10 text-[0.9rem] text-fg-muted">
                Loading...
              </div>
            }
          >
            <Outlet />
          </Suspense>
        </main>
        <SiteFooter />
      </div>
    </SeoOverrideProvider>
  )
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: LandingPage
});

const discoverRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/discover",
  component: DiscoverPage
});

const mcpGuideRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/mcp-guide",
  component: McpGuidePage
});

const howToReadRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/how-to-read",
  component: HowToReadPage
});

const repoDetailRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/repos/$id",
  component: RepoDetailPage
});

const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/login",
  component: LoginPage
});

const privacyRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/privacy",
  component: PrivacyPage
});

const legalRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/legal",
  component: LegalPage
});

const statusRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/status",
  component: StatusPage
});

function requireAuth() {
  const { status } = useAuthStore.getState();
  if (status === "anonymous") {
    throw redirect({
      to: "/login",
      search: { returnTo: currentReturnTo() }
    });
  }
}

const watchlistRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/watchlist",
  beforeLoad: requireAuth,
  component: WatchlistPage
});

const notificationsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/notifications",
  beforeLoad: requireAuth,
  component: NotificationsPage
});

const accountRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/account",
  beforeLoad: requireAuth,
  component: AccountPage
});

const routeTree = rootRoute.addChildren([
  indexRoute,
  discoverRoute,
  howToReadRoute,
  mcpGuideRoute,
  repoDetailRoute,
  loginRoute,
  privacyRoute,
  legalRoute,
  statusRoute,
  watchlistRoute,
  notificationsRoute,
  accountRoute
]);

export const router = createRouter({
  routeTree,
  defaultPreload: "intent"
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
