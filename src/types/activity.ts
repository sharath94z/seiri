import type { RuleActionType } from "./rule";

export type ActivityStatus = "success" | "skipped" | "failed" | "undone";

export type ActivityEntry = {
  id: string;
  timestamp: string;
  status: ActivityStatus;
  filename: string;
  sourcePath: string;
  destinationPath: string | null;
  finalPath: string | null;
  ruleId: string | null;
  ruleName: string | null;
  actionType: RuleActionType | null;
  message: string | null;
  undoEligibleUntil: string | null;
  undoneAt: string | null;
  undoSourceEntryId: string | null;
};
