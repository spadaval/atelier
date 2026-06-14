---
created_at: "2026-06-14T02:52:08.979051706+00:00"
id: "atelier-8vyo"
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
title: "Add Agent Factory stale-state preflight"
updated_at: "2026-06-14T02:52:08.979051706+00:00"
---

## Description

Update Agent Factory guidance so tracker lint/export/read failures caused by canonical or projection invalidity stop workflow mutation until state is repaired.

## Outcome

Orchestrators and workers repair stale state before continuing mission mutation or closeout.

## Evidence

Agent Factory guidance names the preflight and repair behavior; a transcript or docs validation shows the rule is present.
