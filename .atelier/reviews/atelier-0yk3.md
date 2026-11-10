---
schema: atelier.review
schema_version: 2
id: atelier-0yk3
title: finding on atelier-fcb7 @378eb60a
target: atelier-fcb7
identity: grok-4.6-plan-r2
commit: 378eb60a7695bdea855a47332bda78b1218b3445
created_at: 2026-11-10T01:23:31.596270304Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: 5e9fca9f80ac5da997cfe133e9231a31c33b8823
    recorded_at: 2026-11-10T01:30:01.324857806Z
    reason: 'Fixed in 65c404a0 although recommended defer: the fix was a line in each item.'
---

medium atelier-yptz: the planning-commands scenario takes a mission through close while one of two items stays draft, but close in the act table requires every item done or dropped. An implementer who follows the preliminary Done when will weaken close. Smallest fix: the scenario drops the leftover draft, then closes. Fix when step B is planned.
