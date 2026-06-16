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
closed_at: "2026-06-16T20:16:58.665812744+00:00"
status: "done"
title: "Remove closeout issue type and migrate tracker records"
updated_at: "2026-06-16T20:16:58.665812744+00:00"
---

## Description

Remove closeout as a valid issue type and migrate committed tracker state to supported issue types.

## Outcome

- Closeout is removed from built-in issue type lists, workflow policy defaults, storage validation, CLI help, and `.atelier/workflow.yaml`.
- Existing canonical issues using the former closeout issue type are migrated to `validation`, `task`, or another explicit supported type based on their actual work.
- No compatibility alias or old-command shim is added.
- Tracker history remains readable after the migration.

## Evidence

- Command transcript from targeted `rg` over live tracker/config/code returns no former closeout issue-type records or mappings.
- `atelier lint` and `atelier export --check` pass after the migration.
- Focused tests prove creating or updating an issue to `closeout` is rejected and migrated records still parse.
