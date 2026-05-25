import { invoke, isTauri } from "@tauri-apps/api/core";
import { prebuiltRules } from "../constants/prebuiltRules";
import type { ActivityEntry } from "../types/activity";
import type { RetryQueueEntry } from "../types/retryQueue";
import type { RuleActionDefinition, RuleDefinition } from "../types/rule";
import type { SettingsState } from "../types/settings";

const storagePrefix = "seiri";

const legacyRulesKey = `${storagePrefix}:rules`;
const legacySettingsKey = `${storagePrefix}:settings`;
const legacyActivityKey = `${storagePrefix}:activity`;

const storageFileNames = {
  rules: "rules.json",
  settings: "settings.json",
  activity: "activity.json",
  retryQueue: "retry-queue.json",
} as const;

export type StorageStatus = {
  rulesExists: boolean;
  settingsExists: boolean;
  activityExists: boolean;
  retryQueueExists: boolean;
};

export const defaultSettings: SettingsState = {
  onboardingCompleted: false,
  launchAtLogin: true,
  dailySummaryEnabled: true,
  dailySummaryStartedAt: null,
  dailySummaryLastSentAt: null,
  watchedFolders: ["~/Downloads"],
  fileStabilityWaitSeconds: 5,
  conflictBehavior: "rename",
  activityRetentionDays: 30,
  license: {
    tier: "free",
    key: null,
    validatedAt: null,
    expiresAt: null,
  },
};

export const defaultActivityEntries: ActivityEntry[] = [
  {
    id: "activity-seed-1",
    timestamp: "2026-05-17T15:30:00.000Z",
    status: "success",
    filename: "Invoice-April.pdf",
    sourcePath: "~/Downloads/Invoice-April.pdf",
    destinationPath: "~/Documents/Finance",
    finalPath: "~/Documents/Finance/Invoice-April.pdf",
    ruleId: "move-invoices-receipts",
    ruleName: "Move Invoices and Receipts",
    actionType: "move",
    message: "Moved successfully",
    undoEligibleUntil: "2026-05-18T15:30:00.000Z",
    undoneAt: null,
    undoSourceEntryId: null,
  },
];

export const defaultRetryQueueEntries: RetryQueueEntry[] = [];

export async function getStorageStatus(): Promise<StorageStatus> {
  if (!isTauri()) {
    return {
      rulesExists: localStorageKeyExists(legacyRulesKey),
      settingsExists: localStorageKeyExists(legacySettingsKey),
      activityExists: localStorageKeyExists(legacyActivityKey),
      retryQueueExists: false,
    };
  }

  return invoke<StorageStatus>("storage_status");
}

export async function loadRules(): Promise<RuleDefinition[]> {
  if (!isTauri()) {
    return readLocalStorage(legacyRulesKey, prebuiltRules);
  }

  return invoke<RuleDefinition[]>("load_rules");
}

export async function saveRules(rules: RuleDefinition[]) {
  if (!isTauri()) {
    writeLocalStorage(legacyRulesKey, rules);
    return;
  }

  await invoke("save_rules", { rules });
}

export async function loadSettings(): Promise<SettingsState> {
  if (!isTauri()) {
    return readLocalStorage(legacySettingsKey, defaultSettings);
  }

  return invoke<SettingsState>("load_settings");
}

export async function saveSettings(settings: SettingsState) {
  if (!isTauri()) {
    writeLocalStorage(legacySettingsKey, settings);
    return;
  }

  await invoke("save_settings", { settings });
}

export async function loadActivity(): Promise<ActivityEntry[]> {
  if (!isTauri()) {
    return readLocalStorage(legacyActivityKey, defaultActivityEntries);
  }

  return invoke<ActivityEntry[]>("load_activity");
}

export async function saveActivity(entries: ActivityEntry[]) {
  if (!isTauri()) {
    writeLocalStorage(legacyActivityKey, entries);
    return;
  }

  await invoke("save_activity", { entries });
}

export async function loadRetryQueue(): Promise<RetryQueueEntry[]> {
  if (!isTauri()) {
    return defaultRetryQueueEntries;
  }

  return invoke<RetryQueueEntry[]>("load_retry_queue");
}

export async function saveRetryQueue(entries: RetryQueueEntry[]) {
  if (!isTauri()) {
    return;
  }

  await invoke("save_retry_queue", { entries });
}

export async function migrateLegacyStorageIfNeeded() {
  const status = await getStorageStatus();

  if (!status.rulesExists) {
    const legacyRules = readLegacyRules();
    if (legacyRules) {
      await saveRules(legacyRules);
    }
  }

  if (!status.settingsExists) {
    const legacySettings = readLegacySettings();
    if (legacySettings) {
      await saveSettings(legacySettings);
    }
  }

  if (!status.activityExists) {
    const legacyActivity = readLegacyActivity();
    if (legacyActivity) {
      await saveActivity(legacyActivity);
    }
  }

  if (!status.retryQueueExists && isTauri()) {
    await saveRetryQueue(defaultRetryQueueEntries);
  }
}

export function readStorageFileNames() {
  return storageFileNames;
}

function readLegacyRules() {
  const legacy = readLocalStorage<LegacyRuleDefinition[] | null>(legacyRulesKey, null);
  if (!legacy) {
    return null;
  }

  return legacy.map<RuleDefinition>((rule) => {
    const matchingSeed = prebuiltRules.find((seed) => seed.id === rule.id);

    if (matchingSeed) {
      return {
        ...matchingSeed,
        name: rule.name ?? matchingSeed.name,
        enabled: rule.enabled ?? matchingSeed.enabled,
        priority: rule.priority ?? matchingSeed.priority,
        updatedAt: new Date().toISOString(),
      };
    }

    return {
      id: rule.id,
      name: rule.name,
      enabled: rule.enabled,
      priority: rule.priority,
      conditionLogic: "all",
      conditions: [{ type: "extension", value: "*", negate: false }],
      actions: mapLegacyAction(rule.action, rule.destination),
      isPrebuilt: false,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
  });
}

function readLegacySettings() {
  const legacy = readLocalStorage<LegacySettingsState | null>(legacySettingsKey, null);
  if (!legacy) {
    return null;
  }

  return {
    ...defaultSettings,
    launchAtLogin: legacy.launchAtLogin,
    dailySummaryEnabled: legacy.dailySummaryEnabled,
    watchedFolders: legacy.watchedFolders,
    conflictBehavior: legacy.conflictBehavior,
  };
}

function readLegacyActivity() {
  const legacy = readLocalStorage<LegacyActivityEntry[] | null>(legacyActivityKey, null);
  if (!legacy) {
    return null;
  }

  return legacy.map((entry) => ({
    id: entry.id,
    timestamp: entry.timestamp,
    status: entry.status,
    filename: entry.filename,
    sourcePath: resolveLegacySourcePath(entry),
    destinationPath: null,
    finalPath: null,
    ruleId: null,
    ruleName: null,
    actionType: null,
    message: null,
    undoEligibleUntil: null,
    undoneAt: null,
    undoSourceEntryId: null,
  }));
}

function resolveLegacySourcePath(entry: LegacyActivityEntry) {
  if (entry.sourcePath) {
    return entry.sourcePath;
  }

  if (entry.originalPath) {
    return entry.originalPath;
  }

  if (entry.sourceFolder) {
    return `${entry.sourceFolder.replace(/\/$/, "")}/${entry.filename}`;
  }

  return `~/Downloads/${entry.filename}`;
}

function mapLegacyAction(
  action: LegacyRuleDefinition["action"],
  destination: string,
): RuleActionDefinition[] {
  switch (action) {
    case "move":
      return [{ type: "move", value: destination }];
    case "move_with_date":
      return [{ type: "move_with_date", value: destination }];
    case "rename":
      return [{ type: "rename", value: destination }];
    case "trash":
      return [{ type: "trash" }];
    default:
      return [{ type: "notify_only" }];
  }
}

function localStorageKeyExists(key: string) {
  if (typeof window === "undefined") {
    return false;
  }

  return window.localStorage.getItem(key) !== null;
}

function readLocalStorage<T>(key: string, fallback: T): T {
  if (typeof window === "undefined") {
    return fallback;
  }

  try {
    const rawValue = window.localStorage.getItem(key);
    return rawValue ? (JSON.parse(rawValue) as T) : fallback;
  } catch {
    return fallback;
  }
}

function writeLocalStorage<T>(key: string, value: T) {
  if (typeof window === "undefined") {
    return;
  }

  try {
    window.localStorage.setItem(key, JSON.stringify(value));
  } catch (error) {
    console.warn(`Failed to persist localStorage key "${key}"`, error);
  }
}

type LegacyRuleDefinition = {
  id: string;
  name: string;
  enabled: boolean;
  priority: number;
  action: "move" | "move_with_date" | "rename" | "trash";
  destination: string;
  summary: string;
};

type LegacySettingsState = {
  launchAtLogin: boolean;
  dailySummaryEnabled: boolean;
  watchedFolders: string[];
  conflictBehavior: "rename" | "skip";
};

type LegacyActivityEntry = {
  id: string;
  filename: string;
  status: ActivityEntry["status"];
  timestamp: string;
  sourcePath?: string;
  originalPath?: string;
  sourceFolder?: string;
};
