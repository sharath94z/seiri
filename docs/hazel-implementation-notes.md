# Hazel Implementation Notes for Seiri

Source: [Hazel User Guide](https://www.noodlesoft.com/manual/hazel/hazel-basics/)

## What Hazel Confirms

Hazel is a strong reference point for Seiri because it validates the core product shape:

- Folder-scoped automation is the right unit of control.
- Rules should be evaluated in order, with first match winning by default.
- Preview and status views are essential for trust.
- Manual run controls are useful for troubleshooting and backfill.
- Pause and resume need to be first-class states.
- Undo should exist, but only for actions that can be reversed reliably.
- Logs and rule status are part of the product, not just internal debugging.

## Implementation Ideas Worth Carrying Into Seiri

### 1. Folder-scoped rules

Hazel attaches rules to a specific monitored folder. Adding a folder by itself does nothing. This supports Seiri's model of watched folders like Downloads, Desktop, and Documents.

Sources:

- [About Folders & Rules](https://www.noodlesoft.com/manual/hazel/hazel-basics/about-folders-rules)
- [Manage Folders](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/manage-folders/)

### 2. Ordered rule evaluation

Hazel evaluates rules in order and stops at the first match unless a rule explicitly continues evaluation. Seiri should keep this behavior for predictability.

Sources:

- [Understand the Logic of Rules](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/create-edit-rules/understand-the-logic-of-rules/)
- [Action Reference](https://www.noodlesoft.com/manual/hazel/attributes-actions/action-reference/)

### 3. Preview before action

Hazel provides both single-item preview and folder-wide rule status. Seiri should keep both a live preview for one file and a broader preview/status view for folders.

Sources:

- [Preview a Rule](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/create-edit-rules/preview-a-rule/)
- [Show Rule Status](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/manage-rules/show-rule-status/)

### 4. Manual run controls

Hazel supports "Run Rules Now" to reprocess a folder immediately. Seiri should keep a similar manual run path for troubleshooting and explicit backfill.

Source:

- [Run Rules Manually](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/manage-rules/run-rules-manually/)

### 5. Nested conditions

Hazel supports nested boolean logic and condition targets. Seiri does not need Hazel's full flexibility in v1, but nested conditions are a proven model if we want more expressive rules later.

Source:

- [Using Nested Conditions](https://www.noodlesoft.com/manual/hazel/advanced-topics/using-nested-conditions/)

### 6. Multi-action rules

Hazel allows multiple actions in sequence. Seiri can keep v1 simpler, but the engine should be designed so multiple actions can be added later without a rewrite.

Source:

- [Action Reference](https://www.noodlesoft.com/manual/hazel/attributes-actions/action-reference/)

### 7. Conflict handling

Hazel exposes explicit destination conflict handling such as rename or replace. Seiri should keep explicit conflict policy instead of relying on filesystem defaults.

Source:

- [Action Reference](https://www.noodlesoft.com/manual/hazel/attributes-actions/action-reference/)

### 8. Pause and status controls

Hazel has both global stop/start and folder-level pause/resume, plus a menubar status icon. Seiri should keep a simple always-available menubar state and pause control.

Sources:

- [The Hazel Status Menu](https://www.noodlesoft.com/manual/hazel/hazel-basics/hazel-status-menu/)
- [Stopping & Restarting Hazel](https://www.noodlesoft.com/manual/hazel/hazel-basics/stopping-restarting-hazel/)
- [Enable, Disable, or Pause Rules](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/manage-rules/enable-disable-or-pause-rules/)

### 9. Recursive folder processing

Hazel has a special action for processing subfolder contents. It warns that recursion can slow things down. If Seiri adds this later, it should be explicit and carefully bounded.

Sources:

- [Processing Subfolders](https://www.noodlesoft.com/manual/hazel/advanced-topics/processing-subfolders/)
- [Action Reference](https://www.noodlesoft.com/manual/hazel/attributes-actions/action-reference/)

### 10. Undo boundaries

Hazel's revert feature only covers certain actions. Seiri should keep undo scoped only to actions we can reliably reverse.

Sources:

- [Revert a File](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/manage-rules/revert-a-file/)
- [What’s New in Hazel 6](https://www.noodlesoft.com/manual/hazel/whats-new-in-hazel/)

### 11. Logs and troubleshooting

Hazel treats logs, preview, and rule status as the core troubleshooting path. Seiri should make the activity feed and logs equally easy to access.

Sources:

- [Viewing Logs](https://www.noodlesoft.com/manual/hazel/viewing-logs/)
- [Troubleshoot Rules](https://www.noodlesoft.com/manual/hazel/work-with-folders-rules/manage-rules/troubleshoot-rules/)

## Seiri V1 Recommendation

For v1, Seiri should copy Hazel's strongest patterns without copying its complexity:

- watched folders
- ordered rules
- preview
- rule status / activity feed
- manual run
- pause and resume
- safe destination conflict handling
- reversible actions only

The main thing to avoid is trying to match Hazel's full power-user surface too early.
