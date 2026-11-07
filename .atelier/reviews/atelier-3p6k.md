---
schema: atelier.review
schema_version: 2
id: atelier-3p6k
title: 'G12: Local log.showRoot=false hides root-introduced review history from introduction queries and accepts prior disposition identity rewrites. Independent fixture rejects with showRoot=true and passes with false without any canonical change. Immutable review introduction and original identity checks must include root commits irrespective of Git display configuration.'
target: atelier-1ioo
identity: session-reviewer
commit: 6f5188dd5bc990c88733a0027ee703eaaef9f737
created_at: 2026-10-02T00:50:23.319742925Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator
    commit: 5277b54c6b3a787a36bb927928ec1771218da0cb
    recorded_at: 2026-10-02T01:03:03.364604773Z
    resolution_commit: 7896770078f1fd4e0d38df4ed90199f7e664f69d
    reason: 'G12 independently verified: explicit root history protects review introductions and identity under log.showRoot=false while valid historical approvals pass.'
    migrated_from: atelier-490m
---

G12: Local log.showRoot=false hides root-introduced review history from introduction queries and accepts prior disposition identity rewrites. Independent fixture rejects with showRoot=true and passes with false without any canonical change. Immutable review introduction and original identity checks must include root commits irrespective of Git display configuration.
