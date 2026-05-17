# Seiri — Implementation Plan
**Version:** 2.0  
**Status:** Active  
**Last Updated:** May 2026  
**Author:** Sharath Aradhyamath  
**Document Type:** Implementation Plan (What we build, in what order, and why)  

> This plan turns the PRD into an execution sequence.
> The goal is visibility, intentionality, and clean decision gates.
> We do not start a milestone until the previous one is verified and accepted.

---

## Build Principles

These rules apply to every milestone:

- Build vertical slices, not disconnected pieces.
- Prefer proof before polish when a technical dependency is still uncertain.
- Keep the user trust model intact at every stage.
- Do not add a feature only because Hazel has it.
- Every milestone must end in a runnable, testable artifact.
- Every risky dependency gets an explicit validation step before we rely on it.

---

## Current Status

The repo now has:

- Tauri + React project scaffold
- Seiri app shell and routes
- Base frontend structure for pages, types, and stores
- Local frontend persistence scaffold
- Verified frontend build
- Verified Rust `cargo check`

This means **Foundation is partially complete**, but the product is not yet functional.

---

## Definition of Done

A milestone is complete only when all of the following are true:

- The feature works end-to-end in the app, not just in isolated code.
- The implementation matches the PRD for that milestone.
- Main edge cases for that milestone are handled.
- The UI is coherent and not obviously placeholder-quality.
- `npm run build` passes.
- `cargo check` passes.
- Manual verification steps for that milestone are documented and completed.
- Any open risk introduced by the milestone is called out explicitly.

---

## Milestone Overview

| Milestone | Name | Purpose | Exit Artifact |
|---|---|---|---|
| M0 | Foundation Hardening | Stabilize scaffold and project conventions | Reliable app shell and engineering baseline |
| M1 | Engine Feasibility | Prove `organize` can support Seiri's core rule model | Technical decision: keep sidecar or replace |
| M2 | Data Model + Storage | Freeze rule, settings, and activity schemas | Stable local persistence contract |
| M3 | First Vertical Slice | One real automation flow end-to-end | Working `PDF -> move -> log -> undo` slice |
| M4 | Rule Management | Let users create and manage real rules | Functional rules screen and editor |
| M5 | Onboarding + Preview | First-run flow with sim trust model | Usable onboarding flow |
| M6 | Watcher + Retry Queue | Real-time background automation | Stable automatic processing |
| M7 | Activity + Undo | Transparency and reversibility | Trust and recovery layer |
| M8 | Menubar + Settings | Native utility experience | Everyday usable desktop app |
| M9 | Licensing + Release Prep | Monetization and shipping readiness | Launch candidate |

---

## M0 — Foundation Hardening

### Goal

Take the current scaffold and make it stable enough that future milestones do not reshape core structure.

### Scope

- Clean project naming and generated leftovers
- Finalize folder structure
- Add shared utilities and naming conventions
- Decide whether we stay with plain CSS for now or move to Tailwind + shadcn immediately
- Add base Tauri plugin wiring only where needed
- Add `.gitignore`, app metadata sanity, and baseline scripts

### Intent

This milestone is about reducing chaos. It is not feature work.

### Exit Criteria

- App shell opens and routes correctly
- Styling direction is chosen and documented
- Project structure matches how we intend to build
- No demo/template behavior remains

---

## M1 — Engine Feasibility

### Goal

Prove whether `organize` can support Seiri's v1 engine requirements without excessive wrapper logic.

### Scope

- Build a standalone feasibility harness outside the main UX
- Validate:
  - simple extension match
  - ordered rule evaluation
  - nested `OR`
  - `NOT` / exclusion behavior
  - simulate / preview output
  - move
  - rename
  - move with date folders
  - trash
- Check how much translation is needed between Seiri rule JSON and `organize`

### Decision Gate

At the end of this milestone we decide one of:

- Keep `organize` as-is
- Keep `organize` with a Seiri translation layer
- Stop and replace the engine strategy

### Exit Criteria

- We have a written compatibility result
- We know which rule features are native vs wrapped
- We have a clear engine decision before watcher work begins

### M1 Result

Seiri will use `organize` with a thin translation layer. The engine is a good fit for file operations and simulation, but Seiri still needs wrapper logic for richer boolean grouping, source URL provenance, and strict Seiri-side rule semantics.

---

## M2 — Data Model + Storage

### Goal

Freeze the contracts the app will persist and rely on.

### Scope

- Finalize rule schema from PRD v1.2
- Finalize settings schema
- Finalize activity entry schema
- Finalize retry queue schema
- Move from frontend-only local storage scaffolding to real app storage through Tauri
- Decide file locations and write strategy

### Intent

This is the contract milestone. Watcher, rule builder, undo, and onboarding all depend on this.

### Exit Criteria

- Schemas are implemented in TypeScript and Rust-facing command contracts
- Read/write works through the app, not only browser `localStorage`
- Seed data and migrations strategy for v1 is documented

---

## M3 — First Vertical Slice

### Goal

Ship one real, narrow automation flow end-to-end before building the full feature surface.

### Scope

Implement exactly this slice:

- Watched folder: `~/Downloads`
- One prebuilt rule: `extension = pdf`
- Safety: stable file check
- Action: move to `~/Documents/PDFs`
- Log success or failure
- Add undo metadata
- Manual trigger only, no background watcher yet

### Intent

This is the first proof that Seiri is a real product, not just screens and schemas.

### Exit Criteria

- A real PDF file can be processed through the app
- Destination conflict behavior works
- Activity entry is created
- Undo for the move works
- No watcher is required for the slice to succeed

---

## M4 — Rule Management

### Goal

Turn rules from static seeds into real user-managed configuration.

### Scope

- Rules list UI
- Enable / disable
- Reorder by priority
- Add rule
- Edit rule
- Delete rule
- Free tier limit behavior
- Conflict warnings
- Prebuilt rule editing for destination paths

### Intent

The user must be able to shape automation before automation becomes ambient.

### Exit Criteria

- Users can manage rules without touching files manually
- Priority order is visible and persisted
- Rule editor reflects the PRD’s allowed logic model

---

## M5 — Onboarding + Preview

### Goal

Make first-run trust and activation intentional.

### Scope

- Six-step onboarding shell
- Permissions guidance
- Folder selection
- Prebuilt rule enablement
- Sim preview
- Activation flow
- Backfill prompt entry point

### Intent

Onboarding is not just setup. It is how Seiri earns permission to act.

### Exit Criteria

- User can complete onboarding from zero to activated app
- Empty preview state works cleanly
- Free-tier rule blocking is consistent
- Settings and rules persist after activation

---

## M6 — Watcher + Retry Queue

### Goal

Turn the manual vertical slice into real-time automation.

### Scope

- File watcher for selected folders
- Safety checks
- Partial download handling
- Retry queue for safety failures only
- Pause / resume behavior
- Manual `Run Now`
- Backfill execution path

### Intent

This milestone is where Seiri becomes a background product.

### Exit Criteria

- New files are detected and processed automatically
- Unsafe files are retried correctly
- Unmatched files remain silent
- Backfill and watcher do not double-process the same file

---

## M7 — Activity + Undo

### Goal

Make automation inspectable and reversible in a trustworthy way.

### Scope

- Activity feed UI
- Status badges
- Filters
- Undo buttons
- Undo execution via Tauri
- 24-hour eligibility logic
- Error handling for undo edge cases

### Intent

This is the trust layer. We do not ship background automation without it.

### Exit Criteria

- Every success and safety failure appears correctly
- Undo works for supported actions
- Expired undo is handled correctly
- Failure messages are user-readable

---

## M8 — Menubar + Settings

### Goal

Make the app feel like a native macOS utility instead of a dev shell.

### Scope

- Menubar icon and popover
- Open app action
- Pause / resume
- Run now
- Settings UI
- Launch at login
- Notifications toggle and 7-day summary behavior

### Intent

This milestone makes Seiri feel like something a real user can leave running.

### Exit Criteria

- Menubar flow works end-to-end
- Settings affect real behavior
- No Dock-centric assumptions break usage

---

## M9 — Licensing + Release Prep

### Goal

Prepare the app to be sold and shipped safely.

### Scope

- Validate Lemon Squeezy implementation reality
- Implement licence entry and local status handling
- Finalize icons
- Signing and notarization prep
- Update flow validation
- Production build sanity

### Intent

This milestone is release readiness, not product discovery.

### Exit Criteria

- Licensing path is technically real, not hypothetical
- Production build path is documented
- Launch blockers are known and finite

---

## Cross-Cutting Risks

These must be checked continuously, not only at the end:

- `organize` may not cleanly express Seiri's rule model
- Lemon Squeezy key format may not support the current offline assumption
- Trash restore may be less straightforward than the PRD assumes
- Watcher and backfill may race without explicit coordination
- Hidden files and partial downloads may create noisy edge cases if not handled consistently

---

## Working Rules

While implementing, we follow this operating discipline:

- No starting a later milestone just because a piece looks easy
- No UI-only completion claims for behavior that has not run end-to-end
- No watcher work before engine feasibility is decided
- No onboarding polish before the first vertical slice is real
- No licensing build before the core product works

---

## Immediate Next Step

The next intentional step is **M0 completion plus M1 setup**.

That means:

1. finish hardening the current scaffold
2. create the engine feasibility harness
3. validate `organize` against Seiri's real rule requirements
4. make the engine decision before building watcher-dependent features
