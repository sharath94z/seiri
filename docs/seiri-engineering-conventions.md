# Seiri Engineering Conventions

This document is a short working agreement for the Seiri codebase.

## Source Of Truth

- Product behavior lives in the PRD.
- Build order lives in the implementation plan.
- Progress lives in the build checklist.
- Hazelnut-style implementation lessons live in the Hazel notes.

## Milestone Discipline

- Work one milestone at a time.
- Do not start later milestones early.
- Keep each milestone runnable and verifiable.
- Mark checklist items as soon as the work is done.

## State And Structure

- Frontend app state belongs in `src/store/`.
- Shared UI belongs in `src/components/shared/`.
- Page-level screens belong in `src/pages/`.
- Type definitions belong in `src/types/`.
- Shared constants belong in `src/constants/`.
- Storage helpers belong in `src/lib/`.

## Styling Decision

- M0 uses plain CSS.
- Do not introduce Tailwind or shadcn until the component surface justifies it.
- Keep the current visual direction calm, dense, and macOS-native.

## Checklist Discipline

- Treat [docs/seiri-build-checklist.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-build-checklist.md) as persistent memory.
- Update completion states in the same change set where possible.
- Use `[ ]` for not started, `[-]` for in progress, `[x]` for complete, and `[!]` for blocked.

## Change Discipline

- Keep edits small and intentional.
- Avoid feature creep inside a milestone.
- If something belongs to a later milestone, leave it out and document the dependency.
