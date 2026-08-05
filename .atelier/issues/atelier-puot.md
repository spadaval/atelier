---
created_at: "2026-08-05T18:46:45.704369113+00:00"
id: "atelier-puot"
issue_type: "task"
labels:
- "review-finding"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-08-05T19:12:08.305784903+00:00"
status: "done"
title: "Require proof route before epic readiness"
updated_at: "2026-08-05T19:12:08.305784903+00:00"
---

## Description

Repair the blocking PR 77 finding that an epic can become ready without an inspectable proof route. Preserve relaxed draft validation while making expansion prove that the current increment has concrete Evidence guidance or an explicit validation child.

## Outcome

- The epic `ready` transition rejects a draft with child work but no durable proof route and gives actionable recovery guidance.
- A concrete epic Evidence section or explicit validation child satisfies the route without introducing a validation profile or new planning record.
- Command-level tests cover rejection and success from a fresh tracker.

## Evidence

- Focused workflow-validator and CLI integration tests reproduce both rejection and allowed routes.
