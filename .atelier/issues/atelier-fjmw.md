---
created_at: "2026-06-15T05:13:32.914496453+00:00"
id: "atelier-fjmw"
issue_type: "task"
labels:
- "rust"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T07:40:52.931963674+00:00"
status: "done"
title: "Extract workflow policy into atelier-workflow"
updated_at: "2026-06-15T07:40:52.931963674+00:00"
---

## Description

Extract workflow policy parsing, status categories, transition validation, and starter policy constants into `atelier-workflow`.

## Outcome

- `atelier-workflow` depends on `atelier-core` and owns workflow policy semantics.
- Application and CLI code call workflow APIs instead of reaching into old monolith modules.
- Tests that used `atelier::workflow_policy::STARTER_POLICY_YAML` move to the new crate path.

## Evidence

- Workflow unit tests pass in the new crate.
- Search transcript shows old single-crate workflow paths are removed from repo tests.
- Representative `issue transition --options` and workflow validation CLI tests pass.
