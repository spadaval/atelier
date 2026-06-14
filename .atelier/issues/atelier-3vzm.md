---
created_at: "2026-06-14T21:44:54.240353912+00:00"
id: "atelier-3vzm"
issue_type: "task"
labels:
- "docs"
- "tests"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update workflow help and tests for issue proof versus epic review"
updated_at: "2026-06-14T21:44:54.240353912+00:00"
---

## Description

Update CLI help, product docs, and integration tests to describe issue-local proof and epic-level review after the workflow policy and epic closeout behavior are implemented. Outcome: users no longer see ordinary issue review as the default path, and transition help points review work at epics. Evidence: help transcript tests and docs/help parity checks prove the public guidance matches the implemented workflow.

## Outcome

- CLI help and product docs describe issue proof versus epic review using the same terms.
- Transition help points review and validation work at epics by default.
- Tests fail if ordinary issue help reintroduces mandatory review language.

## Evidence

- Help transcript test proves issue close/start guidance does not require default review.
- Help transcript test proves epic transition guidance includes review and validation.
- Docs/help parity check or targeted transcript verifies the documented workflow language.
