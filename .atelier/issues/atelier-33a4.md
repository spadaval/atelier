---
created_at: "2026-06-19T22:42:56.448139225+00:00"
id: "atelier-33a4"
issue_type: "epic"
labels:
- "architecture"
- "workflow-policy"
review:
  kind: pull_request
  number: 7
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cin6"
  - kind: "issue"
    id: "atelier-ji9c"
  - kind: "issue"
    id: "atelier-yhui"
  children:
  - kind: "issue"
    id: "atelier-h7n4"
  - kind: "issue"
    id: "atelier-jwvd"
  - kind: "issue"
    id: "atelier-vzrj"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Epic: Define workflow policy target model"
updated_at: "2026-06-19T23:50:38.433139469+00:00"
---

## Description

Define the target workflow-policy model before implementation. Resolve the semantics for repo-defined issue types, user-facing statuses, internal rollup categories, namespaced validators, transition actions, and branching as workflow actions rather than a parallel branch-policy system.

## Outcome

- A durable target-state artifact defines issue type registry semantics, status/category boundaries, validator/action naming, review validator behavior, and workflow-driven branching.
- Dependent implementation issues can be completed without relying on private chat context.

## Evidence

- Updated ADR, SPEC, CONTEXT, and workflow-configuration docs describe the target model and rejected alternatives.
- Docs explicitly state that statuses are repo-defined user-facing workflow state and categories are high-level rollup or implementation metadata.
- Docs explicitly state that actions are built-in transition-time mutations, not arbitrary hooks.
