---
schema: atelier.item
schema_version: 3
id: atelier-ayrg
title: 'Count fix_now findings resolved by a later approval as fixed in the debrief: atelier-1gb3''s shows 16 of 16 awaiting a decision'
status: dropped
mission: null
labels:
- review
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-09T18:00:38.351435421Z
---

## Goal

The debrief of mission atelier-1gb3 says "16 findings: 16 awaiting a decision", though land passes and every finding was a fix_now finding resolved by an approval after its fix. `crates/atelier-app/src/mission_review.rs` counts a finding as fixed only with an explicit fix_now disposition, so a finding resolved the normal way (recommended fix_now, then a later approval) counts as undisposed. The debrief then tells the human there are 16 open decisions that do not exist.

## Done when

- [ ] A fix_now finding with no disposition, followed by an approval of its target, counts as fixed in the debrief and the mission brief, with the same rule `findings.disposed` uses.
- [ ] A finding still awaiting a decision under `findings.disposed` keeps counting as awaiting, and a test covers both cases.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-1gb3 (Take approved work to the published base in one command, through a forge)

## Report

Dropped: The `debrief` command and `mission_review.rs` were deleted in 0.5.
