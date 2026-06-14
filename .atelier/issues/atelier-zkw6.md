---
created_at: "2026-06-14T02:52:12.214253216+00:00"
id: "atelier-zkw6"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T07:07:00.441691057+00:00"
status: "done"
title: "Add Agent Factory command-surface stop rule"
updated_at: "2026-06-14T07:07:00.441691057+00:00"
---

## Description

After the Agent Factory/Atelier guidance boundary is reconciled, teach agents
to stop after the first clear unrecognized command or wrong command-family
error and consult the bound repository's command map/help instead of probing
neighboring names.

## Outcome

Long retry loops around removed commands are explicitly discouraged by guidance.

## Evidence

- File diff in Agent Factory docs or prompt template includes the stop rule.
- File diff or review artifact shows the command-map reference points to
  Atelier-owned docs/help for this repository rather than embedding a stale
  command list in Agent Factory.
