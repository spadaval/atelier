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

- [ ] `new --mission` records the calling thread as a planner of the mission, and a thread that runs `submit` on the mission's branch without being one takes the plan over and is added. The act table has rows for `submit` and `merge` of a mission's branch: a planner of it, in a checkout on that branch, from proposed.
- [ ] `atelier submit` on a mission's branch pushes it and opens its pull request, and opens the next round after changes.
- [ ] `atelier review <mission>` refuses a planner of the mission. Each reviewer's `verdict` publishes its review file on a ref of its own, named by a rule in the design, and neither `review` nor `verdict` reads another reviewer's ref of the same round. The planner's next `submit` or `merge` brings every published review of the round onto the mission's branch itself. A test runs two reviewers of one round in two clones and both verdicts arrive, with no cherry-pick by hand.
- [ ] `atelier merge` refuses a branch that adds a mission, or changes a mission's plan (its Why, Outcome, Success criteria, Scope or Authority, or any other file in its directory but `notes.md` and its reviews), unless two reviews of the current round, from threads that are not its planners, approve the plan its head holds, and says what is missing. A commit that adds only review files, and one that changes only the front matter, does not make an approval stale.
- [ ] A branch that changes only a mission's `lead`, `notes.md`, Result or status, or adds an item to an open mission, merges on `atelier check` alone. Tests: Success criteria changed after approval is refused; `design.md` beside the mission changed after approval is refused; a note and an added item each merge without a review.
- [ ] A mission's stored status is `open`, `done` or `dropped`, with no `ready` field. One read from a branch that has not merged is shown as proposed. `lead` and `close` run from `open`. `ready` refuses a mission and says a mission needs no go-ahead.
- [ ] `start` and a take-over ask of an item's mission only that it is not dropped, which they keep refusing. An item's `merge` refuses too once its mission is dropped, and names the stop.
- [ ] `drop` on a proposed mission commits on its branch and closes its pull request. Tests cover a stop of a proposed plan, and a stop of an open mission with one ready item and one started item.
- [ ] A mission's Why names whose direction it started from, and `atelier check` says so when the section is empty.
- [ ] `lead` writes the `lead` field through a record-only branch, and `decide` refuses a thread that implemented the item by reading the item's implementers, not the mission's lead.
- [ ] The binary reads a mission record in the old format: `status: ready` reads as `open`, `status: draft` on the base reads as proposed and cannot be led or have its items started, and the `ready` field is ignored. A test reads each open mission record of the five repositories as they stand today.
- [ ] atelier-lkf5 is dropped as moot, or the Report shows it still happens.

## Scope

In: `crates/atelier-core` (`mission.rs`), `crates/atelier-app` (`plan.rs`, `acts.rs`, `decide.rs`), `crates/atelier-cli` (`plan.rs`, `review.rs`, `merge.rs`), their tests.

Out: converting any repository's records (atelier-ql64); the Desk's "say go" (atelier-v958); how a merge finishes with a forge (atelier-t5am); items' `draft` and `ready`, which stay.

## Context

`docs/product.md` lines 109 to 130 define a mission's statuses and `ready` today. The skill's rule of two blind reviews is `skills/agent-factory/roles/planner.md`.
