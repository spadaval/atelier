---
schema: atelier.item
schema_version: 3
id: atelier-tuln
title: atelier log loses an item's merge line when the base held no record before the merge
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T07:51:21.290004765Z
---

## Goal

`atelier log` prints no `merge` line for an item whose merge is the first
commit on the base to hold any record, as in a new repository whose first
item was made by `start --new`. `git log --parents` rewrites that merge to
one parent, since the first parent holds no record, and `Walker::commit`
reads it as an ordinary commit. Found by `atelier-js4a`'s implementer;
`plan-merge` is not affected.

To reproduce: a repository holding only `.atelier/config.toml`, `atelier
start --new "<title>"`, `atelier merge --trivial "<why>"`, `atelier log`.

## Done when

- [ ] A test in `crates/atelier-cli/tests/log.rs` makes that repository and `atelier log` prints the item's `merge` line.
- [ ] `cargo nextest run -p atelier-core -p atelier-cli --locked` passes.

## Scope

In: `crates/atelier-core/src/log.rs` and its tests.
