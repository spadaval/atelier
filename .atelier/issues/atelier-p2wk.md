---
created_at: "2026-07-06T20:37:49.331991658+00:00"
id: "atelier-p2wk"
issue_type: "epic"
labels:
- "mission-review"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t876"
  children:
  - kind: "issue"
    id: "atelier-3v2d"
  - kind: "issue"
    id: "atelier-72k4"
  - kind: "issue"
    id: "atelier-amfw"
  - kind: "issue"
    id: "atelier-fqzt"
  - kind: "issue"
    id: "atelier-wyxn"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Enforce reviewed and dependency-safe mission readiness"
updated_at: "2026-07-06T20:37:49.843257256+00:00"
---

## Description

Implement the repository-owned records, workflow transitions, validators, diagnostics, and migration needed to make independent plan review and declared dependency closure unavoidable at execution time.

## Outcome

- Mission readiness and start transitions enforce independent, fresh approval of the current plan graph and reject open declared blockers.
- Issue execution enforces declared dependencies, normal operator surfaces explain failures, and existing mission readiness state is handled according to the approved migration contract.

## Evidence

Evidence was not specified in the bundle.
