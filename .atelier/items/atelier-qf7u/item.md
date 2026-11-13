---
schema: atelier.item
schema_version: 3
id: atelier-qf7u
title: Treat a repository with no item records as an empty population in the scorecard's sampler
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T18:02:40.545456404Z
---

## Goal

Taken from deferred finding atelier-x3n5/1-c6ma.3, in .atelier/items/atelier-x3n5/reviews/1-c6ma.md:

> Finding 3: Treat a repository with no item records as an empty population
> Recommendation: defer
>
> `scripts/scorecard/draw_samples.py:155` uses snapshot.git(), which sets subprocess check=True. Git grep exits 1 for no matches, so closed_missions() raises CalledProcessError whenever a manifest repository has no item records, even if that repository has no closed missions to sample. I reproduced this with a temporary repository containing only a mission record and a frozen log with its ready and close lines.
>
> An empty item population should return an empty membership map while actual Git failures still fail. This case is not exercised by the populated five-repository rerun, so it can follow separately. Add an empty-record fixture when handling it.

## Done when

- [ ] `closed_missions()` in `scripts/scorecard/draw_samples.py` returns an empty membership map for a repository with no item records, and still fails on any other Git error.
- [ ] A fixture with only a mission record covers it, and fails on the code before the fix.

## Scope

In: `scripts/scorecard/draw_samples.py` and its tests. Out: any other measure or script. This is a bug caused by atelier-x3n5; label it `bug` with `--caused-by atelier-x3n5` once atelier-0019 has merged, since `atelier set` runs only on the base.
