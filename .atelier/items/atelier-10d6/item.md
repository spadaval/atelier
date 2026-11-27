---
schema: atelier.item
schema_version: 3
id: atelier-10d6
title: start --backlog tells a gardener to pick another slug when backlog/garden-proposals exists
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
created_at: 2026-11-22T08:08:13.489330410Z
---

## Goal

`atelier start --backlog <slug>` on a branch that exists answers "Branch
backlog/<slug> exists on origin. Pick another slug, or work on that branch."
Since `atelier-7637` the gardener and the planner reuse one branch,
`backlog/garden-proposals`, and `garden.md` forbids a second slug, so the
first half of that advice is wrong for them and the second half does not say
how. Found by `atelier-7637`'s implementer.

## Done when

- [ ] The refusal names how to work on the existing branch: the checkout that holds it if one does, or else the `git switch` that takes it, and bringing the base in.
- [ ] It no longer offers another slug first, or `garden.md` no longer has to warn against following it.
- [ ] `crates/atelier-cli/tests/branches.rs` covers the new wording.

## Scope

In: `crates/atelier-cli/src/output.rs`, `crates/atelier-cli/src/plan.rs` if the checkout must be looked up, and their tests.
