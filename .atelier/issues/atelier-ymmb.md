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
  - kind: "review"
    id: "atelier-imxz"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T01:01:03.836138476+00:00"
status: "done"
title: "List only startable work in work ready"
updated_at: "2026-10-16T01:01:03.836138476+00:00"
---

## Goal

`atelier work ready` lists missions and epics next to tasks. Missions own no branch, and epics are worked through their children, so neither is something to start.

## Done when

`work ready` lists items that `atelier start` would accept; missions and parents appear only as context (for example a heading or a flag such as `--all`). A test covers a mission with a ready epic and child.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Seen in astromech: both missions and several epics appeared in the ready list. May be shaped by atelier-9vq3.

## Notes

- 2026-10-15 [observation] From review finding atelier-imxz on atelier-ha6k: a todo-category status that cannot start (backlog, draft) is classified Blocked when it has open blockers, so parked work shows in work blocked. When reshaping the ready/blocked lists, classify non-startable statuses as backlog even with blockers, keeping the blocker count.

## Close Reason

Missions and parents with open children are a context classification: work ready, mission and parent views show them in a Parents and Missions panel, quiet ready lists and status's ready count leave them out, and a parent whose children are terminal is ready again. Also (finding atelier-imxz from ha6k) parked statuses with blockers stay backlog. Test: test_work_ready_lists_startable_work_and_shows_parents_and_missions_as_context. Documented in work-model.md and work-view-ordering.md.
