# Prompt — Finish Seiri M0 Foundation Hardening

You are working in the Seiri repository. Your job is to finish the **remaining tasks in M0 — Foundation Hardening** only.

Do not start M1, do not add watcher logic, do not add engine integration, and do not add licensing work. Stay tightly scoped to M0.

## Product Context

Seiri is a macOS file organization utility built with **Tauri + React + TypeScript**. Its positioning is:

- simpler and calmer than Hazel
- trustworthy, reversible, and quiet
- menubar-first macOS utility
- strong on preview, transparency, and safe defaults

The product docs are in:

- [docs/seiri-prd-v1.2.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-prd-v1.2.md)
- [docs/seiri-uiux-spec.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-uiux-spec.md)
- [docs/hazel-implementation-notes.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/hazel-implementation-notes.md)
- [docs/seiri-implementation-plan.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-implementation-plan.md)
- [docs/seiri-build-checklist.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-build-checklist.md)

## Current State

The repo already has:

- a scaffolded Tauri + React app
- Seiri project naming
- a routed app shell
- placeholder pages for onboarding, rules, activity, and settings
- base frontend stores, types, and local persistence scaffold
- successful `npm run build`
- successful `cargo check`

This means M0 is **partially complete**.

## Remaining M0 Tasks

These are the only tasks you should work on:

1. Clean generated scaffold leftovers.
2. Decide and document the styling system for v1.
3. Add baseline `.gitignore` sanity.
4. Add an app-level engineering conventions doc only if it materially helps future implementation.
5. Update the build checklist to reflect completed work.
6. Make sure M0 exit criteria are actually satisfied.

Do not invent new M0 work unless it is required to finish one of those tasks.

## Exact End Outcome Required

When you are done, M0 should mean this:

- No obvious Tauri/Vite demo leftovers remain in the repo.
- The repo structure feels intentional rather than generator-shaped.
- A future contributor can immediately tell:
  - how styling should be done
  - where shared UI should go
  - where state lives
  - what not to change casually
- The app shell still builds after your cleanup.
- The checklist clearly shows M0 complete.

## Styling Decision Guidance

You must make an explicit v1 decision and document it.

Use this decision rule:

- If the current app already has meaningful custom CSS and no Tailwind/shadcn integration yet, prefer:
  - **plain CSS for M0**
  - defer Tailwind/shadcn until there is a real component surface that justifies the switch

This is the preferred decision unless the repo already clearly depends on Tailwind.

If you choose plain CSS now, document:

- why it was chosen
- what would trigger a future migration
- that future work should preserve the existing Seiri visual direction

## Cleanup Guidance

Look for and remove or replace things like:

- generator README text that no longer matches Seiri
- unnecessary public assets like Vite/Tauri logos if unused
- editor-specific files that should not be committed
- demo-specific references in code, config, or metadata

Be conservative:

- do not remove docs created for Seiri
- do not restructure the repo heavily
- do not replace working code just for neatness

## `.gitignore` Guidance

Ensure the repo ignores at minimum:

- `node_modules`
- `dist`
- `src-tauri/target`
- `.DS_Store`
- environment or local editor clutter where appropriate

Do not overcomplicate it.

## Engineering Conventions Doc Guidance

Only add this if it helps future milestone work. If you add it, keep it short and practical. It should cover:

- source-of-truth docs
- milestone discipline
- where app state belongs
- how checklist updates should happen
- not starting later milestones early

If the existing docs already make this obvious, skip the conventions doc.

## Files You Will Likely Touch

- [docs/seiri-build-checklist.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-build-checklist.md)
- [docs/seiri-implementation-plan.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-implementation-plan.md)
- [package.json](/Users/sharatharadhyamath/Documents/github/seiri/package.json)
- [.gitignore](/Users/sharatharadhyamath/Documents/github/seiri/.gitignore)
- [README.md](/Users/sharatharadhyamath/Documents/github/seiri/README.md)
- [public/](/Users/sharatharadhyamath/Documents/github/seiri/public)
- [src/](/Users/sharatharadhyamath/Documents/github/seiri/src)
- [src-tauri/](/Users/sharatharadhyamath/Documents/github/seiri/src-tauri)

## Constraints

- Stay within M0.
- Keep edits intentional and minimal.
- Do not add product features.
- Do not touch M1 engine feasibility.
- Do not add organize integration.
- Do not add licensing implementation.
- Do not create fake completeness. If something is still incomplete, leave it incomplete and reflect that in the checklist.

## Verification Required

Before finishing, run:

```bash
npm run build
source "$HOME/.cargo/env" && cargo check --manifest-path src-tauri/Cargo.toml
```

If either fails, fix it before stopping.

## Final Deliverable

When finished, the repository should contain:

- a cleaned and intentional M0 foundation
- an explicit styling decision for v1
- a sane `.gitignore`
- checklist updated with accurate statuses
- M0 exit criteria either marked complete or clearly showing what remains

Your output should summarize:

1. what you changed
2. what styling decision was made
3. whether M0 is now complete
4. what, if anything, still blocks M0 completion
