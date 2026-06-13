---
created_at: "2026-06-09T19:47:26.301123908+00:00"
id: "atelier-001t"
issue_type: "task"
labels:
- "bulk"
- "links"
- "milestone"
- "mission"
- "plan"
- "task"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-001v"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T15:05:38.585686715+00:00"
status: "done"
title: "Extend bulk apply to typed links and first-class records"
updated_at: "2026-06-10T15:05:38.585686715+00:00"
---

## Description

Extend bulk apply beyond issue hierarchy so authored JSON can create or link missions, milestone checkpoints, plans, evidence, and typed semantic links when those record types are available.

Bulk apply should support mission-centered graph construction: create a mission, attach checkpoint milestones and durable plans, create contributing epics/issues, and connect evidence or validation requirements with explicit typed links. This is the authored-plan path for materializing a coherent mission graph before durable IDs exist.

## Outcome

internal references work across record kinds, typed-link validation is reused, unsupported record kinds fail clearly, and fixtures cover mission-to-plan, mission-to-milestone, mission-to-work, plan-to-work, and evidence-to-validation linkage.

## Evidence

Evidence was not specified in the legacy issue record.
