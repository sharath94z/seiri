import { FolderCog, ListChecks, Rocket, Settings2 } from "lucide-react";
import { NavLink, Outlet } from "react-router-dom";

const navigationItems = [
  { to: "/onboarding", label: "Onboarding", icon: Rocket },
  { to: "/rules", label: "Rules", icon: FolderCog },
  { to: "/activity", label: "Activity", icon: ListChecks },
  { to: "/settings", label: "Settings", icon: Settings2 },
];

type AppShellProps = {
  loading?: boolean;
  statusTitle?: string;
  statusMessage?: string;
};

export function AppShell({
  loading = false,
  statusTitle,
  statusMessage,
}: AppShellProps) {
  const resolvedStatusTitle = statusTitle ?? (loading ? "Hydrating storage" : "M2 contracts active");
  const resolvedStatusMessage =
    statusMessage ??
    (loading
      ? "Seiri is loading the persisted rule, settings, activity, and retry queue contracts."
      : "Rules, settings, activity, and retry state now hydrate through the Tauri-backed persistence boundary.");

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand-block">
          <div className="brand-mark">S</div>
          <div>
            <p className="eyebrow">Seiri</p>
            <h1>File automation, calmly.</h1>
          </div>
        </div>

        <nav className="sidebar-nav" aria-label="Primary navigation">
          {navigationItems.map((item) => {
            const Icon = item.icon;
            return (
              <NavLink
                key={item.to}
                className={({ isActive }) =>
                  `nav-item${isActive ? " nav-item-active" : ""}`
                }
                to={item.to}
              >
                <Icon size={16} strokeWidth={2} />
                <span>{item.label}</span>
              </NavLink>
            );
          })}
        </nav>

        <section className="sidebar-status">
          <p className="eyebrow">Status</p>
          <div className="status-card">
            <div className="status-row">
              <span className="status-dot" aria-hidden="true" />
              <span>{resolvedStatusTitle}</span>
            </div>
            <p>{resolvedStatusMessage}</p>
          </div>
        </section>
      </aside>

      <main className="content-panel">
        {loading ? null : <Outlet />}
      </main>
    </div>
  );
}
