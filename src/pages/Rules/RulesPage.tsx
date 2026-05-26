import { useState } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { RotateCcw, Sparkles } from "lucide-react";
import { SurfaceCard } from "../../components/shared/SurfaceCard";
import { SectionHeader } from "../../components/shared/SectionHeader";
import { prebuiltRules } from "../../constants/prebuiltRules";
import {
  runM3PdfSlice,
  undoActivityEntry,
  type M3RunResult,
  type UndoActivityResult,
} from "../../lib/persistence";
import { useActivityStore } from "../../store/activityStore";
import { useRulesStore } from "../../store/rulesStore";
import type { ActivityEntry } from "../../types/activity";

function isUndoableMove(entry: ActivityEntry | null) {
  return (
    entry !== null &&
    entry.status === "success" &&
    entry.actionType === "move" &&
    entry.undoEligibleUntil !== null &&
    new Date(entry.undoEligibleUntil).getTime() > Date.now() &&
    entry.undoneAt === null
  );
}

export function RulesPage() {
  const rules = useRulesStore((state) => state.rules);
  const activityEntries = useActivityStore((state) => state.entries);
  const setActivityEntries = useActivityStore((state) => state.setEntries);

  const [isRunning, setIsRunning] = useState(false);
  const [isUndoing, setIsUndoing] = useState(false);
  const [latestResult, setLatestResult] = useState<M3RunResult | UndoActivityResult | null>(
    null,
  );
  const [statusMessage, setStatusMessage] = useState(
    "Run the temporary M3 PDF slice to process the first eligible PDF in ~/Downloads.",
  );

  const persistedUndoEntry = activityEntries.find(isUndoableMove) ?? null;
  const latestActivityEntry = latestResult?.activityEntry ?? persistedUndoEntry;
  const canUndo = persistedUndoEntry !== null;

  async function handleRunSlice() {
    setIsRunning(true);

    try {
      const result = await runM3PdfSlice();
      await setActivityEntries(result.entries);
      setLatestResult(result);
      setStatusMessage(result.message);
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Unknown PDF slice error";
      setStatusMessage(message);
      setLatestResult({
        status: "failed",
        message,
        activityEntry: null,
        entries: useActivityStore.getState().entries,
      });
    } finally {
      setIsRunning(false);
    }
  }

  async function handleUndo() {
    if (!persistedUndoEntry) {
      return;
    }

    setIsUndoing(true);

    try {
      const result = await undoActivityEntry(persistedUndoEntry.id);
      await setActivityEntries(result.entries);
      setLatestResult(result);
      setStatusMessage(result.message);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Unknown undo error";
      setStatusMessage(message);
    } finally {
      setIsUndoing(false);
    }
  }

  return (
    <div className="page-layout">
      <SectionHeader
        eyebrow="Automation"
        title="Rules"
        description="This page holds the temporary M3 trigger so we can prove the first real file-action slice without adding the full watcher flow yet."
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

      <SurfaceCard
        title="M3 PDF slice"
        description="Manual, one-file vertical slice that exercises the real forward action path and the undo contract."
        actions={
          <div className="surface-card-actions">
            <button
              className="primary-button"
              type="button"
              onClick={() => void handleRunSlice()}
              disabled={!isTauri() || isRunning || isUndoing}
            >
              <Sparkles size={16} strokeWidth={2} />
              {isRunning ? "Running" : "Run PDF Slice"}
            </button>
            {canUndo ? (
              <button
                className="secondary-button"
                type="button"
                onClick={() => void handleUndo()}
                disabled={isUndoing || isRunning}
              >
                <RotateCcw size={16} strokeWidth={2} />
                {isUndoing ? "Undoing" : "Undo"}
              </button>
            ) : null}
          </div>
        }
      >
        <div className="run-summary">
          <p className="run-summary-label">Latest result</p>
          <p className="run-summary-message">{statusMessage}</p>

          {latestActivityEntry ? (
            <dl className="run-summary-grid">
              <div>
                <dt>File</dt>
                <dd>{latestActivityEntry.filename}</dd>
              </div>
              <div>
                <dt>Status</dt>
                <dd>{latestActivityEntry.status}</dd>
              </div>
              <div>
                <dt>Rule</dt>
                <dd>{latestActivityEntry.ruleName ?? "M3 PDF slice"}</dd>
              </div>
              <div>
                <dt>Location</dt>
                <dd>{latestActivityEntry.finalPath ?? latestActivityEntry.sourcePath}</dd>
              </div>
            </dl>
          ) : (
            <p className="run-summary-empty">
              No run yet. The button above processes the first eligible PDF found in
              ~/Downloads.
            </p>
          )}
        </div>
      </SurfaceCard>
    </div>
  );
}
