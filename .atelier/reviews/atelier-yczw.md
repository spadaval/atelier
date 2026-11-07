---
schema: atelier.review
schema_version: 2
id: atelier-yczw
title: Merge checks can change base checkout ownership; ignored uncommitted approvals are accepted; shell checks leak into transitions; direct done claims omit required fields.
target: atelier-1ioo
identity: gates-reviewer
commit: a3675840c5b138bbef0b5c8f40d65fea8287e743
created_at: 2026-10-01T22:12:25.192163910Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: rebuild-orchestrator
    commit: e12ecb4b7f9c4d0914c3677168b1d7fe972cc377
    recorded_at: 2026-10-01T22:12:25.258910935Z
    resolution_commit: 0708528e58c392cb305943db3b175893c7c0bdea
    reason: Repaired and independently re-reviewed; the final independent approval must cover this resolution.
    migrated_from: atelier-2cmu
---

Merge checks can change base checkout ownership; ignored uncommitted approvals are accepted; shell checks leak into transitions; direct done claims omit required fields.
