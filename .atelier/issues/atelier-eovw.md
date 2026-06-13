---
created_at: "2026-06-13T19:39:11.362324974+00:00"
id: "atelier-eovw"
issue_type: "bug"
labels:
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T19:48:30.850231540+00:00"
status: "done"
title: "Create new issues with configured workflow initial status"
updated_at: "2026-06-13T19:48:30.850231540+00:00"
---

## Description

After `.atelier/workflow.yaml` exists, `atelier issue create` still writes
legacy `status: "open"` into new canonical issue records. The next workflow
transition then rejects the freshly created issue as unmigrated and tells the
operator to rerun `atelier workflow migrate-statuses`.

This follow-up came from independent validation of `atelier-fyms`.
- New issue creation uses the configured workflow `initial_status` for the
  issue type, such as `todo` for the starter policy.
- A freshly created issue in a workflow-enabled repository can immediately run
  `atelier issue transition <id> --options` and `atelier start <id>` without a
  one-off migration.
- Existing status migration behavior remains available for legacy records.
- Focused CLI test or transcript creates an issue after `atelier workflow init`
  and `atelier workflow migrate-statuses`, then proves the new issue is already
  in the configured initial workflow state.
- Regression proof shows `atelier workflow migrate-statuses` still maps legacy
  `open`, `closed`, and `archived` records as before.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
