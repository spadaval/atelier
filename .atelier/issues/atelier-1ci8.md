---
created_at: "2026-10-17T01:10:57.981811445+00:00"
id: "atelier-1ci8"
issue_type: "bug"
labels:
- "cli-output"
- "status"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show active work once in status, with its configured status"
updated_at: "2026-10-17T01:10:57.981811445+00:00"
---

## Goal

On an owner branch, status shows the same item three times:
```
Current work:  1 issue(s)
...
Current Work
------------
  active atelier-43m1 [active] low - Define Close Reason's expected content in work-model.md
...
Owner   : atelier-43m1 [in_progress] Define Close Reason's expected content in work-model.md
```
The panel row prints the category twice (`active … [active]`) and hides the configured status that the Owner line shows. The rows are built with `status: issue.status_category` (crates/atelier-cli/src/commands/status.rs:94). With no active work, status prints both `Current work:  none` and a `Current Work` panel containing `(none)`. human-cli-output.md says rows "should not repeat redundant category/status tokens".

## Done when

- Each active item is one line with its exact status, for example `atelier-43m1  in_progress  low  Define Close Reason…`. The owner is marked inline (for example `owner`) and not repeated in Local State.
- With no active work, there is no empty panel and no separate `Current work: none` line.
- Tests cover owner and no-owner output.

## Context

The row comes from the shared `IssueListPanel` in crates/atelier-cli/src/human_output.rs, so this extends atelier-deyy's dense-row rule to status. atelier-sdb0 reshapes the owner block.
