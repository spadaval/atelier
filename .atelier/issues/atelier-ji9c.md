---
created_at: "2026-06-19T22:42:56.456718894+00:00"
id: "atelier-ji9c"
issue_type: "epic"
labels:
- "implementation"
- "issue-types"
- "workflow-policy"
review:
  kind: pull_request
  number: 10
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kmmv"
  - kind: "issue"
    id: "atelier-qx40"
  children:
  - kind: "issue"
    id: "atelier-b9i4"
  - kind: "issue"
    id: "atelier-bmqo"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Epic: Implement custom issue types"
updated_at: "2026-06-20T01:36:05.824729976+00:00"
---

## Description

Implement repo-defined issue types in workflow policy and remove hard-coded built-in issue-type assumptions from record parsing, issue creation, bundle validation, and workflow applicability.

## Outcome

- Custom issue types are first-class repo policy, not hard-coded Rust constants.
- Every registered issue type is covered exactly once by workflow applicability.

## Evidence

- Workflow loader accepts an explicit issue_types registry and rejects issues whose type is not registered.
- Records, bundle apply/preview, issue create/update, and Agent Factory issue creation validate against the repo registry.
- Tests prove a custom issue type can be created, assigned to a workflow, shown, listed, and transitioned.
