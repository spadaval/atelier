---
schema: atelier.item
schema_version: 3
id: atelier-3sym
title: Print the merge line of the first item merged in a repository in atelier log
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T05:18:16.177391030Z
---

## Goal

`atelier log` prints no `merge` line for the first item merged in a
repository. Found on 2026-11-19 while writing the README's walkthrough
(atelier-41zk) with `atelier 0.7.0`: a new repository, one item from `start
--new`, two review rounds, `merge`, then `log`. The log ended at the round-2
`verdict` line. A second item merged in the same repository got its line,
`<time> <commit> <id> merge <thread> approved -> done`.

The cause is in `walk` in `crates/atelier-core/src/log.rs`. Its `git log`
limits history to `.atelier/items` and `.atelier/missions` and passes
`--parents`, so `%P` gives each commit's parents as Git rewrote them: the
nearest listed commits. Before the first item merges, no commit on the
base's line has changed a record, so the merge commit's first parent has no
listed ancestor and Git drops it. The merge is listed with one parent,
`Walker::commit` takes it for an ordinary commit, and `merge` is never
called. `git log --full-history --parents --format='%h %p' main -- .atelier/items
.atelier/missions` in such a repository shows the merge with one parent.

## Done when

- [ ] In a repository whose base held no item or mission before it, the
  first item's merge prints its `merge` line in `atelier log`, as a later
  item's does.
- [ ] A scenario test covers it: a new repository, one item started, reviewed
  and merged, and the log's last line is the merge. It fails on today's code.
- [ ] The walk still carries a version that does not read to the next commit
  on its line of history, which is what the rewritten parents are for
  (`docs/design.md`, The walk).

## Scope

`crates/atelier-core/src/log.rs`, its tests, and `docs/design.md` where it
describes the walk.
