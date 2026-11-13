---
schema: atelier.item
schema_version: 3
id: atelier-ob9u
title: Name branches by convention and commit every record change on a branch, never on the base
status: draft
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ogpg
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.623183548Z
---

## Goal

Stop the commands writing to the base. Today `new`, `set`, `ready`, `drop`,
`close` and `lead` change records in the base's checkout, and four of them
commit there. After this item each commits on a branch the command names
itself, so an agent never invents a branch name and the base is never the
place a record changes.

## Done when

- [ ] `config.toml` takes one optional branch prefix. With it set to `x/`, an item's branch is `x/<id>`; unset, branches are named as today. A branch name is read back into its record's ID by one function, tested both ways.
- [ ] `new --mission` puts the mission's record on a branch named for the mission, and `new --in <mission>` on that same branch while the mission is not on the base.
- [ ] A record change tied to no mission's plan and no started item (`new`, `set`, `ready` on an item, `drop`, `close`, `lead`) is committed on a `backlog/<slug>` branch, the current one when the checkout is already on one.
- [ ] No command leaves an uncommitted record change behind: `new` and `set` commit, as `ready` and `drop` do today.
- [ ] Run in the base's own checkout, a record-writing command does what the design settled, and a test shows the base's checkout is never switched to another branch by it.
- [ ] `submit` on a branch that changes only records opens its pull request, and `merge` merges it once `atelier check` passes on the merged tree, with no review file asked for. A branch counts as records only when every path it changes against the base is under `.atelier/`; `merge` refuses any other on this path and names the paths, with a test that a `backlog/` branch carrying a change under `crates/` is refused. A branch that adds a mission or changes a mission's plan is refused here and left to atelier-2vdq.
- [ ] The act table has rows for `submit` and `merge` of a `backlog/` branch: any thread, in a checkout on that branch, with no status to come from; and they are not the implementer's acts on an item.
- [ ] Where a remote is configured, `start` pushes the item's new branch before it reports success. When the remote already has that branch, `start` is refused, names the thread its record gives, and removes the local branch it made, leaving the checkout as it was; a later `start` in that clone takes the item over from the remote's branch. When the remote cannot be reached, `start` is refused and says so. With no remote it behaves as today. A test starts one item from two clones of one remote at once: exactly one succeeds, and `atelier list` in the other, after a fetch, shows the item held by the winner.
- [ ] `drop` of a started item commits on the item's own branch and pushes it, and from then the item's `submit` and `merge` are refused; the base's copy follows by the record-only path. `drop` of an item already merged is refused. Tests run both orders from two clones: drop then merge, and merge then drop.
- [ ] `AGENTS.md` no longer says a planning record may be committed on the base; the rest of that file waits for atelier-j5lr.
- [ ] The act table in `docs/design.md` and each command's `--help` match what the commands now do.

## Scope

In: `crates/atelier-core` (config), `crates/atelier-app` and `crates/atelier-cli` for the named commands and `start`, their tests, and the one sentence of `AGENTS.md`.

Out: reading records from branches (atelier-dciw); missions' reviews and merge gate (atelier-2vdq); how a merge finishes with a forge and the pre-push rule (atelier-t5am).

## Context

`start` already creates the item's branch and `submit` already opens a pull request; this item gives the planning commands the same shape. Memory of this repository: `start` run in the base's checkout switched it, which is why the last line but two exists.
