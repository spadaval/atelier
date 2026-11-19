---
schema: atelier.item
schema_version: 3
id: atelier-u6oy
title: Let atelier new file an item on whatever branch the agent is on, and lose none when that branch is dropped
status: draft
mission: atelier-dh7a
labels: []
priority: medium
blocked_by:
- atelier-v06d
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.190889589Z
---

## Goal

`atelier new` belongs on a backlog branch or a proposed mission's, and the
pre-commit hook refuses a new item's record anywhere else. An implementer
that notices a problem outside its item must make a second branch and merge
it to record one sentence, so it mostly does not. One exception already
works: `new --from <finding>` on the branch of an item the caller
implements. After this item `atelier new` works on any branch `start`
made, and the new item reaches the base with that branch's merge.

## Done when

- [ ] On an item's branch, `atelier new "<title>"` writes an open item, the
  commit that adds it passes the pre-commit hook, and after `atelier merge`
  of that branch the item is open on the base. A test holds each step.
- [ ] Before that merge, `atelier list` run elsewhere shows the new item as
  proposed with its branch, as it shows any record waiting to merge.
- [ ] The new item joins no mission unless `--in` names one, and is not
  held by the thread that filed it.
- [ ] A reviewer, who works in the implementer's checkout, can file the
  same way, and the approval of the head it reviewed is not made stale by
  a commit that only adds an item's record. A test holds it, or the Report
  says why the approval must go stale and what the reviewer does instead.
- [ ] Dropping a started item whose branch holds newly filed items loses
  none silently: `drop` carries them to the base with the drop's own
  record change, or names each in what it prints. A test holds whichever
  it is, and `docs/design.md` says which.
- [ ] `atelier new --help`, the action table in `docs/design.md`, and
  `docs/product.md` and `AGENTS.md` where they say which branch filing
  needs, say the same thing as the hook.

## Scope

The rule for `new` in the pre-commit hook and wherever `crates/` holds the
action table; `drop` for a started item; those documents. `new` still
commits nothing and merges nothing. Not `set`, `close`, `drop` of another
item or `lead` on an item's branch: only adding a new item's record. Not
the skill's instruction to use this (atelier-8phx).
