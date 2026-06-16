---
created_at: "2026-06-15T21:31:04.053617759+00:00"
id: "atelier-z0yu"
issue_type: "feature"
labels:
- "migration"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove closeout issue type and migrate tracker records"
updated_at: "2026-06-15T21:31:04.053617759+00:00"
---

## Description

Remove closeout as a valid issue type and migrate committed tracker state to supported issue types.

## Outcome

- Closeout is removed from built-in issue type lists, workflow policy defaults, storage validation, CLI help, and `.atelier/workflow.yaml`.
- Existing canonical issues with `issue_type: "closeout"` are migrated to `validation`, `task`, or another explicit supported type based on their actual work.
- No compatibility alias or old-command shim is added.
- Tracker history remains readable after the migration.

## Evidence

- `rg "issue_type: \"closeout\"|issue_types:.*closeout|closeout: standard_review_proof"` over live tracker/config/code returns no live matches.
- `atelier lint` and `atelier export --check` pass after the migration.
- Focused tests prove creating or updating an issue to `closeout` is rejected and migrated records still parse.
