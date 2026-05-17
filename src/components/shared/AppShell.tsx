import { FolderCog, ListChecks, Rocket, Settings2 } from "lucide-react";
import { NavLink, Outlet } from "react-router-dom";

const navigationItems = [
  { to: "/onboarding", label: "Onboarding", icon: Rocket },
  { to: "/rules", label: "Rules", icon: FolderCog },
  { to: "/activity", label: "Activity", icon: ListChecks },
  { to: "/settings", label: "Settings", icon: Settings2 },
];

export function AppShell() {
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
              <span>Foundation in progress</span>
            </div>
            <p>
              App shell, routes, types, and storage utilities are ready for the
              first working file automation slice.
            </p>
          </div>
        </section>
      </aside>

      <main className="content-panel">
        <Outlet />
      </main>
    </div>
  );
}
