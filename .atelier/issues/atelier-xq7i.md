---
created_at: "2026-06-14T16:30:42.473368999+00:00"
id: "atelier-xq7i"
issue_type: "epic"
labels:
- "docs"
- "postmortem"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-jxs8"
  children:
  - kind: "issue"
    id: "atelier-9soq"
  - kind: "issue"
    id: "atelier-uran"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Make docs/help drift checks visibility-aware"
updated_at: "2026-06-14T16:31:33.611558920+00:00"
---

## Description

Keep command-surface drift checks useful while allowing explicitly hidden or advanced command documentation.

## Outcome

- Command-surface drift checks distinguish visible root-help commands from
  explicitly hidden or advanced command references.
- Hidden commands can be documented in hidden/advanced contexts without being
  treated as visible root-help surfaces.
- Public guidance still fails when removed commands, missing commands, or
  nonexistent options are recommended as normal workflow.

## Evidence

- Focused tests or transcripts show visible-command drift is still detected.
- Focused tests or transcripts show hidden/advanced command references are
  accepted only in approved contexts and still checked for command/option
  existence where appropriate.
- `git diff --check`, `atelier lint`, and command-surface drift checks pass.
