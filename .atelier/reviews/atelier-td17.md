---
schema: atelier.review
schema_version: 2
id: atelier-td17
title: No test covers state-unknown marks in the Unassigned panel.
target: atelier-ihns
identity: Independent reviewer 3 (Claude subagent)
commit: adae1d6c1ed890f1775da3e1428012091755be5c
created_at: 2026-10-17T03:33:56.493816851Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: no_action
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: adae1d6c1ed890f1775da3e1428012091755be5c
    recorded_at: 2026-10-17T03:33:56.785605697Z
    reason: Same in_flight map and mark_line path as the tested ready-list state-unknown case; risk is small.
    migrated_from: atelier-nkrh
---

No test covers state-unknown marks in the Unassigned panel.
