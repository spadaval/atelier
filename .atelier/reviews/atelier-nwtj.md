---
schema: atelier.review
schema_version: 2
id: atelier-nwtj
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:04.366379517Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/adr/0028-simpler-lifecycle.md:209 Replacing 'the reasoning of the rejected alternative Recompute Review Targets From Parent Hierarchy' goes beyond the strategy, which is silent on it, and pre-empts the question the ADR leaves open for atelier-md0v (line 152: what replaces the recorded base for a branch that targets a mission branch). It also conflicts with ADR 0016's new Status line, whose 'the rest stands' keeps that rejection. Fix: say only that the alternative's conclusion (the recorded base is the target) goes with the recorded base, and that whether targets are recomputed from the hierarchy is open under decision 5.
