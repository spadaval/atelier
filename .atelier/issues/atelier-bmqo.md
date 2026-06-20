---
created_at: "2026-06-19T22:42:56.459230718+00:00"
id: "atelier-bmqo"
issue_type: "task"
labels:
- "schema"
- "workflow-policy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-b9i4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add workflow issue_types registry"
updated_at: "2026-06-19T22:42:56.459230718+00:00"
---

## Description

Add the `issue_types` schema to `.atelier/workflow.yaml` and workflow policy parsing. Validate names, labels, and exact workflow coverage from the registry.

## Outcome

- Workflow policy defines an explicit `issue_types` registry with validated names and labels.
- Workflow applicability must cover every registered issue type exactly once and reject unknown issue types in records.

## Evidence

- Unit tests cover valid custom issue types, missing registry entries, duplicate workflow coverage, and unknown issue_type values in records.
