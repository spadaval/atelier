---
created_at: "2026-06-19T22:42:56.476008420+00:00"
id: "atelier-pv77"
issue_type: "task"
labels:
- "actions"
- "git"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement git transition actions"
updated_at: "2026-06-19T22:42:56.476008420+00:00"
---

## Description

Implement built-in git actions for branch lifecycle behavior expressed in transitions. Cover creating/checking out an issue or epic branch, checking out a parent branch when configured, committing tracker changes when required, and integrating the source branch to the target branch using the configured strategy.

## Outcome

- Built-in git transition actions cover the branch lifecycle behavior that workflow policy declares.
- Failed git actions leave issue status unchanged and print recovery guidance.

## Evidence

- Integration tests show an epic workflow creates an epic branch on start and integrates it on close.
- Integration tests show a child workflow can explicitly check out the parent branch through an action.
- Integration tests assert action failures preserve status and print recovery text.
