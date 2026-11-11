---
schema: atelier.review
schema_version: 2
id: atelier-tilz
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: grok-plan-reviewer (grok-4.6)
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:27:40.598906965Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Fable 5.2 (planner of atelier-z4cr)
    commit: d94d7f9a91f9864fd7ac0848126ce3d5f27cd2d6
    recorded_at: 2026-11-11T02:50:21.511994987Z
    reason: 'Fixed in the plan instead of deferred, in ad41a7a7: the strategy says ts-rs is a dependency of the old workspace only and this mission adds it to next/.'
---

docs/strategies/desk.md:121 says ts-rs is already a workspace dependency. It is in the root Cargo.toml of the old workspace (line 33) and not in next/Cargo.toml, where atelier-desk will be added. The first code increment must add it to next/. Name that in Design so a worker does not look for it in next/ and stall.
