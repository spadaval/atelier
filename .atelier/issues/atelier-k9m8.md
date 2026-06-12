---
created_at: "2026-06-12T05:12:44.936931257+00:00"
id: "atelier-k9m8"
issue_type: "task"
labels:
- "mission"
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Add mission reliability status summary"
updated_at: "2026-06-12T05:12:44.936931257+00:00"
---

## Description

Add a mission reliability summary surface so orchestrators can quickly see
whether a mission is ready to implement, validate, or close.

## Outcome

- Mission status shows reliability signals: malformed work, missing Outcome,
  missing Evidence, missing attached proof, ignored-test blockers, docs/help
  drift, and open blockers.
- The summary is bounded and points to focused drill-downs.
- Reliability signals use the same underlying checks as closeout validators.

## Evidence

- CLI transcript tests cover healthy and unhealthy reliability summaries.

- Tests prove summary output agrees with closeout blockers.

- Run focused mission status tests.
