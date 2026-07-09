---
created_at: "2026-07-06T20:37:49.340906873+00:00"
id: "atelier-amfw"
issue_type: "feature"
labels:
- "dependencies"
- "readiness"
- "subskill-implement"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3v2d"
  - kind: "issue"
    id: "atelier-72k4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Enforce direct and transitive dependency readiness"
updated_at: "2026-07-06T20:37:49.340906873+00:00"
---

## Description

Assigned subskill: implement. Add cycle-safe dependency-closure evaluation to executable transitions. Mission start must reject any incomplete declared mission blocker in the direct or transitive closure. Issue work selection and direct issue start must reject incomplete declared issue dependencies. Internal mission work is not required to be complete before the mission shell starts. Completion uses configured terminal statuses rather than hard-coded status names.

## Outcome

- A mission with an incomplete direct or transitive declared blocker cannot start, and the rejection identifies the blocking path.
- An issue with an incomplete declared dependency is neither selectable as ready work nor startable through a direct transition.
- Completed dependency closures permit execution, unrelated mission work is not mistaken for a prerequisite, and cycles fail safely with actionable diagnostics.

## Evidence

Evidence was not specified in the bundle.
