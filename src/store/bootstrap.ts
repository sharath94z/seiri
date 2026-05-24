import { migrateLegacyStorageIfNeeded } from "../lib/persistence";
import { useActivityStore } from "./activityStore";
import { useRetryQueueStore } from "./retryQueueStore";
import { useRulesStore } from "./rulesStore";
import { useSettingsStore } from "./settingsStore";

let bootstrapPromise: Promise<void> | null = null;

export function bootstrapStores() {
  if (bootstrapPromise) {
    return bootstrapPromise;
  }

  bootstrapPromise = (async () => {
    try {
      await migrateLegacyStorageIfNeeded();

      await Promise.all([
        useRulesStore.getState().hydrate(),
        useSettingsStore.getState().hydrate(),
        useActivityStore.getState().hydrate(),
        useRetryQueueStore.getState().hydrate(),
      ]);
    } catch (error) {
      bootstrapPromise = null;
      throw error;
    }
  })();

  return bootstrapPromise;
}
