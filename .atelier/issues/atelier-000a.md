---
created_at: "2026-06-08T17:04:37+00:00"
id: "atelier-000a"
issue_type: "task"
labels:
- "agent-factory"
- "task"
- "tracker"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T17:59:15+00:00"
status: "done"
title: "Configure Beads Dolt remote for Atelier"
updated_at: "2026-06-08T17:59:15+00:00"
---

## Description

Agent-factory onboarding initialized Beads in shared-server mode, but bd doctor reports no Dolt remote configured. Choose the repository-appropriate remote URL and add it with bd dolt remote add origin <url>, then verify bd dolt status, bd dolt pull, bd dolt push, and bd lint.
bd doctor no longer reports Remote Consistency as missing; bd dolt status, bd dolt pull, bd dolt push, and bd lint have recorded pass/fail evidence in the bead notes.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
