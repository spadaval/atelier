---
created_at: "2026-10-17T01:10:57.984917292+00:00"
id: "atelier-oyzb"
issue_type: "task"
labels:
- "cli-output"
- "work-views"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fix work mission and parent footers, omission hints and filters"
updated_at: "2026-10-17T01:10:57.984917292+00:00"
---

## Goal

- The footer's "List mission blockers: atelier work mission <id> --blocked" lists blocked work, not blockers (crates/atelier-cli/src/commands/work.rs:579-584).
- The footer suggests the filter already in use (`--ready` view offers `--ready`).
- Summary panels stop at 8 rows with `... 4 more done work items omitted` (seen on `work mission atelier-kgcr`) and never name `--done`.
- Needs a Human, Parents and Missions and Backlog rows have no filter (`--human`, `--backlog` do not exist), so past 8 rows they are reachable only via `--all`.
- `work parent` has no filter flags and no list hints.
- "Inspect transitions" is offered on done and superseded missions.

## Done when

- The label reads "List blocked mission work"; the active filter is never suggested.
- Every omission line carries its command: `… 4 more: atelier work mission atelier-kgcr --done`.
- `--human` and `--backlog` exist on `work mission`, or summary sections never truncate.
- `work parent` accepts the same filters as `work mission`.
- Terminal missions get no transition hint. Tests cover each.
