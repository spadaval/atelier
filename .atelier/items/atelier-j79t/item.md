---
schema: atelier.item
schema_version: 3
id: atelier-j79t
title: Keep an item's Open items in the debrief when review filed follow-ups
status: dropped
mission: null
labels:
- review
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T04:40:10.011847246Z
---

## Goal

Open items are dropped whenever review filed follow-ups for the entry, but the two are not the same list: lnwf's untested push-event diff check and macOS timing headroom, 7v9q's unexercised tag-push path, and 5tl2's pre-release installer 404 appear nowhere. Show Open items alongside Filed, or decide the rule again.

## Done when

- [ ] A debrief entry shows its Open items alongside Filed follow-ups, or the rule for hiding them is decided again and written in docs/product/mission-reviews.md.
- [ ] A CLI fixture covers it in both forms, including atelier-cagr's case: a follow-up filed on one child leaves a sibling's Open items visible.

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).

## Links

Relationships of the old record that the new format has no field for.

- supersedes: atelier-cagr (Decide a debrief child's Open items by its own follow-ups)
- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)

## Report

Dropped: The `debrief` command was deleted in 0.5.
