import { create } from "zustand";

import type { CurrentUser } from "../lib/types";

type AuthStatus = "loading" | "authenticated" | "anonymous";

type AuthState = {
  status: AuthStatus;
  hydrated: boolean;
  user: CurrentUser | null;
  setUser: (user: CurrentUser | null) => void;
  setLoading: () => void;
  reset: () => void;
};

export const useAuthStore = create<AuthState>((set) => ({
  status: "loading",
  hydrated: false,
  user: null,
  setUser: (user) =>
    set({
      user,
      status: user ? "authenticated" : "anonymous",
      hydrated: true
    }),
  setLoading: () => set({ status: "loading", hydrated: false }),
  reset: () => set({ status: "anonymous", hydrated: true, user: null })
}));
