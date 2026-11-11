---
schema: atelier.item
schema_version: 3
id: atelier-zjgy
title: Find out why so many commits touch only tracker records, and propose what to cut
status: dropped
mission: null
labels:
- tooling
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T05:35:32.950046101Z
---

## Goal

In the 2026-11-08 review audit, 59 of 183 non-merge commits in life-on-rails touched only `.atelier/` (its Atelier era, 2026-11-06 19:00 to 2026-11-07 23:00 local), and 119 of 133 in atelier during mission atelier-mmpv. Find which commands produce these commits and whether batching records would cut them without losing what review needs.

## Done when

- [ ] A note on this item re-counts both ranges (naming the commit range used for each), breaks the tracker-only commits down by the command that made them, and proposes changes, each with its expected saving.

## Scope

The investigation and its note. Changes it proposes are filed as separate items.

## Report

Dropped: Its counts are from the old tool's record commands, all removed in 0.5. Process share is a scorecard measure, so the rerun (atelier-pwhn) answers it.
