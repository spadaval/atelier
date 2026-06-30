---
created_at: "2026-06-30T16:45:29.442320703+00:00"
id: "atelier-3hfq"
issue_type: "task"
labels:
- "tracker"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove tracker.current from workflow policy"
updated_at: "2026-06-30T16:46:04.714754960+00:00"
---

## Description

Remove the projection freshness validator from workflow policy. Projection freshness is an internal storage concern handled by command storage before mutating commands, not a user-configurable lifecycle rule.

## Outcome

- `tracker.current` is removed from `.atelier/workflow.yaml`, the starter/default workflow, supported built-in validator lists, docs, and tests.
- `baseline.default_checks` no longer runs or reports the projection freshness check as a workflow validator.
- Mutating workflow commands still open storage through the canonical-mutation path so stale projections are repaired or rejected before transition logic runs.
- Mission and epic close examples only expose domain, review, Git, lint, and work-completion policy validators.

## Evidence

- Focused tests cover workflow policy without `tracker.current` and command-storage projection freshness behavior.
- `target/debug/atelier check <issue-id>` passes.
- `target/debug/atelier check atelier-sszj` passes after the tracker update.
