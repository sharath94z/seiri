export type RuleAction = "move" | "move_with_date" | "rename" | "trash";

export type RuleDefinition = {
  id: string;
  name: string;
  enabled: boolean;
  priority: number;
  action: RuleAction;
  destination: string;
  summary: string;
};
