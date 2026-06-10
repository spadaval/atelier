---
acceptance: []
blocks:
- "atelier-001v"
created_at: "2026-06-09T19:47:26.301123908+00:00"
depends_on:
- "atelier-0005"
- "atelier-001i"
- "atelier-001u"
evidence_required: []
id: "atelier-001t"
issue_type: "task"
labels:
- "bulk"
- "links"
- "milestone"
- "mission"
- "plan"
- "task"
links: []
parent: "atelier-001n"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Extend bulk apply to typed links and first-class records"
updated_at: "2026-06-09T20:45:21.716823162+00:00"
---

Extend bulk apply beyond issue hierarchy so authored JSON can create or link missions, milestone checkpoints, plans, evidence, and typed semantic links when those record types are available.

Bulk apply should support mission-centered graph construction: create a mission, attach checkpoint milestones and durable plans, create contributing epics/issues, and connect evidence or validation requirements with explicit typed links. This is the authored-plan path for materializing a coherent mission graph before durable IDs exist.

Acceptance: internal references work across record kinds, typed-link validation is reused, unsupported record kinds fail clearly, and fixtures cover mission-to-plan, mission-to-milestone, mission-to-work, plan-to-work, and evidence-to-validation linkage.
