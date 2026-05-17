import type { RuleDefinition } from "../types/rule";

export const prebuiltRules: RuleDefinition[] = [
  {
    id: "move-disk-images",
    name: "Move Disk Images",
    enabled: false,
    priority: 1,
    action: "move",
    destination: "~/Downloads/Installers/",
    summary: "Moves DMG and PKG installers into a dedicated Installers folder.",
  },
  {
    id: "clean-stale-partials",
    name: "Clean Stale Partial Downloads",
    enabled: false,
    priority: 2,
    action: "trash",
    destination: "Trash",
    summary:
      "Trashes abandoned partial downloads once they are older than one day.",
  },
  {
    id: "move-pdfs",
    name: "Move PDFs",
    enabled: false,
    priority: 5,
    action: "move",
    destination: "~/Documents/PDFs/",
    summary: "Moves completed PDF downloads into a dedicated PDFs folder.",
  },
];
