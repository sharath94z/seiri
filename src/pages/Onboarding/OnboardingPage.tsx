import { SurfaceCard } from "../../components/shared/SurfaceCard";
import { SectionHeader } from "../../components/shared/SectionHeader";

const onboardingSteps = [
  "Grant Full Disk Access",
  "Choose folders to watch",
  "Enable pre-built rules",
  "Preview with sim mode",
  "Activate Seiri",
];

export function OnboardingPage() {
  return (
    <div className="page-layout">
      <SectionHeader
        eyebrow="First Run"
        title="Onboarding flow"
        description="This screen anchors the six-step setup path defined in the PRD, without forcing the implementation before the shell is stable."
      />

      <SurfaceCard
        title="Onboarding outline"
        description="The full flow will plug into permissions, folder picking, preview, and activation in later milestones."
      >
        <ol className="numbered-list">
          {onboardingSteps.map((step) => (
            <li key={step}>{step}</li>
          ))}
        </ol>
      </SurfaceCard>
    </div>
  );
}
