import { SurfaceCard } from "../../components/shared/SurfaceCard";
import { SectionHeader } from "../../components/shared/SectionHeader";
import { useActivityStore } from "../../store/activityStore";

export function ActivityPage() {
  const activity = useActivityStore((state) => state.entries);

  return (
    <div className="page-layout">
      <SectionHeader
        eyebrow="Transparency"
        title="Activity feed"
        description="Every successful action and safety failure will show up here so users can trust what Seiri is doing."
      />

      <SurfaceCard
        title="Feed scaffold"
        description="The store and UI are ready for real activity events once the watcher and engine land."
      >
        <ul className="stack-list">
          {activity.map((entry) => (
            <li key={entry.id} className="list-row">
              <div>
                <p>{entry.filename}</p>
                <span>{entry.status}</span>
              </div>
              <code>{entry.timestamp}</code>
            </li>
          ))}
        </ul>
      </SurfaceCard>
    </div>
  );
}
