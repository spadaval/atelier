---
schema: atelier.item
schema_version: 3
id: atelier-jr1b
title: Give the Desk's item row a proposed field, so the clients stop guessing from a branch's name
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T19:03:02.628221248Z
---

## Goal

Taken from deferred finding atelier-rme3/1-0xrz.1, in .atelier/items/atelier-rme3/reviews/1-0xrz.md:

> Finding 1: Branch-name suffix heuristic misclassifies an adding branch ending with the item ID, and `startNote` has no empty-condition fallback
>
> Recommendation: defer
>
> Location: `desk/src/lib/mission.ts:20`, `t3-extension/src/mission/model.ts:215-217`
>
> Both `cannotStartUntil` (`desk/src/lib/mission.ts:20`) and `startNote` (`t3-extension/src/mission/model.ts:215`) distinguish an unmerged branch that adds an item from the item's own branch with `!on.name.endsWith(row.id)`. If an open item is added on an unmerged backlog branch whose slug ends with the item's ID (for example `backlog/<id>` or `origin/backlog/fix-<id>`, valid under `Config::named` in `crates/atelier-core/src/model/config.rs:84-88`), `endsWith(row.id)` is true, so `added` is empty while `row.cannot_start` is also empty. In that case `cannotStartUntil` returns `[]` (no reason shown on the Desk's mission page or Mission Control) and `startNote` falls through to `` `cannot start until ${row.cannot_start.join("; ")}` ``, rendering `"cannot start until "` with a trailing blank. Because `atelier new` allocates random IDs after `start --backlog <slug>`, this is rare in practice; a follow-up can either expose `proposed` on `ItemRow` in `atelier-desk` or check for own-branch naming without matching `backlog/` prefixes and fall back to `"waits"` in `startNote` when `row.cannot_start` is empty.

## Done when

- [ ] `ItemRow` in `crates/atelier-desk` says whether the item is proposed,
  meaning its record waits on an unmerged branch, and which branch.
- [ ] `cannotStartUntil` in `desk/src/lib/mission.ts` and `startNote` in
  `t3-extension/src/mission/model.ts` read that field and no branch name. A
  test in each files an item on `backlog/<its id>` and sees the branch
  named as what it waits on.
- [ ] `startNote` never prints "cannot start until" with nothing after it.

## Scope

`crates/atelier-desk` for the field, `desk/src/api`, the two functions and
their tests. The field is in the Desk's API, not in a record.
