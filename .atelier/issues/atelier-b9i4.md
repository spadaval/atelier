---
created_at: "2026-06-19T22:42:56.461594169+00:00"
id: "atelier-b9i4"
issue_type: "task"
labels:
- "cli"
- "issue-types"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Use repo-defined issue types across CLI surfaces"
updated_at: "2026-06-20T01:28:31.004125443+00:00"
---

## Description

Migrate issue creation, issue update, bundle validation, Agent Factory issue creation, docs, and help text to use repository-defined issue types.

## Outcome

- CLI surfaces that create, update, validate, or document issues use the repository issue-type registry instead of hard-coded built-in issue types.
- Help text and validation errors report the configured issue types for the current repository.

## Evidence

- CLI integration tests create and transition at least one custom issue type.
- CLI integration tests assert help and validation errors name repository-defined issue types instead of a hard-coded built-in list.
