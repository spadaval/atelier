---
created_at: "2026-11-10T21:30:47.812872344+00:00"
id: "atelier-dxrj"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-jciw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jq24"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Settle deferred findings: `list --deferred`, `new --from` and `drop` on a finding"
updated_at: "2026-11-10T21:30:47.812872344+00:00"
---

## Goal

Let deferred findings be settled instead of piling up. A finding a reviewer recommended deferring, or a decider deferred, stays in its review file when its item merges. This item lists the ones nobody has settled and gives the two ways to settle one: take it into a new item, or drop it with a reason.

## Done when

- [ ] `list --deferred` prints each deferred finding of a done item that has neither a `Taken:` nor a `Dropped by` line, with a name that identifies the item and the finding in one argument, and the finding's title. It says so when there is none. A deferred finding is one whose recommendation is defer or that a decision deferred.
- [ ] `new "<title>" --from <finding>` creates a draft item whose Goal quotes the finding and names where it came from, and writes `Taken: <new item> by <thread>` into the finding, in one run: if either write cannot be made, neither is. `--from` takes the name `list --deferred` prints.
- [ ] `drop <finding> --reason "<why>"` writes `Dropped by <thread>: <why>` into the finding. A reason is required.
- [ ] Both are refused, with nothing written, for a finding that is already settled, a finding of an item that is not done, a finding that is not deferred, and a name no review has; each refusal says which.
- [ ] Both work where they are run. On the base they are planning: no commit, the paths written, and a `Then:` line with the Git command that commits them. On an item's own branch, run by its implementer, they write in that checkout, and the write counts as a change outside the item's own directory: a test settles a finding on the branch of an approved item, and `merge` then says what changed and to run `submit`.
- [ ] After either, `list --deferred` no longer shows the finding, and `show` on the finding's item prints the settlement with it.
- [ ] The act table gains the row for settling a finding, and the unit test over the table covers it.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`list --deferred`, `new --from` and `drop` on a finding, in `next/`. The planning commands refuse to run off the base today; these two are the only ones that may run on an item's branch. `escalate` and `decide` are atelier-axfa, built at the same time by another implementer: keep additions to shared files (`main.rs`, `output.rs`, `acts.rs`) to what this item needs, beside the code they extend. Who runs these and when (the gardener's procedure) is the skill's. `log`, which will show a settlement, is mission 4. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records (the `Taken:` and `Dropped by` lines, which `atelier-core` already parses and writes as `Settlement`), the act table's row "`new --from`, `drop` on a finding", and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Commands (Planner), "Where records change" and Review ("Deferred findings stay in their review file"). The planning commands are in `atelier-cli`'s `plan.rs`, `drop` on an item or mission is atelier-jq24's, and `list` is atelier-jciw's.

This is the second of step C's three items in `docs/strategies/redesign-2-core.md`.
