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
          description="The persisted rule contract now matches the v1 schema shape, including condition trees and action arrays."
        >
          <ul className="stack-list">
            {rules.map((rule) => (
              <li key={rule.id} className="list-row">
                <div>
                  <p>{rule.name}</p>
                  <span>
                    Priority {rule.priority} · {rule.conditions.length} condition
                    {rule.conditions.length === 1 ? "" : "s"} · {rule.actions.length} action
                    {rule.actions.length === 1 ? "" : "s"}
                  </span>
                </div>
                <strong>{rule.enabled ? "Enabled" : "Disabled"}</strong>
              </li>
            ))}
          </ul>
        </SurfaceCard>

        <SurfaceCard
          title="Pre-built seeds"
          description="Seeded rules are stored as real rule definitions rather than placeholder summaries."
        >
          <ul className="stack-list">
            {prebuiltRules.map((rule) => (
              <li key={rule.id} className="list-row">
                <div>
                  <p>{rule.name}</p>
                  <span>
                    {rule.actions?.[0]?.type ?? "no action"} · top-level{" "}
                    {rule.conditionLogic}
                  </span>
                </div>
                <code>
                  {rule.actions?.[0] && "value" in rule.actions[0]
                    ? rule.actions[0].value
                    : "Trash"}
                </code>
              </li>
            ))}
          </ul>
        </SurfaceCard>
      </div>
    </div>
  );
}
