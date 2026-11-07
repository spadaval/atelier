---
schema: atelier.review
schema_version: 2
id: atelier-sbm8
title: finding on atelier-r9hb @59ae63e0
target: atelier-r9hb
identity: independent planning reviewer subagent (claude-opus-5-5)
commit: 59ae63e0fc0faa110587261b677874c04e938ec0
created_at: 2026-10-30T06:47:10.909425672Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: planner (claude-opus-5-5, atelier-ll3p author)
    commit: 298bd742a940e9add66c3b95a579760fb26c7e47
    recorded_at: 2026-10-30T06:48:52.317676618Z
    resolution_commit: 298bd742a940e9add66c3b95a579760fb26c7e47
    reason: Fixed in the planning revision
    migrated_from: atelier-30u7
---

R2: Goal and Scope claim a mission's success criteria are covered ('or a mission's success criteria'), but the trigger cannot fire for them and the Done when never tests it. Findings are rarely recorded on a mission, and the failing receipt that a rewritten criterion answers to sits on the validation item, not 'on that item' (e.g. a validator's fail on S-n, then the mission's criterion reworded). A worker can ship detection that never fires for missions and pass every listed test. Fix: either drop the mission clause (and keep criterion review-history-shown item-only, as it is), or define the mission trigger (criteria changed after a failing receipt on any item advancing the mission) and add a test for it. Also define 'later' by Git ancestry (edit commit descends from the finding's or receipt's pinned commit), not record times, since the debrief already mixes clocks (atelier-b3qt). Consider: most cited narrowings (gp9x, wv9x, vy5i, 6bjl) were by the implementer before or without a recorded finding; triggering on any Done when edit after the item's start commit would catch them with a simpler rule.
