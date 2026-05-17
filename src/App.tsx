import { Navigate, Route, Routes } from "react-router-dom";
import { AppShell } from "./components/shared/AppShell";
import { ActivityPage } from "./pages/Activity/ActivityPage";
import { OnboardingPage } from "./pages/Onboarding/OnboardingPage";
import { RulesPage } from "./pages/Rules/RulesPage";
import { SettingsPage } from "./pages/Settings/SettingsPage";

function App() {
  return (
    <Routes>
      <Route element={<AppShell />}>
        <Route index element={<Navigate replace to="/rules" />} />
        <Route path="/rules" element={<RulesPage />} />
        <Route path="/activity" element={<ActivityPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/onboarding" element={<OnboardingPage />} />
      </Route>
    </Routes>
  );
}

export default App;
