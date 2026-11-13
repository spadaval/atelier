---
schema: atelier.item
schema_version: 3
id: atelier-2vdq
title: Plan a mission on its branch and make two approving plan reviews its only gate, with no draft status and no go
status: draft
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ob9u
- atelier-dciw
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.672218464Z
---

## Goal

Give a mission's plan the same flow as any change, and take Sumeet out of the
path. Today a plan is a draft on the base, its reviewers work in detached
checkouts and their files are cherry-picked over, and nothing can start until
`ready --by` records his go-ahead. After this item the plan lives on the
mission's branch with its reviews beside it, merges once two reviewers
approve, and is agreed by being on the base.

## Done when

- [ ] `atelier submit` on a mission's branch pushes it and opens its pull request, and opens the next round after changes.
- [ ] `atelier review <mission>` and `atelier verdict` write the review file on the mission's branch, in the reviewer's own checkout of it, and push it. No cherry-pick is needed, and two reviewers in one round do not conflict.
- [ ] `atelier merge` on a branch that adds a mission, or changes the Why, Outcome, Scope or Authority of one on the base, refuses unless two reviews of the current round approve its head, from threads that are not the planner's, and says what is missing.
- [ ] A mission record has no `draft` status and no `ready` field. `ready` refuses a mission and says a mission needs no go-ahead. `start` and `lead` have no check on a mission's status.
- [ ] A mission's Why names whose direction it started from, and `atelier check` says so when the section is empty.
- [ ] `lead` writes the `lead` field through a record-only branch, and `decide` refuses a thread that implemented the item by reading the item's implementers, not the mission's lead.
- [ ] The binary reads a mission record in the old format: one with `status: draft` on the base reads as proposed, and `ready` is ignored. A test reads each open mission record of the five repositories as they stand today.
- [ ] atelier-lkf5 is dropped as moot, or the Report shows it still happens.

## Scope

In: `crates/atelier-core` (`mission.rs`), `crates/atelier-app` (`plan.rs`, `acts.rs`, `decide.rs`), `crates/atelier-cli` (`plan.rs`, `review.rs`, `merge.rs`), their tests.

Out: converting any repository's records (atelier-ql64); the Desk's "say go" (atelier-v958); how a merge finishes with a forge (atelier-t5am); items' `draft` and `ready`, which stay.

## Context

`docs/product.md` lines 109 to 130 define a mission's statuses and `ready` today. The skill's rule of two blind reviews is `skills/agent-factory/roles/planner.md`.
