---
schema: atelier.item
schema_version: 3
id: atelier-c1z3
title: Ready the scorecard scripts and rubric for the rerun after the cutover
status: draft
mission: null
labels:
- redesign
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T17:38:05.214716740Z
---

## Goal

The baseline (mission atelier-jah8) left four things that do not affect its numbers and will affect the rerun after the cutover. Settle them before that rerun, so the two scorecards can be compared.

## Done when

- [ ] `docs/architecture/quality/scorecard-baseline/rubric.md` gives a test a second rater can apply for 'is this message about the mission' and for 'a state the agent already knew', and one boundary between a tracker-decision fault and a wrong statement in an ADR. The baseline labels stay as they are; the change is dated in the file. Interventions agreed at 73%, and review finding atelier-0l3c and `classified.md`'s limits name these three places.
- [ ] The command forms `scripts/scorecard/README.md` lists as recognised are checked against the commands agents run after the cutover (the redesigned CLI has other commands), on that run's frozen inputs, and the list and its tests are changed where a headline number moves.
- [ ] The done-item counts of `counted.md` and `defects.md` are reconciled or the difference is explained in the report: for atelier 305 against 316, demiurge 107 against 108, life-on-rails 51 against 52.
- [ ] `scripts/scorecard/README.md` says the rerun writes Python bytecode beside the scripts, gives a runtime that holds on a busy host (a validator measured 147 seconds against 'one to two minutes'), and its hash check covers `counted.md` as well as `counted.json`.

## Scope

The scripts, their README and the rubric. No baseline number changes. Mission atelier-295u's item atelier-8b0a also edits these scripts; do this with it or after it.

## Context

Source: the Reports of atelier-3kr7, atelier-5fsj and atelier-6sti, the mission notes of atelier-jah8, and its validation evidence atelier-vih6.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-jah8 (Measure defects after merge and what process costs, before the redesign changes either)
