---
created_at: "2026-06-18T16:45:30.861580764+00:00"
id: "atelier-kryk"
issue_type: "task"
labels:
- "cli"
- "docs"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T18:07:21.625284651+00:00"
status: "done"
title: "Remove durable-session workflow commands and guidance"
updated_at: "2026-06-18T18:07:21.625284651+00:00"
---

## Description

Remove durable-session workflow commands and guidance that conflict with the session-as-issue-events model. Scope includes CLI help, role guides, docs references, and command routing for `session begin` and `session end`. Out of scope: removing inspection-only `session list/show` unless the contract issue decides those names should also change.

## Outcome

Public guidance no longer teaches sessions as records operators begin, end, reuse, or classify as mutating/admin/coordination. Help and role guides present sessions only as derived inspection output for worker/reviewer/validator attempts.

## Evidence

Help/man/docs transcripts or snapshots prove `session begin`, `session end`, mutating session kinds, admin sessions, and coordination sessions are absent from happy-path guidance while inspection-only session output remains documented.
