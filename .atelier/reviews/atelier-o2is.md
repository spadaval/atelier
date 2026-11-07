---
schema: atelier.review
schema_version: 2
id: atelier-o2is
title: Prune apply does not hold an exclusive transaction and can leave partial record removals after a failure.
target: atelier-1ioo
identity: records-reviewer
commit: 8a4b850d103109db66e6672d1cd3391fb604e6ae
created_at: 2026-10-01T22:21:38.038230005Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: rebuild-orchestrator
    commit: c85e4455f6f4fdfd1f8b77cba80350b2b7f6ec7f
    recorded_at: 2026-10-01T22:21:38.101521843Z
    resolution_commit: a3675840c5b138bbef0b5c8f40d65fea8287e743
    reason: Repair committed; final independent acceptance and approval must cover this resolution.
    migrated_from: atelier-ig1q
---

Prune apply does not hold an exclusive transaction and can leave partial record removals after a failure.
