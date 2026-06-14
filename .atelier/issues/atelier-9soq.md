---
created_at: "2026-06-14T16:31:30.996733591+00:00"
id: "atelier-9soq"
issue_type: "feature"
labels:
- "assignee:root"
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
status: "validation"
title: "Teach docs/help drift checker about hidden command visibility"
updated_at: "2026-06-14T17:00:32.800459395+00:00"
---

## Description

Command-surface drift should distinguish visible root-help commands from explicitly hidden or advanced command references in docs.

## Outcome

- The docs/help drift scanner has a visibility model for command references:
  visible workflow, hidden/advanced callable, and removal-history references.
- Hidden/advanced references are not compared against root visible help as if
  they were public commands.
- Hidden/advanced references still fail when they point to a nonexistent
  command or option.
- Removed or unsupported command names are allowed only in explicit
  removal-history or compatibility-classification contexts, and still fail when
  presented as current workflow guidance.

## Evidence

- Unit or integration tests cover a hidden command referenced in an approved
  hidden/advanced context.
- Tests cover a hidden command recommended as normal workflow still failing the
  drift check.
- Tests cover removed commands being allowed only in removal-history context
  and rejected when presented as workflow guidance.
- Tests cover nonexistent options still failing.
- `git diff --check` and `atelier lint` pass.
