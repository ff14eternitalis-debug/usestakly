import {
  Outlet,
  createRootRoute,
  createRoute,
  createRouter,
  redirect
} from "@tanstack/react-router";

import { AppHeader } from "../features/layout/AppHeader";
import { SiteFooter } from "../features/layout/SiteFooter";
import { AccountPage } from "../routes/account";
import { DiscoverPage } from "../routes/discover";
import { HowToReadPage } from "../routes/how-to-read";
import { LandingPage } from "../routes/index";
import { LoginPage } from "../routes/login";
import { McpGuidePage } from "../routes/mcp-guide";
import { NotificationsPage } from "../routes/notifications";
import { RepoDetailPage } from "../routes/repo-detail";
import { WatchlistPage } from "../routes/watchlist";
import { currentReturnTo } from "../lib/return-to";
import { useAuthStore } from "../state/auth-store";

const rootRoute = createRootRoute({
  component: () => (
    <div className="min-h-screen flex flex-col">
      <AppHeader />
      <main className="flex-1">
        <Outlet />
      </main>
      <SiteFooter />
    </div>
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
