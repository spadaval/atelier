---
created_at: "2026-06-17T17:59:08.101216908+00:00"
id: "atelier-98mo"
issue_type: "epic"
labels:
- "readiness"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-4clo"
  - kind: "issue"
    id: "atelier-cglp"
  - kind: "issue"
    id: "atelier-p7oa"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: End-to-end validation and readiness"
updated_at: "2026-06-17T18:01:06.284444365+00:00"
---

## Description

Validate the completed mission end to end and prove the session-as-issue-events
model, Forgejo PR, field, and validator behavior works as a coherent operator
workflow.

## Outcome

- Focused tests cover the field, issue-event-derived session inspection, PR,
  and validator implementation.
- Scenario evidence proves worker/reviewer/validator attribution from issue
  events, sudo PR authorship, unresolved comment rendering, and merged PR close
  gating.
- Closeout checks prove tracker, docs, and code health.

## Evidence

- Evidence record attached to this epic maps focused tests, scenario proof, and
  closeout checks to each mission validation criterion, including the corrective
  session-as-issue-events epic `atelier-ngb2`.
- Command transcript shows `cargo fmt -- --check`, focused nextest suites,
  `atelier lint`, `atelier export --check`, `atelier doctor`, and
  `git diff --check` pass.
