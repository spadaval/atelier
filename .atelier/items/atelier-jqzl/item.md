---
schema: atelier.item
schema_version: 3
id: atelier-jqzl
title: 'Stop telling agents to run merge-ci-passed.sh where it cannot run: 76 runs failed with ''HEAD is not a merge commit'''
status: dropped
mission: atelier-icv4
labels:
- merge
- review
- bug
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T05:11:55.998123821Z
trivial: null
caused_by: null
created_at: 2026-11-09T05:11:55.998123821Z
---

## Goal

Two outputs tell agents to run `scripts/merge-ci-passed.sh` by hand, and it fails everywhere but inside `atelier merge`'s merge checkout: 76 runs ended "merge-ci-passed: HEAD is not a merge commit; this check runs only inside `atelier merge`'s merge checkout." Make every merge check Atelier tells an agent to run work where the agent runs it, and make a refused merge's Next a step that works.

- The review brief says "Run the proof yourself ... The merge checks are:" and lists the script. Reviewers ran it 58 times in 46 reviews and it failed, then ran `atelier merge --check` 44 times to see the gates instead.
- A merge refused by a failed merge check ends `Next: atelier merge <id> --sync, then scripts/merge-ci-passed.sh` (`gate_report.rs`, the `shell.check` arm). On the merged tree the check sees the base, so running it bare in the work checkout cannot reproduce it. The Next also drops the script's own advice, to push the branch and wait for CI. Orchestrators got 44 such refusals and ran the script by hand 13 times.

## Done when

- [ ] Every command the review brief lists as a merge check runs in the work branch's checkout and gives a pass or fail verdict there, not "HEAD is not a merge commit". For this repository's CI check, `scripts/merge-ci-passed.sh` judges the branch head against the base when HEAD is not a merge commit, giving the verdict merge would give. A scripted run on a branch with and without a passing CI run shows it.
- [ ] A merge refused by a failed merge check ends with a Next that works from the work checkout: the check's own advice when it printed one, then `atelier merge <id>`, which reruns the check on the merged tree. A gate report test covers it.

## Scope

`scripts/merge-ci-passed.sh` and `crates/atelier-cli/src/gate_report.rs`.

## Context

Evidence: transcripts 2026-11-04 to 2026-11-09. Failed script runs: Claude reviewers 36 in 32 sessions, Sol reviewers 22 in 14, orchestrators 13, implementers 5. atelier-fad5 (merged 2026-11-09) changed merge's success Next; this item changes the refusal's.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-fad5 (Let merge push the base it integrated, and stop recommending piped gates)

## Report

Dropped: `scripts/merge-ci-passed.sh` was deleted in 0.5; `atelier merge` waits for CI itself and names no script.
