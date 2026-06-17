---
created_at: "2026-06-17T20:03:29.276303235+00:00"
id: "atelier-v7d0"
issue_type: "task"
labels:
- "implementation"
- "milestones"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-aqqc"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T23:01:03.522503786+00:00"
status: "done"
title: "Remove milestone record support and checkpoint storage"
updated_at: "2026-06-17T23:01:03.522503786+00:00"
---

## Description

Remove first-class milestone/checkpoint record behavior. The audit found no
actual committed milestone records and no standalone milestone workflow. Keeping
the latent architecture would force validation/outcome data through an unused
object layer.

## Outcome

- `.atelier/milestones/` is not created by init and is not accepted as a
  canonical rebuild source.
- Milestone record kind registration, parser/rendering code, projection schema,
  mission display groups, relationship roles, and tests are removed.
- Validation/outcome data remains on issue, epic, mission, and evidence records.
- The term checkpoint is reserved for a future design only if a concrete
  reporting workflow requires it.

## Evidence

- Search transcript proves production code no longer creates, projects, links,
  or displays first-class milestone records.
- Focused tests prove unsupported milestone records or bulk inputs are rejected.
- `atelier lint`, `atelier export --check`, and `atelier doctor` pass.
