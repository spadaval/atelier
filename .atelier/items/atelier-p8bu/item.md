---
schema: atelier.item
schema_version: 3
id: atelier-p8bu
title: A skip that passes a plan submit with nothing to commit is not recorded
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
created_at: 2026-11-22T10:17:47.503891208Z
---

## Goal

`ATELIER_SKIP="<reason>" atelier submit` on a mission or backlog branch that
has nothing to commit leaves no trailer, so the reason is printed and then
lost: `atelier log` never shows it. This is the skip `atelier-h6ij` offers
the human to open another plan-review round after two split rounds, so the
one ruling that exit asks for is the one that goes unrecorded.
`roles/planner.md` has the planner write it in `notes.md`, which is a
workaround. Found by `atelier-h6ij`'s implementer.

## Done when

- [ ] A skip that passes a rule at `submit` on a records branch is recorded where `atelier log` prints it, whether or not `submit` has anything else to commit.
- [ ] A test in `crates/atelier-cli/tests/skip.rs` covers the two-split exit passed with nothing to commit.
- [ ] `roles/planner.md` drops the `notes.md` workaround.

## Scope

In: `crates/atelier-cli/src/commands.rs`, `crates/atelier-core`, `skills/agent-factory/roles/planner.md`, and tests.
