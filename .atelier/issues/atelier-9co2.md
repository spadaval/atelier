---
created_at: "2026-10-16T00:34:05.289873857+00:00"
id: "atelier-9co2"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-9co2
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:51:19.518755194+00:00"
status: "done"
title: "Stop bundles creating records in statuses that need a transition"
updated_at: "2026-10-16T18:51:19.518755194+00:00"
---

## Goal

A bundle issue's `status` accepts any status its workflow mentions, including `in_progress`, `review` and `done`. A bundle can therefore create active work with no owner branch, or done work with no done claim, skipping the start and close transitions and their checks (the hole atelier-ha6k closed for `issue create --status`). It also sets `closed_at` only for a status named `closed`, which the starter policy does not have.

## Done when

Bundle create accepts the same statuses as `issue create --status` (the initial status, or a todo-category status one unchecked transition from it), terminal statuses set `closed_at` consistently if they remain allowed at all, and a test covers a refused `done` and `in_progress`.

## Context

Found 2026-10-15 while implementing atelier-4n7v.

## Close Reason

Bundle create entries accept the same statuses as issue create --status (shared workflow_policy::creatable_statuses): the initial status or a todo-category status one unchecked transition away. In_progress, done and other statuses are refused with a message pointing to atelier issue transition; closed_at is no longer set for a status named closed, since terminal statuses cannot be created. Test: test_bundle_apply_refuses_statuses_that_need_a_transition_and_preserves_state (task in_progress and done, mission in_progress and todo). Documented in docs/spec/bundle/schema.md and workflow-configuration.md.
