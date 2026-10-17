---
created_at: "2026-10-16T23:04:58.129176254+00:00"
id: "atelier-7n5u"
issue_type: "bug"
labels:
- "orientation"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-7n5u
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1w8c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Mark ready work that is already in flight on another local branch"
updated_at: "2026-10-17T03:07:39.761133657+00:00"
---

## Goal

`work ready` on master listed atelier-fj1t and atelier-j48s as `ready [todo]` while `status` and `prime` in the same checkout showed them `done on branch bug/atelier-fj1t` and `in_progress` on `bug/atelier-j48s`. An agent reading only the ready list can start work that another session already owns.

## Done when

`work ready` (and the ready section of `work all`) annotates an item whose owner branch exists locally with its state there, e.g. `↳ in_progress on bug/atelier-j48s` or `↳ done on bug/atelier-fj1t, not merged`, using the branch summaries `status` already computes. The item stays listed; current-checkout records are not overlaid. A test covers an in-progress and a done-unmerged branch.

## Scope

Ready-view rendering and the shared branch-summary code. Ready classification does not change.

## Context

`git::branch_lines_for` and the `status` "Other local work branches" section; `docs/product/sessions-and-hooks.md#other-branches`.
