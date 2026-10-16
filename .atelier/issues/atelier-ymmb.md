---
created_at: "2026-10-15T22:08:58.988204415+00:00"
id: "atelier-ymmb"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-ymmb
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9vq3"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "List only startable work in work ready"
updated_at: "2026-10-16T00:56:26.559641076+00:00"
---

## Goal

`atelier work ready` lists missions and epics next to tasks. Missions own no branch, and epics are worked through their children, so neither is something to start.

## Done when

`work ready` lists items that `atelier start` would accept; missions and parents appear only as context (for example a heading or a flag such as `--all`). A test covers a mission with a ready epic and child.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Seen in astromech: both missions and several epics appeared in the ready list. May be shaped by atelier-9vq3.

## Notes

- 2026-10-15 [observation] From review finding atelier-imxz on atelier-ha6k: a todo-category status that cannot start (backlog, draft) is classified Blocked when it has open blockers, so parked work shows in work blocked. When reshaping the ready/blocked lists, classify non-startable statuses as backlog even with blockers, keeping the blocker count.
