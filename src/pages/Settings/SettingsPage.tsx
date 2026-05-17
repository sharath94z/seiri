import { SurfaceCard } from "../../components/shared/SurfaceCard";
import { SectionHeader } from "../../components/shared/SectionHeader";
import { useSettingsStore } from "../../store/settingsStore";

export function SettingsPage() {
  const settings = useSettingsStore((state) => state.settings);

  return (
    <div className="page-layout">
      <SectionHeader
        eyebrow="Preferences"
        title="Settings"
        description="The settings scaffold mirrors the PRD so wiring into Tauri plugins later will not require structural rework."
      />

      <SurfaceCard
        title="Stored defaults"
        description="These values currently persist locally in the frontend and will later migrate behind Tauri storage commands."
      >
        <dl className="settings-list">
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
            <dt>Watched folders</dt>
            <dd>{settings.watchedFolders.join(", ")}</dd>
          </div>
        </dl>
      </SurfaceCard>
    </div>
  );
}
