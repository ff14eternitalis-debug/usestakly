import { create } from "zustand";

type UiState = {
  selectedLibrarySlug: string | null;
  setSelectedLibrarySlug: (slug: string | null) => void;
};

export const useUiStore = create<UiState>((set) => ({
  selectedLibrarySlug: null,
  setSelectedLibrarySlug: (slug) => set({ selectedLibrarySlug: slug })
}));
