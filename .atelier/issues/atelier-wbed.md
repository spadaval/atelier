---
created_at: "2026-06-14T16:31:06.664802074+00:00"
id: "atelier-wbed"
issue_type: "feature"
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
status: "validation"
title: "Route proof approval through workflow validation"
updated_at: "2026-06-14T17:22:28.057195526+00:00"
---

## Description

When parent-level or mission-level proof requires judgment, model that judgment as validation or review workflow state and attached evidence, not as core text matching.

## Outcome

- When parent-level or mission-level proof needs judgment, the required
  judgment is represented as validation/review workflow state on accountable
  issue-shaped work.
- A pass/fail validation transition or explicit validation evidence record is
  the durable approval signal.
- Core closeout code checks for the workflow-approved artifact, not for
  semantic equivalence between prose claims and evidence summaries.

## Evidence

- File diff in `.atelier/workflow.yaml`, `docs/product/workflow-configuration.md`,
  or `docs/product/work-model.md` shows how parent-level proof approval is
  represented without heuristic claim matching.
- Focused tests or transcripts demonstrate a validation/review item approving
  parent proof and a closeout gate accepting that approval.
- `git diff --check` and `atelier lint` pass.
