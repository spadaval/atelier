---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000i"
issue_type: "task"
labels:
- "deferred"
- "domain-model"
- "spec"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-001m"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T13:14:54.372653798+00:00"
status: "done"
title: "Decide future session export and run metadata policy"
updated_at: "2026-06-11T13:14:54.372653798+00:00"
---

## Description

Resolve the deferred question of whether sessions/runs are exported, partially exported, or treated as local runtime metadata after the first workflow/evidence model exists.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.
### Resolution

Deferred. Direct live agent-run tracking is out of scope for the current milestones. The first domain-model and Mission Control slices should not depend on durable run/session records.

### Rationale

Runs may eventually inform Mission Control, retry behavior, token accounting, and operator diagnostics. Deferring this prevents premature coupling between durable work records and live agent execution details.

### Alternatives Considered

- Export sessions as durable records.
- Export summarized run metadata only.
- Keep sessions local and project only aggregate state.
- Make export behavior configurable by workflow.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
