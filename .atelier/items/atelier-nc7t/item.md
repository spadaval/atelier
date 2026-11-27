---
schema: atelier.item
schema_version: 3
id: atelier-nc7t
title: One branch with two merge bases is refused by merge for different stated reasons, none naming the cause
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:40:12.205358202Z
---

## Goal

In the Hub's `v2` on 2026-11-26, items were built on each other's unmerged
branches, so a branch had two merge bases with the base. `atelier merge`
refused such branches for reasons that named something else, each with a
different fix:

- `~/hub-v2-trials/6ce4-merge.log`: "The approval still covers the head",
  listing other items' records as changed, with "send it back to review".
- `~/hub-v2-trials/aj25-merge.log`: the branch "brings these to the base
  started: hub-v2-6ce4: approved", with "take those changes off this
  branch".

The Hub lead's side chat of 2026-11-27 reports a third, "changes a
mission's plan", for which no log was kept. `atelier-xiiw` is the same
kind of fault in a catch-up merge. Left out of mission `atelier-oqsz` to
keep it to the four changes the human agreed.

## Done when

- [ ] A branch that holds another unmerged item's commits is refused by
  `merge` with one message that says so, names the other item, and gives
  a fix that works: merge that item first, then merge the base.
- [ ] A test builds the two cases the logs hold and checks the message.

## Scope

`atelier merge`'s refusals, in `crates/` and their tests. Not whether an
item may build on another's unmerged branch.
