import { create } from "zustand";
import { readStorage, writeStorage } from "../lib/storage";
import type { SettingsState } from "../types/settings";

type SettingsStore = {
  settings: SettingsState;
  setSettings: (settings: SettingsState) => void;
};

const defaultSettings: SettingsState = {
  launchAtLogin: true,
  dailySummaryEnabled: true,
  watchedFolders: ["~/Downloads"],
  conflictBehavior: "rename",
};

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: readStorage<SettingsState>("settings", defaultSettings),
  setSettings: (settings) => {
    writeStorage("settings", settings);
    set({ settings });
  },
}));
