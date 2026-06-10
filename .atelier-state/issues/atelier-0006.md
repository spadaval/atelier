---
acceptance: []
blocks:
- "atelier-0003"
- "atelier-001o"
- "atelier-001x"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-0005"
- "atelier-0007"
- "atelier-000f"
- "atelier-000l"
- "atelier-000n"
- "atelier-000u"
evidence_required: []
id: "atelier-0006"
issue_type: "task"
labels:
- "config"
- "feature"
- "json"
- "mission-control"
- "spec"
- "validator"
links: []
parent: "atelier-000c"
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement Mission Control JSON projection"
updated_at: "2026-06-09T20:45:16.086072579+00:00"
---

Generate mission-control.json or equivalent command JSON that summarizes active missions, milestone checkpoint progress, blockers, workflow/config health, branches, worktrees, stale durable-state projections, required evidence, workflow validator failures, plan drift, recent decisions, and review/validation queues.

Mission Control should consume the same first-class mission records and typed links used by the mission progress view. It must not infer missions from issue labels or parent trees except through documented compatibility migration behavior. Mission rows and details should make objective health legible: done work, ready work, blocked work, backlog, checkpoint progress, active plans, evidence gaps, validator failures, risks, and suggested next CLI actions.

Acceptance:
Projection schema is documented. Output is deterministic and stable for agents. Tests or fixtures cover missions, milestone progress, linked plans, done/ready/blocked/backlog work, blockers, workflow/config health, branches/worktrees, stale durable-state projections, evidence, validator failures, plan drift, recent decisions, and ready-for-review/validation queues. Projection is integrated with export/rebuild if .atelier-state/mission-control.json is canonical. Docs explicitly state that live agent-run tracking is not required for this milestone.
