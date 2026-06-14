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
status: "todo"
title: "Delegate bounded transcript and validation slices earlier"
updated_at: "2026-06-14T02:52:24.362402133+00:00"
---

## Description

Update orchestration guidance to delegate bounded docs drift, command inventory, transcript capture, fixture repair, stale-test inventory, and focused validation to Mini models earlier.

## Outcome

Main orchestrator threads do less bounded search and validation work directly while preserving high-reasoning review for ambiguous closeout or architecture.

## Evidence

- File diff in Agent Factory orchestration guidance includes model-routing examples and proof expectations for these bounded slices.
- `git diff --check` passes for the documentation change.
