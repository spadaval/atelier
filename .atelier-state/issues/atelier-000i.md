---
acceptance: []
blocks:
- "atelier-001m"
created_at: "2026-06-08T17:33:27+00:00"
depends_on: []
evidence_required: []
id: "atelier-000i"
issue_type: "task"
labels:
- "decision"
- "deferred"
- "domain-model"
- "spec"
links: []
parent: null
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Decide future session export and run metadata policy"
updated_at: "2026-06-11T13:14:54.372653798+00:00"
---


Resolve the deferred question of whether sessions/runs are exported, partially exported, or treated as local runtime metadata after the first workflow/evidence model exists.

## Decision

Deferred. Direct live agent-run tracking is out of scope for the current milestones. The first domain-model and Mission Control slices should not depend on durable run/session records.

## Rationale

Runs may eventually inform Mission Control, retry behavior, token accounting, and operator diagnostics. Deferring this prevents premature coupling between durable work records and live agent execution details.

## Alternatives Considered

- Export sessions as durable records.
- Export summarized run metadata only.
- Keep sessions local and project only aggregate state.
- Make export behavior configurable by workflow.
