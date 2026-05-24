import { create } from "zustand";
import { defaultSettings, loadSettings, saveSettings } from "../lib/persistence";
import type { SettingsState } from "../types/settings";

type SettingsStore = {
  settings: SettingsState;
  isHydrated: boolean;
  hydrate: () => Promise<void>;
  setSettings: (settings: SettingsState) => Promise<void>;
};

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: defaultSettings,
  isHydrated: false,
  hydrate: async () => {
    const settings = await loadSettings();
    set({ settings, isHydrated: true });
  },
  setSettings: async (settings) => {
    await saveSettings(settings);
    set({ settings });
  },
}));
