---
created_at: "2026-06-14T02:52:24.362402133+00:00"
id: "atelier-5625"
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
closed_at: "2026-06-14T07:06:05.887001340+00:00"
status: "done"
title: "Delegate bounded transcript and validation slices earlier"
updated_at: "2026-06-14T07:06:05.887001340+00:00"
---

## Description

After the Agent Factory/Atelier guidance boundary is reconciled, update
portable orchestration guidance to delegate bounded docs drift, command
inventory, transcript capture, fixture repair, stale-test inventory, and focused
validation to Mini models earlier.

## Outcome

Main orchestrator threads do less bounded search and validation work directly while preserving high-reasoning review for ambiguous closeout or architecture.

## Evidence

- File diff in Agent Factory orchestration guidance includes model-routing examples and proof expectations for these bounded slices.
- Examples keep model-routing judgment in Agent Factory and route
  repository-specific command recipes to Atelier-owned docs/help.
- `git diff --check` passes for the documentation change.
