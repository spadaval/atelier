---
schema: atelier.review
schema_version: 2
id: atelier-f8p3
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: grok-plan-reviewer (grok-4.6)
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:27:39.609377201Z
event:
  event: finding
  recommendation: fix_now
---

The plan claims the Desk and cutover missions do not wait on each other (docs/strategies/desk.md:148-151, .atelier/issues/atelier-z4cr.md:27), but .atelier/issues/atelier-va99.md:19-20 is blocked_by atelier-wd1i, and the mission Outcome plus desk-matches-cli can only hold on live converted repositories after that cutover. An orchestrator cannot finish atelier-z4cr until qr5s's host cutover runs, a stall Authority does not cover. Fix: either move live install (va99) and desk-matches-cli out as a loose end so this mission completes on scratch clones, or retract the independence claim and say the one-way wait, with what the orchestrator does if wd1i has not run.
