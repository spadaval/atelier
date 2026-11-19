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
- [ ] A reviewer of an item, who works in the implementer's checkout, can
  file the same way. The brief `atelier review` prints says so in place of
  "change no other file", and a test follows that brief as printed: the
  review and the filed item are both committed, and the filed item reaches
  the base with the merge.
- [ ] A commit that only adds a new item's record does not make the
  approval of the branch's own item stale, whoever makes it and whether
  before or after `verdict approve`; `atelier merge` goes through with no
  further round, and a test holds it. Any other change to another item's
  record still makes the approval stale, as now.
- [ ] A plan reviewer, in a detached checkout of its own, is told by its
  brief to publish its verdict first and then file an unrelated problem on
  a backlog branch, so the plan under review and the verdict are untouched.
- [ ] Dropping a started item whose branch holds newly filed items loses
  none silently: `drop` carries them to the base with the drop's own
  record change, or names each in what it prints. A test holds whichever
  it is, and `docs/design.md` says which.
- [ ] `atelier new --help`, the action table in `docs/design.md`, and
  `docs/product.md` and `AGENTS.md` where they say which branch filing
  needs, say the same thing as the hook.

## Scope

The rule for `new` in the pre-commit hook and wherever `crates/` holds the
action table; the briefs `atelier review` prints; `drop` for a started
item; those documents. `new` still
commits nothing and merges nothing. Not `set`, `close`, `drop` of another
item or `lead` on an item's branch: only adding a new item's record. Not
the skill's instruction to use this (atelier-8phx).
