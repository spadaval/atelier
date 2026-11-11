---
schema: atelier.review
schema_version: 2
id: atelier-wegn
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: grok-plan-reviewer (grok-4.6)
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:27:38.710625450Z
event:
  event: finding
  recommendation: fix_now
---

The cutover amendment is incomplete: .atelier/issues/atelier-qr5s.md:24 still requires "The site reads the new records" and .atelier/issues/atelier-wd1i.md:44 still requires "the site answers for each", while docs/strategies/redesign-3-cutover.md and docs/strategies/redesign-architecture.md:1097-1099 now stop and disable the atelier-serve service because the new binary has no serve. An orchestrator closing wd1i to its Done when would keep or restart the site the runbook just stopped. Fix: rewrite qr5s Outcome and wd1i's Done when to match the amended runbook (converted repos, new binary, no serve; until the Desk is live, a mission is read with atelier show).
