---
created_at: "2026-07-06T20:37:49.345233020+00:00"
id: "atelier-72k4"
issue_type: "feature"
labels:
- "cli"
- "mission-review"
- "subskill-implement"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-qi40"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Explain mission review and dependency readiness failures"
updated_at: "2026-07-06T20:37:49.345233020+00:00"
---

## Description

Assigned subskill: implement. Update normal issue detail, mission work, ready-work, transition, manager, planner, and reviewer surfaces so operators do not need private knowledge or raw validator names to recover from a blocked plan. Keep verbose diagnostics available without crowding the normal path.

## Outcome

- Normal output distinguishes review not requested, approval missing, self-approval rejected, blocking findings unresolved, approval stale, direct blocker open, and transitive blocker open.
- Each failure names the relevant mission, review or finding, blocker path, and one state-appropriate next command.
- Ready-work output never presents review-blocked or dependency-blocked work as immediately executable.

## Evidence

Evidence was not specified in the bundle.
