---
schema: atelier.item
schema_version: 3
id: atelier-okfy
title: Show the mission's own Report and item notes in the debrief
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
created_at: 2026-11-05T04:40:10.371706005Z
---

## Goal

The debrief does not show the mission's own Report, the report written when it closes (outcome, deviations, decisions, surprises, open items; read from `## Report`, or `## Close Reason` in older records), including vt8q's explanation that two later code commits don't affect S1, S3 or S4. Item notes such as g8cn's v0.3.1 publication note are missing too, although Left open points at them. Under atelier-jnrh each item's Report is written before review and carried into the merge, and a mission closes once all its work is merged and its validation, if it has one, is done.

## Done when

- [ ] A CLI fixture in a scratch repository shows the mission's Report and the item notes Left open points at, in both the Markdown debrief and the site's debrief page, for a mission closed under the current lifecycle and for one whose records use `## Close Reason`.

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8. vt8q's records become prunable after 2026-11-11, so a fixture is the check. Its mission, atelier-gg6b, was superseded in decision atelier-mzpn; this item stands alone until loose end atelier-jdpf reviews a debrief generated under the new lifecycle.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)

## Report

Dropped: The `debrief` command was deleted in 0.5; `atelier show <mission>` prints the mission's Result.
