import { create } from "zustand";
import { prebuiltRules } from "../constants/prebuiltRules";
import { readStorage, writeStorage } from "../lib/storage";
import type { RuleDefinition } from "../types/rule";

type RulesStore = {
  rules: RuleDefinition[];
  setRules: (rules: RuleDefinition[]) => void;
};

export const useRulesStore = create<RulesStore>((set) => ({
  rules: readStorage<RuleDefinition[]>("rules", prebuiltRules),
  setRules: (rules) => {
    writeStorage("rules", rules);
    set({ rules });
  },
}));
