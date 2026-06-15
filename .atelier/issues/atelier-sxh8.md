---
created_at: "2026-06-14T03:47:10.898366522+00:00"
id: "atelier-sxh8"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4yrt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Remove generic link command and normalize relationship ownership"
updated_at: "2026-06-14T07:50:06.469728470+00:00"
---

## Description

Remove the generic `atelier link` surface entirely. Relationship operations are
owned by record-specific command families: mission work links by `atelier
mission add-work/unlink`, issue hierarchy and blockers by `atelier issue ...`,
evidence attachments by `atelier evidence ...`, and any remaining relation
needs by an explicitly named record-specific surface rather than `atelier
link`.

The current `atelier link` command is misleading because it advertises generic
relationship ownership while mission relationships are not actually handled
there. Do not preserve `atelier link` as a compatibility alias or shim.

## Outcome

`atelier link` is removed from visible help, docs, and command dispatch.
Operators manage mission work links, issue hierarchy, blockers, evidence
attachments, and remaining relationship edges through specific subcommands.
Attempts to use `atelier link` fail with corrective guidance naming the
specific command family to use.

## Evidence

- Product docs or command-surface docs record that generic `atelier link` was
  removed in favor of specific subcommands.
- Root help and command dispatch no longer expose `atelier link`.
- Focused CLI tests cover representative `atelier link` rejection and the
  replacement record-specific commands, including mission work links and issue
  blockers.
- Search transcript shows normal docs and Agent Factory guidance do not
  recommend `atelier link`.
