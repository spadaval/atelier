---
created_at: "2026-06-19T22:42:56.454130436+00:00"
id: "atelier-jwvd"
issue_type: "task"
labels:
- "docs"
- "workflow-policy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-bmqo"
  - kind: "issue"
    id: "atelier-cko9"
  - kind: "issue"
    id: "atelier-ee4u"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Update workflow policy docs for target schema"
updated_at: "2026-06-19T23:41:33.297267291+00:00"
---

## Description

Update product and domain documentation for the new workflow policy language. Cover `issue_types`, user-facing statuses, internal categories, namespaced validators, transition `actions`, static descriptions, and workflow examples that include validation issues with their own workflow.

## Outcome

- Documentation makes repo-defined issue types explicit.
- Documentation distinguishes validators from actions and statuses from categories.

## Evidence

- docs/product/workflow-configuration.md contains the new schema and examples.
- SPEC.md and CONTEXT.md use the new vocabulary consistently.
- No docs present `review` or `validation` as required global categories.
