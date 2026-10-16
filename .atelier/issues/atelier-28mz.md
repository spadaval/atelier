---
created_at: "2026-10-15T18:57:15.135112410+00:00"
id: "atelier-28mz"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-28mz
priority: "P3"
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
closed_at: "2026-10-16T01:25:59.647328012+00:00"
status: "done"
title: "Decide whether children of draft epics can start"
updated_at: "2026-10-16T01:25:59.647328012+00:00"
---

## Goal

Decide and enforce whether work under a `draft` epic can be listed as ready or started.

## Done when

The chosen rule is documented, and tests cover `work ready` and `start` for a child of a draft epic.

## Context

External evaluation 2026-10-15: a child started under a draft epic, and `work mission` lists it as ready. The tracker-commands reference says draft epics are planning context, not ready work, but says nothing about their children. Deferred from the evaluation fixes because it needs a policy decision (a workflow validator or a ready-classification rule).

## Notes

- 2026-10-16 [human] Epics are gone (atelier-9vq3), so the question became children of a parked parent. Sumeet decided 2026-10-15: children park too. Children of a parent in a todo-category status it cannot start from (backlog, draft) classify as backlog, and start and the start transition refuse them until the parent is planned.

## Close Reason

Decided by Sumeet 2026-10-15: children of a parked parent (backlog or another todo-category status it cannot start from) are parked too. They classify as backlog, and atelier start and the start transition refuse them until the parent is planned. Documented in docs/product/work-model.md. Tests: test_children_of_a_parked_parent_are_parked_until_it_is_planned, plus updated ready/work-all/dashboard tests.
