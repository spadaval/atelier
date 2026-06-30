---
created_at: "2026-06-29T17:37:41.950919379+00:00"
id: "atelier-ef94"
issue_type: "epic"
labels:
- "mission"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2uim"
  children:
  - kind: "issue"
    id: "atelier-1a2n"
  - kind: "issue"
    id: "atelier-fxhy"
  - kind: "issue"
    id: "atelier-v132"
  - kind: "issue"
    id: "atelier-w0mr"
  - kind: "issue"
    id: "atelier-xbpd"
  - kind: "issue"
    id: "atelier-zjnp"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1mga"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Simplify mission validation and closeout"
updated_at: "2026-06-29T17:47:53.987890864+00:00"
---

## Description

Mission closeout stops depending on mission-authored validation prose or direct mission evidence. Close readiness comes from linked work state, blockers, configured health gates, and linked validation work when independent judgment is required.

## Outcome

Mission validation and closeout are derived from real work state. Closeout checks linked work, blockers, configured health gates, and linked validation work when required. It does not depend on a mission `Validation` section, direct mission evidence, or planner-authored proof checklists.
