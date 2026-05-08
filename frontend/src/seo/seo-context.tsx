import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactNode
} from "react";

export type SeoOverride = {
  title: string;
  description: string;
  ogType?: "website" | "article";
};

type Ctx = {
  override: SeoOverride | null;
  setOverride: (v: SeoOverride | null) => void;
};

const SeoOverrideContext = createContext<Ctx | null>(null);

export function SeoOverrideProvider({ children }: { children: ReactNode }) {
  const [override, setOverrideState] = useState<SeoOverride | null>(null);
  const setOverride = useCallback((v: SeoOverride | null) => {
    setOverrideState(v);
  }, []);
  const value = useMemo(() => ({ override, setOverride }), [override, setOverride]);
  return (
    <SeoOverrideContext.Provider value={value}>{children}</SeoOverrideContext.Provider>
  );
}

export function useSeoOverride(): Ctx {
  const ctx = useContext(SeoOverrideContext);
  if (!ctx) {
    throw new Error("useSeoOverride must be used within SeoOverrideProvider");
  }
  return ctx;
}
