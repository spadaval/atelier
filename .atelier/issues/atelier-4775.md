---
created_at: "2026-10-17T09:36:28.855906025+00:00"
id: "atelier-4775"
issue_type: "bug"
labels:
- "cli-output"
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
status: "todo"
title: "Agree on close readiness in brief and full issue show"
updated_at: "2026-10-17T09:36:28.855906025+00:00"
---

## Goal

Mission validation (evidence atelier-pnha) found that on an in-progress issue, brief `issue show` lists `close --reason` as allowed, while `issue show --full` Transition Readiness says `close: blocked - missing required field close_reason`. The contract treats a transition whose only gap is a required argument as allowed.

## Done when

- Both views report a transition that needs only an argument the same way, as allowed with its flag.
- A test pins both views for that case.
