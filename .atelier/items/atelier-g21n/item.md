---
schema: atelier.item
schema_version: 3
id: atelier-g21n
title: Leave out a report Decisions part that only points at the item's decision notes
status: draft
mission: null
labels:
- review
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T04:47:42.479935642Z
---

## Goal

Since atelier-mgl0 the debrief shows each report's Decisions part, so a part that only points at the notes ("See the decision notes on this item.") shows the same decision twice next to the notes it names (about 10 reports in this repo; vt8q's l7gl entry).

## Done when

- [ ] A Decisions part that only points at the entry's decision notes is left out when the entry has notes, with a fixture test; parts with their own content still show.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-mgl0 (Fix the debrief defects the S4 and S7 reviews found)
