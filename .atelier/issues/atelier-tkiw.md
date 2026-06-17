---
created_at: "2026-06-17T20:03:38.792621462+00:00"
id: "atelier-tkiw"
issue_type: "task"
labels:
- "bundle"
- "implementation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-mrj5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Enforce v1 bundle schema without plans or milestones"
updated_at: "2026-06-17T21:28:57.366313869+00:00"
---

## Description

Enforce the v1 bundle schema after first-class plan and milestone records
are removed. The schema should create only supported tracker graph records and
relationships.

## Outcome

- Bundle records support only the v1-approved resource kinds.
- The accepted schema is `atelier.bundle`.
- `plans`, `milestones`, `mission.plans`, `mission.milestones`, plan
  `applies_to`, plan `supersedes`, milestone `missions`, and milestone
  `contributing_work` are removed from the accepted schema.
- Validation errors name the unsupported field and point to the current schema.

## Evidence

- Fixture tests reject files containing removed plan or milestone fields.
- Fixture tests cover the supported resource set and relationship fields.
