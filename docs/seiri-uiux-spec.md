# Seiri — UI/UX Specification
**Version:** 1.0  
**Status:** Draft  
**Last Updated:** May 2026  
**Author:** Sharath Aradhyamath  
**Document Type:** UI/UX Specification (How it looks and behaves)  

> This document defines the visual design, component library, screen layouts,
> interaction patterns, and motion behaviour for Seiri.
> For what each feature does, refer to the Seiri PRD.
> For build order, refer to the Seiri Implementation Plan.

---

## Table of Contents

1. [Design Philosophy](#1-design-philosophy)
2. [Design Tokens](#2-design-tokens)
3. [Typography](#3-typography)
4. [Spacing and Layout](#4-spacing-and-layout)
5. [Component Library](#5-component-library)
6. [App Shell](#6-app-shell)
7. [Onboarding Screens](#7-onboarding-screens)
8. [Rules Screen](#8-rules-screen)
9. [Rule Builder Drawer](#9-rule-builder-drawer)
10. [Activity Screen](#10-activity-screen)
11. [Settings Screen](#11-settings-screen)
12. [Menubar Popover](#12-menubar-popover)
13. [Upgrade Modal](#13-upgrade-modal)
14. [States](#14-states)
15. [Motion and Animation](#15-motion-and-animation)
16. [Accessibility](#16-accessibility)
17. [Tailwind Configuration](#17-tailwind-configuration)

---

## 1. Design Philosophy

### Inspiration
Seiri takes visual inspiration from **Raycast's preferences window** — not the command palette. The reference point is a dark, sidebar-driven management interface that feels native to macOS, premium, and quietly confident.

### Four design principles

**1. Calm confidence**
Seiri runs in the background and handles your files. The UI should feel like a control room — composed, in control, never anxious. No flashy gradients, no marketing-speak in the interface, no unnecessary motion. Every element earns its place.

**2. Information density without clutter**
Power users need to see a lot at once. Rules, conditions, activity entries — these are information-dense. Seiri achieves density through tight but consistent spacing, clear typographic hierarchy, and restrained use of colour — not by cramming elements together.

**3. Native macOS feel**
Seiri is a Mac app. It should feel like it belongs on macOS. This means: SF Pro system font, native macOS colours for backgrounds, proper use of vibrancy/translucency where appropriate, and interaction patterns that match macOS conventions. Users should never feel like they're using a web app in a wrapper.

**4. Trust through transparency**
Every action Seiri takes is visible. The UI communicates status clearly — active, paused, running, error — without requiring the user to dig. Colour and iconography are used consistently so users build a mental model quickly.

---

## 2. Design Tokens

### Colour palette

All colours are defined as CSS custom properties and referenced through Tailwind config. Never hardcode hex values in components.

#### Background colours
```css
--color-bg-base:        #111113;  /* Main app background — deepest layer */
--color-bg-elevated:    #1C1C1E;  /* Sidebar, panels — one step up */
--color-bg-surface:     #242426;  /* Cards, list items — two steps up */
--color-bg-overlay:     #2C2C2E;  /* Hover states, selected items */
--color-bg-input:       #1C1C1E;  /* Input fields */
--color-bg-tooltip:     #3A3A3C;  /* Tooltips */
```

#### Text colours
```css
--color-text-primary:   #F5F5F7;  /* Primary text — headings, labels */
--color-text-secondary: #98989D;  /* Secondary text — descriptions, meta */
--color-text-tertiary:  #636366;  /* Tertiary text — placeholders, disabled */
--color-text-inverse:   #111113;  /* Text on light backgrounds */
```

#### Border colours
```css
--color-border-subtle:  #2C2C2E;  /* Subtle dividers, card borders */
--color-border-default: #3A3A3C;  /* Default borders */
--color-border-strong:  #48484A;  /* Focused input borders */
```

#### Accent — Seiri Blue
```css
--color-accent-50:      #f0f9ff;
--color-accent-100:     #e0f2fe;
--color-accent-200:     #bae6fd;
--color-accent-400:     #38bdf8;
--color-accent-500:     #0ea5e9;  /* Primary accent */
--color-accent-600:     #0284c7;  /* Hover state */
--color-accent-700:     #0369a1;  /* Active/pressed state */
--color-accent-900:     #0c4a6e;  /* Subtle accent backgrounds */
```

#### Semantic colours
```css
--color-success:        #34C759;  /* macOS system green */
--color-success-bg:     #1A3A22;  /* Success background tint */
--color-warning:        #FF9F0A;  /* macOS system amber */
--color-warning-bg:     #3A2A10;  /* Warning background tint */
--color-danger:         #FF453A;  /* macOS system red */
--color-danger-bg:      #3A1A18;  /* Danger background tint */
--color-neutral:        #636366;  /* Neutral/inactive */
```

#### Status indicator colours
```css
--color-status-live:    #34C759;  /* Active and running */
--color-status-paused:  #FF9F0A;  /* Paused by user */
--color-status-error:   #FF453A;  /* Error state */
--color-status-idle:    #636366;  /* No activity yet */
```

---

## 3. Typography

### Font stack
```css
font-family: -apple-system, "SF Pro Display", "SF Pro Text",
             BlinkMacSystemFont, "Helvetica Neue", sans-serif;

font-family-mono: "SF Mono", "Fira Code", "Fira Mono",
                  "Roboto Mono", monospace;
```

SF Pro is the native macOS font and requires no import — it is available on all Macs. Never import Google Fonts for Seiri. The native font stack is the right choice.

### Type scale

| Token | Size | Weight | Line Height | Use case |
|---|---|---|---|---|
| `text-2xl` | 24px | 600 semibold | 1.3 | Page titles (rare) |
| `text-xl` | 20px | 600 semibold | 1.3 | Section headers |
| `text-lg` | 17px | 600 semibold | 1.4 | Card titles, rule names |
| `text-base` | 14px | 400 regular | 1.5 | Body text, descriptions |
| `text-sm` | 13px | 400 regular | 1.5 | Secondary info, meta |
| `text-xs` | 11px | 400 regular | 1.4 | Captions, timestamps, badges |
| `text-mono` | 13px | 400 regular | 1.5 | File paths, code, config values |

### Letter spacing
- Headings: `-0.01em` (slight tightening for SF Pro at large sizes)
- Body: `0` (default)
- Uppercase labels: `0.06em` (section labels like "CONDITIONS", "ACTIONS")
- Monospace paths: `0` (default)

### Text rendering
```css
-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;
text-rendering: optimizeLegibility;
```

Apply globally. macOS benefits significantly from antialiasing.

---

## 4. Spacing and Layout

### Base unit
**4px** — all spacing values are multiples of 4px.

### Spacing scale
```
4px   — xs  — tight internal spacing (icon to label, badge padding)
8px   — sm  — compact spacing (list item padding, input padding)
12px  — md  — standard spacing (card padding, gap between elements)
16px  — lg  — comfortable spacing (section gaps)
20px  — xl  — generous spacing (major section separation)
24px  — 2xl — section headers to content
32px  — 3xl — page-level padding
```

### App window dimensions
```
Window width:   900px (min: 800px)
Window height:  600px (min: 500px)
Sidebar width:  220px (fixed, not resizable in v1)
Content area:   fills remaining width
```

### Sidebar layout
```
Sidebar padding:        12px horizontal, 16px vertical
Nav item height:        36px
Nav item padding:       8px horizontal, 0 vertical
Nav item border-radius: 8px
Nav icon size:          16px
Nav icon → label gap:   10px
Section gap:            4px between nav items
```

### Content area layout
```
Content padding:        32px horizontal, 28px vertical
Section header margin:  0 0 20px 0
Card border-radius:     10px
Card padding:           16px
List item height:       56px (rule items), 64px (activity items)
List item padding:      16px horizontal, 12px vertical
```

### Border radius scale
```
4px  — badges, tags, small chips
6px  — buttons (sm), input fields
8px  — buttons (default), nav items, tooltips
10px — cards, list items, drawers
12px — modals, popovers
16px — large modals, onboarding cards
```

---

## 5. Component Library

### 5.1 Button

**Variants:**

**Primary** — used for main CTAs
```
Background:       --color-accent-500
Text:             white
Height:           36px
Padding:          0 16px
Border-radius:    8px
Font:             14px, 500 medium
Hover:            --color-accent-600
Active/pressed:   --color-accent-700 + scale(0.98)
Disabled:         opacity 0.4, cursor not-allowed
Focus ring:       2px solid --color-accent-500, 2px offset
```

**Secondary** — used for alternative actions
```
Background:       --color-bg-overlay
Text:             --color-text-primary
Border:           1px solid --color-border-default
Height:           36px
Padding:          0 16px
Border-radius:    8px
Hover:            --color-bg-overlay + border --color-border-strong
Active:           scale(0.98)
```

**Ghost** — used for tertiary actions, inline actions
```
Background:       transparent
Text:             --color-text-secondary
Height:           32px
Padding:          0 12px
Border-radius:    6px
Hover:            background --color-bg-surface, text --color-text-primary
```

**Danger** — used for destructive actions
```
Background:       --color-danger-bg
Text:             --color-danger
Border:           1px solid rgba(255, 69, 58, 0.3)
Height:           36px
Padding:          0 16px
Border-radius:    8px
Hover:            background rgba(255, 69, 58, 0.2)
```

**Icon button** — used for compact icon-only actions
```
Background:       transparent
Size:             28px × 28px
Border-radius:    6px
Icon size:        14px
Icon colour:      --color-text-tertiary
Hover:            background --color-bg-surface, icon --color-text-secondary
```

**Size variants:**
- `sm`: height 28px, padding 0 10px, font 12px
- `md`: height 36px, padding 0 16px, font 14px (default)
- `lg`: height 40px, padding 0 20px, font 15px (onboarding CTAs only)

---

### 5.2 Input field

```
Background:       --color-bg-input
Text:             --color-text-primary
Placeholder:      --color-text-tertiary
Border:           1px solid --color-border-subtle
Border-radius:    8px
Height:           36px
Padding:          0 12px
Font:             14px regular

Focus:
  Border:         1px solid --color-accent-500
  Box-shadow:     0 0 0 3px rgba(14, 165, 233, 0.15)
  Outline:        none

Error:
  Border:         1px solid --color-danger
  Box-shadow:     0 0 0 3px rgba(255, 69, 58, 0.15)

Disabled:
  Opacity:        0.5
  Cursor:         not-allowed
```

**With icon (left):**
- Icon positioned absolutely, left 10px, vertically centred
- Input padding-left: 36px

**With action (right):**
- Button or icon positioned right 8px, vertically centred
- Input padding-right: 36px

---

### 5.3 Toggle (Switch)

Matches macOS native toggle aesthetic:
```
Track width:      36px
Track height:     20px
Track border-radius: 10px
Knob size:        16px
Knob border-radius: 50%

Off state:
  Track:          --color-bg-overlay
  Knob:           --color-text-tertiary
  Knob position:  left: 2px

On state:
  Track:          --color-accent-500
  Knob:           white
  Knob position:  left: 18px

Transition:       all 150ms ease

Disabled:
  Opacity:        0.4
  Cursor:         not-allowed
```

---

### 5.4 Badge / Tag

Used for extensions, mime types, status labels:
```
Height:           20px
Padding:          0 8px
Border-radius:    4px
Font:             11px, 500 medium
Letter-spacing:   0.02em

Variants:
  default:  bg --color-bg-overlay, text --color-text-secondary
  blue:     bg --color-accent-900, text --color-accent-400
  green:    bg --color-success-bg, text --color-success
  amber:    bg --color-warning-bg, text --color-warning
  red:      bg --color-danger-bg, text --color-danger
```

---

### 5.5 Status indicator dot

Used in menubar popover and sidebar:
```
Size:             8px × 8px
Border-radius:    50%

live:    background --color-status-live
         box-shadow: 0 0 0 2px rgba(52, 199, 89, 0.25)
         animation: pulse 2s ease-in-out infinite

paused:  background --color-status-paused

error:   background --color-status-error
         animation: pulse 1s ease-in-out infinite

idle:    background --color-status-idle
```

**Pulse animation:**
```css
@keyframes pulse {
  0%, 100% { opacity: 1; }
  50%       { opacity: 0.5; }
}
```

---

### 5.6 Dropdown / Select

```
Background:       --color-bg-input
Border:           1px solid --color-border-subtle
Border-radius:    8px
Height:           36px
Padding:          0 32px 0 12px
Chevron icon:     right 10px, --color-text-tertiary

Menu (open):
  Background:     --color-bg-elevated
  Border:         1px solid --color-border-default
  Border-radius:  10px
  Box-shadow:     0 8px 32px rgba(0, 0, 0, 0.4)
  Padding:        4px

Menu item:
  Height:         32px
  Padding:        0 12px
  Border-radius:  6px
  Font:           14px
  Hover:          background --color-bg-overlay
  Selected:       background --color-accent-900, text --color-accent-400
```

---

### 5.7 Card

```
Background:       --color-bg-surface
Border:           1px solid --color-border-subtle
Border-radius:    10px
Padding:          16px

Hover (interactive cards):
  Border:         1px solid --color-border-default
  Background:     --color-bg-overlay
  Transition:     all 120ms ease
```

---

### 5.8 Divider

```
Height:           1px
Background:       --color-border-subtle
Margin:           0 (applied by parent spacing)
```

---

### 5.9 Tooltip

```
Background:       --color-bg-tooltip
Text:             --color-text-primary
Font:             12px regular
Padding:          6px 10px
Border-radius:    8px
Box-shadow:       0 4px 16px rgba(0, 0, 0, 0.3)
Max-width:        240px
Delay:            400ms before show
Animation:        fade in 100ms ease
```

---

### 5.10 Dialog / Modal

```
Backdrop:         rgba(0, 0, 0, 0.6) with backdrop-blur: 4px
Container:        background --color-bg-elevated
                  border: 1px solid --color-border-default
                  border-radius: 16px
                  padding: 24px
                  max-width: 440px
                  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5)

Title:            17px, 600 semibold, --color-text-primary
Body:             14px, regular, --color-text-secondary
                  margin-top: 8px

Actions:          margin-top: 20px
                  display: flex, gap: 8px, justify: flex-end

Animation:
  In:             scale(0.95) → scale(1), opacity 0 → 1, 150ms ease-out
  Out:            scale(1) → scale(0.95), opacity 1 → 0, 100ms ease-in
```

---

### 5.11 Drawer (side panel)

Used for Rule Builder:
```
Width:            420px
Position:         fixed right 0, full height
Background:       --color-bg-elevated
Border-left:      1px solid --color-border-subtle
Box-shadow:       -16px 0 48px rgba(0, 0, 0, 0.3)

Animation:
  In:             translateX(420px) → translateX(0), 200ms ease-out
  Out:            translateX(0) → translateX(420px), 160ms ease-in

Header:
  Height:         56px
  Padding:        0 20px
  Border-bottom:  1px solid --color-border-subtle
  Title:          17px, 600 semibold
  Close button:   icon button, top-right

Content:
  Padding:        24px 20px
  Overflow-y:     auto

Footer:
  Height:         64px
  Padding:        0 20px
  Border-top:     1px solid --color-border-subtle
  Display:        flex, justify: space-between, align: center
```

---

### 5.12 Progress bar

Used in onboarding sim mode:
```
Track:
  Height:         4px
  Background:     --color-bg-overlay
  Border-radius:  2px

Fill:
  Background:     --color-accent-500
  Border-radius:  2px
  Transition:     width 300ms ease

Animated (indeterminate):
  Animation:      shimmer left-to-right, 1.5s ease-in-out infinite
```

---

### 5.13 Empty state

```
Container:
  Display:        flex, flex-direction: column, align-items: center
  Padding:        64px 32px
  Text-align:     center

Icon:
  Size:           40px × 40px
  Color:          --color-text-tertiary
  Margin-bottom:  16px

Title:
  Font:           17px, 500 medium, --color-text-secondary
  Margin-bottom:  8px

Description:
  Font:           14px, regular, --color-text-tertiary
  Max-width:      280px
  Line-height:    1.5
  Margin-bottom:  20px

CTA (optional):
  Primary button
```

---

### 5.14 Notification toast

Appears bottom-right of main window after file actions:
```
Container:
  Background:     --color-bg-elevated
  Border:         1px solid --color-border-default
  Border-radius:  10px
  Padding:        12px 16px
  Box-shadow:     0 8px 24px rgba(0, 0, 0, 0.3)
  Max-width:      320px
  Min-width:      240px

Icon:             16px, status colour
Filename:         14px, 500 medium, --color-text-primary, truncated
Action text:      13px, --color-text-secondary

Animation:
  In:             translateY(16px) → translateY(0), opacity 0 → 1, 200ms ease-out
  Out:            translateY(0) → translateY(16px), opacity 1 → 0, 150ms ease-in
  Auto-dismiss:   3 seconds (brief mode)
```

---

## 6. App Shell

### Overall layout

```
┌──────────────────────────────────────────────────────────────┐
│  Title bar (macOS native, transparent)              28px     │
├────────────────┬─────────────────────────────────────────────┤
│                │                                             │
│    Sidebar     │           Content Area                      │
│    220px       │           fills remaining width             │
│                │                                             │
│  bg-elevated   │  bg-base                                    │
│                │                                             │
│                │                                             │
│                │                                             │
│                │                                             │
│                │                                             │
├────────────────┴─────────────────────────────────────────────┤
│  (no status bar in v1)                                       │
└──────────────────────────────────────────────────────────────┘
```

### Sidebar anatomy

```
┌──────────────────────┐
│                      │  ← 16px top padding
│  🌀  Seiri      ⏸   │  ← App header: logo + pause button
│                      │  ← 16px gap
│  ─────────────────   │  ← Divider
│                      │  ← 12px gap
│  📋  Rules           │  ← Nav item (active: bg-overlay, accent left border)
│  📜  Activity        │  ← Nav item
│  ⚙️   Settings       │  ← Nav item
│                      │
│                      │
│                      │
│  ─────────────────   │  ← Divider (pushed to bottom)
│  ● Live              │  ← Status indicator + label
│  Free · Upgrade ↗   │  ← Plan badge + upgrade link
│                      │  ← 16px bottom padding
└──────────────────────┘
```

**Sidebar header:**
- App name: 15px, 600 semibold, --color-text-primary
- Seiri icon: 20px × 20px, blue gradient mark
- Pause button: icon button (pause icon when live, play icon when paused)
- Pause button tooltip: "Pause Seiri" / "Resume Seiri"

**Nav items:**
```
Height:           36px
Padding:          0 12px
Border-radius:    8px
Gap (icon→text):  10px
Icon size:        16px
Font:             14px, 400 regular

Default:
  Background:     transparent
  Icon:           --color-text-tertiary
  Text:           --color-text-secondary

Hover:
  Background:     rgba(255, 255, 255, 0.05)
  Icon:           --color-text-secondary
  Text:           --color-text-primary

Active (current route):
  Background:     --color-bg-overlay
  Icon:           --color-accent-500
  Text:           --color-text-primary
  Left border:    2px solid --color-accent-500
  Border-radius:  0 8px 8px 0 (for left border effect)
```

**Status row (bottom):**
```
Status dot:       8px, colour per status
Status label:     12px, --color-text-secondary, "Live" / "Paused" / "Error"
Plan badge:       11px, --color-text-tertiary
Upgrade link:     11px, --color-accent-400, hover underline
```

### Content area

```
Background:       --color-bg-base
Padding:          28px 32px
Overflow-y:       auto

Page header:
  Margin-bottom:  24px

  Title:          20px, 600 semibold, --color-text-primary
  Subtitle:       13px, --color-text-secondary, margin-top: 4px
  Actions:        flex, gap: 8px, ml: auto (right-aligned)
```

### Window behaviour
- macOS title bar: `transparent` — window buttons (traffic lights) float over sidebar
- Sidebar top padding: 40px to clear traffic lights
- Window is not full-screen by default
- Minimum size enforced: 800 × 500px

---

## 7. Onboarding Screens

Onboarding uses a **full-window layout** — sidebar is hidden during onboarding. The entire window is used for the onboarding flow.

### Onboarding shell

```
┌──────────────────────────────────────────────────────────────┐
│                                                              │
│              Step indicator (top centre)                     │
│              ● ○ ○ ○ ○ ○                                    │
│                                                              │
│                                                              │
│                  Content (centred)                           │
│                  Max-width: 480px                            │
│                                                              │
│                                                              │
│                                                              │
│              [Back]              [Continue →]                │
│              Navigation (bottom)                             │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Step indicator:**
```
Dots:             8px circles, gap: 6px
Active:           --color-accent-500, scale(1.2)
Completed:        --color-accent-700
Upcoming:         --color-bg-overlay
Transition:       all 200ms ease
```

**Navigation bar:**
```
Position:         fixed bottom 0, full width
Height:           64px
Padding:          0 32px
Background:       --color-bg-base with border-top: 1px solid --color-border-subtle
Display:          flex, justify: space-between, align: center

Back button:      Ghost variant, hidden on Step 1
Continue button:  Primary variant, lg size
```

---

### Step 1 — Welcome

```
Layout:           Full centre, vertical flex, gap: 24px

Logo mark:        48px × 48px, Seiri blue gradient circle
                  with subtle inner glow: box-shadow 0 0 32px rgba(14,165,233,0.3)

Title:            "Your files. Always where        "
                  "you expect them."
                  24px, 600 semibold, --color-text-primary
                  Line-height: 1.25, text-align: centre

Description:      "Seiri watches your folders and automatically
                   moves files to where they belong — silently,
                   reliably, and always correctly."
                  15px, --color-text-secondary, text-align: centre
                  Max-width: 380px, line-height: 1.6

CTA:              Primary button lg: "Get Started →"
                  Margin-top: 8px

Footer note:      "Takes less than 3 minutes"
                  12px, --color-text-tertiary, text-align: centre
```

---

### Step 2 — Permissions

```
Icon:             Lock icon (lucide), 32px, --color-warning

Title:            "One permission needed"
                  20px, 600 semibold

Description:      14px, --color-text-secondary

Steps list:       Numbered 1-4, each step on its own row
                  Number: 20px circle, --color-bg-overlay, 12px font
                  Text: 14px, --color-text-secondary
                  Gap: 12px between steps

CTA button:       Primary: "Open System Settings →"

Status indicator (below button):
  Waiting:        Spinner icon (animated) + "Waiting for permission..."
                  13px, --color-text-tertiary
  Granted:        Check circle icon (green) + "Permission granted"
                  13px, --color-success
  Transition:     fade + slide, 200ms ease

Continue button:  Disabled until permission granted
                  Appears (fades in) after permission detected
```

---

### Step 3 — Choose folders

```
Title:            "Which folders should Seiri watch?"

Folder options:   Vertical list, gap: 8px

Folder item:
  ┌────────────────────────────────────────┐
  │  [checkbox]  📁 Downloads              │
  │              ~/Downloads               │
  └────────────────────────────────────────┘

  Background:     --color-bg-surface
  Border:         1px solid --color-border-subtle
  Border-radius:  10px
  Padding:        14px 16px
  Height:         56px

  Checked state:
    Border:       1px solid --color-accent-500
    Background:   rgba(14, 165, 233, 0.05)
    Checkbox:     filled blue square with checkmark

  Folder name:    14px, 500 medium, --color-text-primary
  Folder path:    12px, --color-text-tertiary, font-mono

Add custom folder:
  Ghost button with + icon: "Add custom folder"
  Opens native macOS folder picker
  Custom folders show with × remove button
```

---

### Step 4 — Pre-built rules

```
Title:            "Quick start with suggested rules"
Subtitle:         "All disabled by default. Enable what you want."
                  13px, --color-text-secondary

Rule list:        Vertical list, gap: 6px

Rule item:
  ┌────────────────────────────────────────────────────┐
  │  [toggle]  Move Disk Images              [●──────] │
  │            .dmg, .pkg → Applications/Installers/   │
  │            Destination: [~/Applications/Installers/]│
  └────────────────────────────────────────────────────┘

  Background:     --color-bg-surface
  Border:         1px solid --color-border-subtle
  Border-radius:  10px
  Padding:        12px 16px

  Enabled state:
    Border:       1px solid --color-border-default
    Background:   --color-bg-overlay

  Rule name:      14px, 500 medium, --color-text-primary
  Rule summary:   12px, --color-text-secondary
  Destination:    Inline editable text field (12px, mono)
                  Pencil icon appears on hover to signal editability

"Enable All" button: Ghost, top right of list header
```

---

### Step 5 — Sim mode preview

```
Title:            "Here's what Seiri would have done"
Subtitle:         "Nothing has been moved. This is a preview only."
                  12px, --color-warning (amber, to emphasise no action taken)

Progress state:
  Progress bar:   Full width, animated shimmer
  Label:          "Scanning your Downloads folder..."
                  13px, --color-text-tertiary

Results state:
  Results list:   Vertical, scrollable, max-height: 280px

  Result item:
    ┌──────────────────────────────────────────────────┐
    │  ✓  invoice_march.pdf                            │
    │     ~/Downloads → ~/Documents/Finance/           │
    │     Matched: Move Invoices and Receipts          │
    └──────────────────────────────────────────────────┘

    Check icon:   14px, --color-success
    Filename:     14px, 500 medium, --color-text-primary, truncated
    Path arrow:   12px, --color-text-tertiary, mono font
    Rule name:    11px, --color-accent-400, badge style

  Summary:        "5 files would be organised"
                  13px, --color-text-secondary, margin-top: 16px

Empty state:      "No matching files found yet. Seiri will start
                   organising files as they arrive."
                  Centred, --color-text-tertiary

Activate button:  "Looks good, activate →"
```

---

### Step 6 — Activation

```
Animation:        Check mark draws itself (SVG path animation, 600ms)
                  Then content fades in below, staggered 100ms each

Check mark:       64px circle, --color-accent-500 background
                  White check mark inside
                  Subtle glow: box-shadow 0 0 32px rgba(14,165,233,0.4)

Title:            "Seiri is running"
                  20px, 600 semibold

Description:      "Your files will be organised automatically
                   from now on."
                  14px, --color-text-secondary

Menubar hint:     Small illustration or icon showing menubar
                  "Seiri lives in your menu bar"
                  12px, --color-text-tertiary

CTA:              "Go to Dashboard →"
                  Primary button lg
```

---

## 8. Rules Screen

### Layout

```
┌─ Content Area ───────────────────────────────────────────────┐
│                                                              │
│  Rules                                      [+ Add Rule]    │
│  3 active · 2 paused             13px secondary             │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Rule list (see below)                               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ℹ️  Rules run top to bottom. First match wins.              │
│     13px, --color-text-tertiary, italic                     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Rule list item

```
┌──────────────────────────────────────────────────────────────┐
│  ⠿   1   Move Disk Images                        [●──────]  │
│          .dmg, .pkg → ~/Applications/Installers/            │
│                                              [Edit] [⋯]     │
└──────────────────────────────────────────────────────────────┘
```

**Dimensions:**
```
Height:           64px
Padding:          0 16px
Border-radius:    10px
Background:       --color-bg-surface
Border:           1px solid --color-border-subtle
Margin-bottom:    6px
```

**Drag handle (⠿):**
```
Width:            20px
Color:            --color-text-tertiary (darker when not hovered)
Hover:            --color-text-secondary
Cursor:           grab
Active drag:      cursor grabbing
```

**Priority number:**
```
Width:            24px
Font:             13px, 500 medium, --color-text-tertiary
Text-align:       centre
```

**Rule name:**
```
Font:             15px, 500 medium, --color-text-primary
Max-width:        calc(100% - 200px)
Overflow:         truncate with ellipsis
```

**Rule summary (below name):**
```
Font:             12px, --color-text-secondary, font-mono
Max-width:        same as name
Overflow:         truncate
Margin-top:       2px
```

**Toggle:**
```
Position:         right side, vertically centred
Uses:             Toggle component (5.3)
```

**Action buttons:**
```
Visibility:       hidden by default, shown on row hover
Position:         right of toggle, gap: 4px

Edit:             Ghost button sm: "Edit"
More (⋯):        Icon button — opens dropdown with:
                  - Duplicate
                  - Delete (red text)
```

**States:**

Enabled:
```
Background:       --color-bg-surface
Border:           1px solid --color-border-subtle
Rule name:        --color-text-primary
```

Disabled:
```
Background:       --color-bg-surface
Border:           1px solid --color-border-subtle
Rule name:        --color-text-tertiary (dimmed)
Summary:          --color-text-tertiary (dimmed)
Toggle:           off state
```

Dragging:
```
Background:       --color-bg-overlay
Border:           1px solid --color-accent-500
Box-shadow:       0 8px 24px rgba(0,0,0,0.3)
Scale:            1.01
Transition:       box-shadow 150ms ease
Other items:      shift to indicate new position with gap animation
```

Locked (free tier, rule #4+):
```
Overlay:          Lock icon (16px) replaces toggle
                  Positioned right, --color-text-tertiary
Rule name:        --color-text-tertiary
Cursor:           default (no drag)
Hover:            shows tooltip "Upgrade to activate unlimited rules"
```

---

## 9. Rule Builder Drawer

### Header
```
Title (new):      "New Rule"
Title (edit):     Rule name (editable inline — click to edit)
Close button:     Icon button (X), top right
```

### Form sections

**Rule name field:**
```
Full-width input at top of drawer
Font:             17px, 500 medium (larger than standard input)
Placeholder:      "Give this rule a name..."
Border:           none (underline style only)
Border-bottom:    1px solid --color-border-subtle
Border-radius:    0
Padding:          0 0 12px 0
Margin-bottom:    24px

Focus:
  Border-bottom:  1px solid --color-accent-500
```

**Section labels (CONDITIONS, ACTIONS):**
```
Font:             11px, 600, letter-spacing: 0.06em
Color:            --color-text-tertiary
Text-transform:   uppercase
Margin-bottom:    10px
```

**Condition row:**
```
Display:          flex, gap: 8px, align: centre
Margin-bottom:    8px

Type dropdown:    flex: 0 0 160px
Value input:      flex: 1
Remove button:    Icon button (×), 28px

Layout:
┌─────────────────┬──────────────────────┬────┐
│  File extension │  pdf, dmg           ×│ × │
└─────────────────┴──────────────────────┴────┘
```

**Add condition / Add action:**
```
Ghost button sm with + icon
Color:            --color-accent-400
Hover:            --color-accent-500
Margin-top:       4px
```

**Conflict warning:**
```
Background:       --color-warning-bg
Border:           1px solid rgba(255, 159, 10, 0.3)
Border-radius:    8px
Padding:          10px 12px
Margin-top:       16px

Icon:             Warning triangle, 14px, --color-warning
Text:             13px, --color-text-secondary
```

**Test this rule button:**
```
Ghost button with Play icon
Full width
Margin-top:       16px
Border:           1px dashed --color-border-default
Border-radius:    8px
Height:           36px
```

### Footer
```
Cancel:           Ghost button
Save Rule:        Primary button
                  Disabled until: name filled + 1 condition + 1 action with value
```

---

## 10. Activity Screen

### Layout

```
┌─ Content Area ───────────────────────────────────────────────┐
│                                                              │
│  Activity                                                    │
│                                                              │
│  [All ▼]  [Any rule ▼]  [Last 7 days ▼]  [🔍 Search...]   │
│                                                              │
│  ─────────────────────────────────────────────────────────  │
│  Today                                                       │
│  ─────────────────────────────────────────────────────────  │
│  [Activity entries]                                          │
│                                                              │
│  Yesterday                                                   │
│  ─────────────────────────────────────────────────────────  │
│  [Activity entries]                                          │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Filter bar:**
```
Display:          flex, gap: 8px, align: centre
Margin-bottom:    20px

Dropdowns:        sm size, compact
Search input:     flex: 1, max-width: 200px, with search icon left
```

**Date group header:**
```
Font:             12px, 500 medium, --color-text-tertiary
Text-transform:   uppercase
Letter-spacing:   0.06em
Padding:          8px 0 6px 0
Border-bottom:    1px solid --color-border-subtle
Margin-bottom:    6px
```

### Activity entry

```
┌──────────────────────────────────────────────────────────────┐
│  ✅  invoice_march.pdf                           2:34 PM     │
│      ~/Downloads → ~/Documents/Finance/                      │
│      Move Invoices and Receipts              [Undo]          │
└──────────────────────────────────────────────────────────────┘
```

**Dimensions:**
```
Min-height:       64px
Padding:          12px 16px
Border-radius:    10px
Background:       --color-bg-surface
Border:           1px solid --color-border-subtle
Margin-bottom:    4px
```

**Status icon:**
```
Size:             16px × 16px
Position:         top-left, aligned with first line of text

✅ success:       Check circle, --color-success
⏭ skipped:       Skip forward, --color-neutral
❌ failed:        X circle, --color-danger
↩ undone:        Corner up left, --color-text-tertiary
```

**Filename:**
```
Font:             14px, 500 medium, --color-text-primary
Max-width:        calc(100% - 120px)
Overflow:         truncate
```

**Timestamp:**
```
Font:             12px, --color-text-tertiary
Position:         right, aligned with filename
```

**Path:**
```
Font:             12px, font-mono, --color-text-secondary
Margin-top:       2px

Source path:      --color-text-tertiary
Arrow (→):        --color-text-tertiary, 0 4px margin
Destination:      --color-accent-400
```

**Rule name:**
```
Font:             11px, --color-text-tertiary
Margin-top:       4px
```

**Undo button:**
```
Position:         bottom right
Visibility:       hidden on default, shown on row hover
Size:             sm ghost button
Color:            --color-accent-400
Disabled state:   --color-text-tertiary, cursor default
                  Tooltip: "Undo window expired"
```

**Error reason (failed entries):**
```
Font:             12px, --color-danger
Margin-top:       4px
```

---

## 11. Settings Screen

### Layout

```
┌─ Content Area ───────────────────────────────────────────────┐
│                                                              │
│  Settings                                                    │
│                                                              │
│  GENERAL ─────────────────────────────────────────────────  │
│  [Setting rows]                                              │
│                                                              │
│  WATCHED FOLDERS ─────────────────────────────────────────  │
│  [Folder rows]                                               │
│                                                              │
│  FILE HANDLING ───────────────────────────────────────────  │
│  [Setting rows]                                              │
│                                                              │
│  ACTIVITY LOG ────────────────────────────────────────────  │
│  [Setting rows]                                              │
│                                                              │
│  ACCOUNT ─────────────────────────────────────────────────  │
│  [Account row]                                               │
│                                                              │
│  ABOUT ───────────────────────────────────────────────────  │
│  [About rows]                                                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Section header:**
```
Font:             11px, 600, uppercase, letter-spacing: 0.06em
Color:            --color-text-tertiary
Margin:           24px 0 10px 0 (32px top for first section)
```

**Setting row:**
```
Display:          flex, justify: space-between, align: centre
Height:           44px
Padding:          0 16px
Background:       --color-bg-surface
Border-radius:    10px (first item top radius, last item bottom radius — grouped)
Border:           1px solid --color-border-subtle

Label:            14px, --color-text-primary
Sublabel:         12px, --color-text-tertiary (below label, optional)
Control:          Toggle / Dropdown / Input (right side)
```

**Grouped rows** (macOS settings style):
```
First item:       border-radius: 10px 10px 0 0
Middle items:     border-radius: 0, border-top: none
Last item:        border-radius: 0 0 10px 10px, border-top: none
Single item:      border-radius: 10px
```

**Folder row (Watched Folders section):**
```
Same as setting row
Right side:       folder path (12px, mono, --color-text-secondary) + × remove button
```

**Add folder row:**
```
Same height, same group style
Color:            --color-accent-400
Font:             14px, with + icon left
```

**Danger row (Clear activity log):**
```
Label color:      --color-danger
```

**Account section:**
```
Free plan row:
  Left:           "Plan" label
  Right:          "Free · 3 rules maximum" badge (neutral)
                  "Upgrade to Pro →" (accent link, 13px)

Monthly row:
  Left:           "Plan" label
  Right:          "Monthly · $4.99/month" (success badge)
                  "Next billing: June 5, 2026" below (12px, tertiary)
                  "Manage subscription →" link

Lifetime row:
  Left:           "Plan" label
  Right:          "Lifetime ⭐" (accent badge)
                  "Purchased: May 5, 2026" below
```

---

## 12. Menubar Popover

### Dimensions
```
Width:            280px
Border-radius:    12px
Background:       --color-bg-elevated with backdrop-filter: blur(20px)
Border:           1px solid --color-border-default
Box-shadow:       0 16px 48px rgba(0,0,0,0.5)
Padding:          0 (sections have their own padding)
```

### Layout

```
┌──────────────────────────────────┐
│  🌀 Seiri              ● Live   │  ← Header (12px padding, 44px height)
│  ────────────────────────────   │  ← Divider
│  Recent Activity                │  ← Section label (11px, uppercase)
│                                 │
│  ✅ invoice_march.pdf           │  ← Activity item
│     → Documents/Finance/        │
│     2 min ago                   │
│                                 │
│  ✅ Xcode_15.dmg                │
│     → Applications/             │
│     1 hr ago                    │
│                                 │
│  View all activity →            │  ← Link row (13px, accent)
│  ────────────────────────────   │  ← Divider
│  ▶  Run Now                     │  ← Action row
│  ⏸  Pause Seiri                 │  ← Action row
│  ⤢  Open Seiri                  │  ← Action row
│  ✕  Quit Seiri                  │  ← Action row (danger colour)
└──────────────────────────────────┘
```

**Header:**
```
Height:           44px
Padding:          0 14px
Display:          flex, align: centre, justify: space-between

App name:         14px, 600, --color-text-primary
Status dot:       8px + label: 12px, --color-text-secondary
```

**Activity item (compact):**
```
Padding:          10px 14px
Border-bottom:    1px solid --color-border-subtle (except last)

Filename:         13px, 500, --color-text-primary, truncated 180px max
Path:             11px, --color-text-secondary, font-mono, truncated
Timestamp:        11px, --color-text-tertiary
```

**Action row:**
```
Height:           36px
Padding:          0 14px
Display:          flex, align: centre, gap: 10px

Icon:             14px, --color-text-secondary
Label:            14px, --color-text-primary

Hover:            background rgba(255,255,255,0.05)

Quit row:
  Icon + Label:   --color-danger
```

**Paused state:**
```
Paused banner:
  Background:     --color-warning-bg
  Border-bottom:  1px solid rgba(255,159,10,0.3)
  Padding:        8px 14px
  Font:           12px, --color-warning
  Text:           "Seiri is paused — files are not being organised"
```

**Error state:**
```
Error banner:
  Background:     --color-danger-bg
  Border-bottom:  1px solid rgba(255,69,58,0.3)
  Padding:        8px 14px
  Font:           12px, --color-danger
  Includes:       "Fix issue →" link
```

---

## 13. Upgrade Modal

```
Max-width:        480px
Uses:             Dialog component (5.10)

Header:
  Icon:           Rocket emoji or Seiri icon with sparkle, 32px
  Title:          "Unlock unlimited rules"
                  20px, 600, --color-text-primary
  Subtitle:       "You've set up 3 rules — Seiri is already saving you time."
                  14px, --color-text-secondary

Pricing cards:    Vertical stack, gap: 10px, margin-top: 20px

  Monthly card:
    Background:   --color-bg-surface
    Border:       1px solid --color-border-default
    Border-radius: 10px
    Padding:      16px
    
    Price:        "$4.99" — 24px, 600, --color-text-primary
                  "/ month" — 14px, --color-text-secondary
    Sub:          "Cancel anytime" — 12px, --color-text-tertiary
    CTA:          Secondary button full-width: "Choose Monthly"

  Lifetime card:
    Background:   rgba(14, 165, 233, 0.05)
    Border:       1px solid --color-accent-500
    Border-radius: 10px
    Padding:      16px
    Badge:        "BEST VALUE" — 10px, uppercase, accent badge, top-right

    Price:        "$24.99" — 24px, 600, --color-text-primary
                  "one-time" — 14px, --color-text-secondary
    Sub:          "All future v1 updates included" — 12px, --color-text-tertiary
    CTA:          Primary button full-width: "Choose Lifetime"

Footer:
  "Already purchased?"  12px, --color-text-tertiary
  "Restore licence"     12px, --color-accent-400, link
  "Maybe later"         Ghost button sm, right-aligned
  Margin-top:           16px
```

---

## 14. States

### Loading states

**Full page loading (initial data fetch):**
```
Skeleton loaders — animated shimmer effect

Shimmer:
  Background:     linear-gradient(
                    90deg,
                    --color-bg-surface 25%,
                    --color-bg-overlay 50%,
                    --color-bg-surface 75%
                  )
  Animation:      shimmer 1.5s ease-in-out infinite
  Border-radius:  Matches real content border-radius

Rule skeleton:    Full width, 64px height, border-radius: 10px
Activity skeleton: Full width, 64px height, border-radius: 10px
```

**Inline loading (button action):**
```
Button shows spinner (16px, white) replacing label
Width stays fixed to prevent layout shift
```

**Sim mode loading:**
```
Progress bar with shimmer animation
Label below: "Scanning [folder name]..."
Animated ellipsis on label
```

---

### Empty states

| Screen | Icon | Title | Description | CTA |
|---|---|---|---|---|
| Rules — no rules | Folder icon | "No rules yet" | "Create your first rule to start organising your files." | "+ Create your first rule" |
| Activity — no entries | Clock icon | "No activity yet" | "Drop a file in your Downloads to see Seiri in action." | None |
| Activity — filtered, no results | Search icon | "No results" | "Try adjusting your filters." | "Clear filters" |
| Settings — no watched folders | Eye-off icon | "No folders watched" | "Add a folder to start organising." | "+ Add folder" |

---

### Error states

**Inline error (form validation):**
```
Red border on input field
Error message below: 12px, --color-danger
Icon: X circle, 12px, left of message
```

**Permission revoked (critical):**
```
Full-width banner in content area:
Background:     --color-danger-bg
Border:         1px solid rgba(255,69,58,0.3)
Padding:        12px 16px
Text:           "Full Disk Access has been revoked. Seiri has paused."
CTA:            "Restore Access →" (inline link)
```

**Sidecar error:**
```
Menubar: red badge on icon
Popover: error banner
Main window: banner below page header
```

---

### Success states

**After rule saved:**
```
Brief toast (bottom right): "Rule saved"
Duration: 2 seconds
Icon: Check, green
```

**After undo:**
```
Brief toast: "[filename] moved back to [folder]"
Duration: 3 seconds
```

**After licence activated:**
```
Full modal replaced with:
Check mark animation
"You're all set! Unlimited rules unlocked."
Auto-dismiss after 3 seconds → rules screen
```

---

## 15. Motion and Animation

### Principles
- **Purposeful** — every animation communicates something: a transition, a state change, feedback
- **Fast** — UI animations should feel instant. 100–200ms for most transitions
- **Native feel** — ease-out for entrances (things decelerate into place), ease-in for exits (things accelerate away), ease-in-out for loops
- **Restrained** — one prominent animation per moment. Never two things moving simultaneously unless they are a coordinated group

### Timing reference

| Use case | Duration | Easing |
|---|---|---|
| Button press feedback | 80ms | ease |
| Hover state change | 120ms | ease |
| Toggle switch | 150ms | ease |
| Modal in | 150ms | ease-out |
| Modal out | 100ms | ease-in |
| Drawer in | 200ms | ease-out |
| Drawer out | 160ms | ease-in |
| Toast in | 200ms | ease-out |
| Toast out | 150ms | ease-in |
| Page transition | 150ms | ease-out |
| Skeleton shimmer | 1500ms | ease-in-out infinite |
| Status dot pulse | 2000ms | ease-in-out infinite |
| Drag (reorder) | 150ms | ease (item shift) |
| Onboarding step | 200ms | ease-out |

### Key animation moments

**Onboarding Step 6 — activation checkmark:**
```
SVG circle: draws itself (stroke-dashoffset animation), 400ms, ease-out
Check mark: draws itself after circle, 200ms, ease-out
Circle glow: fades in with circle, opacity 0 → 0.4, 400ms
Content below: staggered fade-in, 100ms delay per element
```

**Rule saved — list insert:**
```
New rule slides in from top with fade:
opacity: 0 → 1, translateY(-8px) → translateY(0)
Duration: 200ms, ease-out
Other rules: shift down smoothly
```

**Drag to reorder:**
```
Dragging item: scale(1.02), elevated shadow, cursor: grabbing
Target position: gap opens in list (height animated)
On drop: item slides into position, 150ms ease-out, scale returns to 1
```

**File moved — activity feed insert:**
```
New entry slides in from top:
opacity: 0 → 1, translateY(-6px) → translateY(0)
Duration: 200ms, ease-out
Badge on Activity nav item pulses once (scale 1 → 1.3 → 1)
```

**Toggle on/off:**
```
Knob translates left/right: 150ms, ease
Track background transitions: 150ms, ease
```

---

## 16. Accessibility

### Colour contrast
- All primary text on dark backgrounds: minimum 7:1 ratio (WCAG AAA)
- Secondary text: minimum 4.5:1 ratio (WCAG AA)
- Accent blue (#0ea5e9) on dark bg (#111113): passes AA for large text
- Never rely on colour alone to communicate state — always pair with icon or text

### Keyboard navigation
- All interactive elements reachable by Tab
- Focus ring: 2px solid --color-accent-500, 2px offset, visible on all elements
- Drawer: focus trapped within when open
- Modal: focus trapped within when open, returns to trigger on close
- Drag handles: keyboard reordering via arrow keys (↑/↓ to move rule up/down)
- Escape: closes drawer, modal, popover

### Screen reader support
- All icon-only buttons have `aria-label`
- Toggle has `aria-checked` state
- Status indicators have `aria-live` for dynamic updates
- Activity feed updates announced with `aria-live="polite"`
- Drag handles: `role="button"`, `aria-roledescription="Reorder handle"`

### Reduced motion
```css
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```
All animations respect `prefers-reduced-motion`. Skeleton loaders become static. Pulse animations stop. Transitions become instant.

---

## 17. Tailwind Configuration

Complete `tailwind.config.ts` for Seiri:

```typescript
import type { Config } from 'tailwindcss'

export default {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        // Backgrounds
        'bg-base':      '#111113',
        'bg-elevated':  '#1C1C1E',
        'bg-surface':   '#242426',
        'bg-overlay':   '#2C2C2E',
        'bg-input':     '#1C1C1E',
        'bg-tooltip':   '#3A3A3C',

        // Text
        'text-primary':   '#F5F5F7',
        'text-secondary': '#98989D',
        'text-tertiary':  '#636366',

        // Borders
        'border-subtle':  '#2C2C2E',
        'border-default': '#3A3A3C',
        'border-strong':  '#48484A',

        // Accent — Seiri Blue
        accent: {
          50:  '#f0f9ff',
          100: '#e0f2fe',
          200: '#bae6fd',
          400: '#38bdf8',
          500: '#0ea5e9',
          600: '#0284c7',
          700: '#0369a1',
          900: '#0c4a6e',
        },

        // Semantic
        success:      '#34C759',
        'success-bg': '#1A3A22',
        warning:      '#FF9F0A',
        'warning-bg': '#3A2A10',
        danger:       '#FF453A',
        'danger-bg':  '#3A1A18',
      },

      fontFamily: {
        sans: ['-apple-system', 'SF Pro Display', 'SF Pro Text',
               'BlinkMacSystemFont', 'Helvetica Neue', 'sans-serif'],
        mono: ['SF Mono', 'Fira Code', 'Fira Mono', 'monospace'],
      },

      fontSize: {
        'xs':   ['11px', { lineHeight: '1.4' }],
        'sm':   ['13px', { lineHeight: '1.5' }],
        'base': ['14px', { lineHeight: '1.5' }],
        'lg':   ['15px', { lineHeight: '1.5' }],
        'xl':   ['17px', { lineHeight: '1.4' }],
        '2xl':  ['20px', { lineHeight: '1.3' }],
        '3xl':  ['24px', { lineHeight: '1.25' }],
      },

      borderRadius: {
        'sm':  '4px',
        'md':  '6px',
        DEFAULT: '8px',
        'lg':  '10px',
        'xl':  '12px',
        '2xl': '16px',
      },

      boxShadow: {
        'sm':     '0 2px 8px rgba(0,0,0,0.2)',
        DEFAULT:  '0 4px 16px rgba(0,0,0,0.3)',
        'lg':     '0 8px 32px rgba(0,0,0,0.4)',
        'xl':     '0 16px 48px rgba(0,0,0,0.5)',
        '2xl':    '0 24px 64px rgba(0,0,0,0.5)',
        'accent': '0 0 32px rgba(14,165,233,0.3)',
      },

      transitionDuration: {
        '80':  '80ms',
        '120': '120ms',
        '150': '150ms',
        '200': '200ms',
      },

      transitionTimingFunction: {
        'ease-out': 'cubic-bezier(0, 0, 0.2, 1)',
        'ease-in':  'cubic-bezier(0.4, 0, 1, 1)',
      },

      keyframes: {
        shimmer: {
          '0%':   { backgroundPosition: '-200% 0' },
          '100%': { backgroundPosition: '200% 0' },
        },
        pulse: {
          '0%, 100%': { opacity: '1' },
          '50%':      { opacity: '0.5' },
        },
        slideInFromRight: {
          from: { transform: 'translateX(420px)' },
          to:   { transform: 'translateX(0)' },
        },
        slideInFromTop: {
          from: { opacity: '0', transform: 'translateY(-8px)' },
          to:   { opacity: '1', transform: 'translateY(0)' },
        },
        fadeIn: {
          from: { opacity: '0' },
          to:   { opacity: '1' },
        },
        scaleIn: {
          from: { opacity: '0', transform: 'scale(0.95)' },
          to:   { opacity: '1', transform: 'scale(1)' },
        },
      },

      animation: {
        shimmer:          'shimmer 1.5s ease-in-out infinite',
        pulse:            'pulse 2s ease-in-out infinite',
        'pulse-fast':     'pulse 1s ease-in-out infinite',
        'slide-in-right': 'slideInFromRight 200ms ease-out',
        'slide-in-top':   'slideInFromTop 200ms ease-out',
        'fade-in':        'fadeIn 150ms ease-out',
        'scale-in':       'scaleIn 150ms ease-out',
      },
    },
  },
  plugins: [],
} satisfies Config
```

---

## Appendix A: Icon Reference

Seiri uses **Lucide React** for all icons. Never use emoji in the UI (emoji are used only in this spec document for illustration).

| Context | Icon name | Size |
|---|---|---|
| Rules nav | `ClipboardList` | 16px |
| Activity nav | `ScrollText` | 16px |
| Settings nav | `Settings2` | 16px |
| Pause | `Pause` | 16px |
| Play/Resume | `Play` | 16px |
| Run Now | `Zap` | 16px |
| Add rule | `Plus` | 16px |
| Edit rule | `Pencil` | 14px |
| Delete rule | `Trash2` | 14px |
| Duplicate | `Copy` | 14px |
| More actions | `Ellipsis` | 14px |
| Drag handle | `GripVertical` | 16px |
| Undo | `CornerUpLeft` | 14px |
| Success status | `CheckCircle2` | 16px |
| Skipped status | `SkipForward` | 16px |
| Failed status | `XCircle` | 16px |
| Undone status | `CornerUpLeft` | 16px |
| Lock (free tier) | `Lock` | 14px |
| Warning | `AlertTriangle` | 14px |
| Folder | `Folder` | 16px |
| Search | `Search` | 14px |
| Close/Remove | `X` | 14px |
| External link | `ArrowUpRight` | 12px |
| Chevron down | `ChevronDown` | 14px |

---

## Appendix B: File Path Display

File paths are a core part of the Seiri UI. They should always be displayed consistently:

- Font: `font-mono`, `text-sm` (13px)
- Colour: `--color-text-secondary` for paths, `--color-accent-400` for destination paths
- Truncation: truncate from the middle for long paths, never from the end (the filename is the important part)
- Home directory: always display as `~` not `/Users/username`
- Arrow separator: ` → ` in `--color-text-tertiary`
- Max display width: always set, never overflow container

**Middle truncation example:**
```
~/Documents/.../Finance/Invoices/2026/april/invoice_march.pdf
```
Not:
```
~/Documents/Personal/Work/Projects/Finance/Invoices/2026/apri...
```
