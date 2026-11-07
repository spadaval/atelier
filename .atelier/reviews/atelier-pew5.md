---
schema: atelier.review
schema_version: 2
id: atelier-pew5
title: 'ancestry_path (git_freshness.rs:714) costs one pass per date-order inversion along the path, so its worst case is quadratic. Normal histories converge in 2 passes, one marking and one confirming: this repository and a 200k-commit linear chain both took 2 (91 ms for 200k). But rev-list without --topo-order lists in commit-date order. If a side branch''s committer dates fall along the chain (each commit dated before its parent), every parent is listed before its child, so the backwards pass marks one commit per pass. Scratch repro (fast-import): a k-commit skewed side chain merged into a main line one commit at a time. k=2000 took 2,001 passes and 0.34 s. k=8000 took 8,001 passes and 6.4 s, where git rev-list --ancestry-path takes 0.42 s. Answers stay correct. Edge ranges (pin..claim) are usually short and such skew is rare, so this does not block. A linear fix that keeps the same answers: build a parent-to-children map from the output and run a worklist from the pin. The least fixed point is exactly the set reachable forward from the pin. A skewed-dates case could join the parity test.'
target: atelier-v5tt
identity: Independent reviewer (Claude subagent)
commit: 3841028c54e10d40fe6d109c806768be33cf0b2c
created_at: 2026-10-16T22:08:59.894848489Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 5cfd2a84faa7e5282b01ae9fae77119cdd59876a
    recorded_at: 2026-10-16T22:09:31.593549592Z
    follow_up: atelier-7b6e
    reason: 'Deferred to atelier-7b6e: build the parent-to-children map and walk forward from the pin, with a date-skewed parity case.'
    migrated_from: atelier-c8x0
---

ancestry_path (git_freshness.rs:714) costs one pass per date-order inversion along the path, so its worst case is quadratic. Normal histories converge in 2 passes, one marking and one confirming: this repository and a 200k-commit linear chain both took 2 (91 ms for 200k). But rev-list without --topo-order lists in commit-date order. If a side branch's committer dates fall along the chain (each commit dated before its parent), every parent is listed before its child, so the backwards pass marks one commit per pass. Scratch repro (fast-import): a k-commit skewed side chain merged into a main line one commit at a time. k=2000 took 2,001 passes and 0.34 s. k=8000 took 8,001 passes and 6.4 s, where git rev-list --ancestry-path takes 0.42 s. Answers stay correct. Edge ranges (pin..claim) are usually short and such skew is rare, so this does not block. A linear fix that keeps the same answers: build a parent-to-children map from the output and run a worklist from the pin. The least fixed point is exactly the set reachable forward from the pin. A skewed-dates case could join the parity test.
