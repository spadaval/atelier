---
schema: atelier.item
schema_version: 3
id: atelier-sorp
title: Show a proposed plan's items as waves on the Desk, so its order and what starts first are plain
status: draft
mission: null
labels:
- feature
priority: low
blocked_by:
- atelier-v958
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T22:00:46.704477280Z
---

## Goal

Make a proposed plan quicker to judge. The mission page groups items by status, which says where work stands and nothing about its order. A plan's reader wants to know what starts first, what can run side by side, and whether the plan is one long chain. Sumeet's T3 Code fork of April 2026 showed an epic this way, and it is the one view of that fork the Desk lacks and should have: items in waves, each wave the items whose blockers are all in earlier waves.

## Done when

- [ ] A mission's Work tab shows its items that are not done or dropped in waves: the first holds the items nothing open blocks, and each later one the items whose open blockers are all in earlier waves.
- [ ] An item blocked by one outside the mission shows that blocker's ID and status, and is placed as if that blocker were a wave before the first.
- [ ] Items whose blockers form a cycle are listed apart and named as a cycle.
- [ ] The waves come from one function in the core, which the Desk's server calls; the app computes none of it.
- [ ] A unit test covers a chain, a fan-out, an outside blocker and a cycle, and an e2e test finds the waves of a fixture mission on its page at a desktop and a phone width.
- [ ] `docs/architecture/desk.md` names the field and the function that gives it.

## Scope

In: `desk/`, `crates/atelier-desk`, the core function, and `docs/architecture/desk.md`.

Out: a drawn dependency graph; predicting when a wave finishes; changing the order `status` or `list` print.

## Context

An item's `blocked_by` is the only input. The fork's version is `apps/web/src/lib/epicExecutionView.ts` at the tag `t3code-jec.1-pre-compress-20260411-51fce129` in the T3 Code checkout, for the shape of the view only. It waits on atelier-v958, which rewrites the mission page.
