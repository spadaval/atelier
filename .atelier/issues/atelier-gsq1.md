---
created_at: "2026-06-14T05:58:17.150379875+00:00"
id: "atelier-gsq1"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T07:20:09.583939460+00:00"
status: "done"
title: "Add record-specific note commands"
updated_at: "2026-06-14T07:20:09.583939460+00:00"
---

## Description

Replace generic note target-kind syntax and issue update append-notes guidance with record-specific note commands for records that support activity.

## Outcome

Durable activity notes are added through record-specific commands for records
that support activity. Generic `note add <kind> <id>` and
`issue update --append-notes` are not taught as normal note-entry paths.

## Evidence

- Help and docs show record-specific note commands such as issue and mission
  note entry.
- `atelier --help` and focused CLI transcript show generic note target-kind
  syntax is removed from normal root help or rejects with corrective guidance.
- Focused tests or transcripts cover adding notes to an issue and a mission,
  then reading them through history or show output.
- Search transcript shows normal docs and Agent Factory guidance do not route
  handoff notes through `issue update --append-notes`.
- `git diff --check`, `atelier lint`, and focused note tests pass.
