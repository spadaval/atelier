---
created_at: "2026-06-19T22:42:56.480796525+00:00"
id: "atelier-kmmv"
issue_type: "epic"
labels:
- "migration"
- "workflow-policy"
review:
  kind: pull_request
  number: 11
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4xue"
  children:
  - kind: "issue"
    id: "atelier-ih2n"
  - kind: "issue"
    id: "atelier-qx40"
  - kind: "issue"
    id: "atelier-zu0t"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Epic: Migrate workflow policy to cleaned-up model"
updated_at: "2026-06-20T02:04:58.531224155+00:00"
---

## Description

Migrate the repository workflow policy and starter policy to the cleaned-up model after schema and executor support lands.

## Outcome

- The repository workflow policy and starter policy use the cleaned-up model after action, validator, issue-type, and config-boundary support lands.
- Migration removes stale `effects`, old validator names, copied workflow shapes, and tracked runtime-path policy from the committed configuration.

## Evidence

- .atelier/workflow.yaml uses issue_types, namespaced validators, actions, explicit descriptions, and active rollup categories for review/validation statuses.
- Workflow names are direct and readable.
- Validation issue workflow is purpose-built rather than copied from epic workflow.
