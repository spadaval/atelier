---
schema: atelier.item
schema_version: 3
id: atelier-o91z
title: Refuse a malformed Responses line where it is written, instead of erroring every command on the item
status: draft
mission: null
labels:
- redesign
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T02:21:00.069357946Z
---

## Goal

Keep one bad line in an item's Report from stopping every thread that touches the item. In the new binary under `next/`, a line under `### Responses` that is not `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>` makes the record unparseable, so every command on the item exits 2, the reviewer's included. A fresh Codex agent wrote a sentence of prose there in one run of `evals/agent-factory/scenarios/next-loop-changes/` and recovered from `submit`'s error; copying `status`'s own "Finding 1-xxxx.1" form into a response does the same. `status` now says the section takes one line per finding and nothing else, which lowered the rate; the record is still all-or-nothing.

## Done when

- [ ] A Responses line the tool cannot read is reported by `status` and `submit` as a problem with that line, naming it and the form it takes, while `status`, `show` and `review` on the item keep working.

## Context

Found by atelier-t2jo (`evals/agent-factory/results/2026-11-10-next-loop-changes.md`). The body readers are in `next/crates/atelier-records/src/body.rs`. A record that does not parse cannot be skipped with `ATELIER_SKIP`, which is why this matters more than a refusal would.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-t2jo (Run fresh agents through a round of changes requested, the path most items take)
