import { create } from "zustand";
import {
  defaultActivityEntries,
  loadActivity,
  saveActivity,
} from "../lib/persistence";
import type { ActivityEntry } from "../types/activity";

type ActivityStore = {
  entries: ActivityEntry[];
  isHydrated: boolean;
  hydrate: () => Promise<void>;
  addEntry: (entry: ActivityEntry) => Promise<void>;
  setEntries: (entries: ActivityEntry[]) => Promise<void>;
};

export const useActivityStore = create<ActivityStore>((set) => ({
  entries: defaultActivityEntries,
  isHydrated: false,
  hydrate: async () => {
    const entries = await loadActivity();
    set({ entries, isHydrated: true });
  },
  addEntry: async (entry) => {
    const entries = [entry, ...useActivityStore.getState().entries].slice(0, 1000);
    await saveActivity(entries);
    set({ entries });
  },
  setEntries: async (entries) => {
    await saveActivity(entries);
    set({ entries });
  },
}));
