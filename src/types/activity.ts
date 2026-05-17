export type ActivityStatus = "success" | "skipped" | "failed" | "undone";

export type ActivityEntry = {
  id: string;
  filename: string;
  status: ActivityStatus;
  timestamp: string;
};
