---
created_at: "2026-06-17T20:03:24.347689663+00:00"
id: "atelier-uwb6"
issue_type: "task"
labels:
- "artifact-update"
- "records"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-a3e7"
  - kind: "issue"
    id: "atelier-v7d0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T22:43:06.506911616+00:00"
status: "done"
title: "Document plan and milestone removal contract"
updated_at: "2026-06-17T22:43:06.506911616+00:00"
---

## Description

Update the durable product and architecture contract before deleting code. The
decision is that first-class plan records and milestone/checkpoint records are
not part of v1. Plans should be ordinary Markdown artifacts referenced from
accountable work or evidence; validation and outcome data should stay on issues,
epics, missions, and evidence.

## Outcome

- `SPEC.md`, `CONTEXT.md`, product docs, architecture docs, and docs index no
  longer describe first-class plan or milestone records as active v1 behavior.
- The docs explain how to reference plan Markdown from issue/mission/evidence
  bodies without a plan record table.
- The docs explain that milestone/checkpoint semantics are deferred and not a
  validation-data destination.

## Evidence

- Targeted search transcript over docs and tracker records shows remaining
  plan/milestone references are historical, removed-command notes, or explicitly
  deferred.
- `atelier lint` and `git diff --check` pass for the artifact update.
