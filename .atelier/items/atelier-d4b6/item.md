---
schema: atelier.item
schema_version: 3
id: atelier-d4b6
title: Classify draft missions consistently and drop empty context panels
status: dropped
mission: atelier-4pzj
labels:
- cli-output
- work-views
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.984605781Z
---

## Goal

A `todo` mission is classified Context and listed under "Parents and Missions (context)" (fixture atelier-j76w), but a `draft` mission is Backlog because `can_enter_active` is false (crates/atelier-app/src/work_classification.rs:206-272), so on master `work all` lists atelier-1w8c and atelier-r6bt under "Backlog Work" among someday tasks. `work ready` also prints an empty `Parents and Missions (context)` / `(none)` panel.

## Done when

- [ ] Open missions of every status appear in one place with their status word, or `work all` leaves missions to `work missions`.
- [ ] Empty context panels are omitted in `work ready` and `work all`.
- [ ] Tests cover a draft and a todo mission.

## Report

Dropped: The `work` views and their classification were removed in 0.5.
