---
created_at: "2026-06-20T16:54:46.619043460+00:00"
id: "atelier-19xa"
issue_type: "task"
labels:
- "cutting-pass"
- "mission-collapse"
- "relationships"
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
closed_at: "2026-06-20T19:40:57.499256414+00:00"
status: "done"
title: "Replace mission relationship commands with a general link surface"
updated_at: "2026-06-20T19:40:57.499256414+00:00"
---

## Description

Replace mission-specific relationship mutation commands with a general link
surface or type-aware issue relationship update flow. The replacement must
cover mission work links and mission blocker links without preserving
`mission add-work`, `mission unlink`, and `mission add-blocker` as bespoke
verbs.

## Outcome

- Operators can add/remove `advances` work links and `blocked_by` blocker links
  for mission objectives through a general relationship command or type-aware
  issue update/link surface.
- Relationship output names both records, relation type, and next inspection
  commands.
- Mission-specific relationship commands have documented replacements.

## Evidence

- Focused tests prove adding/removing mission work and mission blocker
  relationships through the replacement surface.
- `target/debug/atelier issue show <mission-id>` or the replacement status view
  renders the new relationships.
- Command audit records the removed mission relationship commands and their
  replacements.
