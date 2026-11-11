---
schema: atelier.item
schema_version: 3
id: atelier-wl6u
title: Port cross-repo-regression and vacuous-test-review to Atelier fixtures, so they test the skill again
status: dropped
mission: null
labels:
- agent-factory
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-08T23:03:38.890123921Z
trivial: null
caused_by: null
created_at: 2026-11-08T23:03:38.890123921Z
---

## Goal

Since atelier-nz6a the skill steps aside when `atelier status` says "Not an Atelier repository". The fixtures of `cross-repo-regression` and `vacuous-test-review` are not Atelier repositories, so their skill arm now runs without the skill. In the nz6a matrix both still passed, but only because each run read `procedures/review.md` in the same command as `atelier status` before stepping aside.

## Done when

- [ ] Both fixtures are Atelier repositories (`atelier init` in `setup.sh`), keeping each scenario's pressure: cross-repo-regression's shared tracker and sibling repository, and vacuous-test-review's vacuous test and false "it failed first" claim.
- [ ] Each runs 3 times on the skill arm with Opus, read by hand, with no run stepping aside, and the results are recorded in `evals/agent-factory/results/`.

## Scope

`evals/agent-factory/scenarios/cross-repo-regression`, `evals/agent-factory/scenarios/vacuous-test-review` and the eval README.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-nz6a (Make the skill step aside outside Atelier repositories, and give its install lines first)

## Report

Dropped: Settled by atelier-a4rs: `next-vacuous-test-review` is the Atelier port, and `cross-repo-regression` was deleted at the cutover with the old tool's fixtures and not carried into the ten scenarios.
