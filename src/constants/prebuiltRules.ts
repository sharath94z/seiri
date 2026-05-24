import type { RuleDefinition } from "../types/rule";

const seedTimestamp = "2026-05-17T23:21:42.000Z";

export const prebuiltRules: RuleDefinition[] = [
  {
    id: "move-disk-images",
    name: "Move Disk Images",
    enabled: false,
    priority: 1,
    conditionLogic: "all",
    conditions: [
      {
        conditionLogic: "any",
        conditions: [
          { type: "extension", value: "dmg", negate: false },
          { type: "extension", value: "pkg", negate: false },
        ],
      },
    ],
    actions: [{ type: "move", value: "~/Downloads/Installers/" }],
    isPrebuilt: true,
    createdAt: seedTimestamp,
    updatedAt: seedTimestamp,
  },
  {
    id: "clean-stale-partials",
    name: "Clean Stale Partial Downloads",
    enabled: false,
    priority: 2,
    conditionLogic: "all",
    conditions: [
      {
        conditionLogic: "any",
        conditions: [
          { type: "extension", value: "crdownload", negate: false },
          { type: "extension", value: "part", negate: false },
          { type: "extension", value: "download", negate: false },
        ],
      },
      { type: "date_added_older_than", value: "1 day", negate: false },
    ],
    actions: [{ type: "trash" }],
    isPrebuilt: true,
    createdAt: seedTimestamp,
    updatedAt: seedTimestamp,
  },
  {
    id: "move-pdfs",
    name: "Move PDFs",
    enabled: false,
    priority: 5,
    conditionLogic: "all",
    conditions: [{ type: "extension", value: "pdf", negate: false }],
    actions: [{ type: "move", value: "~/Documents/PDFs/" }],
    isPrebuilt: true,
    createdAt: seedTimestamp,
    updatedAt: seedTimestamp,
  },
];
