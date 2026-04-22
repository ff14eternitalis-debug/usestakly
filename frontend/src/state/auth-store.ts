import { create } from "zustand";

import type { CurrentUser } from "../lib/types";

type AuthStatus = "loading" | "authenticated" | "anonymous";

type AuthState = {
  status: AuthStatus;
  user: CurrentUser | null;
  setUser: (user: CurrentUser | null) => void;
  setLoading: () => void;
  reset: () => void;
};

export const useAuthStore = create<AuthState>((set) => ({
  status: "loading",
  user: null,
  setUser: (user) =>
    set({
      user,
      status: user ? "authenticated" : "anonymous"
    }),
  setLoading: () => set({ status: "loading" }),
  reset: () => set({ status: "anonymous", user: null })
}));
