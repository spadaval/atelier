---
schema: atelier.item
schema_version: 3
id: atelier-zc0o
title: Stop mission control's j and k keys from skipping a row when pressed quickly
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: atelier-cdxg
created_at: 2026-11-16T17:55:07.112337309Z
---

## Goal

Mission control's keys move the selection one row per press, however fast
they come. Today the key handler in `desk/src/pages/MissionControl.tsx`
reads the selected item from the last render, so a second key that arrives
before React draws the first counts from the old row: `j` then `k` from
atelier-uwxy lands on atelier-aab1, not back on atelier-uwxy. Someone
moving quickly through the list skips rows, and Enter or I then opens the
wrong item's thread or page.

It showed as a flake in `desk/e2e/host.spec.ts`'s "mission control's keys
select items..." test, which pressed `j` and `k` back to back: 10 of 200
runs failed on master (2026-11-16, found under atelier-jwfs). atelier-jwfs
made the test wait for each key to show, so the test no longer covers the
quick case.

## Done when

- [ ] Two or more keys pressed inside one render move the selection by one row each, from the row the previous key chose.
- [ ] A test presses `j` then `k` with no wait between and finds atelier-uwxy selected, and passes 100 runs in a row.

## Scope

`desk/src/pages/MissionControl.tsx` and its test in `desk/e2e/host.spec.ts`.
