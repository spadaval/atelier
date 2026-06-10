---
acceptance: []
blocks:
- "atelier-000l"
created_at: "2026-06-08T17:33:27+00:00"
depends_on: []
evidence_required: []
id: "atelier-0001"
issue_type: "task"
labels:
- "decision"
- "spec"
- "validator"
- "workflow"
links: []
parent: null
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Decide the default workflow for tiny tasks"
updated_at: "2026-06-09T19:42:48.124135789+00:00"
---

Resolve the SPEC.md open question about how much process tiny tasks should have by default, including start requirements, close requirements, evidence expectations, workflow validator requirements, and waiver behavior.

## Decision

TODO: define the default lightweight workflow for tiny tasks.

## Rationale

Atelier must be risk-scaled: strict where coordination/correctness needs it, but lightweight enough that small tasks do not become red tape.

## Alternatives Considered

- Minimal open/done workflow.
- Require evidence only at close.
- Require workflow validators only for configured types.
- Use one universal workflow for all tasks.
