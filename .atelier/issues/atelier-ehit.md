---
created_at: "2026-06-20T15:11:00.178864245+00:00"
id: "atelier-ehit"
issue_type: "task"
labels:
- "command-surface"
- "cutting-pass"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-39um"
  - kind: "issue"
    id: "atelier-v2o6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T19:43:35.658639937+00:00"
status: "done"
title: "Fold graph views into issue and objective status views"
updated_at: "2026-06-20T19:43:35.658639937+00:00"
---

## Description

Evaluate and remove the `graph` namespace by moving useful hierarchy, blocker,
and blast-radius information into issue detail, objective status, blocker, and
related domain views.

## Outcome

`graph tree` and `graph impact` no longer exist as special-case views unless a
documented gap remains after issue/objective views are strengthened. The
command audit records the final decision and replacement commands.

## Evidence

- `target/debug/atelier graph --help` fails as an unknown command if the namespace is removed, or the audit documents the retained narrow case.
- `target/debug/atelier issue show <id>` and the objective status view show the
  hierarchy, blockers, and impact context users previously needed from graph
  commands.
- Integration tests cover the replacement view output and removed graph command behavior.
