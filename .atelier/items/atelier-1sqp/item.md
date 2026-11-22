---
schema: atelier.item
schema_version: 3
id: atelier-1sqp
title: Rewrite the cutover kit's agent documents and rehearsal for the branch model
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:33:36.845438528Z
---

## Goal

`scripts/cutover/agents/atelier.md` is the `AGENTS.md` process section the
cutover writes into a repository, and it describes Atelier as it was
before branches (0.6): planning on `master`, `drop`, `close` and `lead`
making their own commit, `git push` of `master`, `atelier escalate` and
`atelier decide`, and a pre-push hook that lets record commits through.
None of that is true now: every change lives on a branch `start` makes,
those commands commit nothing, `escalate` and `decide` are gone, and
pre-push refuses every push that moves `master`. `rehearse.sh` runs the
same older flow. atelier-v06d changed only their `ready` lines and found
the rest (its Report, "Left, for the lead"). The work host has not adopted
yet (atelier-7w9s), and would be told the old model.

## Done when

- [ ] `scripts/cutover/agents/atelier.md` says what this repository's
  `AGENTS.md` says about branches, commits, questions and hooks, and names
  no removed command.
- [ ] `scripts/cutover/rehearse.sh` takes its items from open to done by
  commands the current binary accepts, and a rehearsal run exits 0.
- [ ] `scripts/cutover/agents/product.md` is read and corrected the same
  way.

## Scope

`scripts/cutover/agents/`, `rehearse.sh` and the kit's README where it
describes them. Not the report of stale files in a converted repository,
which is atelier-52g7. Nothing under `crates/`.
