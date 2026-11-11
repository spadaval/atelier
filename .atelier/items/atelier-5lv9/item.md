---
schema: atelier.item
schema_version: 3
id: atelier-5lv9
title: Give the light-path review brief the moving-part rule and Verdict/Sizes Report line atelier-fnlu added to the item brief
status: dropped
mission: null
labels:
- agent-factory
- review
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-09T22:10:10.298438603Z
---

## Goal

The brief `atelier review request` prints for a change with no issue (`crates/atelier-cli/src/commands/light_path.rs`) asks reviewers the same questions as the item brief. atelier-fnlu added a working rule (size the change, ask which requirement needs each moving part it adds) and a Report line asking for Verdict and Sizes to the item brief only, since its scope named `review_request.rs`. The light-path brief still asks only for Correctness and Fitness.

## Done when

- [ ] The light-path brief carries the moving-part working rule, with the commit messages standing in for Done when as the requirements, and its Report line asks for Verdict and Sizes; a light-path brief test covers both lines.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-fnlu (Have reviewers flag code no requirement needs: 1 shrink verdict and no overbuilt-code findings in 1,252 reviews)

## Report

Dropped: The light path and `light_path.rs` were removed in 0.5; `start --new` and `merge --trivial` cover it.
