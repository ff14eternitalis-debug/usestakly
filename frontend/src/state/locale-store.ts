import { create } from "zustand";
import { persist } from "zustand/middleware";

export type Locale = "en" | "fr";

type LocaleState = {
  locale: Locale;
  setLocale: (l: Locale) => void;
  toggle: () => void;
};

export const useLocaleStore = create<LocaleState>()(
  persist(
    (set, get) => ({
      locale: "en",
      setLocale: (l) => set({ locale: l }),
      toggle: () => set({ locale: get().locale === "en" ? "fr" : "en" })
    }),
    { name: "usestakly.locale" }
  )
);
