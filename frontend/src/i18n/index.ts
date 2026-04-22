import { useLocaleStore } from "../state/locale-store";
import { en, type Dict } from "./en";
import { fr } from "./fr";

const DICTS: Record<"en" | "fr", Dict> = { en, fr };

export function useT(): Dict {
  const locale = useLocaleStore((s) => s.locale);
  return DICTS[locale];
}

export type { Dict } from "./en";
