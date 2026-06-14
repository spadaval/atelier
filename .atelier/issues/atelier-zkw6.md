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
status: "todo"
title: "Add Agent Factory command-surface stop rule"
updated_at: "2026-06-14T02:52:12.214253216+00:00"
---

## Description

Teach agents to stop after the first clear unrecognized command or wrong command-family error and consult the command map/help instead of probing neighboring names.

## Outcome

Long retry loops around removed commands are explicitly discouraged by guidance.

## Evidence

Agent Factory docs or prompt template includes the stop rule and a current command-map reference.
