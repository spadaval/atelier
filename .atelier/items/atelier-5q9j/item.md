---
schema: atelier.item
schema_version: 3
id: atelier-5q9j
title: Refuse new on the branch of a plan dropped there, which never merges
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:46:12.547336946Z
---

## Goal

`atelier new` on the branch of a proposed plan that was dropped there
writes the item and says it counts once the branch merges. That branch is
never merged, so the item is lost, and a plan's drop does not name what
its branch had filed either. atelier-u6oy refuses `new` on a dropped
item's branch and names what a started item's drop leaves behind; plans
were outside its Scope. Reproduced in a scratch repository on 2026-11-22
with `start --mission`, `drop`, a commit, then `new`, which exited 0.

## Done when

- [ ] `new` on the branch of a plan dropped there is refused, naming
  `start --backlog`.
- [ ] Dropping a proposed plan names each item on its branch that is not
  one of the plan's, with how to carry it over. A test holds each.

## Scope

`new` and `drop` of a proposed mission.
