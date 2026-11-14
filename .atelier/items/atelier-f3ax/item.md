---
schema: atelier.item
schema_version: 3
id: atelier-f3ax
title: Say atelier-yclc was dropped where the documents and the Desk still wait on it
status: draft
mission: null
labels: []
priority: low
blocked_by:
- atelier-x5al
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-14T07:04:06.812167380Z
---

## Goal

Sumeet dropped atelier-yclc on 2026-11-14: the Desk will not publish the stop of an agreed mission, and the mission's page keeps showing the commands. The documents and the Desk still say that stop waits on atelier-yclc, which sends a reader to a dropped item.

## Done when

- [ ] Each of these says what is true now, that the Desk does not make an agreed mission's stop and its page gives the commands, without naming a wait: `docs/design.md:802`, `docs/architecture/desk.md:610` and `:761`, `docs/product/desk.md:119` and `:171`, the module comment of `crates/atelier-desk/src/stop.rs`, the comment at `crates/atelier-desk/src/api.rs:160` with its generated copy in `desk/src/api/MissionPage.ts`, and the comment at `desk/src/components/Actions.tsx:82`.
- [ ] `docs/architecture/desk.md:938` keeps its reason for one `POST /api/action` and no longer rests it on atelier-yclc.
- [ ] The two refusals in `crates/atelier-desk/src/stop.rs` (lines 72 and 122) lose "(atelier-yclc)" and "yet". The tests that expect them change by those words alone.
- [ ] `git grep atelier-yclc` outside `.atelier/` finds nothing.

## Scope

In: the files named. Out: any change to what the Desk does; atelier-to83's mission record, whose criterion `stop-is-effective` names this stop and is its lead's to settle.

## Context

Line numbers are as of 2026-11-14, before atelier-x5al, which edits `stop.rs` and the two architecture documents. Start it after that item merges.
