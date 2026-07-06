---
created_at: "2026-07-06T17:20:25.691397620+00:00"
id: "atelier-4fip"
issue_type: "epic"
labels:
- "cli"
- "human-output"
- "mission-dashboard"
fields:
  workflow_branch:
    branch_base: mission/atelier-c0mp
    integration_target: mission/atelier-c0mp
    merge_strategy: squash
    owner_issue_id: atelier-4fip
    owner_kind: epic
    review_target: mission/atelier-c0mp
    work_branch: epic/atelier-4fip
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g5fl"
  children:
  - kind: "issue"
    id: "atelier-dy3u"
  - kind: "issue"
    id: "atelier-sjsz"
  - kind: "issue"
    id: "atelier-tdgs"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Build the formatted Mission Overview"
updated_at: "2026-07-06T17:57:59.638708573+00:00"
---

## Description

Evolve `atelier work missions` from a thin mission-type inventory alias into the high-level operational overview for all current missions. This epic owns the mission/epic projection, default filtering, collapsed summaries, formatting, semantic color, help, and focused coverage.

## Outcome

- `atelier work missions` answers which missions and epics constitute the current backlog without printing leaf subtasks by default.
- The default omits done missions, an explicit option includes them, and every displayed count and state is derived from the same mission `advances` and epic-child facts used by scoped dashboards.
- The Mission Overview is visually structured and consistently colored through shared output helpers, with complete colorless behavior.

## Evidence

Evidence was not specified in the bundle.
