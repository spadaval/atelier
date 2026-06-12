---
created_at: "2026-06-10T16:00:59.351671663+00:00"
id: "atelier-t79u"
issue_type: "task"
labels:
- "activity"
- "architecture"
- "docs"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-krhk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Document issue activity history"
updated_at: "2026-06-10T17:38:39.026306267+00:00"
---

## Description

Document the issue activity history model and operator workflows.

What:
- Update product/domain/architecture docs as appropriate for issue-centered activity history.
- Document activity sidecar paths, front matter fields, event types, timestamp IDs, evidence references, history CLI behavior, rebuild/export expectations, and the one-off migration script.
- Make clear that V1 is issue-centered only and that ordinary issue plans are activity entries, while first-class plan records remain for cross-issue, mission-level, migration, or multi-agent coordination plans.

Out of scope:
- Changing behavior without matching implementation issues.

## Outcome

- Docs explain the canonical format well enough for a future agent to add or debug activity records.
- Operator docs include migration-script usage and safety expectations.
- Documentation matches implemented CLI flags and JSON behavior once the implementation issues land.

Recommended subskill: agent-factory docs.

## Evidence

Evidence was not specified in the legacy issue record.
