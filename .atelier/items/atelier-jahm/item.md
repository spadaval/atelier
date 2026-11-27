---
schema: atelier.item
schema_version: 3
id: atelier-jahm
title: merge says a plan's branch merges with no review and that the mission is done
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:40:12.269451610Z
---

## Goal

On 2026-11-27 `atelier merge` merged the plan of mission `atelier-oqsz`,
which two reviewers had approved in round 2, and printed: "The branch
changes records alone, so it merges with no review once its records are
valid", then "atelier-oqsz is done." Both lines are wrong for a plan. A
plan's branch merges on two approving plan reviews, and after the merge
the mission reads `agreed`, not done. A planner reading the first line
learns that a plan needs no review.

## Done when

- [ ] After a plan's merge, `merge` says the plan merged on its two
  approvals and that the mission is agreed. A test covers it.
- [ ] A backlog branch's merge still prints what it does today.

## Scope

What `merge` prints after a mission branch merges, in `crates/` and their
tests.
