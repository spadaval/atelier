---
created_at: "2026-06-13T00:15:44.796943006+00:00"
id: "atelier-a4rz"
issue_type: "task"
labels:
- "cli"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair active-work root status guidance"
updated_at: "2026-06-13T00:15:44.796943006+00:00"
---

## Description

Repair the root status edge found during the `atelier-trr2` signpost validation
rerun: when an issue already has active work, `atelier status` must not still
classify that same issue as ready or suggest starting it again.

## Outcome

- Root `atelier status` excludes the active work item from ready-work counts and
  the Ready In Active Mission list.
- When active work belongs to the active mission, root status reports that work
  as active mission progress and suggests `atelier finish <id>` instead of
  `atelier start <id>` for the same issue.
- Normal status next actions still avoid raw workflow-validator commands.
- Focused transcript tests cover active-work root status guidance.

## Evidence

- Focused CLI integration test for active-work root status guidance.
- Manual or captured transcript for `atelier status` while work is active.
- `atelier lint`, `atelier export --check`, and relevant focused signpost tests
  pass.
