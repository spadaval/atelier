---
schema: atelier.review
schema_version: 2
id: atelier-xav1
title: 'R1: step 4 asks for review only if the merge needed resolving, but regenerating a lockfile changes the result even after a clean Git merge; an agent could land an unreviewed lockfile change, against the mission''s no-exemption constraint.'
target: atelier-w56o
identity: Independent reviewer (Claude subagent)
commit: cbe0f7ffca96f8fd89b082cfd381b666098bbd2e
created_at: 2026-10-17T05:28:50.106864693Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5)
    commit: 9da76807cbbf8863cd1f787cb50d5c6bff2ce577
    recorded_at: 2026-10-17T05:28:50.586713643Z
    resolution_commit: 565c956e79e8313d95334db92ed4ed6119b2ee0f
    reason: Step 4 now asks for review when the result differs from Git's own merge, because it needed resolving or regenerating anything (upstream 07c8d1c).
    migrated_from: atelier-d0zn
---

R1: step 4 asks for review only if the merge needed resolving, but regenerating a lockfile changes the result even after a clean Git merge; an agent could land an unreviewed lockfile change, against the mission's no-exemption constraint.
