---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-0001"
issue_type: "task"
labels:
- "spec"
- "validator"
- "workflow"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000l"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T20:42:33.793331698+00:00"
status: "done"
title: "Decide the default workflow for non-mission issue tasks"
updated_at: "2026-06-11T20:42:33.793331698+00:00"
---

## Description

Resolve the SPEC.md open question about how much process ordinary single-issue
tasks should have by default when they are not part of a mission. Missions are
for work too large for a single agent context; this resolution defines the
lightweight issue workflow that keeps small tasks out of mission ceremony while
still allowing configured stricter workflows for mission-linked or high-risk
work.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.
### Resolution

Single-issue work does not use the mission system. It follows the ordinary issue
workflow gates configured for issues, usually checks such as clean worktree,
claim/work association, no open blockers, and fresh durable tracker state.
Default single-issue work does not require heavier PR-level or mission-level
gates such as `tests_passed` unless repository workflow policy explicitly opts
that issue type into stricter validation.

### Rationale

Atelier must be risk-scaled: strict where coordination/correctness needs it,
but lightweight enough that small tasks do not become red tape. The baseline
case is an issue that can be planned, claimed, implemented, validated, and
closed as one accountable unit without creating or joining a mission. If a piece
of work needs complex validation, cross-slice coordination, PR-level gates, or
durable closeout proof, it usually should be represented as mission work instead
of making the default issue path heavy.

### Alternatives Considered

- Minimal open/done workflow.
- Require evidence only at close.
- Require workflow validators only for configured types.
- Use one universal workflow for all tasks.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
