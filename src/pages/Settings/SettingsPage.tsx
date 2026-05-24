import { SurfaceCard } from "../../components/shared/SurfaceCard";
import { SectionHeader } from "../../components/shared/SectionHeader";
import { readStorageFileNames } from "../../lib/persistence";
import { useSettingsStore } from "../../store/settingsStore";

export function SettingsPage() {
  const settings = useSettingsStore((state) => state.settings);
  const fileNames = readStorageFileNames();

  return (
    <div className="page-layout">
      <SectionHeader
        eyebrow="Preferences"
        title="Settings"
        description="The settings scaffold mirrors the PRD so wiring into Tauri plugins later will not require structural rework."
      />

      <SurfaceCard
        title="Stored defaults"
        description="These values now target the persisted v1 settings contract and the app-support JSON file layout from the PRD."
      >
        <dl className="settings-list">
          <div className="settings-row">
            <dt>Onboarding completed</dt>
            <dd>{settings.onboardingCompleted ? "Yes" : "No"}</dd>
          </div>
          <div className="settings-row">
            <dt>Launch at login</dt>
            <dd>{settings.launchAtLogin ? "Enabled" : "Disabled"}</dd>
          </div>
          <div className="settings-row">
            <dt>Daily summary</dt>
            <dd>{settings.dailySummaryEnabled ? "Enabled" : "Disabled"}</dd>
          </div>
          <div className="settings-row">
            <dt>Conflict behavior</dt>
            <dd>{settings.conflictBehavior}</dd>
          </div>
          <div className="settings-row">
            <dt>File stability wait</dt>
            <dd>{settings.fileStabilityWaitSeconds}s</dd>
          </div>
          <div className="settings-row">
            <dt>Activity retention</dt>
            <dd>{settings.activityRetentionDays} days</dd>
          </div>
          <div className="settings-row">
            <dt>Watched folders</dt>
            <dd>{settings.watchedFolders.join(", ")}</dd>
          </div>
          <div className="settings-row">
            <dt>Licence tier</dt>
            <dd>{settings.license.tier}</dd>
          </div>
          <div className="settings-row">
            <dt>Storage files</dt>
            <dd>
              {fileNames.rules}, {fileNames.settings}, {fileNames.activity},{" "}
              {fileNames.retryQueue}
            </dd>
          </div>
        </dl>
      </SurfaceCard>
    </div>
  );
}
