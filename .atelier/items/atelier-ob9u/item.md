---
schema: atelier.item
schema_version: 3
id: atelier-ob9u
title: Name branches by convention and commit every record change on a branch, never on the base
status: ready
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ogpg
- atelier-xbog
implementers: []
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-13T22:37:19.473562932Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.623183548Z
---

## Goal

Stop record changes landing on the base, and draw one line between the tool
and Git. Today `new`, `set`, `ready`, `drop`, `close` and `lead` change
records in the base's checkout and four of them commit there. After this item
a command that changes a record writes the file and nothing else, the agent
commits and pushes with Git, the hooks validate, and only three commands act
on Git: `start` makes and claims a branch, `submit` publishes it, and `merge`
merges it.

## Done when

- [ ] `new`, `set`, `ready`, `drop`, `close`, `lead`, `review` and `verdict` write record files in the checkout they run in and make no commit, no branch and no push. Each prints the paths it wrote. Tests show `git status` after each has changes and `git log` has none.
- [ ] `start` is the only command that makes a branch, and it makes all three kinds: `start <item>` as today; `start --mission "<title>"`, a new mission's record on a branch named for it; and `start --backlog <slug>`, a `backlog/<slug>` branch for record changes tied to neither. It commits the one record change that is the claim, where there is one.
- [ ] `config.toml` takes one optional branch prefix. With it set to `x/`, every kind of branch carries it: `x/<item-id>`, `x/<mission-id>` and `x/backlog/<slug>`; unset, branches are named as today. A branch name is read back into its record's ID by one function, tested both ways.
- [ ] `start` never switches the base's own checkout to another branch: there it does what the design settled, and a test shows the base's checkout still has the base out afterwards.
- [ ] The pre-commit hook refuses a commit on the base that changes a record, and says to run `start --backlog <slug>` or `start --mission`. `ATELIER_SKIP` passes it with a reason, as for any rule.
- [ ] Where a remote is configured, `start` pushes the new branch before it reports success. When the remote already has it, `start` is refused, names the thread its record gives, and removes the local branch it made, leaving the checkout as it was; a later `start` in that clone takes the item over from the remote's branch. When the remote cannot be reached, `start` is refused and says so. With no remote it behaves as today. A test starts one item from two clones of one remote at once: exactly one succeeds, and `atelier list` in the other, after a fetch, shows the item held by the winner.
- [ ] `submit` refuses while a record the branch changes is uncommitted, naming the paths, then pushes and opens the pull request. On a branch that changes only records, `merge` merges once `atelier check` passes on the merged tree, with no review file asked for. A branch counts as records only when every path it changes against the base is under `.atelier/`; `merge` refuses any other on this path and names the paths, with a test that a `backlog/` branch carrying a change under `crates/` is refused. A branch that adds a mission or changes a mission's plan is refused here and left to atelier-2vdq.
- [ ] The act table has rows for `submit` and `merge` of a `backlog/` branch: any thread, in a checkout on that branch, with no status to come from; and they are not the implementer's acts on an item.
- [ ] `drop` of a started item is written on the item's own branch, by any thread in a checkout of that branch, and committed and pushed with Git; from that push the item's `submit` and `merge` are refused. The base's copy is dropped by the ordinary record-only path, and until it is, `status` and `list` say the drop has not reached the base. `drop` of an item already merged is refused. Tests run both orders from two clones, drop then merge and merge then drop, and show the base's copy dropped after the item's branch is deleted.
- [ ] `status` names every record change in the checkout that is not committed, and every commit on the branch that is not pushed, with the Git command for each.
- [ ] `AGENTS.md` no longer says a planning record may be committed on the base; the rest of that file waits for atelier-j5lr.
- [ ] The act table in `docs/design.md` and each command's `--help` match what the commands now do.

## Scope

In: `crates/atelier-core` (config), `crates/atelier-app` and `crates/atelier-cli` for the named commands and `start`, their tests, and the one sentence of `AGENTS.md`.

Out: reading records from branches (atelier-dciw); missions' reviews and merge gate (atelier-2vdq); how a merge finishes with a forge and the pre-push rule (atelier-t5am).

## Context

`start` already creates the item's branch and `submit` already opens a pull request; this item gives the planning commands the same shape. Memory of this repository: `start` run in the base's checkout switched it, which is why the last line but two exists.
