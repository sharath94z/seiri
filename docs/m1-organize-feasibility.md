# M1 — Organize Feasibility Record

**Status:** locally verified, awaiting branch review and merge  
**Date:** 2026-05-17  
**Purpose:** decide whether `organize` is a viable engine foundation for Seiri v1

## Inputs

- Official `organize` docs and repository
- Seiri PRD v1.2
- Seiri M1 feasibility harness scaffold in [`../tools/m1-feasibility/harness.mjs`](../tools/m1-feasibility/harness.mjs)
- Scenario fixture in [`../tools/m1-feasibility/fixtures/m1-cases.json`](../tools/m1-feasibility/fixtures/m1-cases.json)

## What the harness covers

The harness is intentionally small and isolated. It currently captures the Seiri-relevant scenario set for:

- simple extension matching
- ordered rule evaluation
- nested `OR`
- `NOT` / exclusion behavior
- simulate / preview output
- move
- rename
- date-folder move
- trash
- source URL provenance

The current harness validates the fixture shape, evaluates each scenario against the capability map, and prints a compatibility report with `native`, `wrapper`, or `missing` classification. That classification is based on documented `organize` behavior and Seiri rule requirements, not on executing the real `organize` binary yet. It gives us a repeatable evidence trail for the Seiri-facing rule model, but it is still a documentation-driven feasibility pass rather than a live integration check.

## Compatibility Matrix

| Seiri need | Organize fit | Notes |
|---|---|---|
| Folder-scoped rules | Native | Organize is configured per location and processes rules in order. |
| Ordered rule evaluation | Native | Rule processing is top-to-bottom. Seiri can rely on this as the base ordering model. |
| First-match stop behavior | Wrapper | Organize evaluates in order, but Seiri still needs wrapper logic if we want strict first-match semantics. |
| Simple extension matching | Native | Covered by standard file filters. |
| Nested `OR` / `NOT` logic | Wrapper | Organize supports `all` / `any` / `none` plus negated filters, but not a Seiri-style nested boolean tree as a first-class shape. |
| Simulate / preview | Native | `organize sim` is a built-in dry-run path. |
| Move action | Native | Move is first-class and can also shape destination paths. |
| Rename action | Native | Rename is first-class. |
| Date-folder move | Wrapper | Organize can express date-shaped destinations through templates, but Seiri still needs translation from its own rule syntax. |
| Trash action | Native | Trash is a first-class action. |
| MIME / type matching | Native | Organize matches MIME by extension-derived type. |
| Source URL conditions | Wrapper | No documented native filter for source URL provenance, so Seiri must prefilter it. |

## Decision

**Keep `organize` with a Seiri translation layer.**

## Why

`organize` is a strong native fit for the file-system actions Seiri needs in v1:

- move
- rename
- trash
- simulate / preview
- MIME filtering
- top-to-bottom rule evaluation

The gaps are real but bounded:

- Seiri wants richer boolean grouping than `organize` exposes directly.
- Seiri PRD uses source URL provenance, which `organize` does not document as a native filter and therefore needs wrapper-side prefiltering.
- Seiri may want strict first-match semantics, which should be enforced by the wrapper rather than assumed.
- Date-folder behavior needs translation from Seiri rule intent into organize destination templating.

That means a full rewrite is not justified yet, but neither is a direct 1:1 pass-through. The right move is a thin translation layer that maps Seiri rule JSON into an `organize` config while preserving Seiri semantics where they matter.

## What the wrapper must do

- Convert Seiri rule JSON into `organize` rules.
- Preserve rule priority order.
- Translate nested boolean logic into the closest supported `organize` structure.
- Handle Seiri-only conditions, especially source URL provenance, before the engine runs.
- Keep simulate output separate from real execution.

## What this unblocks

- M2 can freeze the Seiri rule schema with a real engine target in mind.
- M3 can be built against a known execution model.
- We do not need to design a custom filesystem engine unless the wrapper becomes too large.

## Follow-up decisions

- Decide whether source URL provenance stays as a first-class rule field or becomes a wrapper-only prefilter.
- Decide how much of Seiri's boolean tree model should be normalized before engine translation.
- Implement a minimal translation proof in the next milestone.
- Validate the capability claims by invoking the real `organize` binary and/or adding integration tests that confirm the reported classifications for the Seiri-facing rule model.
