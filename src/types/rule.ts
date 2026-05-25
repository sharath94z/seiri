export type RuleConditionLogic = "all" | "any";

export type RuleConditionType =
  | "extension"
  | "filename_contains"
  | "filename_startswith"
  | "filename_endswith"
  | "mime_type"
  | "file_size_gt"
  | "file_size_lt"
  | "date_added_older_than"
  | "source_url_contains"
  | "filename_regex";

export type RuleCondition = {
  type: RuleConditionType;
  value: string;
  negate: boolean;
};

export type RuleConditionGroup = {
  conditionLogic: "any";
  conditions: RuleCondition[];
};

export type RuleConditionNode = RuleCondition | RuleConditionGroup;

export type RuleActionDefinition =
  | { type: "move"; value: string }
  | { type: "move_with_date"; value: string }
  | { type: "rename"; value: string }
  | { type: "trash" }
  | { type: "notify_only" };

export type RuleActionType = RuleActionDefinition["type"];

export type RuleDefinition = {
  id: string;
  name: string;
  enabled: boolean;
  priority: number;
  conditionLogic: RuleConditionLogic;
  conditions: RuleConditionNode[];
  actions: RuleActionDefinition[];
  isPrebuilt: boolean;
  createdAt: string;
  updatedAt: string;
};
