---
created_at: "2026-06-18T16:20:37.743671729+00:00"
id: "atelier-5k7k"
issue_type: "epic"
labels:
- "cli"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-lvgo"
  children:
  - kind: "issue"
    id: "atelier-f2c4"
  - kind: "issue"
    id: "atelier-fdi4"
  - kind: "issue"
    id: "atelier-kryk"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T18:08:30.974911898+00:00"
status: "done"
title: "Epic: Update CLI behavior for issue-event attempts"
updated_at: "2026-06-18T18:08:30.974911898+00:00"
---

## Description

Coordinate the CLI changes for session-as-issue-events. This epic delegates milestone event emission, session inspection rendering, and removal of durable-session workflow commands to focused children.

## Outcome

`start`, validation/evidence commands, `status`, `man`, and `session list/show` automatically reflect issue-event-derived attempts at meaningful milestones. Session commands are inspection-only, and public `mutating`, admin, coordination, begin, and end session language is removed from happy-path guidance.

## Evidence

Child proof demonstrates automatic attempt event behavior, inspection-only session rendering, and help/man/status guidance that no longer treats durable session records as workflow drivers.
