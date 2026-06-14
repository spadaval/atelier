---
created_at: "2026-06-12T02:40:14.225055234+00:00"
id: "atelier-vr9g"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T03:25:39.638484389+00:00"
status: "done"
title: "Design issue transition options surface"
updated_at: "2026-06-12T03:25:39.638484389+00:00"
---

## Description

Design the user-facing issue transition surface so users ask what an issue can do next, not which internal workflow validator to run. Scope: add or specify `atelier issue transition <id> --options` or a clearer equivalent that lists allowed target states/actions, blocked target states, fast gate reasons, and the command to perform the selected transition. Validators must remain fast state checks; expensive proof belongs in attached evidence, not synchronous transition validation. Acceptance: the design explains whether `workflow validate` remains an advanced proof command, how `issue show` summarizes transition readiness without dumping gate internals, and how `issue transition` relates to `issue update --status`, close/reopen, start/finish, and mission closeout.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
