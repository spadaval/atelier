---
acceptance: []
blocks:
- "atelier-000r"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-0009"
- "atelier-001a"
- "atelier-001f"
evidence_required: []
id: "atelier-000j"
issue_type: "task"
labels:
- "domain-model"
- "epic"
- "milestone"
- "spec"
links: []
parent: null
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Milestone 4: First-class records and typed links"
updated_at: "2026-06-09T19:41:24.611564297+00:00"
---

Introduce first-class missions, milestone checkpoint records, plans, evidence, workflow validators, and typed links so generic issues stop carrying every durable concept. Preserve existing issue behavior where useful, but make mission intent, checkpoint state, execution plans, validation proof, workflow transition checks, and semantic links explicit records or commands.

Milestones are validated target states, not work containers or super-epics. Workflow validators belong to workflow policy and enforce transitions; milestone records own validation criteria.

Acceptance:
Data model and CLI support first-class records for missions, milestone checkpoints, plans, evidence, workflow validators, and typed links, or explicitly staged subsets. Issues are no longer the only durable representation for non-issue concepts. Compatibility migration paths are defined where reasonable. Tests cover persistence, links, export/rebuild, JSON output, evidence validation, and workflow validator results.
