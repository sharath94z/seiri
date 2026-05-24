export type RetryQueueEntry = {
  id: string;
  sourcePath: string;
  filename: string;
  firstQueuedAt: string;
  lastCheckedAt: string;
  lastFailureReason: string;
  attemptCount: number;
};
