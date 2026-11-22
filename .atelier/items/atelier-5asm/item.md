---
schema: atelier.item
schema_version: 3
id: atelier-5asm
title: status and verdict print three commands that are then refused
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
created_at: 2026-11-22T10:17:47.603786187Z
---

## Goal

Three lines the tool prints as the next step are refused when run. Each was
found by `atelier-zc9b`'s implementer and left, because that item's Done
when did not cover it.

1. Away from the base checkout, `status` says `atelier start <id>` for an
   item whose branch another worktree has checked out, and `start` refuses
   there. `atelier-zc9b` fixed the same advice in the base checkout only.
2. In a base checkout ahead of its remote, `status` says `Push with: git
   push origin master`, which pre-push refuses.
3. The commit line `verdict` prints after a skip carries the skip's
   trailers but not `ATELIER_SKIP="<reason>"`, so pre-commit refuses it
   until the reviewer adds the variable.

## Done when

- [ ] Each of the three printed lines, run as printed, succeeds or is replaced by one that does; a test runs each.
- [ ] `evals/agent-factory/test.sh` passes.

## Scope

In: `crates/atelier-cli/src/{commands.rs,output.rs,review.rs}`, `crates/atelier-core/src/say.rs`, and tests.
