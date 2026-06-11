---
acceptance: []
created_at: "2026-06-10T16:00:59.246554229+00:00"
evidence_required: []
id: "atelier-8o8v"
issue_type: "task"
labels:
- "activity"
- "assignee:root"
- "cli"
- "durability"
- "issue"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1z0u"
  - kind: "issue"
    id: "atelier-krhk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Write issue mutations as activity entries"
updated_at: "2026-06-10T17:33:14.622812806+00:00"
---

Write activity sidecars directly from future issue mutations.

What:
- Update issue comment/note/handoff/plan/decision workflows to append canonical issue activity entries.
- Record close reasons as `close_reason` activity entries.
- Record status and field mutations as `status_changed` and `field_changed` entries where relevant.
- Record work lifecycle events as `work_started` and `work_finished` entries.
- Preserve existing useful behavior while moving durable future history to sidecars.

Out of scope:
- One-off migration of existing SQLite comments.
- The history query/rendering command.

Acceptance criteria:
- Creating comments/notes, closing/reopening issues, changing fields, and starting/finishing work create expected sidecar activity entries.
- Activity body content matches the user-authored text or event detail without lossy formatting.
- Existing issue show/comment behavior remains compatible with the projection/index model.
- Focused integration tests cover each mutation type introduced here.

Recommended subskill: agent-factory implement.
