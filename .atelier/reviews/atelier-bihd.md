---
schema: atelier.review
schema_version: 2
id: atelier-bihd
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:49.257517317Z
event:
  event: finding
  recommendation: fix_now
---

medium: The inbox can meet every serve-item Done when line while missing the one-click outcome.
.atelier/issues/atelier-zpmb.md:28,33-34 and docs/strategies/fleet-dashboard.md:33-37.
The inbox links each waiting item to its item page, and only a mission item page is required to link a debrief. Thus a debrief read takes at least two clicks. A needs-human validation such as atelier-3km4 is not itself a mission, so its item page is not even required to have the target debrief link. Its body names the mission the human must read. Specify a direct inbox action for the actual reading target, retaining an item-details link separately, and state how that target is identified without guessing from arbitrary prose. Add a fixture with a needs-human debrief-read item and assert the inbox action itself returns that mission's rendered debrief, not merely HTTP 200 for an intermediate record page.
Recommendation: FIX NOW; current item acceptance does not deliver the governing success criterion.
