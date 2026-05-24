export type ConflictBehavior = "rename" | "skip";

export type LicenseTier = "free" | "monthly" | "lifetime";

export type LicenseState = {
  tier: LicenseTier;
  key: string | null;
  validatedAt: string | null;
  expiresAt: string | null;
};

export type SettingsState = {
  onboardingCompleted: boolean;
  launchAtLogin: boolean;
  dailySummaryEnabled: boolean;
  dailySummaryStartedAt: string | null;
  dailySummaryLastSentAt: string | null;
  watchedFolders: string[];
  fileStabilityWaitSeconds: number;
  conflictBehavior: ConflictBehavior;
  activityRetentionDays: number;
  license: LicenseState;
};
