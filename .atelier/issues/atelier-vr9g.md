---
acceptance: []
created_at: "2026-06-12T02:40:14.225055234+00:00"
evidence_required: []
id: "atelier-vr9g"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Design issue transition options surface"
updated_at: "2026-06-12T02:40:14.225055234+00:00"
---

Design the user-facing issue transition surface so users ask what an issue can do next, not which internal workflow validator to run. Scope: add or specify `atelier issue transition <id> --options` or a clearer equivalent that lists allowed target states/actions, blocked target states, fast gate reasons, and the command to perform the selected transition. Validators must remain fast state checks; expensive proof belongs in attached evidence, not synchronous transition validation. Acceptance: the design explains whether `workflow validate` remains hidden/advanced/compatibility, how `issue show` summarizes transition readiness without dumping gate internals, and how `issue transition` relates to `issue update --status`, close/reopen, start/finish, and mission closeout.
