---
schema: atelier.review
schema_version: 2
id: atelier-q2bh
title: issue.rs:243 passes a work-classification label to marked() as if it were a workflow category (works only because done/blocked share names); children print 'needs human' while the mission Work line prints 'needs a human'.
target: atelier-7u4o
identity: Independent reviewer (Claude subagent)
commit: 96654ae0441c66cbe31a04363578bfa207a9a058
created_at: 2026-10-17T02:42:49.401988461Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: e06d0456e69cd5113dfcaa16352192cbbd8e9ea1
    recorded_at: 2026-10-17T02:50:00.094986371Z
    resolution_commit: d2dd80301a5f571a57e21c4d68a14e0b72387bf9
    reason: Child marks come from the work classification; labels consistent.
    migrated_from: atelier-pk0l
---

issue.rs:243 passes a work-classification label to marked() as if it were a workflow category (works only because done/blocked share names); children print 'needs human' while the mission Work line prints 'needs a human'.
