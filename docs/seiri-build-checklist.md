# Seiri — Build Checklist
**Purpose:** persistent execution tracker for Seiri implementation  
**How to use:** update this file as work is completed so progress survives context loss  
**Status legend:** `[ ]` not started, `[-]` in progress, `[x]` complete, `[!]` blocked / needs decision  

---

## Current Snapshot

- Current milestone: `M1 Engine Feasibility`
- Current focus: `run CodeRabbit review and merge M1`
- Last updated: `2026-05-17`

---

## M0 — Foundation Hardening

- [x] Scaffold Tauri + React app in repo
- [x] Rename generated app metadata to Seiri
- [x] Replace starter demo screen with Seiri app shell
- [x] Add routed page structure for onboarding, rules, activity, settings
- [x] Add base frontend stores, types, and local persistence scaffold
- [x] Verify frontend production build
- [x] Verify Rust native build with `cargo check`
- [x] Clean generated scaffold leftovers (`README`, icons strategy, `.vscode`, etc.)
- [x] Decide and document styling system for v1 (`plain CSS now` vs `Tailwind + shadcn now`)
- [x] Add baseline `.gitignore` sanity pass
- [x] Add app-level engineering conventions doc if needed

### M0 Exit Criteria

- [x] App shell is stable and intentional
- [x] No template/demo behavior remains
- [x] Project structure is ready for feature work

### M0 Status

- [x] M0 is complete
- [x] Ready to begin M1

---

## M1 — Engine Feasibility

- [x] Review `organize` docs / repository in an isolated evaluation area
- [x] Create Seiri engine feasibility harness
- [x] Validate simple extension match
- [x] Validate ordered first-match rule behavior
- [x] Validate nested `OR` logic
- [x] Validate `NOT` / exclusion behavior
- [x] Validate simulate / preview output
- [x] Validate move action
- [x] Validate rename action
- [x] Validate date-folder move action
- [x] Validate trash action
- [x] Document gaps between Seiri rule model and `organize`
- [x] Make engine decision: `keep`, `wrap`, or `replace`
- [ ] Run CodeRabbit review on the M1 diff
- [ ] Merge the M1 branch to `main`

### M1 Exit Criteria

- [x] Feasibility results are documented
- [x] Engine decision is explicit
- [ ] M1 branch is reviewed and merged to `main`

---

## M2 — Data Model + Storage

- [ ] Finalize rule schema from PRD v1.2
- [ ] Finalize settings schema
- [ ] Finalize activity schema
- [ ] Finalize retry queue schema
- [ ] Implement storage through Tauri-backed app layer
- [ ] Define persisted file locations
- [ ] Define migration strategy for v1

### M2 Exit Criteria

- [ ] Frontend and native storage contracts are stable
- [ ] Persistence works outside browser-only local storage

---

## M3 — First Vertical Slice

- [ ] Implement manual processing flow for `~/Downloads`
- [ ] Implement stable file check for the slice
- [ ] Implement one `pdf -> move` rule
- [ ] Implement destination conflict handling for the slice
- [ ] Log successful actions
- [ ] Log failed actions
- [ ] Store undo metadata
- [ ] Implement undo for `move`
- [ ] Verify end-to-end with a real file

### M3 Exit Criteria

- [ ] Real PDF file can be processed through the app
- [ ] Activity entry is created
- [ ] Undo works

---

## M4 — Rule Management

- [ ] Build rules list UI
- [ ] Enable / disable rules
- [ ] Reorder rules by priority
- [ ] Add rule creation flow
- [ ] Add rule editing flow
- [ ] Add rule deletion flow
- [ ] Implement free tier active-rule limit behavior
- [ ] Implement rule conflict warnings
- [ ] Support editing prebuilt rule destinations

---

## M5 — Onboarding + Preview

- [ ] Build onboarding shell
- [ ] Add permissions guidance
- [ ] Add folder selection flow
- [ ] Add prebuilt rule enablement step
- [ ] Add sim preview step
- [ ] Add activation step
- [ ] Add backfill prompt entry point

---

## M6 — Watcher + Retry Queue

- [ ] Add real-time watcher for selected folders
- [ ] Add safety checks
- [ ] Add partial download handling
- [ ] Add retry queue for safety failures only
- [ ] Add pause / resume behavior
- [ ] Add manual `Run Now`
- [ ] Add backfill execution path
- [ ] Verify watcher/backfill deduplication

---

## M7 — Activity + Undo

- [ ] Build full activity feed UI
- [ ] Add status badges and filters
- [ ] Add undo affordances
- [ ] Execute undo via Tauri
- [ ] Add 24-hour undo eligibility handling
- [ ] Add user-readable undo failure messages

---

## M8 — Menubar + Settings

- [ ] Add menubar icon and popover
- [ ] Add open app action
- [ ] Add pause / resume from menubar
- [ ] Add `Run Now` from menubar
- [ ] Complete settings UI
- [ ] Wire launch-at-login behavior
- [ ] Wire notification preferences

---

## M9 — Licensing + Release Prep

- [ ] Validate Lemon Squeezy implementation reality
- [ ] Implement licence entry flow
- [ ] Implement local licence status handling
- [ ] Finalize icon set
- [ ] Prepare signing and notarization path
- [ ] Validate updater flow
- [ ] Run production build sanity pass

---

## Open Decisions

- [x] Styling system decision for v1
- [x] `organize` keep / wrap / replace decision
- [ ] Lemon Squeezy native key capability validation
- [ ] Trash restore implementation approach

---

## Notes

- This file is the source of truth for implementation progress.
- Whenever a task is completed, update the checkbox in the same change set if possible.
- If work starts but is not finished, change `[ ]` to `[-]`.
- If a task is blocked on a decision or external dependency, change `[ ]` to `[!]` and add a short note nearby.
- Every new phase or stage should start from a fresh branch.
- A phase only counts as complete after its branch is merged into `main`.
- Every commit should be reviewed with CodeRabbit before merge.
