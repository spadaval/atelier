---
created_at: "2026-06-18T16:45:24.861474469+00:00"
id: "atelier-f2c4"
issue_type: "feature"
labels:
- "cli"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kryk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Render session inspection from issue-event attempts"
updated_at: "2026-06-18T16:45:24.861474469+00:00"
---

## Description

Render session inspection surfaces from issue-event-derived attempts. Scope includes `session list`, `session show`, and any status/history summaries that need to display worker/reviewer/validator attempts. Out of scope: milestone event emission and public help cleanup.

## Outcome

`session list` and `session show` are inspection-only views over derived issue attempts. They show issue, role, serial, lifecycle state, and relevant recent activity from canonical issue events. They do not expose `begin`, `end`, active mutating sessions, admin sessions, or coordination sessions as workflow controls.

## Evidence

CLI tests or transcripts prove session list/show projections render from issue activity events, missing legacy `.atelier/sessions` files do not break inspection, and the commands do not mutate tracker state.
