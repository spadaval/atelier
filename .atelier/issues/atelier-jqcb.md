---
created_at: "2026-06-14T02:52:21.445933576+00:00"
id: "atelier-jqcb"
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
title: "Require claim-specific closeout proof mapping"
updated_at: "2026-06-14T02:52:21.445933576+00:00"
---

## Description

Update guidance so closeout workers name the specific Outcome or Evidence line proved by their evidence, especially for broad or parent-level claims.

## Outcome

Proof summaries are claim-specific rather than broad green-test statements.

## Evidence

- File diff in Agent Factory closeout guidance includes the claim-specific proof requirement and examples.
- `git diff --check` passes for the documentation change.
