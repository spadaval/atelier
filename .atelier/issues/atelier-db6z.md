---
created_at: "2026-06-20T16:54:12.894520655+00:00"
id: "atelier-db6z"
issue_type: "task"
labels:
- "cutting-pass"
- "mission-collapse"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v2o6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T19:36:32.465670249+00:00"
status: "done"
title: "Add type-aware issue status view for mission objectives"
updated_at: "2026-06-20T19:36:32.465670249+00:00"
---

## Description

Add a type-aware computed status view for mission-shaped objective issues. This
view should preserve the useful mission status information without requiring a
mission-specific command namespace.

## Outcome

- The general issue/status surface can render mission objective health,
  linked work buckets, active work, mission blockers, evidence gaps, terminal
  readiness, and next actions.
- Root `status` can point to the record-specific status view without depending
  on active mission focus.
- The existing `mission status` view has a documented replacement path.

## Evidence

- Focused CLI tests prove the replacement status view includes linked work,
  blockers, evidence gaps, terminal checks, and next actions for a mission
  objective.
- Command-audit docs name the replacement for `mission status`.
