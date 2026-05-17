import { create } from "zustand";
import { readStorage, writeStorage } from "../lib/storage";
import type { ActivityEntry } from "../types/activity";

type ActivityStore = {
  entries: ActivityEntry[];
  addEntry: (entry: ActivityEntry) => void;
};

const defaultEntries: ActivityEntry[] = [
  {
    id: "activity-seed-1",
    filename: "Invoice-April.pdf",
    status: "success",
    timestamp: "2026-05-17T15:30:00.000Z",
  },
];

export const useActivityStore = create<ActivityStore>((set) => ({
  entries: readStorage<ActivityEntry[]>("activity", defaultEntries),
  addEntry: (entry) =>
    set((state) => {
      const entries = [entry, ...state.entries].slice(0, 1000);
      writeStorage("activity", entries);
      return { entries };
    }),
}));
