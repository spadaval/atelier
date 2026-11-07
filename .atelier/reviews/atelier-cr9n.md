---
schema: atelier.review
schema_version: 2
id: atelier-cr9n
title: 'G10: The new cat-file --batch -Z invocation introduces an undocumented Git>=2.42 requirement. Independent valid-land reproduction with an extracted Git2.39.5 binary fails unknown switch Z, while host Git2.47 passes the same fixture. Preserve the documented Git prerequisite and exact binary object framing; no system installation was used for the reproduction.'
target: atelier-1ioo
identity: session-reviewer
commit: 6f5188dd5bc990c88733a0027ee703eaaef9f737
created_at: 2026-10-02T00:43:05.724449506Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator
    commit: 5277b54c6b3a787a36bb927928ec1771218da0cb
    recorded_at: 2026-10-02T01:03:03.197217198Z
    resolution_commit: 4a7c80ea54bc967770ec87f7ebf46c6255659b97
    reason: 'G10 independently verified: plain batch framing with raw-tree full-OID resolution preserves newline and binary paths and passes Git2.30 and2.39 fixtures.'
    migrated_from: atelier-asze
---

G10: The new cat-file --batch -Z invocation introduces an undocumented Git>=2.42 requirement. Independent valid-land reproduction with an extracted Git2.39.5 binary fails unknown switch Z, while host Git2.47 passes the same fixture. Preserve the documented Git prerequisite and exact binary object framing; no system installation was used for the reproduction.
