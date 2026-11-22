---
schema: atelier.item
schema_version: 3
id: atelier-h7e9
title: Count branch-filed loose items in the Desk's rail as the roadmap does
status: open
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T19:03:02.727253652Z
---

## Goal

The Desk's rail and its roadmap count a repository's loose items
differently. The rail's count (`loose` in `crates/atelier-desk/src/read.rs`,
about line 304) leaves out an item filed on a branch that has not merged,
and the roadmap's list holds it: 46 against 47 in atelier-rme3's sample
repository (its Report, "Left, for the lead"). Since atelier-u6oy, agents
file on whatever branch they hold, so such items are common and the two
numbers will differ most days.

## Done when

- [ ] The rail's loose count and the length of the roadmap's loose list are
  the same number for a repository with an item filed on an unmerged
  branch, and a test in `crates/atelier-desk` holds it.
- [ ] `docs/architecture/desk.md` says which items the count holds.

## Scope

`crates/atelier-desk` and that document. Not the clients.
