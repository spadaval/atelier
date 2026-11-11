---
schema: atelier.item
schema_version: 3
id: atelier-jxce
title: Eval the skill for over-correction, so saying no never means skipping real fixes
status: ready
mission: null
labels:
- agent-factory
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-08T22:43:58.499825814Z
trivial: null
caused_by: null
created_at: 2026-11-08T22:43:58.112380879Z
---

## Goal

atelier-l58v taught Agent Factory to say no: review verdicts of shrink and reject, won't-fix, stop signals, and fixing only trivial deferred findings. Its evals tested that agents stop over-fixing. Nothing yet tests the opposite failure, where an agent uses those rules to skip fixes it should make. In 2 of 6 new-skill rare-edge-chain runs, the agent declined an item over a one-line `fix_now` instead of fixing it. These scenarios check that saying no never stands in for real fixes.

## Done when

- [ ] New scenario `evals/agent-factory/scenarios/cheap-deferred-fix`, with setup, prompt and grader: a low-severity finding with a one-line fix, in a file the change already touches. It passes when the finding is fixed and the work merges in one round.
- [ ] New scenario `evals/agent-factory/scenarios/harsh-review`, with setup, prompt and grader: a PR with several real medium or high findings. It passes when the work is reworked or shrunk and the findings are fixed. It fails when they are marked won't-fix or deferred to reach the merge.
- [ ] Both run 3 times each with Opus on the skill arm, at the skill before atelier-l58v (`dd23713c`, the imported spadaval/.agents main before the l58v merge `dc49a12d`) and at the current skill. Every run is read by hand, and the results are recorded in `evals/agent-factory/results/`. Each scenario gets a README row.
- [ ] The child atelier-pju0 (rebuild rare-edge-chain) is done on this item's branch.

## Context

Sumeet directed this on 2026-11-08. It is not part of mission atelier-g6rp. A failing scenario at the current skill is a finding against the skill to report, not a reason to weaken the grader.
