---
schema: atelier.review
schema_version: 2
id: atelier-81on
title: No test covers an item owned on several branches (the '; ' join) or an unreadable branch.
target: atelier-7n5u
identity: Independent reviewer (Claude subagent)
commit: 2414e9cb784631ee77c0d7d81c3fab5ff5552bff
created_at: 2026-10-17T03:13:08.584823569Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: de052831e12bf26494833849a691583b7428ab8d
    recorded_at: 2026-10-17T03:17:36.092363334Z
    resolution_commit: 21a4320007af41457ede1b2886d7e2455da24b12
    reason: Tests for multi-branch ownership and unreadable branches.
    migrated_from: atelier-zfdp
---

No test covers an item owned on several branches (the '; ' join) or an unreadable branch.
