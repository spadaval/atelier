---
schema: atelier.review
schema_version: 1
id: atelier-br5k
title: 'crates/atelier-cli/src/main.rs:302-305 (issue create --body help): the help enumerates the accepted sections as ''Goal and Done when (optionally Scope, Context, Unknowns, Design) for work items; Outcome for missions, plus Success criteria without a strategy'' and does not mention Background for either type. Agents learn the body shape from this help, so they will keep putting shared research into Context or a side document, which is the problem atelier-by8u targets. Fix: add Background to both the work-item optional list and the mission clause.'
target: atelier-by8u
identity: Independent reviewer (Claude subagent)
commit: 8d219563b31b3ca6885be30cdbc5c5b84ee204c4
created_at: 2026-10-16T00:53:59.031503759Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/main.rs:302-305 (issue create --body help): the help enumerates the accepted sections as 'Goal and Done when (optionally Scope, Context, Unknowns, Design) for work items; Outcome for missions, plus Success criteria without a strategy' and does not mention Background for either type. Agents learn the body shape from this help, so they will keep putting shared research into Context or a side document, which is the problem atelier-by8u targets. Fix: add Background to both the work-item optional list and the mission clause.
