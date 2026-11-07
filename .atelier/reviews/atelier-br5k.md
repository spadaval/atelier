---
schema: atelier.review
schema_version: 2
id: atelier-br5k
title: 'crates/atelier-cli/src/main.rs:302-305 (issue create --body help): the help enumerates the accepted sections as ''Goal and Done when (optionally Scope, Context, Unknowns, Design) for work items; Outcome for missions, plus Success criteria without a strategy'' and does not mention Background for either type. Agents learn the body shape from this help, so they will keep putting shared research into Context or a side document, which is the problem atelier-by8u targets. Fix: add Background to both the work-item optional list and the mission clause.'
target: atelier-by8u
identity: Independent reviewer (Claude subagent)
commit: 8d219563b31b3ca6885be30cdbc5c5b84ee204c4
created_at: 2026-10-16T00:53:59.031503759Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 3df28a5f40d2468ee052e4935daafaec8a0c0829
    recorded_at: 2026-10-16T00:55:47.437834545Z
    resolution_commit: 3df28a5f40d2468ee052e4935daafaec8a0c0829
    reason: Fixed in 3df28a5f40d2468ee052e4935daafaec8a0c0829
    migrated_from: atelier-nj1t
---

crates/atelier-cli/src/main.rs:302-305 (issue create --body help): the help enumerates the accepted sections as 'Goal and Done when (optionally Scope, Context, Unknowns, Design) for work items; Outcome for missions, plus Success criteria without a strategy' and does not mention Background for either type. Agents learn the body shape from this help, so they will keep putting shared research into Context or a side document, which is the problem atelier-by8u targets. Fix: add Background to both the work-item optional list and the mission clause.
