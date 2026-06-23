---
created_at: "2026-06-23T20:16:52.831565225+00:00"
id: "atelier-59vp"
issue_type: "feature"
labels:
- "cli"
- "tracker"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ih42"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Enforce mission and epic hierarchy rules"
updated_at: "2026-06-23T20:16:52.831565225+00:00"
---

## Description

Enforce the fixed hierarchy model in issue creation, issue parent updates, bundle preview/apply, rebuild/lint validation, and recovery diagnostics. Missions cannot have parents or children; epics cannot have parents and cannot be children; non-epic issue types can be standalone or direct children of epics; non-epic issue types cannot own children.

## Outcome

- Invalid mission/epic hierarchy shapes are rejected with clear fix commands, existing invalid canonical records are reported by lint/rebuild, and ordinary standalone issues remain valid.
- `atelier bundle preview` and `atelier bundle apply` reject bundle issue parents that would create mission-owned children, task-owned children, epic parents, or nested epics.
- Bundle-created mission scope uses `advances` links only; bundle input cannot express mission work through `parent`.
