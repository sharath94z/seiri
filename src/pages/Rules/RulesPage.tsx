import { SurfaceCard } from "../../components/shared/SurfaceCard";
import { SectionHeader } from "../../components/shared/SectionHeader";
import { prebuiltRules } from "../../constants/prebuiltRules";
import { useRulesStore } from "../../store/rulesStore";

export function RulesPage() {
  const rules = useRulesStore((state) => state.rules);

  return (
    <div className="page-layout">
      <SectionHeader
        eyebrow="Automation"
        title="Rules"
        description="This is the control room for Seiri. We start with pre-built rules and a clean structure for custom rule editing."
      />

      <div className="two-column-grid">
        <SurfaceCard
          title="Current rules"
          description="Local state is wired up so rule management can grow without reshaping the app shell."
        >
          <ul className="stack-list">
            {rules.map((rule) => (
              <li key={rule.id} className="list-row">
                <div>
                  <p>{rule.name}</p>
                  <span>Priority {rule.priority}</span>
                </div>
                <strong>{rule.enabled ? "Enabled" : "Disabled"}</strong>
              </li>
            ))}
          </ul>
        </SurfaceCard>

        <SurfaceCard
          title="Pre-built seeds"
          description="These examples come directly from the product spec and give us a stable base for future builder work."
        >
          <ul className="stack-list">
            {prebuiltRules.map((rule) => (
              <li key={rule.id} className="list-row">
                <div>
                  <p>{rule.name}</p>
                  <span>{rule.summary}</span>
                </div>
                <code>{rule.destination}</code>
              </li>
            ))}
          </ul>
        </SurfaceCard>
      </div>
    </div>
  );
}
