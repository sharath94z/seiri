# Seiri — Product Requirements Document (PRD)
**Version:** 1.0  
**Status:** Draft  
**Last Updated:** May 2026  
**Author:** Sharath Aradhyamath  
**Document Type:** Product Requirements (What & Why)  

> This PRD is the single source of truth for what Seiri is and why it exists.
> It is intended for product collaborators, designers, and AI coding agents.
> Implementation phases are defined in a separate Implementation Plan document.
> UI/UX wireframes and component specs are defined in a separate UI/UX Spec document.

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
14. [Error Handling](#14-error-handling)
15. [macOS Permissions](#15-macos-permissions)
16. [Distribution and Updates](#16-distribution-and-updates)
17. [Success Metrics — v1](#17-success-metrics--v1)

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

### Goal 1: Time to value under 3 minutes
A new user should have at least one rule running and one file automatically moved within 3 minutes of first launch. This is the single most important metric for v1.

### Goal 2: Zero surprise file movements
Seiri must never move a file the user did not expect to be moved. Every action must be logged, explainable, and reversible.

### Goal 3: Silent, stable background operation
Seiri runs as a menubar app. It should consume minimal CPU and RAM. It should never crash, freeze, or require user attention during normal operation.

### Goal 4: Solve the top 5 pain points out of the box
Pre-built disabled rules should address: DMG accumulation, PDF clutter, archive accumulation, screenshot organisation, and leftover partial download files. Users should be able to solve their biggest problems by enabling rules, not creating them.

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
| File watching | Tauri file system API | — | Monitors configured folders for new files |
| In-app state | Zustand | 4.x | Lightweight React state management |
| Config storage | JSON via Tauri fs API | — | Stores user rules and preferences locally |
| Payments | Lemon Squeezy | — | Checkout, delivery, subscription management |
| Backend (minimal) | None for v1 | — | No server required — fully local app |
| Distribution | Tauri build pipeline | — | Produces signed and notarized DMG |
| Auto-update | Tauri Updater | — | Silent background updates |
| Analytics | None for v1 | — | No tracking in v1 |

### Key architectural decisions and rationale

**Why Tauri over Electron:**
Tauri uses the system WebView (WKWebView on macOS) instead of bundling Chromium. This results in a significantly smaller app bundle (~15-20MB vs ~150-300MB) and lower RAM usage — critical for a background utility that users expect to forget about.

**Why organize as the engine:**
The `organize` library (MIT licence) is a battle-tested Python file organisation tool with a rich filter and action system. Building an equivalent from scratch would take months. Using it as a sidecar binary via PyInstaller allows Seiri to ship with a proven engine while focusing development effort on the UI and user experience — Seiri's actual differentiator.

**Why no server for v1:**
Seiri is a local utility. Users are trusting it with their files. A fully local app with no network calls (except update checks and licence verification) is simpler, faster, more trustworthy, and cheaper to operate.

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
│  └─────────────────┘    └──────────┬─────────────┘   │
│                                    │                  │
│                          ┌─────────▼──────────┐      │
│                          │  organize sidecar  │      │
│                          │  (PyInstaller bin) │      │
│                          │                    │      │
│                          │  - Rule evaluation │      │
│                          │  - File matching   │      │
│                          │  - File actions    │      │
│                          │  - Sim mode        │      │
│                          └────────────────────┘      │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │              Local Storage (JSON)                │ │
│  │  - rules.json  - settings.json  - activity.json │ │
│  └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
         │                              │
         ▼                              ▼
   macOS File System            Lemon Squeezy
   (watched folders)            (licence check
                                 on activation)
```

### Data flow for a file organisation event

1. User drops a file into a watched folder (e.g. ~/Downloads)
2. Tauri file watcher detects the new file
3. File safety checks run (see Section 13)
4. If safe, Tauri invokes the organize sidecar with the rule config
5. organize evaluates all rules against the file
6. First matching rule's action is executed
7. Result is written to activity.json
8. React UI updates the activity feed
9. macOS notification shown (brief, non-intrusive)
10. If no rule matches, file stays in place — no action, no notification

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

---

### Feature 2: Rule Engine

**Description:**
Rules are the core of Seiri. Each rule defines conditions a file must meet and actions to take when those conditions are met. Rules are evaluated in order — the first matching rule wins.

**User story:**
As a user, I want to define rules that automatically sort my files so I always know where to find them.

**Rule anatomy:**
```
Rule:
  name:        Human-readable label (required)
  enabled:     true | false (default: true)
  priority:    Integer, determines evaluation order (lower = higher priority)
  conditions:  One or more conditions (all must match — AND logic)
  actions:     One or more actions to execute on match
```

**Supported conditions (v1):**

| Condition | Description | Example |
|---|---|---|
| `extension` | File extension matches | `pdf`, `dmg`, `zip` |
| `filename_contains` | Filename contains string (case-insensitive) | `invoice`, `receipt` |
| `filename_startswith` | Filename starts with string | `Screenshot` |
| `filename_endswith` | Filename ends with string | `_final` |
| `mime_type` | MIME type matches | `image`, `video`, `audio` |
| `file_size_gt` | File size greater than | `500 MB` |
| `file_size_lt` | File size less than | `10 KB` |
| `date_added_older_than` | File added more than N days ago | `30 days` |
| `source_url_contains` | Download source URL contains string | `github.com` |
| `filename_regex` | Filename matches regular expression | `^RG\d{12}` |

**Supported actions (v1):**

| Action | Description | Example |
|---|---|---|
| `move` | Move file to destination folder | `~/Documents/PDFs/` |
| `move_with_date` | Move to date-organised subfolder | `~/Photos/{year}/{month}/` |
| `rename` | Rename file using template | `{date}_{filename}` |
| `trash` | Move file to macOS Trash | — |
| `delete_permanently` | Delete without Trash (requires confirmation) | — |
| `notify_only` | Surface in activity feed without moving | — |

**Acceptance criteria:**
- Rules are evaluated in priority order (ascending integer)
- First matching rule wins — subsequent rules are not evaluated for that file
- Rules with `enabled: false` are skipped entirely
- A file with no matching rule is left in place with no action taken
- Rules are stored in `~/Library/Application Support/Seiri/rules.json`
- Maximum of 3 rules enforced for free tier users (UI-level enforcement)
- Rule changes take effect immediately — no restart required
- `delete_permanently` action requires explicit user confirmation dialog before execution

---

### Feature 3: Rule Builder UI

**Description:**
A visual interface for creating, editing, reordering, and enabling/disabling rules. Designed to be usable by non-technical users without ever seeing a config file.

**User story:**
As a user, I want to create rules using a simple visual interface so I never have to write code or edit config files.

**Acceptance criteria:**
- User can create a new rule with a name, at least one condition, and at least one action
- User can add multiple conditions to a rule (AND logic)
- User can reorder rules by dragging
- Rule order is visually numbered (1, 2, 3...) to make priority explicit
- User can toggle a rule on/off with a single click
- When a new rule would conflict with an existing rule (same conditions, different actions), a warning is shown: "This rule may never trigger because Rule #N above it matches the same files"
- Free tier users see a clear upgrade prompt when attempting to create rule #4
- All condition and action fields have inline helper text explaining what they do
- User can test a rule against a specific file using a "Test this rule" button that runs sim mode for that file

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
- User can review and approve before activating
- Sim mode is presented as a mandatory step during onboarding before first activation
- Sim mode results clearly state "No files will be moved. This is a preview only."
- Sim mode completes within 10 seconds for folders with up to 1000 files

---

### Feature 5: Activity Feed

**Description:**
A real-time log of every action Seiri has taken. Provides full transparency and supports one-click undo.

**User story:**
As a user, I want to see exactly what Seiri has done to my files so I always know where something went and can undo a move if needed.

**Acceptance criteria:**
- Every file action is logged with: timestamp, filename, source path, destination path, rule name that triggered it
- Activity feed is accessible from the menubar popover and the main app window
- Each entry has a one-click "Undo" button
- Undo moves the file back to its original location
- Undo is available for 24 hours after the action
- After 24 hours, Undo button is greyed out with tooltip "Undo window expired"
- Activity log persists across app restarts
- Activity log stores a maximum of 1000 entries (oldest entries removed when limit reached)
- Failed actions (file not found, destination not writable) are logged with clear error reason
- Activity feed can be filtered by: date, rule name, action type

---

### Feature 6: System Tray (Menubar)

**Description:**
Seiri lives in the macOS menu bar as a small icon. Clicking it shows a popover with recent activity and quick actions.

**User story:**
As a user, I want Seiri to be always present but never intrusive — I should be able to check what it's done with a single click.

**Acceptance criteria:**
- Seiri icon appears in the macOS menu bar on launch
- Seiri does NOT appear in the Dock during normal operation
- Clicking the menubar icon opens a popover showing:
  - Last 5 activity entries
  - "Open Seiri" button to open the main window
  - "Pause / Resume" toggle
  - "Run Now" button to manually trigger organisation
- When Seiri is paused, the menubar icon changes to indicate paused state
- Seiri launches at login by default (user can disable in Settings)
- Menubar popover closes when user clicks outside it

---

### Feature 7: Onboarding Flow

**Description:**
A guided first-run experience that takes a new user from zero to their first automatic file move in under 3 minutes.

**User story:**
As a new user, I want to be guided through setup so I can get value immediately without reading documentation.

**Acceptance criteria:**

**Step 1 — Welcome screen**
- Shows product name, tagline, and a brief one-line description of what Seiri does
- Single CTA: "Get Started"

**Step 2 — Permissions**
- Explains why Full Disk Access is needed in plain English: "Seiri needs permission to read and move your files. We only access folders you choose to watch."
- Shows a step-by-step guide to granting Full Disk Access in System Settings
- Does not proceed until permission is granted
- If permission is denied, shows a clear message and a button to retry

**Step 3 — Choose folders to watch**
- Shows common Mac folders as options: Downloads, Desktop, Documents
- Downloads is pre-selected by default
- User can add custom folders via a folder picker dialog
- Minimum one folder must be selected to proceed

**Step 4 — Pre-built rules**
- Shows all pre-built rules (see Section 11) as a list
- All are disabled by default
- User can enable any rules with a single toggle
- Each rule shows a plain-English description of what it does and where files will go
- User can edit destination paths before proceeding

**Step 5 — Sim mode preview**
- Seiri runs sim mode against the selected folders with the enabled rules
- Shows a preview: "Here's what Seiri would have done in the last 30 days"
- If no files match, shows: "No matching files found yet. Seiri will start organising as new files arrive."
- User reviews and clicks "Looks good, start organising"

**Step 6 — Activation**
- Seiri activates and begins watching
- Brief celebration: "Seiri is running. Your files will be organised automatically."
- Transitions to main app window

---

### Feature 8: Settings

**Description:**
User preferences and configuration for Seiri's behaviour.

**Acceptance criteria:**
- Settings are accessible from the main window and menubar popover
- Settings are stored locally in `~/Library/Application Support/Seiri/settings.json`

**Settings options (v1):**

| Setting | Default | Description |
|---|---|---|
| Launch at login | Enabled | Start Seiri automatically when Mac starts |
| Show notifications | Enabled | macOS notification for each file moved |
| Notification style | Brief | Brief (auto-dismiss 3s) or Persistent |
| Pause all rules | Disabled | Temporarily stop all organisation |
| Watched folders | ~/Downloads | Folders Seiri monitors |
| File stability wait | 5 seconds | Time to wait before acting on a new file |
| Unsorted folder | ~/Downloads/_unsorted | Where unmatched files are surfaced (optional) |
| Activity log retention | 30 days | How long to keep activity history |
| Account | — | Email, plan type, manage subscription |

---

### Feature 9: Upgrade and Licensing

**Description:**
Free tier enforcement and upgrade flow via Lemon Squeezy.

**Acceptance criteria:**
- Free tier allows maximum 3 rules
- When free user attempts to create rule #4, an upgrade prompt is shown — not an error
- Upgrade prompt copy: "You've set up 3 rules — Seiri is already saving you time. Unlock unlimited rules to keep going."
- Upgrade prompt shows both pricing options: $4.99/month and $24.99 lifetime
- Clicking either option opens Lemon Squeezy checkout in the default browser
- After payment, Lemon Squeezy delivers a download link and activation email
- For returning users reinstalling Seiri, entering their purchase email in Settings restores their licence
- Subscription users who cancel revert to free tier (3 rules) — existing rules beyond 3 are disabled, not deleted
- Licence status is shown in Settings: Free / Monthly / Lifetime

---

## 10. User Flows

### Flow 1: First-time setup
```
Launch Seiri
  → Welcome screen
  → Grant Full Disk Access
  → Select watched folders (Downloads pre-selected)
  → Review and enable pre-built rules
  → Sim mode preview
  → Activate
  → Main window (active state)
```

### Flow 2: File arrives in watched folder
```
File appears in ~/Downloads
  → Tauri watcher detects file
  → File safety checks (see Section 13)
    → Not safe? → Leave file, log as "skipped - in progress"
    → Safe? → Continue
  → organize sidecar evaluates rules in priority order
    → Match found? → Execute action → Log to activity feed → Show notification
    → No match? → Leave file in place → No notification
```

### Flow 3: User creates a new rule
```
Click "Add Rule" in main window
  → Enter rule name
  → Add condition(s) using dropdown selectors
  → Add action(s) using dropdown selectors
  → System checks for conflicts with existing rules
    → Conflict found? → Show warning (non-blocking)
  → Click "Test this rule" (optional) → Sim mode for this rule
  → Click "Save"
  → Rule appears in rule list
  → Rule takes effect immediately
```

### Flow 4: User undoes a file move
```
Open activity feed (menubar or main window)
  → Find the entry to undo
  → Click "Undo"
  → Confirmation: "Move [filename] back to [original location]?"
  → Click "Confirm"
  → File moved back to original location
  → Activity feed entry updated: "Undone at [timestamp]"
```

### Flow 5: User upgrades from free to paid
```
Attempt to create 4th rule
  → Upgrade prompt appears
  → User selects plan ($4.99/month or $24.99 lifetime)
  → Lemon Squeezy checkout opens in browser
  → User completes payment
  → Lemon Squeezy delivers download/activation email
  → User returns to Seiri
  → Enters purchase email in Settings → Account
  → Licence verified
  → Unlimited rules unlocked
  → Continue creating rule
```

---

## 11. Pre-Built Rules

All pre-built rules ship with Seiri disabled by default. They serve as both quick-start options and examples of what rules can do.

| # | Rule Name | Conditions | Action | Default Destination |
|---|---|---|---|---|
| 1 | Move Disk Images | extension: `dmg`, `pkg` | move | `~/Applications/Installers/` |
| 2 | Move PDFs | extension: `pdf` | move | `~/Documents/PDFs/` |
| 3 | Move Archives | extension: `zip`, `tar`, `gz`, `rar`, `7z` | move | `~/Downloads/Archives/` |
| 4 | Move Screenshots | filename_startswith: `Screenshot` + extension: `png` | move | `~/Pictures/Screenshots/` |
| 5 | Move Images | mime_type: `image` (excludes screenshots) | move | `~/Pictures/Downloads/` |
| 6 | Move Videos | mime_type: `video` | move | `~/Movies/Downloads/` |
| 7 | Move Audio | mime_type: `audio` | move | `~/Music/Downloads/` |
| 8 | Clean Partial Downloads | extension: `crdownload`, `part`, `download` | trash | macOS Trash |
| 9 | Move Invoices and Receipts | extension: `pdf` + filename_contains: `invoice` OR `receipt` OR `order` | move | `~/Documents/Finance/` |
| 10 | Archive Old Files | date_added_older_than: `30 days` | move | `~/Downloads/Archive/` |

**Important:** Pre-built rules 1-10 are evaluated in the order listed. Rule 9 (Invoices) is intentionally placed before Rule 2 (PDFs) in priority so that invoice PDFs go to Finance, not the generic PDFs folder. When users reorder rules, this logic may need to be communicated.

---

## 12. Pricing and Licensing

### Tiers

| Tier | Price | Rules | Features |
|---|---|---|---|
| Free | $0 | 3 rules max | All core features, pre-built rules count toward limit |
| Monthly | $4.99/month | Unlimited | All features, priority support |
| Lifetime | $24.99 one-time | Unlimited | All features, all future v1.x updates |

### Payment processor
**Lemon Squeezy** handles all payment, delivery, and subscription management.

### Licence enforcement
- Enforcement is UI-level only in v1
- Free tier limit (3 rules) is enforced in the React frontend
- No server-side enforcement in v1
- Licence verification is performed once at activation by matching purchase email against Lemon Squeezy order records
- After activation, the app stores licence status locally — no repeated server calls required
- Subscription cancellation is detected on next app launch (once per day licence check)

### Subscription cancellation behaviour
- User cancels subscription via Lemon Squeezy customer portal
- On next daily licence check, app detects cancelled subscription
- Rules beyond 3 are disabled (not deleted)
- User is shown a non-intrusive notification: "Your Seiri subscription has ended. Rules 4 and above have been paused. Resubscribe to reactivate them."

---

## 13. File Safety Model

This section defines the rules that govern when Seiri is and is not permitted to act on a file.

### The fundamental rule
> A file is only moved when TWO independent conditions are both true:
> 1. It matches a configured rule
> 2. It is verified safe to move
>
> Either condition failing means the file is left exactly where it is.

### Safety checklist
Before any file action, ALL of the following must be true:

| Check | Description |
|---|---|
| File extension is not a partial download extension | Not `.crdownload`, `.part`, `.download`, `.tmp` |
| File is not hidden | Filename does not start with `.` |
| File size has been stable for at least N seconds | Two size checks separated by the configured stability wait (default: 5s) confirm no change |
| File is not locked by another process | macOS file lock check passes |
| Destination folder exists or can be created | Write permission check on destination |
| File still exists at source | Re-confirmed immediately before move (race condition protection) |

### Failure behaviour
If any safety check fails:
- File is left in its current location
- No notification is shown to the user
- Event is logged in activity feed as: "Skipped — [reason]"
- Seiri will re-evaluate the file on the next watcher event

### Destination conflict behaviour
If a file with the same name already exists at the destination:
- Seiri does NOT overwrite
- Default behaviour: append timestamp to filename — `invoice.pdf` becomes `invoice_20260505_143022.pdf`
- User can configure conflict behaviour in Settings: `rename` (default) | `skip` | `ask`

### Never delete without explicit action
Seiri will never permanently delete a file unless the user has explicitly configured a `delete_permanently` action for a rule AND confirmed the deletion dialog. All other removal actions use macOS Trash.

---

## 14. Error Handling

### Principle
Seiri fails silently and safely. Errors are logged but never interrupt the user unless action is required.

| Error Scenario | Behaviour |
|---|---|
| Watched folder no longer exists | Log warning, skip folder, show yellow indicator in Settings |
| Destination folder cannot be created | Skip action, log error with reason |
| File move fails (permissions) | Skip action, log error, show in activity feed with "Failed" badge |
| organize sidecar crashes | Log crash, attempt restart up to 3 times, if still failing show menubar error indicator |
| organize sidecar not found | Show critical error on launch: "Seiri engine missing. Please reinstall." |
| Lemon Squeezy unreachable during licence check | Use last known licence status, retry next launch |
| Sim mode takes longer than 30 seconds | Cancel sim, show: "Folder contains too many files to preview quickly. Seiri will organise files as they arrive." |
| Undo fails (file already moved again) | Show: "Could not undo. The file may have been moved or renamed since." |
| macOS Full Disk Access revoked | Show persistent menubar warning, pause all rules, prompt user to re-grant |

---

## 15. macOS Permissions

Seiri requires the following macOS permissions. All permission requests must be explained to the user in plain English before the system dialog appears.

| Permission | Why it is needed | When requested |
|---|---|---|
| Full Disk Access | To read files in watched folders and move them to destination folders | Onboarding Step 2 |
| Login Items | To start Seiri automatically when the Mac starts | Onboarding Step 6 (post-activation) |
| Notifications | To show brief notifications when files are moved | First time a file is moved |

### Permissions Seiri does NOT request
- Microphone
- Camera
- Location
- Contacts
- Calendar
- Network (beyond update checks and licence verification)

This list should be surfaced on the Seiri website and in the app's privacy section to build user trust.

---

## 16. Distribution and Updates

### Build requirements
- Apple Developer Account ($99/year) — required for code signing and notarization
- App must be signed with a Developer ID certificate
- App must be notarized via Apple's notarization service before distribution
- Tauri build pipeline handles signing and notarization with correct configuration

### Distribution format
- Primary: Signed `.dmg` file containing `Seiri.app`
- Hosted on: `seiri.app/download` (direct link)
- Also hosted on: GitHub Releases (as backup mirror)

### Auto-update
- Tauri Updater checks for updates on app launch and once every 24 hours
- Updates are downloaded silently in the background
- User is notified via menubar: "Seiri updated to v1.0.x" after update is applied
- User is never prompted to manually download an update
- Update server: GitHub Releases JSON endpoint

### Version naming
- v1.0.0 — initial launch
- v1.0.x — bug fixes and stability improvements
- v1.x.0 — new features within v1 scope
- v2.0.0 — major version (engine rewrite, Windows support, etc.)

---

## 17. Success Metrics — v1

### Primary metric
- **Time to first automatic file move:** Median under 3 minutes from first launch

### Acquisition metrics
- 500 downloads within 30 days of launch
- 100 paying users within 60 days of launch
- Product Hunt launch — top 5 product of the day

### Engagement metrics
- 70% of users who complete onboarding have at least one rule active after 7 days
- 50% of free users have enabled at least one pre-built rule
- Less than 5% of users disable Seiri within the first week

### Trust metrics
- Zero reported cases of files moved incorrectly due to Seiri error (vs user misconfiguration)
- Less than 1% of file moves result in an undo action

### Conversion metrics
- 10% of free users upgrade to paid within 30 days
- Monthly to lifetime upgrade rate: tracked but no target for v1

### Support metrics
- Average response time to support email: under 24 hours
- Most common support request category: used to identify v1.1 priorities

---

## Appendix A: Glossary

| Term | Definition |
|---|---|
| Rule | A user-defined combination of conditions and actions that Seiri evaluates against files |
| Condition | A criteria a file must meet for a rule to match (extension, filename, size, etc.) |
| Action | What Seiri does when a rule matches (move, rename, trash, etc.) |
| Sim mode | A preview mode that shows what Seiri would do without touching any files |
| Sidecar | A companion binary (organize) that Tauri manages alongside the main app |
| Activity feed | The log of all actions Seiri has taken |
| Stability check | The process of verifying a file is fully written before acting on it |
| Watched folder | A folder Seiri monitors for new files |
| Unsorted folder | A designated folder where files with no matching rule can be surfaced |
| Partial download | A file currently being downloaded, identified by extension or file lock |

---

## Appendix B: Key Constraints Summary

For AI coding agents — the following constraints are non-negotiable:

1. Never move a file that fails any safety check
2. Never overwrite a file at the destination — always rename on conflict
3. Never permanently delete without explicit user confirmation
4. Free tier maximum is 3 rules — enforced in UI, not backend
5. organize sidecar is the sole file action executor — Tauri/React never directly move files
6. All file actions are logged — no silent operations
7. Undo must be available for 24 hours after any move action
8. App must function fully offline after licence activation
9. No user data leaves the device except: update version check, licence verification
10. Rules are evaluated strictly in priority order — first match wins, no exceptions
