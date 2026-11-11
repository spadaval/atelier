---
schema: atelier.item
schema_version: 3
id: atelier-pju0
title: Rebuild the rare-edge-chain eval fixture so its fix carries only the two planted findings
status: dropped
mission: null
labels:
- agent-factory
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T22:19:27.985044619Z
---

## Goal

Make `evals/agent-factory/scenarios/rare-edge-chain` (now in this repository) a fair test of whether an orchestrator leaves a deferred low finding alone. Its reviewed fix (`files/follower_fix.py`) keeps carrying unplanted defects. atelier-l58v found one regression (a touched, just-closed log took the session) and its correction (agents-skills 23a1ed2) added another: in `catch_up`, a J1 whose clock ran ahead and whose last lines are stamped later than now beats the game's log at restart. Reviewers in both arms of the rerun found further real cases where stamps are compared across logs after a clock correction. Every run then fixes or declines over those, so neither arm is tested on the target behaviour.

## Done when

- [ ] The fixture's fix has exactly the two planted findings (the `fix_now` regression and the rare idle case recommended defer), shown by probes for each, and a probe for every unplanted case named in atelier-l58v's reruns (`~/.cache/af-evals/runs/l58v-fq99-*`). Prefer a smaller fixture with no cross-log stamp comparison over patching this one again.
- [ ] rare-edge-chain runs 3 times with Opus at the current skill and at the commit before atelier-l58v, read by hand, and the results are recorded in `evals/agent-factory/results/`.

## Context

atelier-l58v's rare-edge-chain results were judged inconclusive: main 1/3 against new 2/3 on the first fixture, and 2/3 each after the first correction. A candidate second correction ("a log whose last line is stamped later than now does not win the comparison", about 11 lines in `follower_fix.py`) passed probes but was never committed or run.

## Links

Relationships of the old record that the new format has no field for.

- parent: atelier-jxce (Eval the skill for over-correction, so saying no never means skipping real fixes)
- discovered_from review: atelier-fq99

## Report

Dropped: The `rare-edge-chain` scenario was deleted at the cutover with the old tool's fixtures.
