export type ConflictBehavior = "rename" | "skip";

export type SettingsState = {
  launchAtLogin: boolean;
  dailySummaryEnabled: boolean;
  watchedFolders: string[];
  conflictBehavior: ConflictBehavior;
};
