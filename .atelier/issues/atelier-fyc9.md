---
created_at: "2026-06-21T16:37:30.767887580+00:00"
id: "atelier-fyc9"
issue_type: "feature"
labels:
- "mission-rework"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Normalize mission work and blocker relationships under issue commands"
updated_at: "2026-06-21T16:37:30.767887580+00:00"
---

## Description

Finish the replacement of mission add-work, unlink, and add-blocker surfaces with general issue link/block commands.

## Outcome

- `atelier issue link <mission-id> <issue-id> --role advances` records mission work.
- `atelier issue unlink <mission-id> <issue-id> --role advances` removes mission work.
- `atelier issue block <mission-id> <blocker-id>` records direct mission blockers without confusing them with issue-owned blockers.
- Wrong-kind and invalid-role errors name the accepted relationship shape.

## Evidence

- Focused CLI tests cover add/remove work, direct blockers, wrong-kind IDs, and status rendering.
- `atelier lint` passes.
