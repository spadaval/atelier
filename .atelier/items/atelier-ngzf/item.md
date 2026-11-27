---
schema: atelier.item
schema_version: 3
id: atelier-ngzf
title: Say Blocked by in the extension's hover card only for blockers that are not done
status: open
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T19:03:02.677722798Z
---

## Goal

The extension's hover card for a record prints "Blocked by" with an item's
blockers when every one of them is done: line 38 of
`t3-extension/src/references/HoverCard.tsx` falls back to the record's
`blocked_by` when the row's `waiting_on` is empty, and `waiting_on` is empty
exactly when nothing is left to wait on. So an item that can start is shown
as blocked. Found by atelier-rme3's implementer by reading the code (its
Report, "Left, for the lead"); nobody has seen it on a page yet.

## Done when

- [ ] The hover card of an item whose blockers are all done says nothing
  of being blocked, and a test holds it.
- [ ] The hover card of an item with a blocker that is not done still
  names that blocker.

## Scope

`t3-extension/src/references/` and its test.
