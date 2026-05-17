# Seiri — Product Requirements Document (PRD)
**Version:** 1.2  
**Status:** Approved  
**Last Updated:** May 2026  
**Author:** Sharath Aradhyamath  
**Document Type:** Product Requirements (What & Why)  

> **Changelog v1.1 → v1.2**
>
> Eight targeted fixes. No structural changes. Every section not listed below is identical to v1.1.
>
> 1. **Licensing — resolved internal inconsistency.** Removed the contradictory weekly re-validation
>    for monthly users. Model is now purely offline: monthly keys carry an expiry timestamp baked
>    in at purchase time. Lemon Squeezy re-issues a new key each billing cycle automatically.
>    Cancellation is detected when the current key expires naturally. No network call ever required
>    after first activation. Sections affected: 7 (tech stack rationale), 9 Feature 12, 12, 15
>    (error handling), Appendix A, Appendix B constraint 8.
>
> 2. **Licensing — softened cryptographic signing claim.** The PRD no longer asserts that Lemon
>    Squeezy natively produces cryptographically signed JWT keys. Instead it describes the intent
>    (local validation, no API credentials in binary) and flags that the exact implementation must
>    be validated against Lemon Squeezy's actual licence key API before building. Sections
>    affected: 7 (rationale), 9 Feature 12.
>
> 3. **Partial download safety — explicit exception added.** The safety checklist now has a
>    documented exception for stale partial files acted on by Rule 2 (trash action only). The
>    checklist method column is precise: partial extensions fail safety by default, but pass when
>    ALL of: file is older than 1 day, size is stable, no file lock, and the matching rule's
>    action is `trash`. Section affected: 13.
>
> 4. **Goal 1 — revised to structurally achievable wording.** "At least one rule active" removed
>    from Goal 1 because onboarding explicitly allows zero rules. Goal 1 now reads: "Onboarding
>    completed and Seiri ready to run within 5 minutes." Section affected: 5.
>
> 5. **Free tier conflict in onboarding — resolved.** Onboarding Step 4 previously showed a soft
>    warning when more than 3 rules were enabled, implying all were active. This conflicted with
>    Feature 12 which blocks enablement at rule #4. Step 4 now blocks the 4th toggle with an
>    upgrade prompt — consistent with post-onboarding behaviour. Section affected: 9 Feature 10.
>
> 6. **Pre-built rule condition syntax — rewritten to match schema.** Extension OR lists in the
>    pre-built rules table are now written as nested condition groups matching the actual
>    `conditionLogic` schema, not flat OR strings. Section affected: 11.
>
> 7. **Retry queue scope — safety failures only.** Unmatched files are no longer added to the
>    retry queue or logged as "No rule matched." They are left silently in place. The watcher
>    will re-evaluate them if rules change. Only files that fail safety checks are retried.
>    Sections affected: 8 (data flow), 9 Features 1, 8, 10 Flow 2, 10 Flow 3, 13, Appendix B
>    constraint 15.
>
> 8. **Undo ownership — clarified.** Appendix B now explicitly states that undo operations are
>    executed by Tauri directly, not the organize sidecar. The sidecar executes forward file
>    actions only. Section affected: Appendix B constraint 5.

---

## Table of Contents

1. [Product Vision](#1-product-vision)
2. [North Star Principle](#2-north-star-principle)
3. [Problem Statement](#3-problem-statement)
4. [Target User](#4-target-user)
5. [Product Goals — v1](#5-product-goals--v1)
6. [Explicit Out of Scope — v1](#6-explicit-out-of-scope--v1)
7. [Tech Stack](#7-tech-stack)
8. [System Architecture](#8-system-architecture)
9. [Core Features](#9-core-features)
10. [User Flows](#10-user-flows)
11. [Pre-Built Rules](#11-pre-built-rules)
12. [Pricing and Licensing](#12-pricing-and-licensing)
13. [File Safety Model](#13-file-safety-model)
14. [Undo Contract](#14-undo-contract)
15. [Error Handling](#15-error-handling)
16. [Notifications](#16-notifications)
17. [macOS Permissions](#17-macos-permissions)
18. [Distribution and Updates](#18-distribution-and-updates)
19. [Success Metrics — v1](#19-success-metrics--v1)

---

## 1. Product Vision

Seiri is a macOS file organisation utility that automatically moves, sorts, and manages files based on user-defined rules — silently running in the background so users never have to think about where their files are.

The name **Seiri (整理)** is the first step of the Japanese 5S methodology, meaning to sort and eliminate what does not belong. It perfectly describes what this product does.

Seiri is the answer for Mac users who have looked at Hazel, felt overwhelmed by its complexity, and closed the tab. It is powerful under the hood but approachable on the surface — designed for people who want their Mac organised without becoming a rules engineer.

---

## 2. North Star Principle

> **"Install once. Forget it. Always know where your files are."**

Every product decision must be evaluated against this principle.

- If a feature adds friction to setup → reconsider it
- If a behaviour could surprise the user → make it opt-in
- If the tool could make a mistake → make it do nothing instead
- If something requires ongoing user attention → it is not done yet

**Correctness over cleverness.** Seiri should be boring and reliable, not impressive and unpredictable.

---

## 3. Problem Statement

Mac users universally struggle with file accumulation and disorganisation. Research across user communities, support forums, and documentation reveals consistent pain points:

### 3.1 The Downloads Folder Black Hole
macOS saves files to Downloads silently with no prompts, alerts, or cleanup suggestions. Over time the folder accumulates years of files. Users report hundreds to thousands of files with no clear way to find what they need. One documented case showed a user with over 110GB of files in Downloads alone — unaware the folder existed.

### 3.2 Abandoned DMG Files
Every macOS app installation produces a `.dmg` file. Once the app is dragged to Applications, the `.dmg` serves no purpose — but macOS never removes it. These accumulate silently, consuming disk space indefinitely.

### 3.3 Desktop Clutter
The Desktop is treated as a temporary workspace but becomes permanent storage. Files pile up, creating visual noise and making it harder to focus.

### 3.4 Duplicate Files
Duplicate photos, documents, and downloads accumulate across the file system. Users report finding multiple copies of the same file in different locations, consuming storage and causing confusion when editing.

### 3.5 Fear of Deleting
Users know their Downloads folder is a mess but are afraid to clean it because they don't know what is safe to delete. This fear causes the problem to compound over time.

### 3.6 Manual Solutions Are Unsustainable
The advice users find online — sort by size, delete DMGs manually, set a weekly reminder to clean up — requires ongoing effort and discipline. Users adopt these habits briefly then revert to accumulation. A one-time setup that runs automatically is the only sustainable solution.

### 3.7 Existing Tools Are Too Complex
Hazel solves this problem but requires users to write rules in a GUI that feels like configuring software rather than describing intent. The learning curve causes users to abandon it before getting value. There is a large unserved market of users who need Hazel's power but not Hazel's complexity.

---

## 4. Target User

### Primary User
**The organised-minded Mac power user who is not a developer.**

- Uses a Mac daily for work — designer, writer, consultant, freelancer, remote worker
- Downloads frequently — installers, PDFs, invoices, assets, attachments
- Finds their Downloads folder embarrassing but doesn't know how to fix it permanently
- Has heard of Hazel but found it intimidating or not worth the setup effort
- Willing to pay for tools that genuinely save time and mental energy
- Spends $1000+ on Mac hardware — $25 for organisation software is trivial if it works

### Secondary User
**The developer or technical user who wants automation without writing scripts.**

- Knows what Hazel is, may even own it
- Wants something lighter, more modern, and easier to configure
- Appreciates the sim mode and audit log
- Will explore advanced rules and push the product further

### Anti-User (not the target for v1)
- Enterprise IT administrators managing fleets of Macs
- Users who want cloud sync or cross-device file management
- Users looking for a full file manager replacement
- Windows users

---

## 5. Product Goals — v1

### Goal 1: Onboarding completed and Seiri ready to run within 5 minutes
← **Fixed in v1.2**

A new user should complete onboarding and have Seiri running and watching their folders within 5 minutes of first launch. Zero rules enabled is a valid and acceptable end state of onboarding — Seiri is still running and the user can add rules at any time. This goal is structurally guaranteed: completing onboarding does not depend on existing files, matching rules, or any external condition.

The previous wording ("at least one rule active") was removed because onboarding explicitly allows the user to proceed with zero rules. A goal that conflicts with a permitted product behaviour is not a useful goal.

### Goal 2: Zero surprise file movements
Seiri must never move a file the user did not expect to be moved. Every action must be logged, explainable, and reversible. The backfill prompt (Section 10, Flow 7) ensures existing files are never touched without explicit user approval.

### Goal 3: Silent, stable background operation
Seiri runs as a menubar app. It should consume minimal CPU and RAM. It should never crash, freeze, or require user attention during normal operation. Notifications are intentionally minimal — a daily summary for 7 days only, then silence.

### Goal 4: Solve the top 5 pain points out of the box
Pre-built disabled rules should address: DMG accumulation, PDF clutter, archive accumulation, screenshot organisation, and stale partial download files. Users should be able to solve their biggest problems by enabling rules, not creating them.

### Goal 5: Establish a paying user base
Acquire at least 100 paying users within 60 days of launch. This validates the market and funds continued development.

---

## 6. Explicit Out of Scope — v1

The following will NOT be built in v1. These are deferred deliberately to keep scope tight.

- Windows or Linux support
- AI-powered file classification
- Cloud storage integration (iCloud, Dropbox, Google Drive)
- File content reading for classification (e.g. reading inside a PDF to categorise it)
- Mobile companion app
- Team or multi-user features
- Enterprise MDM deployment
- Centralised rule management across multiple Macs
- Folder sync or backup features
- File tagging beyond what macOS natively supports
- Plugin or extension system
- CLI interface
- Import rules from Hazel
- Permanent file deletion (`delete_permanently` action — deferred to v2)
- Per-file notifications

---

## 7. Tech Stack

| Layer | Technology | Version | Purpose |
|---|---|---|---|
| App shell | Tauri | 2.x | Native macOS app wrapper, system tray, file dialogs, permissions |
| UI framework | React | 18.x | All user interface components |
| UI language | TypeScript | 5.x | Type-safe frontend code |
| Styling | Tailwind CSS | 3.x | Utility-first styling |
| Component library | shadcn/ui | Latest | Base UI components |
| File organisation engine | organize (Python) | 3.x | Rule evaluation and file actions |
| Engine distribution | PyInstaller | Latest | Compiles organize + Python runtime into standalone binary |
| Engine integration | Tauri Sidecar | — | Manages organize binary lifecycle within the app |
| File watching | Tauri file system API + notify crate | — | Monitors configured folders for new files |
| In-app state | Zustand | 4.x | Lightweight React state management |
| Config storage | JSON via Tauri fs API | — | Stores user rules and preferences locally |
| Payments | Lemon Squeezy | — | Checkout, licence key generation, subscription management |
| Licence validation | Lemon Squeezy licence key with expiry timestamp | — | Local validation against expiry field in key. No backend required after activation. |
| Backend | None | — | No server required. Licence keys validated locally against expiry timestamp. |
| Distribution | Tauri build pipeline | — | Produces signed and notarized DMG |
| Auto-update | Tauri Updater | — | Silent background updates via GitHub Releases |
| Analytics | None | — | No telemetry in v1. Metrics derived from local signals only. |

### Key architectural decisions and rationale

**Why Tauri over Electron:**
Tauri uses the system WebView (WKWebView on macOS) instead of bundling Chromium. This results in a significantly smaller app bundle (~15-20MB vs ~150-300MB) and lower RAM usage — critical for a background utility that users expect to forget about.

**Why organize as the engine:**
The `organize` library (MIT licence) is a battle-tested Python file organisation tool with a rich filter and action system. Building an equivalent from scratch would take months. Using it as a sidecar binary via PyInstaller allows Seiri to ship with a proven engine while focusing development effort on the UI and user experience — Seiri's actual differentiator.

**Why no backend:**
Seiri is a local utility. Users are trusting it with their files. A fully local app with no network calls (except update checks and one-time licence activation) is simpler, faster, more trustworthy, and cheaper to operate.

**Why licence keys with expiry timestamps over other approaches:**
← **Updated in v1.2**

Storing Lemon Squeezy API credentials inside the app binary is a security vulnerability — the app could be decompiled and the credentials extracted. The chosen model avoids this entirely: Lemon Squeezy generates a licence key that contains an expiry timestamp. The app reads the expiry locally — no API call, no credentials in the binary. Monthly keys are issued with a short expiry (e.g. 35 days); Lemon Squeezy automatically re-issues a new key each billing cycle. Lifetime keys carry no expiry. If a monthly subscription is cancelled, the current key expires naturally at its timestamp — the user simply loses access when they next open the app after expiry.

**Implementation note for builders:** The exact format of Lemon Squeezy licence keys (whether they support embedded expiry timestamps, signed JWTs, or another format) must be validated against Lemon Squeezy's current licence key API documentation before implementation begins. The intent described above — local validation, no API credentials in the binary, expiry-based cancellation detection — is the requirement. The implementation must satisfy that intent using whatever Lemon Squeezy actually supports. If Lemon Squeezy does not support embedded expiry natively, a minimal licence verification endpoint (a single Vercel function) should be evaluated as the fallback.

---

## 8. System Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Seiri.app (Tauri)                  │
│                                                       │
│  ┌─────────────────┐    ┌────────────────────────┐   │
│  │   React UI      │    │   Tauri Rust Core      │   │
│  │                 │◄──►│                        │   │
│  │  - Rule builder │    │  - File system watcher │   │
│  │  - Activity feed│    │  - Sidecar manager     │   │
│  │  - Settings     │    │  - macOS permissions   │   │
│  │  - Onboarding   │    │  - System tray         │   │
│  │  - Sim preview  │    │  - Auto-updater        │   │
│  │  - Backfill     │    │  - Retry queue         │   │
│  │  - Undo (UI)    │    │  - Undo executor       │   │
│  └─────────────────┘    └──────────┬─────────────┘   │
│                                    │                  │
│                          ┌─────────▼──────────┐      │
│                          │  organize sidecar  │      │
│                          │  (PyInstaller bin) │      │
│                          │                    │      │
│                          │  - Rule evaluation │      │
│                          │  - File matching   │      │
│                          │  - Forward file    │      │
│                          │    actions only    │      │
│                          │  - Sim mode        │      │
│                          └────────────────────┘      │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │              Local Storage (JSON)                │ │
│  │  rules.json · settings.json · activity.json     │ │
│  │  retry-queue.json                               │ │
│  └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
         │                              │
         ▼                              ▼
   macOS File System            Lemon Squeezy
   (watched folders)            (licence key
                                 activation —
                                 one time only)
```

### Data flow for a file organisation event
← **Updated in v1.2: step 10 revised — unmatched files no longer added to retry queue**

1. User drops a file into a watched folder (e.g. ~/Downloads)
2. Tauri file watcher detects the new file
3. File safety checks run (see Section 13)
4. If safety checks fail → file added to retry queue → logged as "Skipped — [reason]" → stop
5. If safety checks pass → Tauri invokes the organize sidecar with the rule config
6. organize evaluates all rules against the file in priority order
7. First matching rule's action is executed by the sidecar
8. Result is written to activity.json
9. React UI updates the activity feed
10. File move count is incremented in daily summary counter
11. If no rule matches → file is left in place silently → no log entry → no retry queue

---

## 9. Core Features

### Feature 1: File Watcher

**Description:**
Seiri continuously monitors one or more user-configured folders for new or modified files. When a qualifying file appears, it is evaluated against the user's rules.

**User story:**
As a user, I want Seiri to automatically detect when a new file lands in my Downloads folder so I never have to manually trigger organisation.

**Acceptance criteria:**
- Seiri watches all folders configured in Settings
- Watcher starts automatically when Seiri launches
- Watcher resumes automatically after Mac sleep/wake
- Watcher handles external drives being disconnected and reconnected gracefully
- Maximum of 10 watched folders supported in v1
- Watcher does not consume more than 0.5% CPU during idle monitoring
- Watcher correctly ignores hidden files (files starting with `.`)
- Watcher correctly ignores files currently being written (see File Safety Model, Section 13)
- Files that fail safety checks are added to the retry queue (see Section 13)
- Files with no matching rule are left in place silently — no log entry, no retry queue ← **Updated in v1.2**

---

### Feature 2: Rule Engine

**Description:**
Rules are the core of Seiri. Each rule defines conditions a file must meet and actions to take when those conditions are met. Rules are evaluated in order — the first matching rule wins.

**User story:**
As a user, I want to define rules that automatically sort my files so I always know where to find them.

**Rule anatomy:**
```
Rule:
  id:             UUID (generated on creation)
  name:           Human-readable label (required)
  enabled:        true | false (default: true)
  priority:       Integer, determines evaluation order (lower = higher priority, 1 = highest)
  conditionLogic: "all" | "any"  (AND vs OR at the top level)
  conditions:     One or more condition objects or nested condition groups
  actions:        One or more actions to execute on match
  isPrebuilt:     true | false
  createdAt:      ISO timestamp
  updatedAt:      ISO timestamp
```

**Condition logic model:**

Seiri supports both AND and OR logic at the top level, and NOT/exclusion on individual conditions:

```
conditionLogic: "all"  → ALL conditions must match (AND)
conditionLogic: "any"  → AT LEAST ONE condition must match (OR)

Each condition also supports:
  negate: true  → condition must NOT match (NOT/exclusion)

Nested groups:
  A condition can itself be a group with its own conditionLogic: "any"
  This allows: ALL of [ext=pdf] AND [ANY of [name=invoice, name=receipt]]
  Maximum two levels of nesting enforced.
  Inner groups may only use conditionLogic: "any" (OR).
```

**Examples:**

Rule matching invoice OR receipt OR order PDFs:
```json
{
  "conditionLogic": "all",
  "conditions": [
    { "type": "extension", "value": "pdf", "negate": false },
    {
      "conditionLogic": "any",
      "conditions": [
        { "type": "filename_contains", "value": "invoice", "negate": false },
        { "type": "filename_contains", "value": "receipt", "negate": false },
        { "type": "filename_contains", "value": "order",   "negate": false }
      ]
    }
  ]
}
```

Rule matching images that are NOT screenshots:
```json
{
  "conditionLogic": "all",
  "conditions": [
    { "type": "mime_type",          "value": "image",      "negate": false },
    { "type": "filename_startswith","value": "Screenshot", "negate": true  }
  ]
}
```

**Supported conditions (v1):**

| Condition | Description | Example value |
|---|---|---|
| `extension` | File extension matches | `pdf`, `dmg`, `zip` |
| `filename_contains` | Filename contains string (case-insensitive) | `invoice`, `receipt` |
| `filename_startswith` | Filename starts with string | `Screenshot` |
| `filename_endswith` | Filename ends with string | `_final` |
| `mime_type` | MIME type category matches | `image`, `video`, `audio`, `document`, `archive` |
| `file_size_gt` | File size greater than threshold | `500 MB` |
| `file_size_lt` | File size less than threshold | `10 KB` |
| `date_added_older_than` | File added more than N days/weeks/months ago | `30 days` |
| `source_url_contains` | Download source URL contains string (see reliability note) | `github.com` |
| `filename_regex` | Filename matches regular expression | `^RG\d{12}` |

**Note on `source_url_contains`:**
macOS stores download source URLs as an extended attribute (`com.apple.metadata:kMDItemWhereFroms`). Availability varies by browser and application:
- **Supported:** Safari, Chrome, Firefox, most standard download managers
- **Not supported:** AirDrop transfers, email attachment saves, files created by apps that do not set this attribute, and files moved from other locations

When the source URL attribute is absent, the condition evaluates as **false** — the rule does not match. This is the safe failure mode. The condition builder UI must show a tooltip explaining this limitation. `source_url_contains` conditions should always be paired with at least one other condition (e.g. extension) so the rule remains useful when the URL attribute is unavailable.

**Supported actions (v1):**

| Action | Description | Example |
|---|---|---|
| `move` | Move file to destination folder | `~/Documents/PDFs/` |
| `move_with_date` | Move to date-organised subfolder using file metadata date | `~/Photos/{year}/{month}/` |
| `rename` | Rename file using a template | `{date}_{filename}` |
| `trash` | Move file to macOS Trash | — |
| `notify_only` | Log to activity feed without moving the file | — |

**Removed from v1:** `delete_permanently` — deferred to v2. Permanent deletion in a background automation tool creates an irrecoverable failure mode that conflicts with the trust model. Trash is the safe equivalent for v1.

**Acceptance criteria:**
- Rules are evaluated in priority order (ascending integer — 1 = highest priority)
- First matching rule wins — subsequent rules are not evaluated for that file
- Rules with `enabled: false` are skipped entirely
- A file with no matching rule is left in place silently — no log entry, no notification, no retry ← **Updated in v1.2**
- Rules are stored in `~/Library/Application Support/Seiri/rules.json`
- Maximum of 3 active (enabled) rules enforced for free tier users (UI-level enforcement)
- Rule changes take effect immediately — no restart required
- `conditionLogic: "any"` (OR) is supported at top level and in one nested group
- Individual conditions support `negate: true` for NOT/exclusion logic
- Maximum two levels of condition nesting enforced

---

### Feature 3: Rule Builder UI

**Description:**
A visual interface for creating, editing, reordering, enabling/disabling, and deleting rules. Designed to be usable by non-technical users without ever seeing a config file.

**User story:**
As a user, I want to create rules using a simple visual interface so I never have to write code or edit config files.

**Acceptance criteria:**
- User can create a new rule with a name, at least one condition, and at least one action
- User can toggle between "Match ALL conditions" and "Match ANY condition" at the top level (AND/OR)
- User can add an inline OR group within a condition row: "...OR filename contains receipt OR filename contains order"
- User can negate any individual condition by toggling "does not" instead of "does" in the condition builder
- User can add multiple conditions
- User can reorder rules by dragging — priority numbers update immediately
- User can toggle a rule on/off with a single click
- When a new rule would conflict with an existing rule, a non-blocking warning is shown
- Free tier: when a free user attempts to toggle on a 4th active rule, an upgrade prompt is shown immediately — the toggle does not activate ← **Updated in v1.2 — consistent with onboarding behaviour**
- All condition and action fields have inline helper text
- `source_url_contains` condition shows tooltip: "Works with Safari, Chrome and Firefox downloads. May not be available for all file sources."
- User can test a rule against files in the watched folder using a "Test this rule" button that runs sim mode for that rule only

---

### Feature 4: Simulation Mode (Sim Mode)

**Description:**
Before Seiri acts on any real files, users can preview exactly what would happen. Sim mode runs the rule engine against existing files and shows a detailed preview without touching anything.

**User story:**
As a new user, I want to see what Seiri would do to my existing files before I let it run, so I can trust it before giving it permission to act.

**Acceptance criteria:**
- Sim mode is accessible at any time from the main UI
- Sim mode runs the organize `--simulate` flag internally
- Results show: file name, current location, matched rule name, proposed action, proposed destination
- Results are grouped by rule
- Sim mode is presented as a mandatory step during onboarding before first activation
- Sim mode results clearly state "No files will be moved. This is a preview only."
- Sim mode completes within 10 seconds for folders with up to 1000 files
- If sim mode takes longer than 30 seconds: cancel and show "Folder contains too many files to preview. Seiri will organise files as they arrive."
- If no files match: show "No matching files found yet. Seiri will start organising as new files arrive." — this is a valid and expected state, not an error

---

### Feature 5: Backfill — Organise Existing Files

**Description:**
After initial activation, Seiri offers a one-time prompt to organise files already present in the watched folder. This is separate from the ongoing watcher which only handles new files.

**User story:**
As a new user who has just set up Seiri, I want the option to organise my existing Downloads folder so I get immediate value, not just future value.

**Acceptance criteria:**
- Immediately after onboarding activation, Seiri scans the watched folder(s) and counts existing files
- If existing files are found, a one-time prompt is shown:
  "Seiri found [N] existing files in your Downloads folder. Would you like to organise them now?"
  - "Organise existing files" → runs sim mode first → shows preview → user confirms → runs organize → logs results in activity feed
  - "Only organise new files" → Seiri watches from this point forward only, no backfill
- If no existing files are found, no prompt is shown
- Backfill requires explicit user confirmation after sim preview — it never runs automatically
- Backfill is a one-time operation — it is never repeated automatically
- After backfill completes or is declined, the watcher takes over for all future files
- Backfill results are logged to the activity feed exactly like normal moves and are individually undoable within 24 hours
- If user chooses "Only organise new files", they can trigger a manual backfill later via "Run Now" in the menubar

---

### Feature 6: Activity Feed

**Description:**
A real-time log of every action Seiri has taken. Provides full transparency and supports undo within the undo contract defined in Section 14.

**User story:**
As a user, I want to see exactly what Seiri has done to my files so I always know where something went and can undo a move if needed.

**Acceptance criteria:**
- Every successful file action and every safety-check failure is logged with: timestamp, filename, source path, destination path (where applicable), rule name, action type, and status
- Unmatched files are NOT logged — they are left silently in place ← **Updated in v1.2**
- Activity feed is accessible from the menubar popover and the main app window
- Each eligible entry has a one-click "Undo" button (eligibility per Section 14 Undo Contract)
- Activity feed shows status badges: success, skipped (safety failure), failed, undone
- Activity log enforces BOTH constraints simultaneously:
  - Maximum **1000 entries** — when exceeded, oldest entries are removed first
  - Maximum **30 days retention** (configurable in Settings, range: 7–90 days) — entries older than the retention period are removed on app launch
  - Whichever limit is reached first takes effect
- Activity feed can be filtered by: date, rule name, action type, status
- Failed actions are logged with a clear human-readable error reason

---

### Feature 7: Undo

**Description:**
The ability to reverse a Seiri action within 24 hours. Undo is a core trust feature — users must be able to confidently correct any mistake Seiri makes.

**Undo is executed by Tauri directly — not the organize sidecar.** The sidecar is responsible for forward file actions only. Undo operations (moving files back, restoring from Trash, renaming back) are performed by Tauri's file system APIs.

**Full undo contract is defined in Section 14.**

---

### Feature 8: Retry Queue
← **Updated in v1.2: scope narrowed to safety failures only**

**Description:**
Files that fail safety checks are added to a retry queue and re-evaluated periodically. Files with no matching rule are NOT added to the retry queue — they are left silently in place and will be naturally re-evaluated by the watcher if rules change.

**User story:**
As a user, I want Seiri to eventually organise files that weren't ready the first time due to safety checks, without me having to manually trigger anything.

**Acceptance criteria:**
- A file is added to `retry-queue.json` ONLY when it fails a safety check (partial extension, file lock, size instability, hidden file) ← **Updated in v1.2**
- Files with no matching rule are NOT added to the retry queue ← **Updated in v1.2**
- Retry queue is evaluated every 60 seconds
- Each file in the queue is re-checked against all safety criteria
- Each file is also re-evaluated against all current active rules (in case rules changed since the file was queued)
- If a file passes safety checks and matches a rule → execute action → log → remove from queue
- If a file passes safety checks but still has no matching rule → remove from queue silently ← **Updated in v1.2**
- If a file has been in the retry queue for more than 24 hours and still fails safety checks → remove from queue, log as "Skipped — file could not be safely processed after 24 hours"
- If a file no longer exists when re-evaluated → remove from queue silently
- Retry queue survives app restart — it is persisted to `retry-queue.json`
- Maximum 500 entries in retry queue — if exceeded, oldest entries are removed

---

### Feature 9: System Tray (Menubar)

**Description:**
Seiri lives in the macOS menu bar as a small icon. Clicking it shows a popover with recent activity and quick actions.

**Acceptance criteria:**
- Seiri icon appears in the macOS menu bar on launch
- Seiri does NOT appear in the Dock during normal operation
- Clicking the menubar icon opens a popover showing:
  - Last 5 successful activity entries
  - "Open Seiri" button to open the main window
  - "Pause / Resume" toggle
  - "Run Now" button to manually trigger organisation on all watched folders
- When Seiri is paused, the menubar icon changes to indicate paused state
- Seiri launches at login by default (user can disable in Settings)
- Menubar popover closes when user clicks outside it

---

### Feature 10: Onboarding Flow

**Description:**
A guided first-run experience that takes a new user from zero to Seiri actively watching their folders within 5 minutes.

**Acceptance criteria:**

**Step 1 — Welcome**
- Shows product name, tagline, one-line description
- Single CTA: "Get Started"
- Progress indicator shows 6 steps

**Step 2 — Permissions**
- Explains Full Disk Access in plain English:
  "macOS requires this permission for Seiri to read and move files. macOS grants broad access with this setting, but Seiri only ever reads and moves files in the specific folders you choose to watch — nothing else is accessed."
- Step-by-step guide to granting Full Disk Access
- App polls for permission every 2 seconds — no app restart required
- Does not proceed until permission is granted
- Permission denied state shows retry option

**Step 3 — Choose folders**
- Shows Downloads, Desktop, Documents as pre-built options
- Downloads is pre-selected
- Minimum one folder must be selected to proceed
- Custom folder picker via native macOS dialog

**Step 4 — Pre-built rules**
← **Updated in v1.2: free tier now blocks 4th enabled rule with upgrade prompt**

- All pre-built rules shown, all disabled by default
- User enables desired rules with toggle
- Each rule shows plain-English description and destination path
- Destination paths are editable inline
- When a free tier user attempts to enable a 4th rule, the toggle does not activate — an upgrade prompt is shown immediately instead
- The upgrade prompt in onboarding is identical to the post-onboarding upgrade prompt
- User can proceed with zero rules enabled — this is a valid and acceptable end state

**Step 5 — Sim mode preview**
- Runs automatically on step load
- Shows preview of what would happen to existing files given the enabled rules
- Empty state ("no matching files yet") is a valid outcome — not an error
- "Looks good, activate" proceeds regardless of whether files were found

**Step 6 — Activation**
- Enabled pre-built rules saved to `rules.json`
- Selected folders saved to `settings.json`
- `onboardingCompleted` set to `true`
- File watcher starts immediately
- Launch at login configured via Tauri autostart plugin
- Backfill prompt shown if existing files found (see Feature 5)
- Notification permission requested
- User transitions to main window `/rules`

---

### Feature 11: Settings

**Description:**
User preferences and app configuration.

**Settings options (v1):**

| Setting | Default | Description |
|---|---|---|
| Launch at login | Enabled | Start Seiri automatically on login |
| Daily summary notification | Enabled | One notification per day summarising moves. Auto-disables after 7 days. |
| Watched folders | ~/Downloads | Folders Seiri monitors |
| File stability wait | 5 seconds | Time to wait before acting on a new file |
| Conflict behaviour | Rename | What to do when destination file already exists: Rename (default) / Skip |
| Activity log retention | 30 days | How long to keep activity history (range: 7–90 days) |
| Account | — | Licence key entry, plan type, manage subscription link |

**Removed from settings:** `ask` option for conflict behaviour — asking the user breaks silent background operation. Only `rename` and `skip` are supported in v1.

---

### Feature 12: Upgrade and Licensing
← **Updated in v1.2: licensing model revised to offline expiry timestamps**

**Description:**
Free tier enforcement and upgrade flow via Lemon Squeezy licence keys with embedded expiry timestamps.

**Acceptance criteria:**
- Free tier allows maximum 3 active (enabled) rules. Disabled rules do not count toward the limit.
- When a free user attempts to enable a 4th rule (during onboarding or in the main app), the toggle does not activate — an upgrade prompt is shown immediately
- Upgrade prompt copy: "You've set up 3 rules — Seiri is already saving you time. Unlock unlimited rules to keep going."
- Upgrade prompt shows both options: $4.99/month and $24.99 lifetime
- Clicking either option opens Lemon Squeezy checkout in the default browser
- After payment, Lemon Squeezy emails a licence key to the user

**Licence key model:**
← **Updated in v1.2**

Seiri uses an offline expiry-based licence key model. The key contains an expiry timestamp that the app reads locally — no network call is required after the key is entered.

- **Monthly keys:** issued with an expiry date approximately 35 days from purchase date. Lemon Squeezy automatically re-issues a new key to the user each billing cycle before the current key expires. If the subscription is cancelled, the current key expires naturally at its timestamp and is not renewed. The user loses access when they next open the app after expiry.
- **Lifetime keys:** issued with no expiry field. Always valid once activated.
- **No weekly re-validation:** Seiri never makes a network call to check subscription status after first activation. Cancellation is detected purely through key expiry.

**Implementation note:** The exact format and capabilities of Lemon Squeezy licence keys must be validated against their current API documentation before building this feature. The requirement is: (a) no API credentials stored in the app binary, (b) local validation without a network call, (c) expiry-based cancellation detection for monthly keys. If Lemon Squeezy's native licence key format does not support embedded expiry, a minimal serverless verification function should be evaluated as a fallback.

**Licence key activation flow:**
1. User enters licence key in Settings → Account
2. App parses the key and reads the expiry timestamp (lifetime keys have no expiry)
3. App validates: key is well-formed, not expired, and matches expected format
4. On valid key: `licenceTier` updated to `monthly` or `lifetime`, unlimited rules unlocked immediately, key and validation timestamp stored in `settings.json`
5. On invalid key: "This licence key is not valid. Please check your purchase email and try again."
6. On expired key: "This licence key has expired. Please check your email for a renewed key, or resubscribe."
7. App checks key expiry on every launch (local read only — no network call). If expired: tier reverted to free, rules beyond 3 disabled (not deleted), user notified.

**Subscription cancellation behaviour:**
- Monthly user cancels via Lemon Squeezy customer portal
- Current key expires at its embedded timestamp (within ~35 days)
- On next app launch after expiry: tier reverted to free, rules beyond 3 active rules disabled (not deleted)
- User sees: "Your Seiri subscription has ended. Rules 4 and above have been paused. Resubscribe to reactivate them."
- User can resubscribe, receive a new key, and re-enter it to restore access immediately

---

## 10. User Flows

### Flow 1: First-time setup
```
Launch Seiri
  → Welcome screen
  → Grant Full Disk Access
  → Select watched folders (Downloads pre-selected)
  → Review and enable pre-built rules
      → Free user: 4th rule toggle blocked → upgrade prompt shown
  → Sim mode preview (empty state is valid, proceed regardless)
  → Activate
  → Backfill prompt (if existing files found)
      → "Organise existing files" → sim preview → confirm → run → activity feed
      → "Only organise new files" → skip
  → Main window /rules
```

### Flow 2: New file arrives in watched folder
← **Updated in v1.2: unmatched files no longer logged or retried**

```
File appears in ~/Downloads
  → Tauri watcher detects file
  → File safety checks run (Section 13)
      → Fails safety? → Add to retry queue → log "Skipped — [reason]" → stop
      → Passes safety? → Continue
  → organize sidecar evaluates rules in priority order
      → Match found? → Execute action → log to activity → increment daily counter
      → No match? → Leave file silently in place → no log entry → no retry
```

### Flow 3: Retry queue processing (every 60 seconds)
← **Updated in v1.2: only safety failures are in queue — no unmatched files**

```
Load retry-queue.json
  → For each file in queue (all are safety failures):
      → File still exists? → No: remove from queue silently → next
      → Safety checks pass now?
          → No: still failing
              → In queue < 24 hours? → keep, retry next cycle
              → In queue ≥ 24 hours? → remove, log "Skipped — could not safely process"
          → Yes: safety resolved
              → Evaluate rules
                  → Match found? → execute → log → remove from queue
                  → No match? → remove from queue silently (leave file in place)
```

### Flow 4: User creates a new rule
```
Click "+ Add Rule"
  → Enter rule name
  → Select condition logic: "Match ALL" or "Match ANY"
  → Add condition(s)
      → Each condition: type → value → optional negate toggle
      → Optional: add OR group within a condition row
  → Add action(s)
  → Conflict check runs → warning shown if overlap detected (non-blocking)
  → Click "Test this rule" (optional) → sim mode for this rule only
  → Click "Save"
  → Rule appears in list, takes effect immediately
```

### Flow 5: User undoes a file move
```
Open activity feed
  → Find entry with [Undo] button (available within 24 hours of action)
  → Click [Undo]
  → Confirmation dialog shown
  → Confirm
  → Tauri executes undo per action type (Section 14) — sidecar not involved
  → Activity entry updated to "undone"
  → If undo fails: specific error message shown per Section 14 edge cases
```

### Flow 6: User upgrades from free to paid
```
Attempt to enable 4th rule (onboarding or main app)
  → Upgrade prompt shown immediately — toggle does not activate
  → User selects plan → Lemon Squeezy checkout in browser
  → Purchase complete → licence key delivered by email
  → Return to Seiri → Settings → Account → Enter licence key
  → App parses key, checks expiry, validates format
  → Tier updated → rules unlocked → user continues
```

### Flow 7: Backfill existing files
```
Post-activation (or via "Run Now"):
  → Seiri scans watched folders for existing files
  → Count shown: "Found [N] existing files"
  → Sim mode runs → preview shown
  → User reviews and confirms
  → organize sidecar runs on existing files
  → Results logged to activity feed
  → Each move individually undoable within 24 hours
  → Watcher continues for all future files
```

### Flow 8: Monthly licence expiry
```
User opens Seiri after key expiry date
  → App reads key from settings.json on launch
  → Expiry timestamp has passed
  → Tier reverted to free locally (no network call)
  → Rules beyond 3 active rules disabled (not deleted)
  → Banner shown: "Your Seiri subscription has ended. Rules 4 and above have been paused."
  → User can resubscribe → receive new key → enter in Settings → access restored
```

---

## 11. Pre-Built Rules
← **Updated in v1.2: condition syntax rewritten to match nested conditionLogic schema**

All pre-built rules ship with Seiri disabled by default. They serve as both quick-start options and working examples of what rules can do.

**Priority order is critical.** Rules are listed in priority order — lower number = evaluated first. Rule 1 is evaluated before Rule 2, and so on. The first matching rule wins. Rules targeting specific subsets must appear before rules targeting broader categories.

### Pre-built rules — schema-accurate conditions

**Rule 1 — Move Disk Images** (Priority 1)
```json
{
  "name": "Move Disk Images",
  "conditionLogic": "all",
  "conditions": [
    {
      "conditionLogic": "any",
      "conditions": [
        { "type": "extension", "value": "dmg", "negate": false },
        { "type": "extension", "value": "pkg", "negate": false }
      ]
    }
  ],
  "actions": [{ "type": "move", "value": "~/Downloads/Installers/" }]
}
```

**Rule 2 — Clean Stale Partial Downloads** (Priority 2)
```json
{
  "name": "Clean Stale Partial Downloads",
  "conditionLogic": "all",
  "conditions": [
    {
      "conditionLogic": "any",
      "conditions": [
        { "type": "extension", "value": "crdownload", "negate": false },
        { "type": "extension", "value": "part",       "negate": false },
        { "type": "extension", "value": "download",   "negate": false }
      ]
    },
    { "type": "date_added_older_than", "value": "1 day", "negate": false }
  ],
  "actions": [{ "type": "trash" }]
}
```

**Rule 3 — Move Screenshots** (Priority 3)
```json
{
  "name": "Move Screenshots",
  "conditionLogic": "all",
  "conditions": [
    { "type": "filename_startswith", "value": "Screenshot", "negate": false },
    {
      "conditionLogic": "any",
      "conditions": [
        { "type": "extension", "value": "png", "negate": false },
        { "type": "extension", "value": "jpg", "negate": false }
      ]
    }
  ],
  "actions": [{ "type": "move", "value": "~/Pictures/Screenshots/" }]
}
```

**Rule 4 — Move Invoices and Receipts** (Priority 4)
```json
{
  "name": "Move Invoices and Receipts",
  "conditionLogic": "all",
  "conditions": [
    { "type": "extension", "value": "pdf", "negate": false },
    {
      "conditionLogic": "any",
      "conditions": [
        { "type": "filename_contains", "value": "invoice", "negate": false },
        { "type": "filename_contains", "value": "receipt", "negate": false },
        { "type": "filename_contains", "value": "order",   "negate": false }
      ]
    }
  ],
  "actions": [{ "type": "move", "value": "~/Documents/Finance/" }]
}
```

**Rule 5 — Move PDFs** (Priority 5)
```json
{
  "name": "Move PDFs",
  "conditionLogic": "all",
  "conditions": [
    { "type": "extension", "value": "pdf", "negate": false }
  ],
  "actions": [{ "type": "move", "value": "~/Documents/PDFs/" }]
}
```

**Rule 6 — Move Archives** (Priority 6)
```json
{
  "name": "Move Archives",
  "conditionLogic": "all",
  "conditions": [
    {
      "conditionLogic": "any",
      "conditions": [
        { "type": "extension", "value": "zip", "negate": false },
        { "type": "extension", "value": "tar", "negate": false },
        { "type": "extension", "value": "gz",  "negate": false },
        { "type": "extension", "value": "rar", "negate": false },
        { "type": "extension", "value": "7z",  "negate": false }
      ]
    }
  ],
  "actions": [{ "type": "move", "value": "~/Downloads/Archives/" }]
}
```

**Rule 7 — Move Images** (Priority 7)
```json
{
  "name": "Move Images",
  "conditionLogic": "all",
  "conditions": [
    { "type": "mime_type",          "value": "image",      "negate": false },
    { "type": "filename_startswith","value": "Screenshot", "negate": true  }
  ],
  "actions": [{ "type": "move", "value": "~/Pictures/Downloads/" }]
}
```

**Rule 8 — Move Videos** (Priority 8)
```json
{
  "name": "Move Videos",
  "conditionLogic": "all",
  "conditions": [
    { "type": "mime_type", "value": "video", "negate": false }
  ],
  "actions": [{ "type": "move", "value": "~/Movies/Downloads/" }]
}
```

**Rule 9 — Move Audio** (Priority 9)
```json
{
  "name": "Move Audio",
  "conditionLogic": "all",
  "conditions": [
    { "type": "mime_type", "value": "audio", "negate": false }
  ],
  "actions": [{ "type": "move", "value": "~/Music/Downloads/" }]
}
```

**Rule 10 — Archive Old Files** (Priority 10)
```json
{
  "name": "Archive Old Files",
  "conditionLogic": "all",
  "conditions": [
    { "type": "date_added_older_than", "value": "30 days", "negate": false }
  ],
  "actions": [{ "type": "move", "value": "~/Downloads/Archive/" }]
}
```

### Priority rationale — important for AI coding agents

- **Rule 1 (Disk Images) before Rule 6 (Archives):** `.dmg` files would otherwise match Rule 6 as archives. Rule 1 must precede Rule 6.
- **Rule 2 (Stale Partials) before Rule 6 (Archives):** Stale `.download` files would otherwise be moved as archives. Rule 2 must precede Rule 6. Note: active partial downloads are blocked by the safety model — Rule 2 only acts on genuinely stale files (older than 1 day, stable, unlocked).
- **Rule 3 (Screenshots) before Rule 7 (Images):** Screenshots are images and would match Rule 7. Rule 3 must precede Rule 7.
- **Rule 4 (Invoices) before Rule 5 (PDFs):** Invoice PDFs are PDFs and would match Rule 5. Rule 4 must precede Rule 5.

### DMG destination note
Pre-built Rule 1 moves DMGs to `~/Downloads/Installers/` rather than `~/Applications/Installers/`. This is intentional — the Applications folder conventionally contains `.app` bundles, not installer files. Users can change this destination during onboarding or at any time in the rule editor.

---

## 12. Pricing and Licensing

### Tiers

| Tier | Price | Active Rules | Features |
|---|---|---|---|
| Free | $0 | 3 active rules max | All core features. Pre-built rules count toward limit if enabled. |
| Monthly | $4.99/month | Unlimited | All features. Key expires ~35 days after issue; re-issued automatically each cycle. |
| Lifetime | $24.99 one-time | Unlimited | All features, all future v1.x updates. Key has no expiry. |

**Free tier clarification:** The limit applies to **active (enabled) rules only**. Disabled rules do not count. A free user can have many rules defined but only 3 enabled at a time.

### Payment processor
**Lemon Squeezy** handles all payment, delivery, and licence key generation. Acts as Merchant of Record — handles global tax compliance on your behalf.

### Licence model
← **Updated in v1.2: offline expiry timestamps, no weekly network calls**

Seiri uses an offline expiry-based licence key model:

- **Monthly keys** contain an expiry timestamp (~35 days from issue). Lemon Squeezy re-issues a new key automatically each billing cycle. If cancelled, the key is not renewed and expires naturally.
- **Lifetime keys** contain no expiry and are valid permanently once activated.
- **No network calls after activation.** The app reads the expiry timestamp locally on every launch. No pinging Lemon Squeezy, no API credentials in the binary.
- Licence status stored in `settings.json` after first validation.

**Implementation note:** Validate Lemon Squeezy's actual licence key format and capabilities before building. The requirement is local validation with no API credentials in the binary. If native key format does not support embedded expiry, evaluate a minimal serverless fallback.

### Subscription cancellation behaviour
- Monthly user cancels via Lemon Squeezy customer portal
- Lemon Squeezy stops issuing renewal keys
- Current key expires at its embedded timestamp (within ~35 days)
- On next app launch after expiry: tier reverted to free, rules beyond 3 active rules disabled (not deleted)
- User notified: "Your Seiri subscription has ended. Rules 4 and above have been paused. Resubscribe to reactivate them."
- User can resubscribe, receive a new key, enter it in Settings → Account to restore access immediately

---

## 13. File Safety Model

This section defines the rules that govern when Seiri is and is not permitted to act on a file.

### The fundamental rule
> A file is only actioned when TWO independent conditions are both true:
> 1. It satisfies a configured rule's conditions
> 2. It passes all safety checks
>
> Safety check failure → file added to retry queue → logged as skipped.
> No matching rule → file left silently in place → no log entry → no retry.

### How partial downloads work — important clarification

When a browser downloads a file, it writes a temporary file with a partial download extension while the transfer is in progress:
- Chrome: `file.zip.crdownload`
- Firefox: `file.zip.part`
- Safari: `file.zip.download`

Once the download completes, the browser renames the file to its final name (`file.zip`). The partial extension disappears entirely.

This means:
- **During download:** Safety check blocks action on `file.zip.crdownload` ✅
- **After download complete:** File is `file.zip` — safety checks run normally, rules evaluate as expected ✅
- **Stale abandoned partial:** A `file.zip.crdownload` left behind by a cancelled download, older than 1 day, stable and unlocked — can pass the partial extension safety check exception and be trashed by Rule 2 ✅

Safety checks and Rule 2 work together correctly — they are not in conflict.

### Safety checklist
← **Updated in v1.2: explicit exception added for stale partials matched by Rule 2**

Before any file action, ALL of the following must be true:

| Check | Method | Failure behaviour |
|---|---|---|
| File extension is not an active partial download extension | Extension is NOT in: `.crdownload`, `.part`, `.download`, `.tmp` — **Exception:** this check is bypassed when ALL of the following are true: (a) file is older than 1 day, (b) file size is stable, (c) no file lock, AND (d) the matched rule's action is `trash`. This exception exists specifically to allow Rule 2 (Clean Stale Partial Downloads) to function. | Add to retry queue |
| File is not hidden | Filename does not start with `.` | Add to retry queue |
| File size stable for at least N seconds | Two size readings separated by `fileStabilityWaitSeconds` (default: 5s) are identical | Add to retry queue, recheck on next cycle |
| File is not locked by another process | macOS file lock check via Tauri | Add to retry queue |
| Destination folder exists or can be created | Write permission test on destination path | Log error, skip — do NOT add to retry queue. User must fix the destination path. |
| File still exists at source | Re-confirmed immediately before move | Remove from queue silently |

### Retry behaviour
← **Updated in v1.2: only safety failures are retried**

Files that fail safety checks (except destination folder issues) are added to `retry-queue.json` and re-evaluated every 60 seconds:

- Each re-evaluation re-runs all safety checks
- If safety checks now pass: evaluate rules. If a rule matches → execute. If no rule matches → remove from queue silently (leave file in place).
- Files remain in the queue for up to 24 hours
- After 24 hours still failing safety checks: removed from queue, logged as "Skipped — file could not be safely processed after 24 hours"
- Files are never left in an ambiguous state — they either get actioned or receive a clear log entry

Files with no matching rule are **never added to the retry queue**. They are left in place silently. If the user later adds a matching rule, "Run Now" or the watcher's next event will pick them up.

### Destination conflict behaviour
If a file with the same name already exists at the destination:
- Seiri does NOT overwrite
- `rename` (default): append timestamp — `invoice.pdf` → `invoice_20260505_143022.pdf`
- `skip`: leave file in place, log as "Skipped — file already exists at destination"
- User configures preference in Settings → Conflict behaviour

### Never delete without Trash
Seiri never permanently deletes files in v1. The `delete_permanently` action is not implemented. All removal actions use macOS Trash, which the user can empty manually at their discretion.

---

## 14. Undo Contract

The undo feature is central to Seiri's trust model. This section defines the complete contract for what undo does and does not cover.

**Undo execution ownership:** All undo operations are executed by Tauri directly using macOS file system APIs. The organize sidecar is not involved in undo. The sidecar handles forward file actions only.

### General undo rules
- Undo is available for **24 hours** after a successful action
- After 24 hours: Undo button is greyed out with tooltip "Undo window expired"
- Undo requires user confirmation before executing
- Undo is logged in the activity feed as a separate "undone" entry
- Only one undo per action — you cannot undo an undo

### Undo behaviour by action type

| Action | Undo behaviour |
|---|---|
| `move` | Tauri moves file back from destination to original source path |
| `move_with_date` | Tauri moves file back from date-organised subfolder to original source path |
| `rename` | Tauri renames file back to its original filename at its current location |
| `trash` | Tauri restores file from macOS Trash to its original source path using macOS Trash restore API |
| `notify_only` | No undo available — no file was changed. No Undo button shown. |

### Edge cases and failure modes

| Scenario | Behaviour |
|---|---|
| Original source folder no longer exists | Error: "Cannot undo — the original folder no longer exists." Do not create the folder automatically. |
| File was moved again after Seiri moved it | Error: "Cannot undo — the file has been moved since Seiri organised it." |
| File was renamed after Seiri moved it | Error: "Cannot undo — the file has been renamed since Seiri organised it." |
| File was deleted from destination | Error: "Cannot undo — the file no longer exists at its organised location." |
| Destination had conflict rename (invoice_20260505.pdf) | Tauri moves the conflict-renamed file back to source with its conflict name. Original filename is not restored — it was already in use when the move occurred. |
| Trash restore fails — file already emptied from Trash | Error: "Cannot undo — the file has been permanently deleted from the Trash." |
| Trash restore — file already manually restored by user | Error: "Cannot undo — the file has already been restored from the Trash." |

### What undo does NOT cover
- Actions older than 24 hours
- `notify_only` actions (no file was changed)
- Files the user manually moved after Seiri organised them
- Batch backfill — each individual file move within a backfill CAN be undone individually within 24 hours of that specific move

---

## 15. Error Handling

### Principle
Seiri fails silently and safely. Errors are logged but never interrupt the user unless action is required from them.

| Error Scenario | Behaviour |
|---|---|
| Watched folder no longer exists | Log warning, skip folder, show yellow indicator in Settings next to that folder |
| Destination folder cannot be created (permissions) | Skip action, log error with reason — do NOT add to retry queue. User must fix the destination path. |
| File move fails (permissions) | Skip action, log error in activity feed with "Failed" badge and human-readable reason |
| organize sidecar crashes | Log crash, attempt restart up to 3 times with 5-second delay. If still failing: red indicator in menubar, pause all rules, show error in popover |
| organize sidecar not found | Critical error on launch: "Seiri engine is missing. Please reinstall Seiri." Provide download link. |
| Licence key malformed or unreadable | Show: "This licence key is not valid. Please check your purchase email and try again." |
| Licence key expired (monthly, cancelled) | On launch: tier reverted to free, rules beyond 3 disabled, banner shown: "Your Seiri subscription has ended. Rules 4 and above have been paused." |
| Sim mode timeout (>30 seconds) | Cancel sim, show: "Too many files to preview. Seiri will organise files as they arrive." Allow user to proceed. |
| Undo fails for any reason | Show specific error message per Section 14 edge cases. Never silently fail an undo attempt. |
| macOS Full Disk Access revoked | Persistent red banner in main window and menubar. Pause all rules. Prompt: "Seiri needs Full Disk Access to work. [Restore Access →]" |
| Retry queue full (500 entries) | Remove oldest entries. Log warning in activity feed. |
| Activity log full (1000 entries) | Remove oldest entries silently. No user notification needed. |

---

## 16. Notifications

### Model
Seiri follows a **7-day daily summary** notification model, then goes permanently silent. The goal is to build awareness and trust during the critical first week, then fulfil the "forget it" promise completely.

### Notification schedule

**Days 1–7 after activation:**
- One daily summary notification, sent at end of day (6pm local time)
- Only sent if at least one file was moved that day
- Content: "Seiri organised [N] files today. Tap to see details."
- Tapping opens the main window with the activity feed filtered to today
- If no files were moved that day: no notification sent

**After day 7:**
- Zero notifications, permanently
- User checks the activity feed and menubar icon for status
- No exceptions — no nudges, no marketing, no engagement prompts

**All users (free and paid):**
- Daily summary disabled after day 7 for all users
- Users who want ongoing notifications can re-enable the daily summary toggle in Settings at any time
- Once manually re-enabled after day 7, it runs indefinitely until turned off again

### Settings

```
Notifications
  Daily summary     [●──────]
  Auto-disables after 7 days. Check your activity feed anytime.
```

Single toggle. No mode selection.

### Notification permission
- Requested once during onboarding Step 6 (activation)
- If denied: Seiri functions normally, no repeated permission requests
- If denied: Settings shows "Notifications: Permission not granted" with link to System Settings

### What Seiri never sends
- Per-file notifications
- Marketing or promotional notifications
- "You haven't opened Seiri in a while" engagement nudges
- Error notifications (errors shown in menubar indicator and app UI only)
- Update notifications (updates applied silently; menubar shows version bump once)

---

## 17. macOS Permissions

Seiri requires the following macOS permissions. All permission requests must be explained to the user in plain English before the system dialog appears.

| Permission | Why it is needed | Honest explanation for users | When requested |
|---|---|---|---|
| Full Disk Access | To read files in watched folders and move them to destination folders | "macOS requires this permission for Seiri to move files. macOS grants broad access with this setting, but Seiri only ever reads and moves files in the specific folders you choose to watch — nothing else is accessed." | Onboarding Step 2 |
| Login Items | To start Seiri automatically when the Mac starts | "So Seiri is always running in the background without you needing to open it manually." | Onboarding Step 6 |
| Notifications | To send a daily summary of organised files for the first 7 days | "A brief daily update for your first week. Turns off automatically after that." | Onboarding Step 6 |

### Privacy statement (for website and app About screen)
Seiri accesses only the folders you select. It does not read file contents. It does not send file names, paths, or any personal data to any server. The only network requests Seiri makes are: checking for app updates (version number only, to GitHub) and activating your licence key (one-time only, on first entry). No analytics, no telemetry, no tracking.

### Permissions Seiri does NOT request
- Microphone
- Camera
- Location
- Contacts
- Calendar
- Photos library
- Any network access beyond update checks and one-time licence activation

---

## 18. Distribution and Updates

### Build requirements
- Apple Developer Account ($99/year) — required for code signing and notarization
- App signed with Developer ID Application certificate
- App notarized via Apple's notarization service (required since macOS Catalina)
- Tauri build pipeline handles signing and notarization with correct configuration

### Distribution format
- Primary: Signed `.dmg` containing `Seiri.app`
- Hosted at: `seiri.app/download`
- Mirror: GitHub Releases

### Auto-update
- Tauri Updater checks for updates on launch and every 24 hours
- Updates downloaded silently in background
- Applied on next app launch
- Menubar shows "Updated to v1.0.x" once after update — then silent
- User never prompted to manually download an update

### Version naming
- `v1.0.0` — initial launch
- `v1.0.x` — bug fixes, safety improvements
- `v1.x.0` — new features within v1 scope
- `v2.0.0` — major version (engine rewrite, new platform, etc.)

---

## 19. Success Metrics — v1

### Measurement approach
Seiri has no analytics or telemetry in v1. All metrics must be observable without tracking users. Metrics are derived from:
- **Support emails** — qualitative signal from real users
- **Lemon Squeezy dashboard** — purchases, conversions, refunds, subscription status
- **GitHub Releases download counts** — acquisition signal
- **Product Hunt and community engagement** — launch signal
- **User interviews** — qualitative depth (target 5–10 users in first 30 days)

Metrics that require telemetry (active rules after 7 days, undo rate, disable rate) are tracked qualitatively through user interviews in v1. If v1 succeeds, privacy-preserving opt-in telemetry will be evaluated for v2.

### Acquisition metrics (observable via Lemon Squeezy + download counts)
- 500 downloads within 30 days of launch
- 100 paying users within 60 days of launch
- Product Hunt launch — top 5 product of the day

### Revenue metrics (observable via Lemon Squeezy)
- Monthly Recurring Revenue (MRR) after 60 days
- Lifetime vs monthly purchase ratio
- Refund rate — target under 5%

### Support metrics (observable via email)
- Response time under 24 hours
- Most common support category — used to prioritise v1.1
- Target: zero reports of files lost or moved incorrectly due to a Seiri bug

### Qualitative metrics (via user interviews)
- Did the user complete onboarding without help?
- Did the user understand what rules do before creating one?
- Did the user trust Seiri enough to leave it running?
- Did the user recommend it to anyone?

---

## Appendix A: Glossary

| Term | Definition |
|---|---|
| Rule | A user-defined combination of condition logic, conditions, and actions |
| Condition | A criterion a file must meet for a rule to match |
| Condition logic | Whether conditions use AND ("all") or OR ("any") logic |
| Negate | A flag on a condition that inverts its match — "does NOT contain" |
| Action | What Seiri does when a file matches a rule |
| Sim mode | A preview mode that shows what Seiri would do without touching any files |
| Sidecar | The organize binary (compiled via PyInstaller) managed by Tauri. Executes forward file actions only. |
| Activity feed | The complete log of all file actions and safety-check failures |
| Stability check | Verifying a file's size is not changing before acting on it |
| Watched folder | A folder Seiri monitors for new and modified files |
| Backfill | The one-time operation to organise files already present when Seiri is first activated |
| Retry queue | The queue of files that failed safety checks, pending re-evaluation. Unmatched files are NOT queued. |
| Partial download | A file currently being downloaded, identified by a partial extension (e.g. .crdownload) |
| Stale partial | An abandoned partial download file, older than 1 day, stable, and unlocked — eligible for Rule 2 to trash |
| Licence key | A key from Lemon Squeezy containing tier and expiry information, validated locally on each app launch |
| Expiry timestamp | The date embedded in a monthly licence key after which the key is considered expired |
| Daily summary | The single daily notification summarising how many files Seiri organised (active for first 7 days only) |

---

## Appendix B: Key Constraints Summary
← **Updated in v1.2: constraints 5, 8, 15 revised**

For AI coding agents — the following constraints are non-negotiable and must be respected in every implementation decision:

1. **Never move a file that fails any safety check** — add to retry queue instead. The stale partial exception in Section 13 is the only exception and is narrowly defined.
2. **Never overwrite a file at the destination** — rename with timestamp or skip, per user setting.
3. **Never permanently delete files in v1** — use macOS Trash only. `delete_permanently` is not implemented.
4. **Free tier maximum is 3 active (enabled) rules** — enforced in UI at the moment of toggle, both during onboarding and in the main app. Disabled rules do not count.
5. **The organize sidecar executes forward file actions only.** Tauri/React never directly call the sidecar for undo. Undo operations (move back, rename back, restore from Trash) are executed by Tauri directly using macOS file system APIs. ← **Updated in v1.2**
6. **All safety-check failures are logged.** Unmatched files are NOT logged and are left silently in place. No silent safety failures.
7. **Undo must be available for 24 hours after any reversible action** — per the full undo contract in Section 14. Undo is executed by Tauri, not the sidecar.
8. **App must function fully offline after licence activation.** No network calls required after the licence key is first entered. Key expiry is checked locally on every launch by reading the timestamp in the stored key. ← **Updated in v1.2**
9. **No user data leaves the device** except: update version check (to GitHub) and one-time licence key activation (to Lemon Squeezy on first entry).
10. **Rules are evaluated strictly in priority order** — first match wins, no exceptions.
11. **Notifications are daily summary only, for 7 days** — never per-file, never after day 7 unless user manually re-enables the toggle.
12. **Condition logic supports AND, OR, and NOT** — implemented via `conditionLogic` field and `negate` flag per Section 9 Feature 2. Maximum two levels of nesting.
13. **Pre-built rules are disabled by default** — Seiri never acts on files without explicit user activation.
14. **Backfill is one-time and requires explicit user confirmation after sim preview** — never runs automatically or repeatedly.
15. **Retry queue contains safety failures only.** Unmatched files are never queued. A file that passes safety checks but has no matching rule is removed from the queue silently and left in place. ← **Updated in v1.2**
