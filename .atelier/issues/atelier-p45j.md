---
created_at: "2026-06-15T04:02:11.965417659+00:00"
id: "atelier-p45j"
issue_type: "bug"
labels:
- "cli"
- "output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Clarify ready-list parent headers for blocked epics"
updated_at: "2026-06-15T04:05:54.474930114+00:00"
---

## Description

`atelier issue list --ready` can print a blocked parent epic as the visible section header for a ready child. The graph is correct, but the output can make the parent look like a selectable ready item instead of contextual grouping. This is especially confusing when the epic is intentionally blocked on validation or closeout.

## Outcome

- Ready-list output clearly distinguishes parent/grouping context from ready executable child rows.
- A blocked parent epic shown only as context visibly indicates that it is blocked or not itself ready.
- Operators can identify the actual ready item without reading blocker drill-down output separately.

## Evidence

- CLI transcript or focused test shows a blocked epic with a ready child renders the child as the ready item and the parent as contextual/non-ready.
- CLI transcript proves `atelier issue blocked <epic-id>` and `atelier issue list --ready` agree about parent blocked state.
- Help or output snapshot review confirms the ready-list hierarchy no longer implies blocked parent epics are selectable ready work.
- `target/debug/atelier lint`, `target/debug/atelier export --check`, and focused CLI tests pass.
