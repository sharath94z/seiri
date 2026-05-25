import { create } from "zustand";
import { prebuiltRules } from "../constants/prebuiltRules";
import { loadRules, saveRules } from "../lib/persistence";
import type { RuleDefinition } from "../types/rule";

type RulesStore = {
  rules: RuleDefinition[];
  isHydrated: boolean;
  hydrate: () => Promise<void>;
  setRules: (rules: RuleDefinition[]) => Promise<void>;
};

export const useRulesStore = create<RulesStore>((set) => ({
  rules: prebuiltRules,
  isHydrated: false,
  hydrate: async () => {
    const rules = await loadRules();
    set({ rules, isHydrated: true });
  },
  setRules: async (rules) => {
    await saveRules(rules);
    set({ rules });
  },
}));
