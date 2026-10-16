---
created_at: "2026-10-15T23:04:25.454420356+00:00"
id: "atelier-o6h9"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-o6h9
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-28mz"
  - kind: "issue"
    id: "atelier-by8u"
  - kind: "issue"
    id: "atelier-e4c5"
  - kind: "issue"
    id: "atelier-ymmb"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9vq3"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T00:32:34.508251374+00:00"
status: "done"
title: "Replace the epic type with parent issues"
updated_at: "2026-10-16T00:32:34.508251374+00:00"
---

## Goal

Implement the atelier-9vq3 decision: drop the `epic` issue type so any non-mission issue can be a parent that owns its children's branch, one level deep. Themes go to labels; missions keep the why.

## Done when

- `epic` is no longer a built-in or default issue type, and `atelier init` no longer writes it; the `objective` workflow applies to missions only.
- Any non-mission issue may have children; a child cannot have children. The parent owns the branch for its children and cannot reach a terminal status until every child is terminal. Tests cover a task parent with children through start, land and merge.
- Unknowns and Design are optional sections on any issue; Outcome/Proof/Delivers are gone from issue bodies.
- `atelier check` refuses a record of a type the policy does not declare, and an epic-shaped body, with guidance that names the migration.
- A one-shot migration retypes existing epics (Outcome to Goal, Proof and Delivers to Done when, draft to the someday state from atelier-ha6k, or todo where none exists), keeps their links, and is documented for adopters such as astromech.
- `work epic` and the epic-specific views and help name parents instead; CONTEXT.md, the product docs and ADR text describe parents.

## Context

Decided by Sumeet on 2026-10-15 in atelier-9vq3 (option a). Removes the special cases in atelier-28mz, atelier-e4c5 and atelier-ymmb, since one rule covers parents of any type. Agent Factory's vendored skill also says epic throughout; per ADR 0020 that text changes upstream.

## Close Reason

The epic type is gone: any non-mission work item may have children one level deep, owns their branch, and closes only after them (work.terminal on the starter close). Work items carry Goal/Done when with optional Scope, Context, Unknowns, Design; missions keep Outcome/Success criteria. Old epic records fail with a pointer to atelier migrate epics, which retypes them (Outcome->Goal, Proof/Delivers->Done when, draft->backlog), keeps links and drops epic from the policy; this repo's one epic was migrated, and astromech's 19 migrate cleanly in a scratch copy. work epic is now work parent; the epic/ evidence alias is gone. Docs, CONTEXT.md, the bundle contract and ADR 0021 describe parents. Tests: a_task_parent_owns_its_children_through_start_land_and_merge, test_migrate_epics_retypes_epic_records_and_the_policy, hierarchy tests.
