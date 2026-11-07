---
schema: atelier.review
schema_version: 2
id: atelier-9b77
title: 'R2: ''land branches that change the same files back to back, so only one absorbs the other''s changes'' gives a reason that does not hold once each branch merges the base only when next: order does not change the number of conflicted merges.'
target: atelier-w56o
identity: Independent reviewer (Claude subagent)
commit: cbe0f7ffca96f8fd89b082cfd381b666098bbd2e
created_at: 2026-10-17T05:28:50.196710680Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5)
    commit: 9da76807cbbf8863cd1f787cb50d5c6bff2ce577
    recorded_at: 2026-10-17T05:28:50.743219638Z
    resolution_commit: 565c956e79e8313d95334db92ed4ed6119b2ee0f
    reason: Removed the back-to-back rule upstream (07c8d1c) and from the item's done-when, with a dated note; mission S2 does not include it.
    migrated_from: atelier-m4fo
---

R2: 'land branches that change the same files back to back, so only one absorbs the other's changes' gives a reason that does not hold once each branch merges the base only when next: order does not change the number of conflicted merges.
