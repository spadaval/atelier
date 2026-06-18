---
created_at: "2026-06-18T16:45:16.595923819+00:00"
id: "atelier-fdi4"
issue_type: "feature"
labels:
- "cli"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-f2c4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Emit issue-event attempt milestones from workflow commands"
updated_at: "2026-06-18T16:45:16.595923819+00:00"
---

## Description

Teach normal workflow and proof commands to emit the issue activity events needed to derive worker/reviewer/validator attempts. Scope includes `start`, evidence/validation milestones, and workflow transitions that create or close role attempts. Out of scope: rendering session inspection output and PR-specific attribution.

## Outcome

`start`, evidence recording or attachment, validation request/approval paths, and relevant workflow transitions record structured issue activity metadata for worker, reviewer, and validator attempts. Derived attempt serials are stable per issue and role, and no standalone session record is required to explain who did the work.

## Evidence

Focused unit or CLI tests prove each milestone writes parseable issue activity metadata, attempt serials are derived deterministically, and the same scenarios do not create or require `.atelier/sessions` records.
