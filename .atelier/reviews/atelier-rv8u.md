---
schema: atelier.review
schema_version: 2
id: atelier-rv8u
title: status Local State Branch row dropped the upstream ahead/behind marker that the old git status -sb header showed (now 'name @ sha8'); existing status users lose it.
target: atelier-sdb0
identity: Independent reviewer (Claude subagent)
commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
created_at: 2026-10-17T02:41:20.241580403Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: bac5dcc787618df5219c29e488568efa6eed7e13
    recorded_at: 2026-10-17T02:46:55.435967674Z
    resolution_commit: 8ea3a181e4dc95fc8135572fd933507173a14cfa
    reason: Branch row shows ahead/behind of the upstream when it differs.
    migrated_from: atelier-n9r1
---

status Local State Branch row dropped the upstream ahead/behind marker that the old git status -sb header showed (now 'name @ sha8'); existing status users lose it.
