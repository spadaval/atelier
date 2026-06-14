---
created_at: "2026-06-14T16:31:30.996733591+00:00"
id: "atelier-9soq"
issue_type: "feature"
labels:
- "docs"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-uran"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Teach docs/help drift checker about hidden command visibility"
updated_at: "2026-06-14T16:31:30.996733591+00:00"
---

## Description

Command-surface drift should distinguish visible root-help commands from explicitly hidden or advanced command references in docs.

## Outcome

- The docs/help drift scanner has a visibility model for command references:
  visible, hidden/advanced, removed/deferred, and normal workflow.
- Hidden/advanced references are not compared against root visible help as if
  they were public commands.
- Hidden/advanced references still fail when they point to a nonexistent
  command or option, unless the surrounding context explicitly marks the command
  removed/deferred.

## Evidence

- Unit or integration tests cover a hidden command referenced in an approved
  hidden/advanced context.
- Tests cover a hidden command recommended as normal workflow still failing the
  drift check.
- Tests cover nonexistent options still failing.
- `git diff --check` and `atelier lint` pass.
