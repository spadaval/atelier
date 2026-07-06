---
created_at: "2026-06-29T18:20:43.204322941+00:00"
id: "atelier-p19n"
issue_type: "task"
labels:
- "cli"
- "dashboard"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:10:52.658360051+00:00"
status: "done"
title: "Implement mission-scoped dashboard filters and shared state labels"
updated_at: "2026-06-30T15:10:52.658360051+00:00"
---

## Description

Add mission-scoped drill-down flags to `atelier work mission <mission-id>` and fix shared row classification so ready and blocked mean the same thing across work and issue-listing surfaces.

## Outcome

`atelier work mission <mission-id>` supports `--ready`, `--blocked`, `--active`, `--done`, and `--all`, and every flag shows rows scoped to that mission only. Ready work means unblocked todo-category scoped work, not only literal status `ready`. Blocked work includes workflow blocked-category rows and rows with open blocker links, with visible blocker IDs. The shared state-label behavior is consistent across `status`, `work ready`, `work mission`, and `issue list` where those surfaces display the same rows.
