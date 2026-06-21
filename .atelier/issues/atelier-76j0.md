---
created_at: "2026-06-21T16:37:30.771488742+00:00"
id: "atelier-76j0"
issue_type: "validation"
labels:
- "mission-rework"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Run mission rework scenario validation"
updated_at: "2026-06-21T20:14:15.580744229+00:00"
---

## Description

Independently validate the new mission/objective workflow from a clean checkout-style path.

## Outcome

- Validation transcript creates or uses a declared mission/objective type, links work, inspects status, records evidence, exercises blockers, and reaches terminal readiness.
- Validation transcript proves old mission-only commands do not remain as supported workflow.
- Residual risks and any follow-up issue IDs are recorded.

## Evidence

- First-class evidence record with command transcripts and claim-by-claim result classification validates this issue.
- `atelier lint` passes after validation evidence is attached.
