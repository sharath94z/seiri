import { create } from "zustand";
import {
  defaultRetryQueueEntries,
  loadRetryQueue,
  saveRetryQueue,
} from "../lib/persistence";
import type { RetryQueueEntry } from "../types/retryQueue";

type RetryQueueStore = {
  entries: RetryQueueEntry[];
  isHydrated: boolean;
  hydrate: () => Promise<void>;
  setEntries: (entries: RetryQueueEntry[]) => Promise<void>;
};

export const useRetryQueueStore = create<RetryQueueStore>((set) => ({
  entries: defaultRetryQueueEntries,
  isHydrated: false,
  hydrate: async () => {
    const entries = await loadRetryQueue();
    set({ entries, isHydrated: true });
  },
  setEntries: async (entries) => {
    await saveRetryQueue(entries);
    set({ entries });
  },
}));
